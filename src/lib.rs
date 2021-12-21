#![allow(dead_code, unused_imports)]
mod historical;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum F1Error {
    #[error("API JSON deserialization problem")]
    JsonDeserialization,
    #[error("API not reachable")]
    ApiNotReachable,
}
