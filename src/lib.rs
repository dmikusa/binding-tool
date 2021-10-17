use std::ffi::OsString;
use std::io::{prelude::*, stdin};
use std::{env, fs, path, str};

use anyhow::{anyhow, bail, ensure, Context, Result};
use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches, SubCommand,
    Values,
};

pub struct Parser<'a, 'b> {
    app: clap::App<'a, 'b>,
}

impl<'a, 'b> Parser<'a, 'b> {
    /// Parse application arguments
    ///
    /// ### Examples
    ///
    /// Basic: Add a single parameter without a name
    ///
    /// ```
    /// let args = binding_tool::Parser::new().parse_args(vec!["bt", "add", "-t", "binding", "-p", "foo=bar"]);
    /// let cmd = args.subcommand_matches("add").unwrap();
    ///
    /// assert_eq!(cmd.value_of("TYPE").unwrap(), "binding");
    ///
    /// let params:Vec<_> = cmd.values_of("PARAM").unwrap().collect();
    /// assert_eq!(params, vec!["foo=bar"]);
    /// assert_eq!(cmd.value_of("NAME"), None);
    /// ```
    ///
    /// More Advanced: Add with multiple parameters and a name
    ///
    /// ```
    /// let args = binding_tool::Parser::new().parse_args(vec!["bt", "-f", "add", "-n", "better_name", "-t", "binding", "-p", "foo=bar", "-p", "gorilla=banana"]);
    /// let cmd = args.subcommand_matches("add").unwrap();
    ///
    /// assert_eq!(cmd.value_of("TYPE").unwrap(), "binding");
    ///
    /// let params:Vec<_> = cmd.values_of("PARAM").unwrap().collect();
    /// assert_eq!(params, vec!["foo=bar", "gorilla=banana"]);
    /// assert_eq!(cmd.value_of("NAME").unwrap(), "better_name");
    /// assert_eq!(args.is_present("FORCE"), true);
    /// ```
    ///
    /// Basic: Delete an entire binding
    ///
    /// ```
    /// let args = binding_tool::Parser::new().parse_args(vec!["bt", "delete", "-t", "binding"]);
    /// let cmd = args.subcommand_matches("delete").unwrap();
    ///
    /// assert_eq!(cmd.value_of("TYPE").unwrap(), "binding");
    /// assert!(cmd.values_of("PARAM").is_none());
    /// assert_eq!(cmd.value_of("NAME"), None);
    /// ```
    ///
    /// More Advanced: Delete parts of a binding
    ///
    /// ```
    /// let args = binding_tool::Parser::new().parse_args(vec!["bt", "-f", "delete", "-n", "better_name", "-t", "binding", "-p", "foo=bar"]);
    /// let cmd = args.subcommand_matches("delete").unwrap();
    ///
    /// assert_eq!(cmd.value_of("TYPE").unwrap(), "binding");
    ///
    /// let params:Vec<_> = cmd.values_of("PARAM").unwrap().collect();
    /// assert_eq!(params, vec!["foo=bar"]);
    /// assert_eq!(cmd.value_of("NAME").unwrap(), "better_name");
    /// assert_eq!(args.is_present("FORCE"), true);
    /// ```
    ///
    /// Convenience: add ca-certificates
    ///
    /// ```
    /// let args = binding_tool::Parser::new().parse_args(vec!["bt", "-f", "ca-certs", "-n", "my-certs", "-c", "/path/to/ca.crt"]);
    /// let cmd = args.subcommand_matches("ca-certs").unwrap();
    ///
    ///
    /// let certs:Vec<_> = cmd.values_of("CERT").unwrap().collect();
    /// assert_eq!(certs, vec!["/path/to/ca.crt"]);
    /// assert_eq!(cmd.value_of("NAME").unwrap(), "my-certs");
    /// assert_eq!(args.is_present("FORCE"), true);
    /// ```
    ///
    /// Convenience: add dependency-mappings
    ///
    /// ```
    /// let args = binding_tool::Parser::new().parse_args(vec!["bt", "dependency-mapping", "-n", "my-deps", "-f", "/path/to/file.zip"]);
    /// let cmd = args.subcommand_matches("dependency-mapping").unwrap();
    ///
    /// let files:Vec<_> = cmd.values_of("FILE").unwrap().collect();
    /// assert_eq!(files, vec!["/path/to/file.zip"]);
    /// assert_eq!(cmd.value_of("NAME").unwrap(), "my-deps");
    /// assert_eq!(cmd.is_present("FORCE"), false);
    /// ```
    ///
    /// Convenience: add dependency-mappings from buildpack
    ///
    /// ```
    /// let args = binding_tool::Parser::new().parse_args(vec!["bt", "dependency-mapping", "-b", "buildpack/id-1:v1.0.1", "-b", "buildpack/id-2:v2.1.0"]);
    /// let cmd = args.subcommand_matches("dependency-mapping").unwrap();
    ///
    /// let bps:Vec<_> = cmd.values_of("BUILDPACK").unwrap().collect();
    /// assert_eq!(bps, vec!["buildpack/id-1:v1.0.1", "buildpack/id-2:v2.1.0"]);
    /// assert_eq!(cmd.is_present("FORCE"), false);
    /// ```
    ///
    /// Convenience: add arguments for docker run
    ///
    /// ```
    /// let args = binding_tool::Parser::new().parse_args(vec!["bt", "args", "-d"]);
    /// let cmd = args.subcommand_matches("args").unwrap();
    ///
    /// assert_eq!(cmd.is_present("DOCKER"), true);
    /// assert_eq!(cmd.is_present("PACK"), false);
    /// ```
    ///
    /// Convenience: add arguments for pack build
    ///
    /// ```
    /// let args = binding_tool::Parser::new().parse_args(vec!["bt", "args", "-p"]);
    /// let cmd = args.subcommand_matches("args").unwrap();
    ///
    /// assert_eq!(cmd.is_present("DOCKER"), false);
    /// assert_eq!(cmd.is_present("PACK"), true);
    /// ```
    ///
    pub fn parse_args<I, T>(self, args: I) -> clap::ArgMatches<'a>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        self.app.get_matches_from(args)
    }

    pub fn new() -> Parser<'a, 'b> {
        Parser {
            app: App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .arg(
                Arg::with_name("FORCE")
                    .short("f")
                    .long("force")
                    .takes_value(false)
                    .help("force update if key exists"),
            )
            .subcommand(
                SubCommand::with_name("add")
                    .alias("a")
                    .arg(
                        Arg::with_name("NAME")
                            .short("n")
                            .long("name")
                            .value_name("name")
                            .required(false)
                            .help("optional name for the binding,\nname defaults to the type"),
                    )
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
                    .about("Add or modify a binding")
                    .after_help(include_str!("help/additional_help.txt")),
            )
            .subcommand(
                SubCommand::with_name("delete")
                    .alias("d")
                    .arg(
                        Arg::with_name("NAME")
                            .short("n")
                            .long("name")
                            .value_name("name")
                            .required_unless("TYPE")
                            .help("optional name for the binding,\nname defaults to the type"),
                    )
                    .arg(
                        Arg::with_name("TYPE")
                            .short("t")
                            .long("type")
                            .value_name("type")
                            .required_unless("NAME")
                            .help("type of binding")
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("PARAM")
                            .short("p")
                            .long("param")
                            .value_name("key=val")
                            .multiple(true)
                            .required(false)
                            .help("key/value to set for the type"),
                    )
                    .about("Delete a binding")
                    .after_help(include_str!("help/additional_help.txt")),
            )
            .subcommand(
                SubCommand::with_name("ca-certs")
                    .alias("cc")
                    .arg(
                        Arg::with_name("NAME")
                            .short("n")
                            .long("name")
                            .value_name("name")
                            .required(false)
                            .help("optional name for the binding,\nname defaults to the type"),
                    )
                    .arg(
                        Arg::with_name("CERT")
                            .short("c")
                            .long("cert")
                            .value_name("cert")
                            .required(true)
                            .multiple(true)
                            .help("path to a CA certificate to add"),
                    )
                    .about("Convenience for adding `ca-certificates` bindings")
                    .after_help(include_str!("help/additional_help.txt")),
            )
            .subcommand(
                SubCommand::with_name("dependency-mapping")
                    .alias("dm")
                    .arg(
                        Arg::with_name("NAME")
                            .short("n")
                            .long("name")
                            .value_name("name")
                            .required(false)
                            .help("optional name for the binding,\nname defaults to the type"),
                    )
                    .arg(
                        Arg::with_name("FILE")
                            .short("f")
                            .long("file")
                            .value_name("file")
                            .multiple(true)
                            .conflicts_with("BUILDPACK")
                            .help("path to local dependency file"),
                    )
                    .arg(
                        Arg::with_name("BUILDPACK")
                            .short("b")
                            .long("buildpack")
                            .value_name("buildpack")
                            .multiple(true)
                            .conflicts_with("FILE")
                            .help("buildpack ID from which dependencies will be loaded"),
                    )
                    .about("Convenience for adding `dependency-mapping` bindings")
                    .after_help(include_str!("help/additional_help.txt")),
            )
            .subcommand(
                SubCommand::with_name("args")
                    .arg(
                        Arg::with_name("DOCKER")
                            .short("d")
                            .long("docker")
                            .takes_value(false)
                            .conflicts_with("PACK")
                            .help("generates binding args for `docker run`"),
                    )
                    .arg(
                        Arg::with_name("PACK")
                            .short("p")
                            .long("pack")
                            .takes_value(false)
                            .conflicts_with("DOCKER")
                            .help("generates binding args for `pack build`"),
                    )
                    .about(
                        "Convenience that generates binding args for `pack build` and `docker run`",
                    )
                    .after_help(include_str!("help/additional_help.txt")),
            )
        }
    }
}

fn service_binding_root() -> String {
    // binding root = SERVICE_BINDING_ROOT (or default to "./bindings")
    match env::var("SERVICE_BINDING_ROOT") {
        Ok(root) => root,
        Err(_) => env::current_dir()
            .unwrap()
            .join("bindings")
            .to_str()
            .unwrap()
            .into(),
    }
}

trait BindingConfirmer {
    fn confirm(&self) -> bool;
}

pub enum BindingConfirmers {
    Console,
    Always,
    Never,
}

struct ConsoleBindingConfirmer {}

impl BindingConfirmer for ConsoleBindingConfirmer {
    fn confirm(&self) -> bool {
        println!("The binding alread exists, do you wish to continue? (yes or no)");

        let mut input: String = String::new();
        let res = stdin().lock().read_line(&mut input);
        let input = input.trim().to_lowercase();
        res.is_ok() && (input == "y" || input == "yes")
    }
}

struct AlwaysBindingConfirmer {}

impl BindingConfirmer for AlwaysBindingConfirmer {
    fn confirm(&self) -> bool {
        true
    }
}

struct NeverBindingConfirmer {}

impl BindingConfirmer for NeverBindingConfirmer {
    fn confirm(&self) -> bool {
        false
    }
}

pub struct BindingProcessor<'a> {
    bindings_home: &'a str,
    binding_type: Option<&'a str>,
    binding_name: Option<&'a str>,
    confirmer: BindingConfirmers,
}

impl<'a> BindingProcessor<'a> {
    pub fn new(
        bindings_home: &'a str,
        binding_type: Option<&'a str>,
        binding_name: Option<&'a str>,
        confirmer: BindingConfirmers,
    ) -> BindingProcessor<'a> {
        BindingProcessor {
            bindings_home,
            binding_type,
            binding_name,
            confirmer,
        }
    }

    pub fn delete_bindings(
        self: &BindingProcessor<'a>,
        _binding_key_vals: Option<Values<'a>>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn add_bindings(
        self: &BindingProcessor<'a>,
        binding_key_vals: Option<Values<'a>>,
    ) -> Result<()> {
        for binding_key_val in binding_key_vals.unwrap() {
            self.add_binding(binding_key_val)?;
        }

        Ok(())
    }

    fn add_binding<S: AsRef<str>>(self: &BindingProcessor<'a>, binding_key_val: S) -> Result<()> {
        ensure!(
            self.binding_type.is_some(),
            "binding type is required when adding a binding"
        );
        let binding_type = self.binding_type.unwrap();
        let binding_path =
            path::Path::new(self.bindings_home).join(self.binding_name.unwrap_or(binding_type));

        if let Some((binding_key, binding_value)) = binding_key_val.as_ref().split_once("=") {
            let writer = BindingWriter::new(binding_path, binding_type, binding_key, binding_value);

            if writer.binding_key_path().exists() {
                let result = match &self.confirmer {
                    BindingConfirmers::Always => AlwaysBindingConfirmer {}.confirm(),
                    BindingConfirmers::Never => NeverBindingConfirmer {}.confirm(),
                    BindingConfirmers::Console => ConsoleBindingConfirmer {}.confirm(),
                };
                anyhow::ensure!(result, "binding already exists");
            }

            writer.write()
        } else {
            Err(anyhow!(
                "could not parse key/value -> {}",
                binding_key_val.as_ref()
            ))
        }
    }
}

struct BindingWriter<'a, P> {
    path: P,
    b_type: &'a str,
    key: &'a str,
    value: &'a str,
}

impl<'a, P> BindingWriter<'a, P>
where
    P: AsRef<path::Path>,
{
    fn new(path: P, b_type: &'a str, key: &'a str, value: &'a str) -> BindingWriter<'a, P> {
        BindingWriter {
            path,
            b_type,
            key,
            value,
        }
    }

    fn binding_key_path(&self) -> path::PathBuf {
        self.path.as_ref().join(self.key)
    }

    fn write(&self) -> Result<()> {
        fs::create_dir_all(self.path.as_ref())
            .with_context(|| format!("{}", self.path.as_ref().to_string_lossy()))?;

        self.write_type()?;

        if self.value.starts_with('@') {
            self.write_key_as_file()?;
        } else {
            self.write_key_as_value()?;
        }

        Ok(())
    }

    fn write_type(&self) -> Result<()> {
        let mut type_file = fs::File::create(self.path.as_ref().join("type"))
            .with_context(|| "cannot open type file")?;
        type_file
            .write_all(self.b_type.as_bytes())
            .with_context(|| "cannot write the type file")
    }

    fn write_key_as_file(&self) -> Result<u64> {
        let src = self.value.trim_start_matches('@');
        let src_path = path::Path::new(src)
            .canonicalize()
            .with_context(|| format!("cannot canonicalize path to source file: {}", src))?;
        fs::copy(&src_path, self.binding_key_path()).with_context(|| {
            format!(
                "failed to copy {} to {}",
                src_path.to_string_lossy(),
                self.binding_key_path().to_string_lossy()
            )
        })
    }

    fn write_key_as_value(&self) -> Result<()> {
        let mut binding_file = fs::File::create(self.binding_key_path()).with_context(|| {
            format!(
                "cannot open binding key path: {}",
                self.binding_key_path().to_string_lossy()
            )
        })?;
        binding_file
            .write_all(self.value.as_bytes())
            .with_context(|| {
                format!(
                    "cannot write to binding key path: {}",
                    self.binding_key_path().to_string_lossy()
                )
            })
    }
}

pub trait CommandHandler<'a> {
    fn handle(&self, args: Option<&ArgMatches>) -> Result<()>;
}

pub enum Command {
    Add(AddCommandHandler),
    Delete(DeleteCommandHandler),
    CaCerts(CaCertsCommandHandler),
    DependencyMapping(DependencyMappingCommandHandler),
    Args(ArgsCommandHandler),
}

impl str::FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "add" => Ok(Command::Add(AddCommandHandler {})),
            "delete" => Ok(Command::Delete(DeleteCommandHandler {})),
            "ca-certs" => Ok(Command::CaCerts(CaCertsCommandHandler {})),
            "dependency-mapping" => Ok(Command::DependencyMapping(
                DependencyMappingCommandHandler {},
            )),
            "args" => Ok(Command::Args(ArgsCommandHandler {})),
            _ => bail!("could not part argument"),
        }
    }
}

pub struct AddCommandHandler {}

impl<'a> CommandHandler<'a> for AddCommandHandler {
    fn handle(&self, args: Option<&ArgMatches>) -> Result<()> {
        ensure!(args.is_some(), "missing required args");
        let args = args.unwrap();

        let binding_key_vals = args.values_of("PARAM");
        ensure!(
            binding_key_vals.is_some(),
            "binding parameter (key=val) is required"
        );

        let binding_type = args.value_of("TYPE");
        let binding_name = args.value_of("NAME");
        let bindings_home = service_binding_root();

        let confirmer = if args.is_present("FORCE") {
            BindingConfirmers::Always
        } else {
            BindingConfirmers::Console
        };

        // process bindings
        let btp = BindingProcessor::new(&bindings_home, binding_type, binding_name, confirmer);
        btp.add_bindings(binding_key_vals)
    }
}

pub struct DeleteCommandHandler {}

impl<'a> CommandHandler<'a> for DeleteCommandHandler {
    fn handle(&self, args: Option<&ArgMatches>) -> Result<()> {
        ensure!(args.is_some(), "missing required args");
        let args = args.unwrap();

        // one of these is required (it's not OK to unwrap)
        let binding_type = args.value_of("TYPE");
        let binding_name = args.value_of("NAME");

        // not required (it's not OK to unwrap)
        let binding_key_vals = args.values_of("PARAM");

        // binding root = SERVICE_BINDING_ROOT (or default to "./bindings")
        let bindings_home = service_binding_root();

        let confirmer = if args.is_present("FORCE") {
            BindingConfirmers::Never
        } else {
            BindingConfirmers::Console
        };

        // process bindings
        let btp = BindingProcessor::new(&bindings_home, binding_type, binding_name, confirmer);
        btp.delete_bindings(binding_key_vals)
    }
}

pub struct CaCertsCommandHandler {}

impl<'a> CommandHandler<'a> for CaCertsCommandHandler {
    fn handle(&self, _args: Option<&ArgMatches>) -> Result<()> {
        Ok(())
    }
}

pub struct DependencyMappingCommandHandler {}

impl<'a> CommandHandler<'a> for DependencyMappingCommandHandler {
    fn handle(&self, _args: Option<&ArgMatches>) -> Result<()> {
        Ok(())
    }
}

pub struct ArgsCommandHandler {}

impl<'a> CommandHandler<'a> for ArgsCommandHandler {
    fn handle(&self, _args: Option<&ArgMatches>) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_binding_args_it_creates_binding() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmppath = tmpdir.path().to_string_lossy();

        let bp = BindingProcessor::new(&tmppath, Some("testType"), None, BindingConfirmers::Never);
        let res = bp.add_binding("key=val");

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
    fn given_duplicate_binding_key_it_doesnt_overwrite_binding() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmppath = tmpdir.path().to_string_lossy();

        let bp1 = BindingProcessor::new(&tmppath, Some("testType"), None, BindingConfirmers::Never);
        let res = bp1.add_binding("key=val");

        assert!(res.is_ok());
        assert!(tmpdir.path().join("testType/type").exists());
        assert!(tmpdir.path().join("testType/key").exists());

        let bp1 = BindingProcessor::new(&tmppath, Some("testType"), None, BindingConfirmers::Never);
        let res = bp1.add_binding("key=other_val");
        assert!(res.is_err());

        let data = fs::read(tmpdir.path().join("testType/type"));
        assert!(data.is_ok());
        assert_eq!(data.unwrap(), b"testType");

        let data = fs::read(tmpdir.path().join("testType/key"));
        assert!(data.is_ok());
        assert_eq!(data.unwrap(), b"val");
    }

    #[test]
    fn given_duplicate_binding_but_different_key_adds_key_to_binding() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmppath = tmpdir.path().to_string_lossy();

        let bp1 = BindingProcessor::new(&tmppath, Some("testType"), None, BindingConfirmers::Never);
        let res = bp1.add_binding("key=val");

        assert!(res.is_ok());
        assert!(tmpdir.path().join("testType/type").exists());
        assert!(tmpdir.path().join("testType/key").exists());

        let bp1 = BindingProcessor::new(&tmppath, Some("testType"), None, BindingConfirmers::Never);
        let res = bp1.add_binding("other_key=other_val");
        assert!(res.is_ok());
        assert!(tmpdir.path().join("testType/other_key").exists());

        let data = fs::read(tmpdir.path().join("testType/type"));
        assert!(data.is_ok());
        assert_eq!(data.unwrap(), b"testType");

        let data = fs::read(tmpdir.path().join("testType/other_key"));
        assert!(data.is_ok());
        assert_eq!(data.unwrap(), b"other_val");
    }

    #[test]
    fn given_duplicate_binding_and_same_key_confirm_updates_key() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmppath = tmpdir.path().to_string_lossy();

        let bp1 = BindingProcessor::new(&tmppath, Some("testType"), None, BindingConfirmers::Never);
        let res = bp1.add_binding("key=val");

        assert!(res.is_ok());
        assert!(tmpdir.path().join("testType/type").exists());
        assert!(tmpdir.path().join("testType/key").exists());

        let bp1 =
            BindingProcessor::new(&tmppath, Some("testType"), None, BindingConfirmers::Always);
        let res = bp1.add_binding("key=new_val");
        assert!(res.is_ok());
        assert!(tmpdir.path().join("testType/key").exists());

        let data = fs::read(tmpdir.path().join("testType/type"));
        assert!(data.is_ok());
        assert_eq!(data.unwrap(), b"testType");

        let data = fs::read(tmpdir.path().join("testType/key"));
        assert!(data.is_ok());
        assert_eq!(data.unwrap(), b"new_val");
    }

    #[test]
    fn given_binding_args_with_name_it_creates_binding_using_name() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmppath = tmpdir.path().to_string_lossy();

        let bp = BindingProcessor::new(
            &tmppath,
            Some("testType"),
            Some("diff-name"),
            BindingConfirmers::Never,
        );
        let res = bp.add_binding("key=val");

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

        let bp = BindingProcessor::new(&tmppath, Some("testType"), None, BindingConfirmers::Never);
        let res = bp.add_binding("key=@val");

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

        let bp = BindingProcessor::new(&tmppath, Some("testType"), None, BindingConfirmers::Never);
        let res = bp.add_binding(format!("key=@{}", val_path.to_string_lossy()));

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
