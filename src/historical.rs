use crate::ergast;
use crate::Constructor;
use crate::Driver;
use crate::F1Error;
use crate::SessionType;
use chrono::NaiveTime;
use serde::Deserialize;

pub fn get_season(year: i32) -> Result<Vec<ErgastWeekend>, F1Error> {
    let v = ergast::get(year.to_string())?;
    let weekends = v["RaceTable"]["Races"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| {
            ErgastWeekend::new(
                item["season"].as_str().unwrap().parse().unwrap(),
                item["round"].as_str().unwrap().parse().unwrap(),
            )
            .unwrap()
        })
        .collect();

    Ok(weekends)
}

pub struct ErgastWeekend {
    pub year: i32,
    pub round: i32,
    pub name: String,
}

impl ErgastWeekend {
    pub fn new(year: i32, round: i32) -> Result<Self, F1Error> {
        let url = format!("{}/{}", year, round);
        let v = ergast::get(url)?;

        Ok(Self {
            year,
            round,
            name: v["RaceTable"]["Races"][0]["raceName"]
                .as_str()
                .unwrap_or_default()
                .parse()
                .unwrap_or_default(),
        })
    }

    fn race_results(&self) -> Result<Vec<RaceResult>, F1Error> {
        let url = format!("{}/{}/results", self.year, self.round);
        let v = ergast::get(url)?;

        let ret: Vec<RaceResult> =
            serde_json::from_value(v["RaceTable"]["Races"][0]["Results"].clone())
                .map_err(|_| F1Error::JsonDeserialization)?;

        Ok(ret)
    }

    fn qualifying_results(&self) -> Result<Vec<QualiResult>, F1Error> {
        let url = format!("{}/{}/qualifying", self.year, self.round);
        let v = ergast::get(url)?;

        let ret: Vec<QualiResult> =
            serde_json::from_value(v["RaceTable"]["Races"][0]["QualifyingResults"].clone())
                .map_err(|_| F1Error::JsonDeserialization)?;

        Ok(ret)
    }
}

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

impl From<ErgastWeekend> for crate::Weekend {
    fn from(w: ErgastWeekend) -> Self {
        let sessions = vec![
            w.qualifying_results().unwrap_or_default().into(),
            w.race_results().unwrap_or_default().into(),
        ];

        Self {
            name: w.name,
            sessions,
        }
    }
}

#[derive(Deserialize, Clone)]
struct QualiResult {
    number: String,
    position: String,
    #[serde(rename = "Driver")]
    driver: ErgastDriver,
    #[serde(rename = "Constructor")]
    constructor: ErgastConstructor,
    #[serde(default)]
    #[serde(rename = "Q1")]
    q1_time: String,
    #[serde(default)]
    #[serde(rename = "Q2")]
    q2_time: String,
    #[serde(default)]
    #[serde(rename = "Q3")]
    q3_time: String,
}

#[derive(Deserialize, Clone)]
struct RaceResult {
    number: String,
    position: String,
    #[serde(rename = "Driver")]
    driver: ErgastDriver,
    #[serde(rename = "Constructor")]
    constructor: ErgastConstructor,
    #[serde(default)]
    #[serde(rename = "FastestLap")]
    fastest_lap: FastestLap,
}

#[derive(Deserialize, Clone, Default)]
struct FastestLap {
    #[serde(default)]
    #[serde(rename = "Time")]
    time: Time,
}

#[derive(Deserialize, Clone, Default)]
struct Time {
    time: String,
}

#[derive(Deserialize, Clone)]
struct ErgastDriver {
    #[serde(rename = "driverId")]
    driver_id: String,
    #[serde(rename = "permanentNumber")]
    permanent_number: String,
    code: String,
    #[serde(rename = "givenName")]
    given_name: String,
    #[serde(rename = "familyName")]
    family_name: String,
    #[serde(rename = "dateOfBirth")]
    date_of_birth: String,
    nationality: String,
}

#[derive(Deserialize, Clone)]
struct ErgastConstructor {
    #[serde(rename = "constructorId")]
    constructor_id: String,
    name: String,
    nationality: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_quali_results() {
        let weekend = ErgastWeekend::new(2021, 22).unwrap();
        let results = weekend.qualifying_results().unwrap();
        let max = results[0].clone();
        assert_eq!(max.position, "1");
    }

    #[test]
    fn test_quali_results_with_from() {
        let weekend = ErgastWeekend::new(2021, 22).unwrap();
        let results: Session = weekend.qualifying_results().unwrap().into();

        assert_eq!(results.r#type, SessionType::Qualifying);

        let max = &results.standings[0];
        assert_eq!(max.position, 1);
        assert_eq!(max.lap_time, NaiveTime::from_hms_milli(0, 1, 22, 109));

        let lewis = &results.standings[1];
        assert_eq!(lewis.position, 2);
        assert_eq!(lewis.lap_time, NaiveTime::from_hms_milli(0, 1, 22, 480));

        let alonso = &results.standings[10];
        assert_eq!(alonso.position, 11);
        assert_eq!(alonso.lap_time, NaiveTime::from_hms_milli(0, 1, 23, 460));
    }

    #[test]
    fn test_race_results() {
        let weekend = ErgastWeekend::new(2021, 22).unwrap();
        let results = weekend.race_results().unwrap();
        let max = results[0].clone();
        assert_eq!(max.position, "1");
    }

    #[test]
    fn test_race_results_with_from() {
        let weekend = ErgastWeekend::new(2021, 22).unwrap();
        let results: Session = weekend.race_results().unwrap().into();

        assert_eq!(results.r#type, SessionType::Race);

        let max = &results.standings[0];
        assert_eq!(max.position, 1);
        assert_eq!(max.lap_time, NaiveTime::from_hms_milli(0, 1, 26, 103));

        let lewis = &results.standings[1];
        assert_eq!(lewis.position, 2);
        assert_eq!(lewis.lap_time, NaiveTime::from_hms_milli(0, 1, 26, 615));
    }

    #[test]
    fn test_get_weekend() {
        let weekend = ErgastWeekend::new(2021, 22).unwrap();
        assert_eq!(weekend.name, "Abu Dhabi Grand Prix");
        assert_eq!(weekend.year, 2021);
        assert_eq!(weekend.round, 22);
    }

    #[test]
    fn test_get_2021_season() {
        let weekends = get_season(2021).unwrap();
        assert_eq!(weekends.len(), 22);

        let first = weekends.first().unwrap();
        assert_eq!(first.name, "Bahrain Grand Prix");
    }
}
