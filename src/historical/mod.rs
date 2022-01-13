mod api_structs;
mod convert;

use crate::ergast;
use crate::Constructor;
use crate::Driver;
use crate::F1Error;
use crate::SessionType;
use chrono::Local;
use chrono::NaiveTime;
use serde::Deserialize;

use self::api_structs::ErgastCircuit;
use self::api_structs::QualiResult;
use self::api_structs::RaceResult;

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
    circuit: ErgastCircuit,
}

impl ErgastWeekend {
    pub fn new(year: i32, round: i32) -> Result<Self, F1Error> {
        let url = format!("{}/{}", year, round);
        let v = ergast::get(url)?;
        let c: ErgastCircuit =
            serde_json::from_value(v["RaceTable"]["Races"][0]["Circuit"].clone())
                .map_err(|_| F1Error::JsonDeserialization)?;

        Ok(Self {
            year,
            round,
            name: v["RaceTable"]["Races"][0]["raceName"]
                .as_str()
                .unwrap_or_default()
                .parse()
                .unwrap_or_default(),
            circuit: c,
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
