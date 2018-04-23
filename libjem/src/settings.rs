// extern crate serde;
extern crate serde_json;
extern crate serde;

// use std::fs::File;
use std::env;
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
    pub fn get() -> Option<String> {
        env::var("JAVA_ENV_MANAGER_HOME").ok().or(
            env::home_dir().and_then(|path|
                path.join(".java-env-manager/").to_str()
                    .map(|s| s.to_string())))
    }
}
