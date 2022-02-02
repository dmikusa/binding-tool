use clap::{app_from_crate, App, Arg, ArgGroup};
use std::ffi::OsString;

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
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "add", "-t", "binding", "-p", "foo=bar"]);
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
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "add", "-f", "-n", "better_name", "-t", "binding", "-p", "foo=bar", "-p", "gorilla=banana"]);
    /// let cmd = args.subcommand_matches("add").unwrap();
    ///
    /// assert_eq!(cmd.value_of("TYPE").unwrap(), "binding");
    ///
    /// let params:Vec<_> = cmd.values_of("PARAM").unwrap().collect();
    /// assert_eq!(params, vec!["foo=bar", "gorilla=banana"]);
    /// assert_eq!(cmd.value_of("NAME").unwrap(), "better_name");
    /// assert_eq!(cmd.is_present("FORCE"), true);
    /// ```
    ///
    /// Basic: Delete an entire binding
    ///
    /// ```
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "delete", "-n", "binding"]);
    /// let cmd = args.subcommand_matches("delete").unwrap();
    ///
    /// assert_eq!(cmd.value_of("NAME").unwrap(), "binding");
    /// ```
    ///
    /// More Advanced: Delete parts of a binding
    ///
    /// ```
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "delete", "-f", "-n", "better_name", "-k", "foo"]);
    /// let cmd = args.subcommand_matches("delete").unwrap();
    ///
    /// let keys:Vec<_> = cmd.values_of("KEY").unwrap().collect();
    /// assert_eq!(keys, vec!["foo"]);
    /// assert_eq!(cmd.value_of("NAME").unwrap(), "better_name");
    /// assert_eq!(cmd.is_present("FORCE"), true);
    /// ```
    ///
    /// Convenience: add ca-certificates
    ///
    /// ```
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "ca-certs", "-f", "-n", "my-certs", "-c", "/path/to/ca.crt"]);
    /// let cmd = args.subcommand_matches("ca-certs").unwrap();
    ///
    ///
    /// let certs:Vec<_> = cmd.values_of("CERT").unwrap().collect();
    /// assert_eq!(certs, vec!["/path/to/ca.crt"]);
    /// assert_eq!(cmd.value_of("NAME").unwrap(), "my-certs");
    /// assert_eq!(cmd.is_present("FORCE"), true);
    /// ```
    ///
    /// Convenience: add dependency-mappings
    ///
    /// ```
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "dependency-mapping", "-n", "my-deps", "-t", "/path/to/file.zip"]);
    /// let cmd = args.subcommand_matches("dependency-mapping").unwrap();
    ///
    /// let files:Vec<_> = cmd.values_of("TOML").unwrap().collect();
    /// assert_eq!(files, vec!["/path/to/file.zip"]);
    /// assert_eq!(cmd.value_of("NAME").unwrap(), "my-deps");
    /// ```
    ///
    /// Convenience: add dependency-mappings from buildpack
    ///
    /// ```
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "dependency-mapping", "-b", "buildpack/id-1:v1.0.1", "-b", "buildpack/id-2:v2.1.0"]);
    /// let cmd = args.subcommand_matches("dependency-mapping").unwrap();
    ///
    /// let bps:Vec<_> = cmd.values_of("BUILDPACK").unwrap().collect();
    /// assert_eq!(bps, vec!["buildpack/id-1:v1.0.1", "buildpack/id-2:v2.1.0"]);
    /// ```
    ///
    /// Convenience: add arguments for docker run
    ///
    /// ```
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "args", "-d"]);
    /// let cmd = args.subcommand_matches("args").unwrap();
    ///
    /// assert_eq!(cmd.is_present("DOCKER"), true);
    /// assert_eq!(cmd.is_present("PACK"), false);
    /// ```
    ///
    /// Convenience: add arguments for pack build
    ///
    /// ```
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "args", "-p"]);
    /// let cmd = args.subcommand_matches("args").unwrap();
    ///
    /// assert_eq!(cmd.is_present("DOCKER"), false);
    /// assert_eq!(cmd.is_present("PACK"), true);
    /// ```
    ///
    /// Convenience: don't set the type of args and fails
    ///
    /// ```
    /// let res = binding_tool::args::Parser::new().try_parse_args(vec!["bt", "args"]);
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
        let force = Arg::new("FORCE")
            .short('f')
            .long("force")
            .takes_value(false)
            .help("force update if key exists");

        Parser {
            app: app_from_crate!()
            .subcommand(
                App::new("add")
                    .alias("a")
                    .arg(&force)
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
                    .arg(&force)
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
                    .arg(&force)
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
                    .arg(&force)
                    .arg(
                        Arg::new("NAME")
                            .short('n')
                            .long("name")
                            .value_name("name")
                            .required(false)
                            .help("optional name for the binding,\nname defaults to the type"),
                    )
                    .arg(
                        Arg::new("TOML")
                            .short('t')
                            .long("toml")
                            .value_name("toml")
                            .multiple_occurrences(true)
                            .conflicts_with("BUILDPACK")
                            .help("path to local buildpack.toml file with metadata dependencies"),
                    )
                    .arg(
                        Arg::new("BUILDPACK")
                            .short('b')
                            .long("buildpack")
                            .value_name("buildpack")
                            .multiple_occurrences(true)
                            .conflicts_with("TOML")
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

impl<'a, 'b> Default for Parser<'a> {
    fn default() -> Self {
        Self::new()
    }
}
