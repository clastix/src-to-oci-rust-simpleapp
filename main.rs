use std::env;
use std::{thread, time};

fn main() {

    let hello_msg = env::var("HELLO_MSG").unwrap_or("world".to_string());
    let one_hour = time::Duration::from_secs(3600);

    println!("Hello {}!", hello_msg);

    thread::sleep(one_hour);
}
