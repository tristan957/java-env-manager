#[macro_use]
extern crate serde_derive;

pub mod add;
pub mod settings;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}