use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid byte value, only [a-z] and [A-Z] ASCII")]
    InvalidByteValues,

    #[error("Invalid str length, str used for construction must be 4 bytes long (aka. 4 characters)")]
    InvalidStringLength,
}
