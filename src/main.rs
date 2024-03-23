mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
mod png;

use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    todo!()
}
