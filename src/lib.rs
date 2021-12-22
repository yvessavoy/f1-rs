#![allow(dead_code, unused_imports)]
mod ergast;
pub mod historical;

use chrono::prelude::*;
use std::collections::HashMap;
use std::sync::MutexGuard;
use std::sync::PoisonError;
use thiserror::Error;

const EARLIEST_SEASON: i32 = 1950;

#[derive(Error, Debug)]
pub enum F1Error {
    #[error("API JSON deserialization problem")]
    JsonDeserialization,
    #[error("API not reachable")]
    ApiNotReachable,
    #[error("Cache Error")]
    Cache,
}

pub fn get_available_seasons() -> Vec<i32> {
    (EARLIEST_SEASON..Utc::now().year() + 1).rev().collect()
}
