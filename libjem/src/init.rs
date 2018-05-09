extern crate serde;
extern crate serde_json;

use std::fs;
use std::error::Error;
use std::path::Path;

use settings::Settings;

pub fn init() -> Result<(), Box<Error>> {
    let mut program_dir = Settings::get_program_dir().expect("No program directory found");
    
    if Path::new(&program_dir).exists() {
        println!("{} already exists", match program_dir.into_string() {
            Ok(s) => s,
            Err(_) => String::from("Directory")
        });
        return Ok(())
    }

    match fs::create_dir(&program_dir) {
        Ok(_) => {},
        Err(e) => return Err(Box::new(e))
    }

    program_dir.push("/settings.json");
    match fs::File::create(program_dir) {
        Ok(f) => match serde_json::to_writer_pretty(&f, &Settings::default()) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(Box::new(e))
        },
        Err(e) => return Err(Box::new(e))
    }

    // program_dir.push("/settings.json");
    // let file = match fs::File::create(program_dir) {
    //     Ok(f) => f,
    //     Err(e) => return Err(Box::new(e))
    // };

    // match serde_json::to_writer_pretty(&file, &Settings::default()) {
    //     Ok(_) => Ok(()),
    //     Err(e) => Err(Box::new(e))
    // }
}
