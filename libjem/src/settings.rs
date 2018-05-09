extern crate serde;
extern crate serde_json;

use std::env;
use std::ffi::OsString;

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

    pub fn location() -> Option<OsString> {
        match Settings::get_program_dir() {
            Some(mut os) => {
                os.push("/settings.json");
                Option::from(os)
            },
            None => Option::default()
        }
    }

    pub fn get_program_dir() -> Option<OsString> {
        env::var_os("JAVA_ENV_MANAGER_HOME").or_else(|| {
            env::home_dir().map(|path| {
                path.join(".java-env-manager").into()
            })
        })
    }
}
