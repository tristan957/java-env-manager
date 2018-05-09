use std::ffi::OsStr;

pub fn add(name: &str, path: &OsStr) {
    println!("{} {}", name, path.to_str().unwrap());
}
