extern crate serde;
extern crate serde_json;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct Distribution {
    name: String,
    path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    distributions: Vec<Distribution>,
    set: String,
}

impl Settings {
    pub fn default() -> Settings {
        Settings { distributions: Vec::new(), set: String::default() }
    }

    pub fn get() -> Result<Settings, Box<Error>> {
        let path = Settings::location().ok_or("Location not found")?;
        let file = fs::File::open(path)?;
        let s = serde_json::from_reader(file)?;

        Ok(s)
    }

    pub fn get_program_dir() -> Option<OsString> {
        env::var_os("JAVA_ENV_MANAGER_HOME").or_else(|| {
            env::home_dir().map(|path| {
                path.join(".java-env-manager").into()
            })
        })
    }

    pub fn location() -> Option<OsString> {
        match Settings::get_program_dir() {
            Some(mut os) => {
                os.push("/settings.json");
                Option::from(os)
            },
            None => Option::default()
        }
    }

    pub fn set(&self) -> Result<(), Box<Error>> {
        let path = Settings::location().ok_or("Location not found")?;
        let file = fs::OpenOptions::new()
            .write(true)
            .open(path)?;
        serde_json::to_writer_pretty(&file, &self)?;

        Ok(())
    }
}
