use error::{Error, ErrorKind, Result};
use settings::Settings;
use std::{ffi::OsString, fs, os::unix, path::Path};
// use std::os::windows;

pub fn set(name: &str) -> Result<()> {
    let mut settings = Settings::get()?;
    let mut path: Option<OsString> = None;

    for distro in settings.get_distributions() {
        if name == distro.get_name() {
            let mut p = OsString::from(distro.get_path());
            p.push("/bin");
            path = Some(p);
        }
    }

    if path.is_none() {
        return Err(Error::new(ErrorKind::NameNotFound))
    }

    let mut program_dir =
        Settings::get_program_dir().ok_or_else(|| Error::new(ErrorKind::SettingsNotFound))?;
    program_dir.push("/bin");
    if Path::new(&program_dir).exists() {
        fs::remove_dir(&program_dir)?;
    }

    if cfg!(target_os = "windows") {
        // windows::fs::symlink_dir(&path, &program_dir)?;
    } else {
        unix::fs::symlink(path.unwrap(), &program_dir)?;
    }

    settings.set_set(name);
    settings.save()
}
