// Copyright 2022-Present the original author or authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::{anyhow, Context, Result};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, prelude::*};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{path, thread};
use toml::Value as Toml;
use url::Url;

#[derive(Clone)]
pub(super) struct Dependency {
    pub(super) sha256: String,
    pub(super) uri: String,
}

impl Dependency {
    pub(super) fn filename(&self) -> Result<String> {
        Url::parse(&self.uri)?
            .path_segments()
            .ok_or_else(|| anyhow!("no path segments for {}", &self.uri))
            .map(|s| {
                s.last()
                    .map(|s| s.to_owned())
                    .ok_or_else(|| anyhow!("no path for {}", &self.uri))
            })?
    }

    pub(super) fn checksum_matches(&self, binding_path: &path::Path) -> Result<bool> {
        let dest = binding_path.join("binaries").join(self.filename()?);
        if !dest.exists() {
            return Ok(false);
        }

        let mut fp = File::open(&dest).with_context(|| format!("cannot open file {dest:?}"))?;

        let mut hasher = Sha256::new();
        io::copy(&mut fp, &mut hasher)?;
        let hash = hex::encode(hasher.finalize());

        Ok(hash == self.sha256)
    }

    pub(super) fn download(&self, agent: &ureq::Agent, binding_path: &path::Path) -> Result<()> {
        if self.checksum_matches(binding_path)? {
            return Ok(());
        }

        let dest = binding_path.join("binaries").join(self.filename()?);
        let mut fp = File::create(&dest).with_context(|| format!("cannot open file {dest:?}"))?;

        let mut reader = agent.get(&self.uri).call()?.into_reader();

        std::io::copy(&mut reader, &mut fp).with_context(|| "copy failed")?;
        Ok(())
    }
}

pub(super) fn parse_buildpack_toml_from_disk(path: &path::Path) -> Result<Vec<Dependency>> {
    let mut input = String::new();

    File::open(path)
        .and_then(|mut f| f.read_to_string(&mut input))
        .unwrap();

    transform(input.parse()?)
}

pub(super) fn parse_buildpack_toml_from_network(buildpack: &str) -> Result<Vec<Dependency>> {
    let uri = format!("https://raw.githubusercontent.com/{buildpack}/main/buildpack.toml");

    let agent = configure_agent()?;
    let res = agent
        .get(&uri)
        .call()
        .with_context(|| format!("failed on url {uri}"))?
        .into_string()
        .with_context(|| format!("failed on url {uri}"))?;

    transform(res.parse()?)
}

pub(super) fn download_dependencies(
    deps: Vec<Dependency>,
    binding_path: path::PathBuf,
) -> Result<()> {
    let max_simult: usize = std::env::var("BT_MAX_SIMULTANEOUS")
        .unwrap_or_else(|_| String::from("5"))
        .parse()?;

    let agent = Arc::new(configure_agent()?);
    let binding_path = Arc::new(binding_path);
    let deps = Arc::new(Mutex::new(deps));

    let mut join_handles: Vec<JoinHandle<_>> = vec![];

    for _i in 0..max_simult {
        let agent = Arc::clone(&agent);
        let binding_path = Arc::clone(&binding_path);
        let deps = Arc::clone(&deps);

        join_handles.push(thread::spawn(move || {
            while let Some(d) = deps.lock().expect("unable to get lock").pop() {
                match d.download(&agent, &binding_path) {
                    Ok(_) => (),
                    Err(err) => panic!("Download of {} failed with error {}", d.uri, err),
                }
            }
        }))
    }

    for handle in join_handles {
        if let Err(err) = handle.join() {
            if let Ok(msg) = err.downcast::<String>() {
                return Err(anyhow!("thread panic: {}", msg));
            }
        }
    }

    Ok(())
}

fn configure_agent() -> Result<ureq::Agent> {
    let conn_timeout: u64 = std::env::var("BT_CONN_TIMEOUT")
        .unwrap_or_else(|_| String::from("5"))
        .parse()?;

    let timeout: u64 = std::env::var("BT_REQ_TIMEOUT")
        .unwrap_or_else(|_| String::from("30"))
        .parse()?;

    Ok(ureq::builder()
        .timeout_connect(std::time::Duration::from_secs(conn_timeout))
        .timeout(std::time::Duration::from_secs(timeout))
        .build())
}

fn transform(toml: Toml) -> Result<Vec<Dependency>> {
    let bp_toml = toml
        .as_table()
        .with_context(|| "buildpack.toml format is invalid")?;

    let metadata = bp_toml
        .get("metadata")
        .with_context(|| "no metadata present in buildpack.toml")?
        .as_table()
        .with_context(|| "metadata should be a table")?;

    let deps_metadata = metadata
        .get("dependencies")
        .with_context(|| "no dependencies present")?
        .as_array()
        .with_context(|| "dependencies should be an array")?;

    let mut deps = vec![];

    for d in deps_metadata {
        let table = d
            .as_table()
            .with_context(|| "dependency should be a table")?;

        let sha256 = table
            .get("sha256")
            .with_context(|| "sha256 field is required")?
            .as_str()
            .with_context(|| "sha256 should be a string")?
            .to_owned();
        let uri = table
            .get("uri")
            .with_context(|| "uri field is required")?
            .as_str()
            .with_context(|| "uri should be a string")?
            .to_owned();

        deps.push(Dependency { sha256, uri })
    }

    Ok(deps)
}

#[cfg(test)]
mod tests {
    use super::{transform, Dependency};

    #[test]
    fn dependency_filename() {
        assert_eq!(
            "filename",
            Dependency {
                sha256: "".into(),
                uri: "https://example.com/filename".into(),
            }
            .filename()
            .unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "no path segments for")]
    fn dependency_filename_no_path() {
        assert_eq!(
            "filename",
            Dependency {
                sha256: "".into(),
                uri: "data:text/plain,HelloWorld".into(),
            }
            .filename()
            .unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "no metadata present in buildpack.toml")]
    fn transform_no_metadata() {
        transform(toml::from_str(r#"foo = "bar""#).unwrap()).unwrap();
    }

    #[test]
    #[should_panic(expected = "metadata should be a table")]
    fn transform_metadata_not_a_table() {
        transform(toml::from_str(r#"metadata = "bar""#).unwrap()).unwrap();
    }

    #[test]
    #[should_panic(expected = "no dependencies present")]
    fn transform_metadata_not_dependency() {
        transform(
            toml::from_str(
                r#"[[metadata.configurations]]
                    foo = "bar""#,
            )
            .unwrap(),
        )
        .unwrap();
    }

    #[test]
    #[should_panic(expected = "dependencies should be an array")]
    fn transform_metadata_dependencies_should_be_an_array() {
        transform(
            toml::from_str(
                r#"[metadata]
                    dependencies = "foo""#,
            )
            .unwrap(),
        )
        .unwrap();
    }

    #[test]
    #[should_panic(expected = "dependency should be a table")]
    fn transform_metadata_dependency_should_be_a_table() {
        transform(
            toml::from_str(
                r#"[metadata]
                    dependencies = [1, 2, 3]"#,
            )
            .unwrap(),
        )
        .unwrap();
    }

    #[test]
    #[should_panic(expected = "sha256 field is required")]
    fn transform_metadata_dependency_should_have_an_sha256() {
        transform(
            toml::from_str(
                r#"[[metadata.dependencies]]
                    foo = "bar""#,
            )
            .unwrap(),
        )
        .unwrap();
    }

    #[test]
    #[should_panic(expected = "sha256 should be a string")]
    fn transform_metadata_dependency_sha256_should_be_str() {
        transform(
            toml::from_str(
                r#"[[metadata.dependencies]]
                    sha256 = 1"#,
            )
            .unwrap(),
        )
        .unwrap();
    }

    #[test]
    #[should_panic(expected = "uri field is required")]
    fn transform_metadata_dependency_should_have_an_uri() {
        transform(
            toml::from_str(
                r#"[[metadata.dependencies]]
                    sha256 = "sha256"
                    foo = "bar""#,
            )
            .unwrap(),
        )
        .unwrap();
    }

    #[test]
    #[should_panic(expected = "uri should be a string")]
    fn transform_metadata_dependency_uri_should_be_str() {
        transform(
            toml::from_str(
                r#"[[metadata.dependencies]]
                    sha256 = "sha256"
                    uri = 1"#,
            )
            .unwrap(),
        )
        .unwrap();
    }
}
