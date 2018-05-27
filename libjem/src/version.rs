use settings::{Distribution, Settings};

pub fn version() -> Option<Distribution> {
    match Settings::get() {
        Ok(settings) => {
            let name = settings.get_set();
            for distro in settings.get_distributions() {
                if distro.get_name() == name {
                    return Option::from(distro.clone())
                }
            }
        },
        Err(_) => return Option::default(),
    }

    Option::default()
}
