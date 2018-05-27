use settings::Settings;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn which() -> Option<PathBuf> {
    match Settings::get_program_dir() {
        Some(mut program_dir) => {
            program_dir.push("/bin");
            let path = Path::new(&program_dir);
            if path.exists() {
                match fs::read_link(path) {
                    Ok(p) => return Some(p),
                    Err(_) => return None,
                }
            }
        },
        None => return None,
    }

    None
}
