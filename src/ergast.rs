use crate::F1Error;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

const BASE_URL: &str = "https://ergast.com/api/f1";

lazy_static! {
    static ref CACHE: Mutex<HashMap<String, serde_json::Value>> = Mutex::new(HashMap::new());
}

pub fn get(url: String) -> Result<serde_json::Value, F1Error> {
    if CACHE.lock().map_err(|_| F1Error::Cache)?.contains_key(&url) {
        return Ok(CACHE.lock().map_err(|_| F1Error::Cache)?[&url].clone());
    }

    let r = reqwest::blocking::get(format!("{}/{}.json", BASE_URL, url))
        .map_err(|_| F1Error::ApiNotReachable)?;

    let v: serde_json::Value = r.json().map_err(|_| F1Error::JsonDeserialization)?;

    CACHE
        .lock()
        .map_err(|_| F1Error::Cache)?
        .insert(url, v["MRData"].clone());

    Ok(v["MRData"].clone())
}
