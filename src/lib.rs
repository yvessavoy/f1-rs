#![allow(dead_code, unused_imports)]
mod ergast;
mod historical;
mod season;

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

#[derive(PartialEq, Debug)]
pub enum SessionType {
    Practice1,
    Practice2,
    Practice3,
    Qualifying,
    Sprint,
    Race,
}

#[derive(Debug, Default)]
pub struct Constructor {
    id: String,
    name: String,
}

pub struct Driver {
    id: String,
    first_name: String,
    last_name: String,
    screen_name: String,
}

pub struct Standing {
    driver: Driver,
    constructor: Constructor,
    position: i32,
    lap_time: chrono::NaiveTime,
}

pub struct Session {
    r#type: SessionType,
    finished: bool,
    standings: Vec<Standing>,
}

pub struct Weekend {
    name: String,
    sessions: Vec<Session>,
}

pub fn get_available_seasons() -> Vec<i32> {
    (EARLIEST_SEASON..Utc::now().year() + 1).rev().collect()
}
