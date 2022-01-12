use crate::historical::get_season;
use crate::{F1Error, Weekend};

pub struct Season {
    year: i32,
    pub weekends: Vec<Weekend>,
}

impl Season {
    pub fn new(year: i32) -> Result<Self, F1Error> {
        Ok(Self {
            year,
            weekends: get_season(year)?
                .into_iter()
                .map(std::convert::Into::into)
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_bahrain_2021_quali() {
        let season = Season::new(2021).unwrap();
        let bahrain = season.weekends.first().unwrap();

        let quali = bahrain.sessions.first().unwrap();

        assert_eq!(quali.standings[0].driver.screen_name, "VER");
        assert_eq!(quali.standings[0].position, 1);
        assert_eq!(
            quali.standings[0].lap_time,
            NaiveTime::from_hms_milli(0, 1, 28, 997)
        );

        assert_eq!(quali.standings[10].driver.screen_name, "PER");
        assert_eq!(quali.standings[10].position, 11);
        assert_eq!(
            quali.standings[10].lap_time,
            NaiveTime::from_hms_milli(0, 1, 30, 659)
        );
    }

    #[test]
    fn test_bahrain_2021_race() {
        let season = Season::new(2021).unwrap();
        let bahrain = season.weekends.first().unwrap();

        let race = bahrain.sessions.last().unwrap();

        assert_eq!(race.standings[0].driver.screen_name, "HAM");
        assert_eq!(race.standings[0].position, 1);
        assert_eq!(
            race.standings[0].lap_time,
            NaiveTime::from_hms_milli(0, 1, 34, 015)
        );

        assert_eq!(race.standings[1].driver.screen_name, "VER");
        assert_eq!(race.standings[1].position, 2);
        assert_eq!(
            race.standings[1].lap_time,
            NaiveTime::from_hms_milli(0, 1, 33, 228)
        );
    }

    #[test]
    fn test_2021_season() {
        let season = Season::new(2021).unwrap();
        assert_eq!(season.year, 2021);

        let first_race = season.weekends.first().unwrap();
        assert_eq!(first_race.name, "Bahrain Grand Prix");
        assert_eq!(
            first_race.sessions.first().unwrap().r#type,
            SessionType::Qualifying
        );
        assert_eq!(
            first_race.sessions.last().unwrap().r#type,
            SessionType::Race
        );
    }
}
