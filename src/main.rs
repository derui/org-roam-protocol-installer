extern crate org_roam_protocol_installer;

use org_roam_protocol_installer::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    Config::new(&args);

    println!("Hello, world!");
}
