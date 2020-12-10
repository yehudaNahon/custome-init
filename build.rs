// build.rs
use config_struct::{Error, StructOptions};
use std::env;

fn main() -> Result<(), Error> {
    config_struct::create_struct(
        env::var_os("CONFIG_FILE")
            .expect("please provide a configuration file to build the init process"),
        "src/config.rs",
        &StructOptions::default(),
    )
}
