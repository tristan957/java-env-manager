#[macro_use]
extern crate clap;
extern crate libjem;

use clap::App;
use std::error::Error;
use libjem::add::add;
use libjem::init::init;
// use libjem::list::list;
use libjem::remove::remove;
use libjem::set::set;

fn main() -> Result<(), Box<Error>> {
    let yaml = load_yaml!("../cli.yaml");
    let app = App::from_yaml(yaml);
    let mut app_clone = app.clone();
    let matches = app.get_matches();

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            let name = add_matches.value_of("name").expect("add requires a name\nUSAGE: java-env-manager add --name <name> --path <path>");
            let path = add_matches.value_of_os("path").expect("add requires a path\nUSAGE: java-env-manager add --name <name> --path <path>");
            
            if let Err(e) = add(name, path) {
                eprintln!("Unable to add Java distribution");
                return Err(e)
            }
        },
        ("doctor", Some(_doctor_matches)) => {
            println!("doctor");
        },
        ("help", Some(_help_matches)) => {
            app_clone.print_help()?;
        },
        ("init", Some(_init_matches)) => {
            if let Err(e) = init() {
                eprintln!("Unable to initialize the Java Environment Manager");
                return Err(e)
            }
        },
        ("list", Some(_list_matches)) => {
            // match list() {
            //     Ok(distros) => {
            //         for distro in distros {
            //             println!("{}", distro);
            //         }
            //     },
            //     Err(e) => {
            //         eprintln!("Unable to list distributions");
            //         return Err(e)
            //     }
            // }
        },
        ("remove", Some(remove_matches)) => {
            let name = remove_matches.value_of("name").expect("remove requires a name\nUSAGE: java-env-manager remove --name <name> --path <path>");
            if let Err(e) = remove(name) {
                eprintln!("Unable to remove distribution");
                return Err(e)
            }
        },
        ("set", Some(set_matches)) => {
            let name = set_matches.value_of("name").expect("set requires a name\nUSAGE: java-env-manager remove --name <name> --path <path>");
            match set(name) {
                Ok(true) => {},
                Ok(false) => eprintln!("Name does not exist in list of distributions"),
                Err(e) => {
                    eprintln!("Unable to change settings");
                    return Err(e)
                }
            }
        },
        ("update", Some(_update_matches)) => {
            println!("update");
        },
        ("version", Some(_version_matches)) => {
            println!("version");
        },
        ("which", Some(_which_matches)) => {
            // which().ok_or("Hello World")
        },
        ("", None) => app_clone.print_help()?,
        _ => app_clone.print_help()?,
    }

    Ok(())
}
