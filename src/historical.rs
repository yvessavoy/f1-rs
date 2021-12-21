use crate::F1Error;
use serde::Deserialize;

const BASE_URL: &str = "https://ergast.com/api/f1";

pub struct Weekend {
    year: i32,
    round: i32,
}

impl Weekend {
    pub fn new(year: i32, round: i32) -> Self {
        Self { year, round }
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
    use super::Weekend;

    #[test]
    fn test_quali_results() {
        let weekend = Weekend::new(2021, 22);
        let results = weekend.qualifying_results().unwrap();
        let max = results[0].clone();
        assert_eq!(max.position, "1");
    }

    #[test]
    fn test_race_results() {
        let weekend = Weekend::new(2021, 22);
        let results = weekend.race_results().unwrap();
        let max = results[0].clone();
        assert_eq!(max.position, "1");
    }
}
