use std::error::Error;
use settings::Settings;

pub fn set(name: &str) -> Result<bool, Box<Error>> {
    let mut settings = Settings::get()?;
    {
        let distros = settings.get_distributions();
        if distros.into_iter().any(|d| name == d.get_name()) == false {
            return Ok(false)
        }
    }

    // TODO actually create the symlink here

    settings.set_set(name);
    settings.save()?;
    
    Ok(true)
}
