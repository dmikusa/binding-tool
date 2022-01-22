use std::ffi::OsString;
use std::io::{prelude::*, stdin, Stdout};
use std::{env, fs, path, str};

use anyhow::{anyhow, bail, ensure, Context, Result};
use clap::{app_from_crate, App, Arg, ArgGroup, ArgMatches};

pub struct Parser<'a> {
    app: clap::App<'a>,
}

impl<'a, 'b> Parser<'a> {
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
    /// let args = binding_tool::Parser::new().parse_args(vec!["bt", "delete", "-n", "binding"]);
    /// let cmd = args.subcommand_matches("delete").unwrap();
    ///
    /// assert_eq!(cmd.value_of("NAME").unwrap(), "binding");
    /// ```
    ///
    /// More Advanced: Delete parts of a binding
    ///
    /// ```
    /// let args = binding_tool::Parser::new().parse_args(vec!["bt", "-f", "delete", "-n", "better_name", "-k", "foo"]);
    /// let cmd = args.subcommand_matches("delete").unwrap();
    ///
    /// let params:Vec<_> = cmd.values_of("KEY").unwrap().collect();
    /// assert_eq!(params, vec!["foo"]);
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
    /// Convenience: don't set the type of args and fails
    ///
    /// ```
    /// let res = binding_tool::Parser::new().try_parse_args(vec!["bt", "args"]);
    /// assert!(res.is_err(), "should require a argument");
    /// ```
    ///
    pub fn parse_args<I, T>(self, args: I) -> clap::ArgMatches
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        self.app.get_matches_from(args)
    }

    pub fn try_parse_args<I, T>(self, args: I) -> clap::Result<clap::ArgMatches>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        self.app.try_get_matches_from(args)
    }

    pub fn new() -> Parser<'a> {
        Parser {
            app: app_from_crate!()
            .arg(
                Arg::new("FORCE")
                    .short('f')
                    .long("force")
                    .takes_value(false)
                    .help("force update if key exists"),
            )
            .subcommand(
                App::new("add")
                    .alias("a")
                    .arg(
                        Arg::new("NAME")
                            .short('n')
                            .long("name")
                            .value_name("name")
                            .required(false)
                            .help("optional name for the binding,\nname defaults to the type"),
                    )
                    .arg(
                        Arg::new("TYPE")
                            .short('t')
                            .long("type")
                            .value_name("type")
                            .help("type of binding")
                            .required(true),
                    )
                    .arg(
                        Arg::new("PARAM")
                            .short('p')
                            .long("param")
                            .value_name("key=val")
                            .multiple_occurrences(true)
                            .required(true)
                            .help("key/value to set for the type"),
                    )
                    .about("Add or modify a binding")
                    .after_help( include_str!("help/additional_help_param.txt")),
            )
            .subcommand(
                App::new("delete")
                    .alias("d")
                    .arg(
                        Arg::new("NAME")
                            .short('n')
                            .long("name")
                            .value_name("name")
                            .required(true)
                            .help("name for the binding"),
                    )
                    .arg(
                        Arg::new("KEY")
                            .short('k')
                            .long("key")
                            .value_name("key")
                            .multiple_occurrences(true)
                            .required(false)
                            .help("specific key to delete"),
                    )
                    .about("Delete a binding")
                    .after_help(include_str!("help/additional_help_binding.txt")),
            )
            .subcommand(
                App::new("ca-certs")
                    .alias("cc")
                    .arg(
                        Arg::new("NAME")
                            .short('n')
                            .long("name")
                            .value_name("name")
                            .required(false)
                            .help("optional name for the binding,\nname defaults to the type"),
                    )
                    .arg(
                        Arg::new("CERT")
                            .short('c')
                            .long("cert")
                            .value_name("cert")
                            .required(true)
                            .multiple_occurrences(true)
                            .help("path to a CA certificate to add"),
                    )
                    .about("Convenience for adding `ca-certificates` bindings")
                    .after_help(include_str!("help/additional_help_binding.txt")),
            )
            .subcommand(
                App::new("dependency-mapping")
                    .alias("dm")
                    .arg(
                        Arg::new("NAME")
                            .short('n')
                            .long("name")
                            .value_name("name")
                            .required(false)
                            .help("optional name for the binding,\nname defaults to the type"),
                    )
                    .arg(
                        Arg::new("FILE")
                            .short('f')
                            .long("file")
                            .value_name("file")
                            .multiple_occurrences(true)
                            .conflicts_with("BUILDPACK")
                            .help("path to local dependency file"),
                    )
                    .arg(
                        Arg::new("BUILDPACK")
                            .short('b')
                            .long("buildpack")
                            .value_name("buildpack")
                            .multiple_occurrences(true)
                            .conflicts_with("FILE")
                            .help("buildpack ID from which dependencies will be loaded"),
                    )
                    .about("Convenience for adding `dependency-mapping` bindings")
                    .after_help(include_str!("help/additional_help_binding.txt")),
            )
            .subcommand(
                App::new("args")
                    .arg(
                        Arg::new("DOCKER")
                            .short('d')
                            .long("docker")
                            .takes_value(false)
                            .conflicts_with("PACK")
                            .help("generates binding args for `docker run`"),
                    )
                    .arg(
                        Arg::new("PACK")
                            .short('p')
                            .long("pack")
                            .takes_value(false)
                            .conflicts_with("DOCKER")
                            .help("generates binding args for `pack build`"),
                    )
                    .group(
                        ArgGroup::new("TYPES")
                            .args(&["DOCKER", "PACK"])
                            .required(true)
                    )
                    .about(
                        "Convenience that generates binding args for `pack build` and `docker run`",
                    )
                    .after_help(include_str!("help/additional_help_binding.txt")),
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

    pub fn delete_bindings<I: Iterator<Item = &'a str> + Clone>(
        self: &BindingProcessor<'a>,
        binding_keys: I,
    ) -> Result<()> {
        let root = path::Path::new(self.bindings_home);
        ensure!(root.is_dir(), "bindings home must be a directory");

        let binding_path = path::Path::new(self.bindings_home).join(self.binding_name.unwrap());

        for binding_key in binding_keys.clone() {
            let binding_key_path = binding_path.join(binding_key);
            if binding_key_path.exists() {
                fs::remove_file(binding_key_path)?;
            }
        }

        if binding_keys.count() == 0 {
            fs::remove_dir_all(binding_path)?
        }

        Ok(())
    }

    pub fn add_bindings<I: Iterator<Item = &'a str>>(
        self: &BindingProcessor<'a>,
        binding_key_vals: I,
    ) -> Result<()> {
        for binding_key_val in binding_key_vals {
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
    fn handle(&mut self, args: Option<&ArgMatches>) -> Result<()>;
}

pub enum Command {
    Add(AddCommandHandler),
    Delete(DeleteCommandHandler),
    CaCerts(CaCertsCommandHandler),
    DependencyMapping(DependencyMappingCommandHandler),
    Args(ArgsCommandHandler<Stdout>),
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
            "args" => Ok(Command::Args(ArgsCommandHandler {
                output: std::io::stdout(),
            })),
            _ => bail!("could not part argument"),
        }
    }
}

pub struct AddCommandHandler {}

impl<'a> CommandHandler<'a> for AddCommandHandler {
    fn handle(&mut self, args: Option<&ArgMatches>) -> Result<()> {
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
        btp.add_bindings(binding_key_vals.unwrap())
    }
}

pub struct DeleteCommandHandler {}

impl<'a> CommandHandler<'a> for DeleteCommandHandler {
    fn handle(&mut self, args: Option<&ArgMatches>) -> Result<()> {
        ensure!(args.is_some(), "missing required args");
        let args = args.unwrap();

        // required (it's OK to unwrap)
        let binding_name = args.value_of("NAME");
        ensure!(binding_name.is_some(), "binding name is required");

        // not required, but OK to use default (empty iterator)
        let binding_key_vals = args.values_of("PARAM").unwrap_or_default();

        // binding root = SERVICE_BINDING_ROOT (or default to "./bindings")
        let bindings_home = service_binding_root();

        let confirmer = if args.is_present("FORCE") {
            BindingConfirmers::Never
        } else {
            BindingConfirmers::Console
        };

        // process bindings
        let btp = BindingProcessor::new(&bindings_home, None, binding_name, confirmer);
        btp.delete_bindings(binding_key_vals)
    }
}

pub struct CaCertsCommandHandler {}

impl<'a> CommandHandler<'a> for CaCertsCommandHandler {
    fn handle(&mut self, _args: Option<&ArgMatches>) -> Result<()> {
        todo!()
    }
}

pub struct DependencyMappingCommandHandler {}

impl<'a> CommandHandler<'a> for DependencyMappingCommandHandler {
    fn handle(&mut self, _args: Option<&ArgMatches>) -> Result<()> {
        todo!()
    }
}

pub struct ArgsCommandHandler<T> {
    output: T,
}

impl<'a, T> CommandHandler<'a> for ArgsCommandHandler<T>
where
    T: Write,
{
    fn handle(&mut self, args: Option<&ArgMatches>) -> Result<()> {
        ensure!(args.is_some(), "missing required args");
        let args = args.unwrap();

        // binding root = SERVICE_BINDING_ROOT (or default to "./bindings")
        let bindings_root = service_binding_root();
        let bindings_home = path::Path::new(&bindings_root);

        if !bindings_home.exists() {
            return Ok(());
        }

        let binding_count = bindings_home
            .read_dir()?
            .filter_map(|res| res.ok())
            .filter(|entry| entry.path().is_dir() && entry.path().join("type").exists())
            .count();
        if binding_count == 0 {
            return Ok(());
        }

        match (args.is_present("DOCKER"), args.is_present("PACK")) {
            (false, true) => write!(
                self.output,
                r#"--volume "{}:/binding" --env SERVICE_BINDING_ROOT=/bindings"#,
                bindings_root
            )?,
            (true, false) => write!(
                self.output,
                r#"--volume "{}:/binding" --env SERVICE_BINDING_ROOT=/bindings"#,
                bindings_root
            )?,
            _ => bail!("cannot have both docker and pack flags"),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::Utf8Error;

    use super::*;

    pub struct TestBuffer {
        buffer: Vec<u8>,
    }

    impl Write for TestBuffer {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buffer.write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.buffer.flush()
        }
    }

    impl TestBuffer {
        pub fn new() -> TestBuffer {
            TestBuffer { buffer: vec![] }
        }

        pub fn writer(&mut self) -> &mut impl Write {
            &mut self.buffer
        }

        pub fn string(&self) -> Result<&str, Utf8Error> {
            str::from_utf8(&self.buffer)
        }
    }

    #[test]
    fn given_no_bindings_root_set_it_returns_current_working_directory() {
        let root = super::service_binding_root();
        assert!(root.starts_with(env::current_dir().unwrap().to_str().unwrap()));
    }

    #[test]
    fn given_bindings_root_set_it_returns_bindings_root_dir() {
        temp_env::with_var("SERVICE_BINDING_ROOT", Some("/bindings"), || {
            let root = super::service_binding_root();
            assert!(root.starts_with("/bindings"));
        });
    }

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

        let cur_dir = env::current_dir();
        assert!(res.is_ok());

        let res = env::set_current_dir(&tmpdir);
        assert!(res.is_ok());

        let bp = BindingProcessor::new(&tmppath, Some("testType"), None, BindingConfirmers::Never);
        let res = bp.add_binding("key=@val");

        {
            let res = env::set_current_dir(cur_dir.unwrap());
            assert!(res.is_ok());
        }

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

    #[test]
    fn given_binding_it_deletes_the_binding() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmppath = tmpdir.path().to_string_lossy();

        let bp = BindingProcessor::new(
            &tmppath,
            Some("some-type"),
            Some("diff-name"),
            BindingConfirmers::Never,
        );
        let res = bp.add_binding("key=val");

        assert!(res.is_ok());
        assert!(tmpdir.path().join("diff-name/type").exists());
        assert!(tmpdir.path().join("diff-name/key").exists());

        let tmp: Vec<&str> = vec![];
        let res = bp.delete_bindings(tmp.into_iter());
        assert!(res.is_ok());
        assert!(!tmpdir.path().join("diff-name/type").exists());
        assert!(!tmpdir.path().join("diff-name/key").exists());
    }

    #[test]
    fn given_binding_and_key_it_deletes_the_specific_binding_key_only() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmppath = tmpdir.path().to_string_lossy();

        let bp = BindingProcessor::new(
            &tmppath,
            Some("some-type"),
            Some("diff-name"),
            BindingConfirmers::Never,
        );
        let res = bp.add_binding("key1=val1");
        assert!(res.is_ok());

        let res = bp.add_binding("key2=val2");
        assert!(res.is_ok());

        assert!(tmpdir.path().join("diff-name/type").exists());
        assert!(tmpdir.path().join("diff-name/key1").exists());
        assert!(tmpdir.path().join("diff-name/key2").exists());

        let tmp: Vec<&str> = vec!["key1"];
        let res = bp.delete_bindings(tmp.into_iter());
        assert!(res.is_ok());
        assert!(tmpdir.path().join("diff-name/type").exists());
        assert!(!tmpdir.path().join("diff-name/key1").exists());
        assert!(tmpdir.path().join("diff-name/key2").exists());
    }

    #[test]
    fn given_a_binding_args_outputs() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmppath = tmpdir.path().to_string_lossy();

        temp_env::with_var("SERVICE_BINDING_ROOT", Some(tmpdir.as_ref()), || {
            // make some bindings, required

            let bp = BindingProcessor::new(
                &tmppath,
                Some("some-type"),
                Some("diff-name"),
                BindingConfirmers::Never,
            );
            let res = bp.add_binding("key1=val1");
            assert!(res.is_ok());

            // check args
            let args = Parser::new().parse_args(vec!["bt", "args", "--docker"]);
            let cmd = args.subcommand_matches("args").unwrap();
            let mut tb = TestBuffer::new();
            let res = ArgsCommandHandler {
                output: tb.writer(),
            }
            .handle(Some(cmd));
            assert!(res.is_ok(), "args handler should succeed");
            assert_eq!(
                tb.string().unwrap(),
                format!(
                    r#"--volume "{}:/binding" --env SERVICE_BINDING_ROOT=/bindings"#,
                    tmppath
                )
            );
        });
    }

    #[test]
    fn write_to_test_buffer() {
        struct Junk<'t, T>
        where
            T: Write,
        {
            output: &'t mut T,
        }

        impl<'t, T> Junk<'t, T>
        where
            T: Write,
        {
            pub fn do_stuff(&mut self) {
                write!(self.output, "Hello World!").unwrap();
            }
        }

        let mut tb = TestBuffer::new();
        let mut j = Junk {
            output: tb.writer(),
        };
        j.do_stuff();
        assert_eq!(tb.string().unwrap(), "Hello World!");

        let mut buf = std::io::stdout();
        let mut j = Junk { output: &mut buf };
        j.do_stuff();
    }
}
