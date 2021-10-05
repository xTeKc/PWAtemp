use std::env::var;

fn main() {
    let _aid = var("API_ID").unwrap_or("none".to_string());
    let _su = var("SERVER_URL").unwrap_or("none".to_string());
}
