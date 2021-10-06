use std::env;
// use std::fs;
// use std::io::Error;


// fn env() -> Result<(), Error> {
//     // read `config_path` from the environment variable `CONFIG`.
//     // If `CONFIG` isn't set, fall back to a default config path.
//     let _env = env::var("_env")
//         .unwrap_or(".env".to_string());

//     let env: String = fs::read_to_string(_env)?;

//     println!("{}", env);

//     Ok(())
// }


fn env() {
    let app_id = "APP_ID";
    env::set_var(app_id, "VALUE");
    assert_eq!(env::var(app_id), Ok("VALUE".to_string()));
    println!("{}", app_id);
}


fn main() {
    env();
}