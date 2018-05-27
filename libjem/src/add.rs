use settings::{Distribution, Settings};
use std::{error::Error, ffi::OsStr};

pub fn add(name: &str, path: &OsStr) -> Result<(), Box<Error>> {
    let mut settings = Settings::get()?;
    settings.add(Distribution::new(name, path));
    settings.save()
}
