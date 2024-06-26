//! Getting information about the weather in a given location

use anyhow::Result;
use chrono::prelude::*;
use reqwest;
use serde_json::Value;

use crate::conf::Units;

#[derive(Debug)]
pub struct Weather<'a> {
    location: &'a str,
    key: &'a str,
    pub units: Units,
}

impl<'a> Weather<'a> {
    pub fn new(loc: &'a str, key: &'a str) -> Self {
        Self {
            location: loc,
            units: Units::default(),
            key,
        }
    }

    pub fn set_units_type(mut self, units: Units) -> Self {
        self.units = units;
        self
    }

    fn fmt_get_query(&self) -> String {
        format!(
            "https://api.openweathermap.org/data/2.5/weather?appid={}&q={}&units={}",
            self.key,
            self.location,
            self.units.to_string()
        )
    }

    pub async fn get(&self) -> Result<Value> {
        let query = reqwest::get(&self.fmt_get_query())
            .await?
            .json::<Value>()
            .await?;
        Ok(query)
    }
}

pub fn get_location_time(data: &Value) -> Result<DateTime<FixedOffset>> {
    let dt = data["dt"].to_string().parse::<i64>()?;
    let tz = data["timezone"].to_string().parse::<i32>()?;

    let date = DateTime::from_timestamp(dt, 0)
        .unwrap_or_default()
        .with_timezone(&FixedOffset::east_opt(tz).unwrap());

    Ok(date)
}
