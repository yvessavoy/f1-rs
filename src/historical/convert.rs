use crate::historical::api_structs::{
    ErgastCircuit, ErgastConstructor, ErgastDriver, QualiResult, RaceResult,
};
use crate::{Circuit, Constructor, Driver, Season, SessionType, Standing, Weekend};
use chrono::NaiveTime;

use super::ErgastWeekend;

impl From<ErgastConstructor> for Constructor {
    fn from(c: ErgastConstructor) -> Self {
        Self {
            name: c.name,
            id: c.constructor_id,
        }
    }
}

impl From<ErgastDriver> for Driver {
    fn from(d: ErgastDriver) -> Self {
        Self {
            id: d.driver_id,
            first_name: d.given_name,
            last_name: d.family_name,
            screen_name: d.code,
        }
    }
}

impl From<RaceResult> for crate::Standing {
    fn from(r: RaceResult) -> Self {
        let splits = r.fastest_lap.time.time.split(':').collect::<Vec<&str>>();
        let mut minutes: u32 = splits.first().unwrap_or(&"0").parse().unwrap_or_default();
        let splits = splits
            .last()
            .unwrap_or(&"0")
            .split('.')
            .collect::<Vec<&str>>();
        let mut seconds: u32 = splits.first().unwrap_or(&"0").parse().unwrap_or_default();
        let millis: u32 = splits.last().unwrap_or(&"0").parse().unwrap_or_default();

        if seconds > 59 {
            minutes += seconds / 60;
            seconds = seconds % 60;
        }

        let parsed_time = NaiveTime::from_hms_milli(0, minutes, seconds, millis);

        Self {
            driver: r.driver.into(),
            constructor: r.constructor.into(),
            position: r.position.parse().unwrap_or_default(),
            lap_time: parsed_time,
        }
    }
}

impl From<Vec<RaceResult>> for crate::Session {
    fn from(v: Vec<RaceResult>) -> Self {
        Self {
            r#type: SessionType::Race,
            finished: true, // Ergast data is historical, so it's always finished
            standings: v.into_iter().map(std::convert::From::from).collect(),
        }
    }
}

impl From<QualiResult> for crate::Standing {
    fn from(r: QualiResult) -> Self {
        let q_time = if !r.q3_time.is_empty() {
            r.q3_time
        } else if !r.q2_time.is_empty() {
            r.q2_time
        } else {
            r.q1_time
        };

        let splits = q_time.split(':').collect::<Vec<&str>>();
        let minutes: u32 = splits.first().unwrap_or(&"0").parse().unwrap_or_default();
        let splits = splits
            .last()
            .unwrap_or(&"0")
            .split('.')
            .collect::<Vec<&str>>();
        let seconds: u32 = splits.first().unwrap_or(&"0").parse().unwrap_or_default();
        let millis: u32 = splits.last().unwrap_or(&"0").parse().unwrap_or_default();

        let parsed_time = NaiveTime::from_hms_milli(0, minutes, seconds, millis);

        Self {
            driver: r.driver.into(),
            constructor: r.constructor.into(),
            position: r.position.parse().unwrap_or_default(),
            lap_time: parsed_time,
        }
    }
}

impl From<Vec<QualiResult>> for crate::Session {
    fn from(v: Vec<QualiResult>) -> Self {
        Self {
            r#type: SessionType::Qualifying,
            finished: true, // Ergast data is historical, so it's always finished
            standings: v.into_iter().map(std::convert::From::from).collect(),
        }
    }
}

impl From<ErgastCircuit> for crate::Circuit {
    fn from(c: ErgastCircuit) -> Self {
        Self {
            latitude: c.location.lat.parse().unwrap_or_default(),
            longitude: c.location.long.parse().unwrap_or_default(),
            locality: c.location.locality,
            country: c.location.country,
            id: c.id,
            name: c.name,
        }
    }
}

impl From<ErgastWeekend> for crate::Weekend {
    fn from(w: ErgastWeekend) -> Self {
        let sessions = vec![
            w.qualifying_results().unwrap_or_default().into(),
            w.race_results().unwrap_or_default().into(),
        ];

        Self {
            name: w.name,
            sessions,
            circuit: w.circuit.into(),
        }
    }
}
