use error::Result;
use settings::{Distribution, Settings};
use std::ffi::OsStr;

/// Appends a distribution to the settings configuration
pub fn add(name: &str, path: &OsStr) -> Result<()> {
    let mut settings = Settings::get()?;
    settings.add(Distribution::new(name, path));
    settings.save()
}
