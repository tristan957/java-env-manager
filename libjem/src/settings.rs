extern crate serde;
extern crate serde_json;

use error::{Error, ErrorKind, Result};
use std::{
    env,
    ffi::{OsStr, OsString},
    fmt,
    fs,
};

/// Structural representation of what composes a distribution
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Distribution {
    name: String,
    path: OsString,
}

/// Structural representation the settings
#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    distributions: Vec<Distribution>,
    set:           String,
}

impl Distribution {
    /// Returns the name
    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns the path
    pub fn get_path(&self) -> &OsStr {
        self.path.as_ref()
    }

    /// Creates a new [`Distribution`] object
    pub fn new(name: &str, path: &OsStr) -> Self {
        Distribution {
            name: String::from(name),
            path: OsString::from(path),
        }
    }

    /// Set the path of the [`Distribution`]
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
    /// Adds a [`Distribution`] to the [`Settings`]
    pub fn add(&mut self, distro: Distribution) {
        self.distributions.push(distro);
    }

    /// Return a default [`Settings`] object
    pub fn default() -> Self {
        Settings {
            distributions: Vec::new(),
            set:           String::default(),
        }
    }

    /// Get the currently set [`Settings`]
    pub fn get() -> Result<Settings> {
        let path = Settings::location().ok_or_else(|| Error::new(ErrorKind::SettingsReadFailure))?;
        let file = fs::File::open(path)?;
        let settings = serde_json::from_reader(file)?;

        Ok(settings)
    }

    /// Get a cloned vector of [`Distributions`]
    pub fn get_distributions(&self) -> Vec<Distribution> {
        self.distributions.clone()
    }

    /// Return a mutable reference to the vector [`Distributions`]
    pub fn get_distributions_as_mut(&mut self) -> &mut Vec<Distribution> {
        self.distributions.as_mut()
    }

    /// Gets the location of the program directory
    pub fn get_program_dir() -> Option<OsString> {
        env::var_os("JAVA_ENV_MANAGER_HOME")
            .or_else(|| env::home_dir().map(|path| path.join(".java-env-manager").into()))
    }

    /// Returns the currently set name
    pub fn get_set(&self) -> &str {
        self.set.as_ref()
    }

    /// Return the location of the `settings.json` file
    pub fn location() -> Option<OsString> {
        match Settings::get_program_dir() {
            Some(mut os) => {
                os.push("/settings.json");
                Some(os)
            },
            None => None,
        }
    }

    /// Write the [`Settings`] to the `settings.json` file
    pub fn save(&self) -> Result<()> {
        let path = Settings::location().ok_or_else(|| Error::new(ErrorKind::SettingsReadFailure))?;
        let file = fs::OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(path)?;
        serde_json::to_writer_pretty(file, &self)?;

        Ok(())
    }

    /// Set the new set name
    pub fn set_set(&mut self, name: &str) {
        self.set = String::from(name);
    }
}
