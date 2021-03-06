use error::{Error, ErrorKind, Result};
use settings::Settings;
use std::ffi::OsStr;

/// Update the path of a distribution
///
/// # Errors
///
/// * [`ErrorKind::NameNotFound`](enum.ErrorKind.html)
pub fn update(name: &str, path: &OsStr) -> Result<()> {
    let mut settings = Settings::get()?;
    let mut name_found = false;
    {
        for distro in settings.get_distributions_as_mut() {
            if distro.get_name() == name {
                distro.set_path(path);
                name_found = true;
            }
        }
    }

    if name_found {
        return settings.save()
    }

    Err(Error::new(ErrorKind::NameNotFound))
}
