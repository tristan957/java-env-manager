use error::{Error, ErrorKind, Result};
use settings::Settings;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Returns a path to the actual binaries that were symlinked
///
/// # Errors
///
/// * [`ErrorKind::BinariesNotFound`](enum.ErrorKind.html)
/// * [`ErrorKind::SettingsNotFound`](enum.ErrorKind.html)
pub fn which() -> Result<PathBuf> {
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
