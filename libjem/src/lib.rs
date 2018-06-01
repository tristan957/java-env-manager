//! `libjem` serves as a backend for the command line executable and GUI.
//! Each command is separated into its own module. More testing to come.

#[macro_use]

extern crate serde_derive;

pub mod add;
pub mod doctor;
pub mod error;
pub mod init;
pub mod remove;
pub mod set;
pub mod settings;
pub mod update;
pub mod version;
pub mod which;

#[cfg(test)]
mod tests;
