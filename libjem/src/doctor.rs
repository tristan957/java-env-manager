use error::{Error, ErrorKind, Result};
use settings::Settings;
use std::path::Path;

pub fn doctor() -> Result<()> {
    let settings = Settings::get()?;
    let set = settings.get_set();

    let mut name_found = false;
    for distro in settings.get_distributions() {
        let name = distro.get_name();
        let path = distro.get_path();

        if !Path::new(&path).exists() {
            return Err(Error::new_with_desc(
                ErrorKind::PathNotFound,
                format!(
                    "The path for {} ({}) does not exist",
                    name,
                    path.to_str().unwrap_or("unable to display path")
                ),
            ))
        }

        if name == set {
            name_found = true;
        } else if name == set && name_found {
            return Err(Error::new_with_desc(
                ErrorKind::DuplicateNames,
                format!("The name, {}, exists twice", name),
            ))
        }
    }

    Ok(())
}
