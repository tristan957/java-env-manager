use error::Result;
use settings::Settings;

// Removes a distribution from the settiings based on its name
pub fn remove(name: &str) -> Result<()> {
    let mut settings = Settings::get()?;
    {
        let distros = settings.get_distributions_as_mut();
        distros
            .iter()
            .position(|d| name == d.get_name())
            .map(|p| distros.remove(p));
    }

    settings.save()
}
