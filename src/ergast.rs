use serde::Deserialize;

const BASE_URL: &str = "https://ergast.com/api/f1";

#[derive(Deserialize)]
struct ErgastQualiResponse {
    #[serde(rename = "MRData")]
    data: ErgastQualiData,
}

#[derive(Deserialize)]
struct ErgastRaceResponse {
    #[serde(rename = "MRData")]
    data: ErgastRaceData,
}

#[derive(Deserialize)]
struct ErgastQualiData {
    #[serde(rename = "RaceTable")]
    race_table: QualiTable,
}

#[derive(Deserialize)]
struct ErgastRaceData {
    #[serde(rename = "RaceTable")]
    race_table: RaceTable,
}

#[derive(Deserialize)]
struct QualiTable {
    #[serde(rename = "Races")]
    races: Vec<Quali>,
}

#[derive(Deserialize)]
struct RaceTable {
    #[serde(rename = "Races")]
    races: Vec<Race>,
}

#[derive(Deserialize)]
struct Quali {
    #[serde(rename = "raceName")]
    race_name: String,
    date: String,
    time: String,
    #[serde(rename = "QualifyingResults")]
    results: Vec<QualiResult>,
}

#[derive(Deserialize)]
struct Race {
    #[serde(rename = "raceName")]
    race_name: String,
    date: String,
    time: String,
    #[serde(rename = "Results")]
    results: Vec<RaceResult>,
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

pub fn get_quali_results(year: i32, weekend: i32) -> Vec<QualiResult> {
    let url = format!("{}/{}/{}/qualifying.json", BASE_URL, year, weekend);
    let r: ErgastQualiResponse = reqwest::blocking::get(url).unwrap().json().unwrap();

    r.data.race_table.races[0].results.clone()
}

pub fn get_race_results(year: i32, weekend: i32) -> Vec<RaceResult> {
    let url = format!("{}/{}/{}/results.json", BASE_URL, year, weekend);
    let r: ErgastRaceResponse = reqwest::blocking::get(url).unwrap().json().unwrap();

    r.data.race_table.races[0].results.clone()
}

#[cfg(test)]
mod tests {
    use crate::ergast::get_race_results;

    use super::get_quali_results;

    #[test]
    fn test_quali_results() {
        let results = get_quali_results(2021, 22);
        let max = results[0].clone();
        assert_eq!(max.position, "1");
    }

    #[test]
    fn test_race_results() {
        let results = get_race_results(2021, 22);
        let max = results[0].clone();
        assert_eq!(max.position, "1");
    }
}
