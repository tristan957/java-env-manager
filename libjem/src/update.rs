use error::{Error, ErrorKind};
use settings::Settings;
use std::ffi::OsStr;

pub fn update(name: &str, path: &OsStr) -> Result<(), Error> {
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
