#[macro_use]

extern crate clap;
extern crate libjem;

use clap::App;
use libjem::{
    add::add,
    error::Error,
    init::init,
    remove::remove,
    set::set,
    settings::Settings,
    update::update,
    version::version,
    which::which,
};

fn main() -> Result<(), Error> {
    let yaml = load_yaml!("../cli.yaml");
    let app = App::from_yaml(yaml);
    let mut app_clone = app.clone();

    let matches = app.get_matches();
    if matches.is_present("version") {
        println!("Java Environment Manager -- {}", env!("CARGO_PKG_VERSION"));
        return Ok(())
    }

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
                return Err(e)
            }
        },
        ("doctor", Some(_doctor_matches)) => {
            println!("doctor");
        },
        ("help", Some(_help_matches)) => {
            app_clone
                .print_help()
                .expect("Unable to print help information");
            println!();
        },
        ("init", Some(_init_matches)) => {
            if let Err(e) = init() {
                eprintln!("Unable to initialize the Java Environment Manager");
                return Err(e)
            }
        },
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
            },
            Err(e) => {
                eprintln!("Unable to list distributions");
                return Err(e)
            },
        },
        ("remove", Some(remove_matches)) => {
            let name = remove_matches
                .value_of("name")
                .expect("remove requires a name\nUSAGE: java-env-manager remove --name <name>");

            if let Err(e) = remove(name) {
                eprintln!("Unable to remove distribution");
                return Err(e)
            }
        },
        ("set", Some(set_matches)) => {
            let name = set_matches.value_of("name").expect(
                "set requires a name\nUSAGE: java-env-manager set --name <name> --path <path>",
            );

            match set(name) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Unable to change settings: {}", e.description());
                    return Err(e)
                },
            }
        },
        ("update", Some(update_matches)) => {
            let name = update_matches.value_of("name").expect(
                "update requires a name\nUSAGE: java-env-manager update --name <name> --path \
                 <path>",
            );
            let path = update_matches.value_of_os("path").expect(
                "update requires a path\nUSAGE: java-env-manager update --name <name> --path \
                 <path>",
            );

            if let Err(_e) = update(name, path) {
                eprintln!(
                    "Failed to update {} with path {}",
                    name,
                    path.to_str().unwrap_or("[path failed to display]")
                );
            }
        },
        ("version", Some(_version_matches)) => match version() {
            Ok(distro) => println!(
                "{} ({})",
                distro.get_name(),
                distro
                    .get_path()
                    .to_str()
                    .unwrap_or("Unable to display path")
            ),
            Err(e) => {
                println!("No distribution set");
                return Err(e)
            },
        },
        ("which", Some(_which_matches)) => match which() {
            Ok(p) => println!("{}", p.to_str().unwrap_or("Unable to display path")),
            Err(e) => {
                println!("No distribution set");
                return Err(e)
            },
        },
        ("", None) => {
            app_clone
                .print_help()
                .expect("Unable to print help information");
            println!();
        },
        _ => {
            app_clone
                .print_help()
                .expect("Unable to print help information");
            println!();
        },
    }

    Ok(())
}
