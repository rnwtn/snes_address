use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AddressError {
    #[error("The translated address would be out of bounds")]
    OutOfBounds,
}
