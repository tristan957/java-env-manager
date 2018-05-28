use error::Error;
use settings::{Distribution, Settings};
use std::ffi::OsStr;

pub fn add(name: &str, path: &OsStr) -> Result<(), Error> {
    let mut settings = Settings::get()?;
    settings.add(Distribution::new(name, path));
    settings.save()
}
