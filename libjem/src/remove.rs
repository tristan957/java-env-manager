use std::error::Error;
use settings::Settings;

pub fn remove(name: &str) -> Result<(), Box<Error>> {
    let mut settings = Settings::get()?;
    {
        let distros = settings.get_distributions_as_mut();
        distros.iter()
            .position(|d| name == d.get_name())
            .map(|p| distros.remove(p));
    }
    settings.set()?;

    Ok(())
}
