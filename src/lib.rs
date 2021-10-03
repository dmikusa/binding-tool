use std::ffi::OsString;
use std::io::prelude::*;
use std::{env, fs, path};

use anyhow::{anyhow, Result};
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, Values};

/// Parse application arguments
///
/// ### Examples
///
/// Basic: Single parameter without a name
///
/// ```
/// let args = binding_tools::parse_args(vec!["bt", "-t", "binding", "-p", "foo=bar"]);
///
/// assert_eq!(args.value_of("TYPE").unwrap(), "binding");
///
/// let params:Vec<_> = args.values_of("PARAM").unwrap().collect();
/// assert_eq!(params, vec!["foo=bar"]);
///
/// assert_eq!(args.value_of("NAME"), None);
/// ```
///
/// More Advanced: Multiple parameters and a name
///
/// ```
/// let args = binding_tools::parse_args(vec!["bt", "-t", "binding", "-n", "better_name", "-p", "foo=bar", "-p", "gorilla=banana"]);
///
/// assert_eq!(args.value_of("TYPE").unwrap(), "binding");
///
/// let params:Vec<_> = args.values_of("PARAM").unwrap().collect();
/// assert_eq!(params, vec!["foo=bar", "gorilla=banana"]);
///
/// assert_eq!(args.value_of("NAME").unwrap(), "better_name");
/// ```
///
pub fn parse_args<'a, I, T>(args: I) -> clap::ArgMatches<'a>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    return App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .after_help(include_str!("help/additional_help.txt"))
        .arg(
            Arg::with_name("TYPE")
                .short("t")
                .long("type")
                .value_name("type")
                .help("type of binding")
                .required(true),
        )
        .arg(
            Arg::with_name("PARAM")
                .short("p")
                .long("param")
                .value_name("key=val")
                .multiple(true)
                .required(true)
                .help("key/value to set for the type"),
        )
        .arg(
            Arg::with_name("NAME")
                .short("n")
                .long("name")
                .value_name("name")
                .required(false)
                .help("optional name for the binding, name defaults to the type"),
        )
        .get_matches_from(args);
}

pub struct BindingProcessor<'a> {
    bindings_home: &'a str,
    binding_type: &'a str,
    binding_name: Option<&'a str>,
}

impl<'a> BindingProcessor<'a> {
    pub fn new(
        bindings_home: &'a str,
        binding_type: &'a str,
        binding_name: Option<&'a str>,
    ) -> BindingProcessor<'a> {
        BindingProcessor {
            bindings_home,
            binding_type,
            binding_name,
        }
    }

    pub fn process_bindings(
        self: &BindingProcessor<'a>,
        binding_key_vals: Values<'a>,
    ) -> Result<()> {
        for binding_key_val in binding_key_vals.clone() {
            self.process_binding(binding_key_val)?;
        }

        Ok(())
    }

    fn process_binding<S: AsRef<str>>(
        self: &BindingProcessor<'a>,
        binding_key_val: S,
    ) -> Result<()> {
        let binding_path = path::Path::new(self.bindings_home)
            .join(self.binding_name.unwrap_or(self.binding_type));

        fs::create_dir_all(&binding_path)?;

        // TODO: prompt instead of blindly overwriting files
        let mut type_file = fs::File::create(&binding_path.join("type"))?;
        type_file.write_all(self.binding_type.as_bytes())?;

        if let Some((binding_key, binding_value)) = binding_key_val.as_ref().split_once("=") {
            let binding_key_path = binding_path.join(binding_key);

            if binding_value.starts_with("@") {
                fs::copy(binding_value.trim_start_matches("@"), binding_key_path)?;
            } else {
                let mut binding_file = fs::File::create(binding_key_path)?;
                binding_file.write_all(binding_value.as_bytes())?;
            }

            Ok(())
        } else {
            Err(anyhow!(
                "could not parse key/value -> {}",
                binding_key_val.as_ref()
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_binding_args_it_creates_binding() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmppath = tmpdir.path().to_string_lossy();

        let bp = BindingProcessor::new(&tmppath, "testType", None);
        let res = bp.process_binding("key=val");

        assert!(res.is_ok());
        assert!(tmpdir.path().join("testType/type").exists());
        assert!(tmpdir.path().join("testType/key").exists());

        let data = fs::read(tmpdir.path().join("testType/type"));
        assert!(data.is_ok());
        assert_eq!(data.unwrap(), b"testType");

        let data = fs::read(tmpdir.path().join("testType/key"));
        assert!(data.is_ok());
        assert_eq!(data.unwrap(), b"val");
    }

    #[test]
    fn given_binding_args_with_name_it_creates_binding_using_name() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmppath = tmpdir.path().to_string_lossy();

        let bp = BindingProcessor::new(&tmppath, "testType", Some("diff-name"));
        let res = bp.process_binding("key=val");

        assert!(res.is_ok());
        assert!(tmpdir.path().join("diff-name/type").exists());
        assert!(tmpdir.path().join("diff-name/key").exists());

        let data = fs::read(tmpdir.path().join("diff-name/type"));
        assert!(data.is_ok());
        assert_eq!(data.unwrap(), b"testType");

        let data = fs::read(tmpdir.path().join("diff-name/key"));
        assert!(data.is_ok());
        assert_eq!(data.unwrap(), b"val");
    }

    #[test]
    fn given_binding_args_with_value_relative_file_creates_binding_using_file_contents() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmppath = tmpdir.path().to_string_lossy();

        let res = fs::write(tmpdir.path().join("val"), "actual value");
        assert!(res.is_ok());

        let res = env::set_current_dir(&tmpdir);
        assert!(res.is_ok());

        let bp = BindingProcessor::new(&tmppath, "testType", None);
        let res = bp.process_binding("key=@val");

        assert!(res.is_ok(), "{}", res.unwrap_err());
        assert!(tmpdir.path().join("testType/type").exists());
        assert!(tmpdir.path().join("testType/key").exists());

        let data = fs::read(tmpdir.path().join("testType/type"));
        assert!(data.is_ok());
        assert_eq!(data.unwrap(), b"testType");

        let data = fs::read(tmpdir.path().join("testType/key"));
        assert!(data.is_ok());
        assert_eq!(data.unwrap(), b"actual value");
    }

    #[test]
    fn given_binding_args_with_value_full_file_path_creates_binding_using_file_contents() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmppath = tmpdir.path().to_string_lossy();

        let res = env::set_current_dir(&tmpdir);
        assert!(res.is_ok());

        let res = fs::create_dir_all(tmpdir.path().join("test"));
        assert!(res.is_ok());

        let val_path = tmpdir.path().join("test/val");
        let res = fs::write(tmpdir.path().join("test/val"), "actual value");
        assert!(res.is_ok());

        let bp = BindingProcessor::new(&tmppath, "testType", None);
        let res = bp.process_binding(format!("key=@{}", val_path.to_string_lossy()));

        assert!(res.is_ok(), "{}", res.unwrap_err());
        assert!(tmpdir.path().join("testType/type").exists());
        assert!(tmpdir.path().join("testType/key").exists());

        let data = fs::read(tmpdir.path().join("testType/type"));
        assert!(data.is_ok());
        assert_eq!(data.unwrap(), b"testType");

        let data = fs::read(tmpdir.path().join("testType/key"));
        assert!(data.is_ok());
        assert_eq!(data.unwrap(), b"actual value");
    }
}
