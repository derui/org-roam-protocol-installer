extern crate org_roam_protocol_installer;

use org_roam_protocol_installer::config::{get_app, Config};

fn main() {
    let matches = get_app().get_matches();

    let config = Config::new(&matches);
}
