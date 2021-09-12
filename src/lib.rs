use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

// TODO: test arg parsing & really everything
pub fn parse_args<'a>() -> clap::ArgMatches<'a> {
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
        .get_matches();
}
