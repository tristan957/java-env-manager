use std::error::Error;
use std::ffi::OsStr;
use settings::{Distribution, Settings};

pub fn add(name: &str, path: &OsStr) -> Result<(), Box<Error>> {
    let mut settings = Settings::get()?;
    settings.add(Distribution::new(name, path));
    settings.save()
}
