use error::{Error, ErrorKind, Result};
use settings::Settings;
use std::{fs, path::Path};

/// Initializes the program
///
/// # Errors
///
/// * [`ErrorKind::SettingsNotFound`](enum.ErrorKind.html)
pub fn init() -> Result<()> {
    let mut program_dir =
        Settings::get_program_dir().ok_or_else(|| Error::new(ErrorKind::SettingsNotFound))?;

    // create the program directory if it doesn't exist
    if !Path::new(&program_dir).exists() {
        fs::create_dir_all(&program_dir)?;
    }

    // create the settings file
    program_dir.push("/settings.json");
    fs::File::create(&program_dir)?;

    Settings::default().save()
}
