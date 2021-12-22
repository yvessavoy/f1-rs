use crate::F1Error;
use serde::Deserialize;

const BASE_URL: &str = "https://ergast.com/api/f1";

pub fn get_season(year: i32) -> Result<Vec<Weekend>, F1Error> {
    let url = format!("{}/{}.json", BASE_URL, year);
    let r = reqwest::blocking::get(url).map_err(|_| F1Error::ApiNotReachable)?;
    let v: serde_json::Value = r.json().map_err(|_| F1Error::JsonDeserialization)?;
    let weekends = v["MRData"]["RaceTable"]["Races"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| {
            Weekend::new(
                item["season"].as_str().unwrap().parse().unwrap(),
                item["round"].as_str().unwrap().parse().unwrap(),
            )
            .unwrap()
        })
        .collect();

    Ok(weekends)
}

pub struct Weekend {
    year: i32,
    round: i32,
    name: String,
}

impl Weekend {
    pub fn new(year: i32, round: i32) -> Result<Self, F1Error> {
        let url = format!("{}/{}/{}.json", BASE_URL, year, round);
        let r = reqwest::blocking::get(url).map_err(|_| F1Error::ApiNotReachable)?;
        let v: serde_json::Value = r.json().map_err(|_| F1Error::JsonDeserialization)?;

        Ok(Self {
            year,
            round,
            name: v["MRData"]["RaceTable"]["Races"][0]["raceName"]
                .as_str()
                .unwrap_or_default()
                .parse()
                .unwrap_or_default(),
        })
    }

    pub fn race_results(&self) -> Result<Vec<RaceResult>, F1Error> {
        let url = format!("{}/{}/{}/results.json", BASE_URL, self.year, self.round);
        let r = reqwest::blocking::get(url).map_err(|_| F1Error::ApiNotReachable)?;
        let v: serde_json::Value = r.json().map_err(|_| F1Error::JsonDeserialization)?;

        let ret: Vec<RaceResult> =
            serde_json::from_value(v["MRData"]["RaceTable"]["Races"][0]["Results"].clone())
                .map_err(|_| F1Error::JsonDeserialization)?;

        Ok(ret)
    }

    pub fn qualifying_results(&self) -> Result<Vec<QualiResult>, F1Error> {
        let url = format!("{}/{}/{}/qualifying.json", BASE_URL, self.year, self.round);
        let r = reqwest::blocking::get(url).map_err(|_| F1Error::ApiNotReachable)?;
        let v: serde_json::Value = r.json().map_err(|_| F1Error::JsonDeserialization)?;

        let ret: Vec<QualiResult> = serde_json::from_value(
            v["MRData"]["RaceTable"]["Races"][0]["QualifyingResults"].clone(),
        )
        .map_err(|_| F1Error::JsonDeserialization)?;

        Ok(ret)
    }
}

#[derive(Deserialize, Clone)]
pub struct QualiResult {
    number: String,
    position: String,
    #[serde(rename = "Driver")]
    driver: Driver,
    #[serde(rename = "Constructor")]
    constructor: Constructor,
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
pub struct RaceResult {
    number: String,
    position: String,
    #[serde(rename = "Driver")]
    driver: Driver,
    #[serde(rename = "Constructor")]
    constructor: Constructor,
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
struct Driver {
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
struct Constructor {
    #[serde(rename = "constructorId")]
    constructor_id: String,
    name: String,
    nationality: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quali_results() {
        let weekend = Weekend::new(2021, 22).unwrap();
        let results = weekend.qualifying_results().unwrap();
        let max = results[0].clone();
        assert_eq!(max.position, "1");
    }

    #[test]
    fn test_race_results() {
        let weekend = Weekend::new(2021, 22).unwrap();
        let results = weekend.race_results().unwrap();
        let max = results[0].clone();
        assert_eq!(max.position, "1");
    }

    #[test]
    fn test_get_weekend() {
        let weekend = Weekend::new(2021, 22).unwrap();
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
