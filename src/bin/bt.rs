use anyhow::Result;
use std::io::prelude::*;
use std::{env, fs, path};

fn main() -> Result<()> {
    let matches = binding_tools::parse_args();

    let binding_type = matches.value_of("TYPE").unwrap(); // required
    let binding_name = matches.value_of("NAME"); // optional
    let binding_key_vals = matches.values_of("PARAM").unwrap(); // required

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

    for bkv in binding_key_vals {
        let binding_path =
            path::Path::new(&bindings_home).join(binding_name.unwrap_or(binding_type));

        fs::create_dir_all(&binding_path)?;

        // TODO: prompt instead of blindly overwriting files
        let mut type_file = fs::File::create(&binding_path.join("type"))?;
        type_file.write_all(binding_type.as_bytes())?;

        if let Some((binding_key, binding_value)) = bkv.split_once("=") {
            let mut binding_file = fs::File::create(&binding_path.join(binding_key))?;
            binding_file.write_all(binding_value.as_bytes())?;
            // TODO: handle `@` and copy file contents
        } else {
            println!("could not parse key/value -> {}\n{}", bkv, matches.usage());
        }
    }

    Ok(())
}
