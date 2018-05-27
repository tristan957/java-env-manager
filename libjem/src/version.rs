use settings::{Distribution, Settings};

pub fn version() -> Option<Distribution> {
    match Settings::get() {
        Ok(settings) => {
            let name = settings.get_set();
            for distro in settings.get_distributions() {
                if distro.get_name() == name {
                    return Some(distro.clone())
                }
            }
        },
        Err(_) => return None,
    }

    None
}
