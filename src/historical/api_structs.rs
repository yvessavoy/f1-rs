use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub(crate) struct QualiResult {
    pub number: String,
    pub position: String,
    #[serde(rename = "Driver")]
    pub driver: ErgastDriver,
    #[serde(rename = "Constructor")]
    pub constructor: ErgastConstructor,
    #[serde(default)]
    #[serde(rename = "Q1")]
    pub q1_time: String,
    #[serde(default)]
    #[serde(rename = "Q2")]
    pub q2_time: String,
    #[serde(default)]
    #[serde(rename = "Q3")]
    pub q3_time: String,
}

#[derive(Deserialize, Clone)]
pub(crate) struct RaceResult {
    pub number: String,
    pub position: String,
    #[serde(rename = "Driver")]
    pub driver: ErgastDriver,
    #[serde(rename = "Constructor")]
    pub constructor: ErgastConstructor,
    #[serde(default)]
    #[serde(rename = "FastestLap")]
    pub fastest_lap: FastestLap,
}

#[derive(Deserialize, Clone, Default)]
pub(crate) struct FastestLap {
    #[serde(default)]
    #[serde(rename = "Time")]
    pub time: Time,
}

#[derive(Deserialize, Clone, Default)]
pub(crate) struct Time {
    pub time: String,
}

#[derive(Deserialize, Clone, Default)]
pub(crate) struct ErgastCircuit {
    #[serde(rename = "circuitId")]
    pub id: String,
    #[serde(rename = "circuitName")]
    pub name: String,
    #[serde(rename = "Location")]
    pub location: Location,
}

#[derive(Deserialize, Clone, Default)]
pub(crate) struct Location {
    pub lat: String,
    pub long: String,
    pub locality: String,
    pub country: String,
}

#[derive(Deserialize, Clone)]
pub(crate) struct ErgastDriver {
    #[serde(rename = "driverId")]
    pub driver_id: String,
    #[serde(rename = "permanentNumber")]
    pub permanent_number: String,
    pub code: String,
    #[serde(rename = "givenName")]
    pub given_name: String,
    #[serde(rename = "familyName")]
    pub family_name: String,
    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: String,
    pub nationality: String,
}

#[derive(Deserialize, Clone)]
pub(crate) struct ErgastConstructor {
    #[serde(rename = "constructorId")]
    pub constructor_id: String,
    pub name: String,
    pub nationality: String,
}
