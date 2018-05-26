use std::error::Error;
use std::os::unix;
// use std::os::windows;
use std::ffi::OsString;
use std::path::Path;
use std::fs;
use settings::Settings;

pub fn set(name: &str) -> Result<bool, Box<Error>> {
    let mut settings = Settings::get()?;
    let mut path = OsString::default();
    {
        let distros = settings.get_distributions();
        if distros.into_iter().any(|d| { 
                path = OsString::from(d.get_path());
                path.push("/bin");
                name == d.get_name()
            }) == false {
            return Ok(false)
        }
    }

    let mut program_dir = Settings::get_program_dir().unwrap_or_default();
    program_dir.push("/bin");
    if Path::new(&program_dir).exists() {
        fs::remove_dir(&program_dir)?;
    }

    if cfg!(target_os = "windows") {
        // windows::fs::symlink_dir(&path, &program_dir)?;
    } else {
        unix::fs::symlink(&path, &program_dir)?;
    }

    settings.set_set(name);
    settings.save()?;
    
    Ok(true)
}
