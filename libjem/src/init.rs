use std::error::Error;
use std::fs;
use std::path::Path;

use settings::Settings;

pub fn init() -> Result<(), Box<Error>> {
    let mut program_dir = Settings::get_program_dir().expect("No program directory found");

    if Path::new(&program_dir).exists() {
        println!(
            "{} already exists",
            match program_dir.into_string() {
                Ok(s) => s,
                Err(_) => String::from("Directory"),
            }
        );
        return Ok(());
    }

    fs::create_dir(&program_dir)?;
    program_dir.push("/settings.json");
    fs::File::create(&program_dir)?;
    Settings::default().save()?;

    Ok(())
}
