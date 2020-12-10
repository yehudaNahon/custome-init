// build.rs
use config_struct::{Error, StructOptions};

fn main() -> Result<(), Error> {
    config_struct::create_struct("config.json", "src/config.rs", &StructOptions::default())
}
