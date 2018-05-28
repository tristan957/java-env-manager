use error::{Error, ErrorKind};
use settings::{Distribution, Settings};

pub fn version() -> Result<Distribution, Error> {
    let settings = Settings::get()?;
    let name = settings.get_set();

    for distro in settings.get_distributions() {
        if distro.get_name() == name {
            return Ok(distro.clone())
        }
    }

    Err(Error::new(ErrorKind::NameNotFound))
}
