use super::super::settings::Settings;
use std::{env, ffi::OsString};

#[allow(non_snake_case)]
#[test]
fn get_program_dir_with_JAVA_ENV_MANAGER_HOME_set() {
    let value = OsString::from(".java-env-manager");
    env::set_var("JAVA_ENV_MANAGER_HOME", &value);
    let program_dir = Settings::get_program_dir();

    match program_dir {
        Some(p) => assert_eq!(value, p),
        None => panic!("Unable to read back environment variable"),
    }
}

#[allow(non_snake_case)]
#[test]
fn get_program_dir_with_XDG_CONFIG_HOME_set() {
    let mut value = OsString::from(".config");
    env::set_var("XDG_CONFIG_HOME", &value);
    let program_dir = Settings::get_program_dir();

    value.push("/java-env-manager");
    match program_dir {
        Some(p) => assert_eq!(value, p),
        None => panic!("Unable to read back environment variable"),
    }
}
