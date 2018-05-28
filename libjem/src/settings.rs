extern crate serde;
extern crate serde_json;

use error::{Error, ErrorKind, Result};
use std::{
    env,
    ffi::{OsStr, OsString},
    fmt,
    fs,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Distribution {
    name: String,
    path: OsString,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    distributions: Vec<Distribution>,
    set:           String,
}

impl Distribution {
    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn get_path(&self) -> &OsStr {
        self.path.as_ref()
    }

    pub fn new(name: &str, path: &OsStr) -> Self {
        Distribution {
            name: String::from(name),
            path: OsString::from(path),
        }
    }

    pub fn set_path(&mut self, path: &OsStr) {
        self.path = OsString::from(path);
    }
}

impl fmt::Display for Distribution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {:?})", self.name, self.path)
    }
}

impl Settings {
    pub fn add(&mut self, distro: Distribution) {
        self.distributions.push(distro);
    }

    pub fn default() -> Self {
        Settings {
            distributions: Vec::new(),
            set:           String::default(),
        }
    }

    pub fn get() -> Result<Settings> {
        let path = Settings::location().ok_or_else(|| Error::new(ErrorKind::SettingsReadFailure))?;
        let file = fs::File::open(path)?;
        let settings = serde_json::from_reader(file)?;

        Ok(settings)
    }

    pub fn get_distributions(&self) -> Vec<Distribution> {
        self.distributions.clone()
    }

    pub fn get_distributions_as_mut(&mut self) -> &mut Vec<Distribution> {
        self.distributions.as_mut()
    }

    pub fn get_program_dir() -> Option<OsString> {
        env::var_os("JAVA_ENV_MANAGER_HOME")
            .or_else(|| env::home_dir().map(|path| path.join(".java-env-manager").into()))
    }

    pub fn get_set(&self) -> &str {
        self.set.as_ref()
    }

    pub fn location() -> Option<OsString> {
        match Settings::get_program_dir() {
            Some(mut os) => {
                os.push("/settings.json");
                Some(os)
            },
            None => None,
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Settings::location().ok_or_else(|| Error::new(ErrorKind::SettingsReadFailure))?;
        let file = fs::OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(path)?;
        serde_json::to_writer_pretty(file, &self)?;

        Ok(())
    }

    pub fn set_set(&mut self, name: &str) {
        self.set = String::from(name);
    }
}
