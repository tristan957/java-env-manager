use error::{Error, ErrorKind};
use settings::Settings;
use std::{fs, path::Path};

pub fn init() -> Result<(), Error> {
    let mut program_dir = Settings::get_program_dir().expect("No program directory found");
    if Path::new(&program_dir).exists() {
        return Err(Error::new(ErrorKind::PathExists))
    }

    fs::create_dir(&program_dir)?;
    program_dir.push("/settings.json");
    fs::File::create(&program_dir)?;

    Settings::default().save()
}
