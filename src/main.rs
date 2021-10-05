use std::env;

fn main() {
    // let _aid = var("API_ID").unwrap_or("none".to_string());
    // let _su = var("SERVER_URL").unwrap_or("none".to_string());
    // println!("{} {}", _aid, _su);

    let _aid = "API_ID";
    match env::var(_aid) {
        Ok(val) => println!("{}: {:?}", _aid, val),
        Err(e) => println!("couldn't interpret {}: {}", _aid, e),
    }
}
