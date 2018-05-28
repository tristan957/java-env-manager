use error::{Error, ErrorKind};
use settings::Settings;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn which() -> Result<PathBuf, Error> {
    let mut program_dir =
        Settings::get_program_dir().ok_or_else(|| Error::new(ErrorKind::SettingsNotFound))?;
    program_dir.push("/bin");

    let path = Path::new(&program_dir);
    if path.exists() {
        let pathbuf = fs::read_link(path)?;
        return Ok(pathbuf)
    }

    Err(Error::new(ErrorKind::BinariesNotFound))
}
