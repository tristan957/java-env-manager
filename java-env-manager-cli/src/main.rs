#[macro_use]
extern crate clap;
extern crate libjem;

use clap::App;
use libjem::settings::Settings;

fn main() {
    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            match Settings::get() {
                Ok(_) => {},
                Err(error) => println!("{}", error),
            };
            // libjem::add::command();
        },
        ("doctor", Some(doctor_matches)) => {
            println!("doctor");
        },
        ("help", Some(help_matches)) => {
            println!("help");
        },
        ("init", Some(init_matches)) => {
            println!("init");
        },
        ("list", Some(list_matches)) => {
            println!("list");
        },
        ("remove", Some(remove_matches)) => {
            println!("remove");
        },
        ("set", Some(set_matches)) => {
            println!("set");
        },
        ("update", Some(update_matches)) => {
            println!("update");
        },
        ("version", Some(version_matches)) => {
            println!("version");
        },
        ("which", Some(which_matches)) => {
            println!("which");
        },
        ("", None) => println!("help"),
        _ => println!("error bitch"),
    }
}
