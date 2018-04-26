use std::ffi::OsStr;

pub fn command(name: &str, path: &OsStr) {
    println!("{} {}", name, path.to_str().unwrap());
}
