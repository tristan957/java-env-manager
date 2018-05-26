#[macro_use]
extern crate clap;
extern crate libjem;

use clap::App;
use libjem::add::add;
use libjem::init::init;
use libjem::remove::remove;
use libjem::set::set;
use libjem::settings::Settings;
use libjem::version::version;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let yaml = load_yaml!("../cli.yaml");
    let app = App::from_yaml(yaml);
    let mut app_clone = app.clone();
    let matches = app.get_matches();

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            let name = add_matches.value_of("name").expect(
                "add requires a name\nUSAGE: java-env-manager add --name <name> --path <path>",
            );
            let path = add_matches.value_of_os("path").expect(
                "add requires a path\nUSAGE: java-env-manager add --name <name> --path <path>",
            );

            if let Err(e) = add(name, path) {
                eprintln!("Unable to add Java distribution");
                return Err(e);
            }
        }
        ("doctor", Some(_doctor_matches)) => {
            println!("doctor");
        }
        ("help", Some(_help_matches)) => {
            app_clone.print_help()?;
            println!();
        }
        ("init", Some(_init_matches)) => {
            if let Err(e) = init() {
                eprintln!("Unable to initialize the Java Environment Manager");
                return Err(e);
            }
        }
        ("list", Some(_list_matches)) => match Settings::get() {
            Ok(settings) => {
                for distro in settings.get_distributions() {
                    let name = distro.get_name();
                    if name == settings.get_set() {
                        println!("* {}", name)
                    } else {
                        println!("  {}", name);
                    }
                }
            }
            Err(e) => {
                eprintln!("Unable to list distributions");
                return Err(e);
            }
        },
        ("remove", Some(remove_matches)) => {
            let name = remove_matches.value_of("name").expect("remove requires a name\nUSAGE: java-env-manager remove --name <name> --path <path>");
            if let Err(e) = remove(name) {
                eprintln!("Unable to remove distribution");
                return Err(e);
            }
        }
        ("set", Some(set_matches)) => {
            let name = set_matches.value_of("name").expect(
                "set requires a name\nUSAGE: java-env-manager remove --name <name> --path <path>",
            );
            match set(name) {
                Ok(true) => {}
                Ok(false) => eprintln!("Name does not exist in list of distributions"),
                Err(e) => {
                    eprintln!("Unable to change settings");
                    return Err(e);
                }
            }
        }
        ("update", Some(_update_matches)) => {
            println!("update");
        }
        ("version", Some(_version_matches)) => match version() {
            Some(distro) => println!(
                "{} ({})",
                distro.get_name(),
                distro
                    .get_path()
                    .to_str()
                    .unwrap_or("Unable to display path")
            ),
            None => println!("No distribution set"),
        },
        ("which", Some(_which_matches)) => {
            // which().ok_or("Hello World")
        }
        ("", None) => {
            app_clone.print_help()?;
            println!();
        }
        _ => {
            app_clone.print_help()?;
            println!();
        }
    }

    Ok(())
}
