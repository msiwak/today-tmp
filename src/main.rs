use clap::{crate_version, load_yaml, App};

fn main() {
    let yaml = load_yaml!("../args.yaml");
    let _app = App::from_yaml(yaml).version(crate_version!()).get_matches();
}
