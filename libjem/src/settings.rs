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
    pub fn new<S, T>(name: S, path: T) -> Self
    where
        S: Into<String>,
        T: Into<OsString>,
    {
        Distribution {
            name: name.into(),
            path: path.into(),
        }
    }

    /// Set the path of the [`Distribution`]
    pub fn set_path<T>(&mut self, path: T)
    where
        T: Into<OsString>,
    {
        self.path = path.into();
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

    /// Gets the location of the program directory.
    /// This method will search through the following until one exists
    ///
    /// 1. `$JAVA_ENV_MANAGER_HOME`
    /// 2. `$XDG_CONFIG_HOME/java-env-manager`
    /// 3. `$HOME/.config/java-env-manager`
    /// 4. `$HOME/.java-env-manager`
    pub fn get_program_dir() -> Option<OsString> {
        env::var_os("JAVA_ENV_MANAGER_HOME").or_else(|| {
            env::var_os("XDG_CONFIG_HOME")
                .and_then(|mut path| {
                    path.push("/java-env-manager");
                    Some(path)
                })
                .or_else(|| {
                    env::home_dir()
                        .and_then(|p| {
                            let path = p.join(".config");
                            if path.as_path().exists() {
                                return Some(path.join("java-env-manager").into())
                            }
                            None
                        })
                        .or_else(|| {
                            env::home_dir().and_then(|p| Some(p.join(".java-env-manager").into()))
                        })
                })
        })
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
    pub fn set_set<T>(&mut self, name: T)
    where
        T: Into<String>,
    {
        self.set = name.into();
    }
}
