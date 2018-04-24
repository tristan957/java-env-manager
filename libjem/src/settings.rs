// extern crate serde;
extern crate serde_json;
extern crate serde;

// use std::fs::File;
use std::env;
use std::ffi::OsString;
// use std::io;

#[derive(Debug, Deserialize, Serialize)]
struct Distribution {
    name: String,
    path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    distributions: Vec<Distribution>,
    set: bool,
}

impl Settings {
    pub fn get() -> Option<OsString> {
        env::var_os("JAVA_ENV_MANAGER_HOME").or_else(|| {
            env::home_dir().map(|path| {
                path.join(".java-env-manager").into()
            })
        })
    }
}
