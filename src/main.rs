extern crate org_roam_protocol_installer;

use std::process::exit;

use org_roam_protocol_installer::config::{application_definition, Config};

fn main() {
    let matches = application_definition().get_matches();

    match Config::new(&matches) {
        Ok(config) => {
            org_roam_protocol_installer::run(config).unwrap();
        }
        Err(e) => {
            eprint!("Error occurred: {}", e);
            exit(1);
        }
    }
}
