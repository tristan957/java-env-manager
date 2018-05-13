#[macro_use]
extern crate clap;
extern crate libjem;

use clap::App;
use libjem::add::add;
use libjem::init::init;

fn main() {
    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            let name = add_matches.value_of("name").expect("add requires a name\nUSAGE: java-env-manager add --name <name> --path <path>");
            let path = add_matches.value_of_os("path").expect("add requires a path\nUSAGE: java-env-manager add --name <name> --path <path>");
            
            if let Err(_e) = add(name, path) {
                eprintln!("Unable to add Java distribution")
            }
        },
        ("doctor", Some(_doctor_matches)) => {
            println!("doctor");
        },
        ("help", Some(_help_matches)) => {
            println!("help");
        },
        ("init", Some(_init_matches)) => {
            if let Err(_e) = init() {
                eprintln!("Unable to initialize the Java Environment Manager")
            }
        },
        ("list", Some(_list_matches)) => {
            println!("list");
        },
        ("remove", Some(_remove_matches)) => {
            println!("remove");
        },
        ("set", Some(_set_matches)) => {
            println!("set");
        },
        ("update", Some(_update_matches)) => {
            println!("update");
        },
        ("version", Some(_version_matches)) => {
            println!("version");
        },
        ("which", Some(_which_matches)) => {
            println!("which");
        },
        ("", None) => println!("help"),
        _ => println!("error bitch"),
    }
}
