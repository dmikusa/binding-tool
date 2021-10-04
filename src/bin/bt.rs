use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    let matches = binding_tools::parse_args(env::args());

    // required (it's OK to unwrap)
    let binding_type = matches.value_of("TYPE").unwrap();
    let binding_key_vals = matches.values_of("PARAM").unwrap();

    // optional (it's not OK to unwrap)
    let binding_name = matches.value_of("NAME");

    // binding root = SERVICE_BINDING_ROOT (or default to "./bindings")
    let bindings_home = match env::var("SERVICE_BINDING_ROOT") {
        Ok(root) => root,
        Err(_) => env::current_dir()
            .unwrap()
            .join("bindings")
            .to_str()
            .unwrap()
            .into(),
    };

    // process bindings
    return if matches.is_present("FORCE") {
        let btp = binding_tools::BindingProcessor::new(
            &bindings_home,
            binding_type,
            binding_name,
            binding_tools::TrueBindingConfirmer {},
        );

        btp.process_bindings(binding_key_vals)
    } else {
        let btp = binding_tools::BindingProcessor::new(
            &bindings_home,
            binding_type,
            binding_name,
            binding_tools::ConsoleBindingConfirmer {},
        );

        btp.process_bindings(binding_key_vals)
    };
}
