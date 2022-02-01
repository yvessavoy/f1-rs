#![allow(dead_code, unused_imports)]
mod ergast;
mod historical;
mod livetiming;
mod season;
mod weekend;

pub use season::Season;
pub use weekend::Weekend;

use chrono::prelude::*;
use std::collections::HashMap;
use std::fmt;
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

#[derive(PartialEq, Debug, Clone)]
pub enum SessionType {
    Practice1,
    Practice2,
    Practice3,
    Qualifying,
    Sprint,
    Race,
}

impl Default for SessionType {
    fn default() -> Self {
        Self::Race
    }
}

impl fmt::Display for SessionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Constructor {
    id: String,
    name: String,
}

#[derive(Clone, Debug, Default)]
pub struct Driver {
    id: String,
    first_name: String,
    last_name: String,
    pub screen_name: String,
}

#[derive(Clone, Debug)]
pub struct Standing {
    pub driver: Driver,
    constructor: Constructor,
    pub position: i32,
    pub lap_time: chrono::NaiveTime,
}

impl Default for Standing {
    fn default() -> Self {
        Self {
            driver: Default::default(),
            constructor: Default::default(),
            position: Default::default(),
            lap_time: NaiveTime::from_hms(0, 0, 0),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Session {
    pub r#type: SessionType,
    finished: bool,
    pub standings: Vec<Standing>,
}

#[derive(Clone, Debug, Default)]
pub struct Circuit {
    id: String,
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub locality: String,
    pub country: String,
}

pub fn get_available_seasons() -> Vec<i32> {
    (EARLIEST_SEASON..Utc::now().year() + 1).rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_type_to_string() {
        let quali = SessionType::Qualifying;
        assert_eq!(quali.to_string(), "Qualifying".to_string());

        let race = SessionType::Race;
        assert_eq!(race.to_string(), "Race".to_string());
    }
}
