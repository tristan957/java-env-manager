#[macro_use]

extern crate serde_derive;

pub mod add;
pub mod doctor;
pub mod init;
pub mod remove;
pub mod set;
pub mod settings;
pub mod update;
pub mod version;
pub mod which;

#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
