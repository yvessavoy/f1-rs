use crate::{historical::ErgastWeekend, Circuit, F1Error, Session};

#[derive(Clone, Debug, Default)]
pub struct Weekend {
    pub name: String,
    pub sessions: Vec<Session>,
    pub circuit: Circuit,
}

impl Weekend {
    pub fn new(year: i32, round: i32) -> Result<Self, F1Error> {
        Ok(ErgastWeekend::new(year, round)?.into())
    }
}
