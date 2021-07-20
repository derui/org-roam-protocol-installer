extern crate org_roam_protocol_installer;

use org_roam_protocol_installer::config::Config;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args);
}
