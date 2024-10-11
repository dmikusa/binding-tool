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

use clap::{command, Arg, ArgAction, ArgGroup, Command};
use std::ffi::OsString;

pub struct Parser {
    app: Command,
}

impl Parser {
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
    /// assert_eq!(cmd.get_one::<String>("TYPE").unwrap(), "binding");
    ///
    /// let params:Vec<_> = cmd.get_many::<String>("PARAM").unwrap().collect();
    /// assert_eq!(params, vec!["foo=bar"]);
    /// assert_eq!(cmd.get_one::<String>("NAME"), None);
    /// ```
    ///
    /// More Advanced: Add with multiple parameters and a name
    ///
    /// ```
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "add", "-f", "-n", "better_name", "-t", "binding", "-p", "foo=bar", "-p", "gorilla=banana"]);
    /// let cmd = args.subcommand_matches("add").unwrap();
    ///
    /// assert_eq!(cmd.get_one::<String>("TYPE").unwrap(), "binding");
    ///
    /// let params:Vec<_> = cmd.get_many::<String>("PARAM").unwrap().collect();
    /// assert_eq!(params, vec!["foo=bar", "gorilla=banana"]);
    /// assert_eq!(cmd.get_one::<String>("NAME").unwrap(), "better_name");
    /// assert_eq!(cmd.contains_id("FORCE"), true);
    /// ```
    ///
    /// Basic: Delete an entire binding
    ///
    /// ```
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "delete", "-n", "binding"]);
    /// let cmd = args.subcommand_matches("delete").unwrap();
    ///
    /// assert_eq!(cmd.get_one::<String>("NAME").unwrap(), "binding");
    /// ```
    ///
    /// More Advanced: Delete parts of a binding
    ///
    /// ```
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "delete", "-f", "-n", "better_name", "-k", "foo"]);
    /// let cmd = args.subcommand_matches("delete").unwrap();
    ///
    /// let keys:Vec<_> = cmd.get_many::<String>("KEY").unwrap().collect();
    /// assert_eq!(keys, vec!["foo"]);
    /// assert_eq!(cmd.get_one::<String>("NAME").unwrap(), "better_name");
    /// assert_eq!(cmd.contains_id("FORCE"), true);
    /// ```
    ///
    /// Convenience: add ca-certificates
    ///
    /// ```
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "ca-certs", "-f", "-n", "my-certs", "-c", "/path/to/ca.crt"]);
    /// let cmd = args.subcommand_matches("ca-certs").unwrap();
    ///
    ///
    /// let certs:Vec<_> = cmd.get_many::<String>("CERT").unwrap().collect();
    /// assert_eq!(certs, vec!["/path/to/ca.crt"]);
    /// assert_eq!(cmd.get_one::<String>("NAME").unwrap(), "my-certs");
    /// assert_eq!(cmd.contains_id("FORCE"), true);
    /// ```
    ///
    /// Convenience: add dependency-mappings
    ///
    /// ```
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "dependency-mapping", "-n", "my-deps", "-t", "/path/to/file.zip"]);
    /// let cmd = args.subcommand_matches("dependency-mapping").unwrap();
    ///
    /// let files:Vec<_> = cmd.get_many::<String>("TOML").unwrap().collect();
    /// assert_eq!(files, vec!["/path/to/file.zip"]);
    /// assert_eq!(cmd.get_one::<String>("NAME").unwrap(), "my-deps");
    /// ```
    ///
    /// Convenience: add dependency-mappings from buildpack
    ///
    /// ```
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "dependency-mapping", "-b", "buildpack/id-1:v1.0.1", "-b", "buildpack/id-2:v2.1.0"]);
    /// let cmd = args.subcommand_matches("dependency-mapping").unwrap();
    ///
    /// let bps:Vec<_> = cmd.get_many::<String>("BUILDPACK").unwrap().collect();
    /// assert_eq!(bps, vec!["buildpack/id-1:v1.0.1", "buildpack/id-2:v2.1.0"]);
    /// ```
    ///
    /// Convenience: configure bash
    ///
    /// ```
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "init", "bash"]);
    /// let cmd = args.subcommand_matches("init").unwrap();
    ///
    /// assert_eq!(cmd.get_one::<String>("SHELL").unwrap(), "bash");
    /// ```
    ///
    /// Convenience: don't set the type of args and fails
    ///
    /// ```
    /// let res = binding_tool::args::Parser::new().try_parse_args(vec!["bt", "init"]);
    /// assert!(res.is_err(), "should require a argument");
    /// ```
    ///
    ///
    /// Convenience: add arguments for docker run
    ///
    /// ```
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "args", "-d"]);
    /// let cmd = args.subcommand_matches("args").unwrap();
    ///
    /// assert_eq!(cmd.value_source("DOCKER"), Some(clap::parser::ValueSource::CommandLine));
    /// assert_eq!(cmd.value_source("PACK"), Some(clap::parser::ValueSource::DefaultValue));
    /// ```
    ///
    /// Convenience: add arguments for pack build
    ///
    /// ```
    /// let args = binding_tool::args::Parser::new().parse_args(vec!["bt", "args", "-p"]);
    /// let cmd = args.subcommand_matches("args").unwrap();
    ///
    /// assert_eq!(cmd.value_source("DOCKER"), Some(clap::parser::ValueSource::DefaultValue));
    /// assert_eq!(cmd.value_source("PACK"), Some(clap::parser::ValueSource::CommandLine));
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

    pub fn try_parse_args<I, T>(self, args: I) -> clap::error::Result<clap::ArgMatches>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        self.app.try_get_matches_from(args)
    }

    pub fn new() -> Parser {
        let force = Arg::new("FORCE")
            .short('f')
            .long("force")
            .action(ArgAction::SetTrue)
            .help("force update if key exists");

        Parser {
            app: command!()
            .subcommand(
                Command::new("add")
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
                            .action(ArgAction::Append)
                            .required(true)
                            .help("key/value to set for the type"),
                    )
                    .about("Add or modify a binding")
                    .after_help( include_str!("help/additional_help_param.txt")),
            )
            .subcommand(
                Command::new("delete")
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
                            .action(ArgAction::Append)
                            .required(false)
                            .help("specific key to delete"),
                    )
                    .about("Delete a binding")
                    .after_help(include_str!("help/additional_help_binding.txt")),
            )
            .subcommand(
                Command::new("ca-certs")
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
                            .action(ArgAction::Append)
                            .help("path to a CA certificate to add"),
                    )
                    .about("Convenience for adding `ca-certificates` bindings")
                    .after_help(include_str!("help/additional_help_binding.txt")),
            )
            .subcommand(
                Command::new("dependency-mapping")
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
                            .action(ArgAction::Append)
                            .conflicts_with("BUILDPACK")
                            .help("path to local buildpack.toml file with metadata dependencies"),
                    )
                    .arg(
                        Arg::new("BUILDPACK")
                            .short('b')
                            .long("buildpack")
                            .value_name("buildpack")
                            .action(ArgAction::Append)
                            .conflicts_with("TOML")
                            .help("buildpack ID and optional version from which dependencies will be loaded\n    \
                                Example: `buildpack/id@version` or `buildpack/id`"),
                    )
                    .about("Convenience for adding `dependency-mapping` bindings")
                    .after_help(include_str!("help/additional_help_binding.txt")),
            )
            .subcommand(
                Command::new("init")
                    .arg(
                        Arg::new("SHELL")
                            .value_name("shell")
                            .required(true)
                            .value_parser(["bash", "fish", "zsh"])
                            .help("type of shell script to generate"))
                    .about(
                        "Generates shell wrappers that make using `pack build` and `docker run` easier",
                    ),
            )
            .subcommand(
                Command::new("args")
                    .arg(
                        Arg::new("DOCKER")
                            .short('d')
                            .long("docker")
                            .action(ArgAction::SetTrue)
                            .help("generates binding args for `docker run`"),
                    )
                    .arg(
                        Arg::new("PACK")
                            .short('p')
                            .long("pack")
                            .action(ArgAction::SetTrue)
                            .help("generates binding args for `pack build`"),
                    )
                    .group(
                        ArgGroup::new("TYPES")
                            .args(["DOCKER", "PACK"])
                            .multiple(false)
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

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
