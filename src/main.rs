mod logger;
use std::env;

fn main() {
    let PORT_S = env::var("PORT").unwrap_or("9000".to_string());
    let HOST = env::var("HOST").unwrap_or("localhost".to_string());
    let PORT = PORT_S.parse::<i32>().unwrap();

    logger::info(format!("Booting on {}", PORT).as_str());
    

}
