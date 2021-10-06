use std::env;
use std::fs;
use std::io::Error;


fn main() -> Result<(), Error> {
    // read `config_path` from the environment variable `CONFIG`.
    // If `CONFIG` isn't set, fall back to a default config path.
    let env_path = env::var(".env");

    
    
    Ok(())
}