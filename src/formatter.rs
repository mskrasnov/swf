//! Formats and outputs weather information to the terminal

use chrono::prelude::*;
use colored::Colorize;
use serde_json::Value;

use crate::ascii::Ascii;
use crate::conf::Units;
use crate::weather::get_location_time;

pub enum FmtMode {
    All,
    WeatherType,
    Temperature,
    FeelsLike,
    Pressure,
}

fn val_to_str(data: &Value) -> String {
    /* да, можно было воспользоваться data.as_str(), но тогда
     * пришлось бы обрабатывать случай, когда нам попался None
     */
    data.to_string().replace('"', "")
}

impl FmtMode {
    fn location(&self, data: &Value) -> String {
        format!(
            "{}/{}",
            val_to_str(&data["sys"]["country"]),
            val_to_str(&data["name"]),
        )
    }

    fn fmt_hour_minute(&self, num: u32) -> String {
        if num < 10 {
            format!("0{num}")
        } else {
            num.to_string()
        }
    }

    fn datetime(&self, data: &Value) -> String {
        let date = get_location_time(data).unwrap();
        let hour = date.hour();
        let min = date.minute();

        format!(
            "{}{}{}",
            self.fmt_hour_minute(hour),
            ":".bold().blink(),
            self.fmt_hour_minute(min),
        )
    }

    fn weather_type(&self, data: &Value) -> String {
        format!("{}", val_to_str(&data["weather"][0]["description"]))
    }

    fn temperature(&self, data: &Value, units: Units) -> String {
        format!(
            "{}°{}",
            data["main"]["temp"],
            match units {
                Units::Metric => "C",
                Units::Imperial => "F",
            }
        )
    }

    fn feels_like(&self, data: &Value, units: Units) -> String {
        format!(
            "feels like {}°{}",
            data["main"]["feels_like"],
            match units {
                Units::Metric => "C",
                Units::Imperial => "F",
            }
        )
    }

    fn _pressure(&self, data: &Value) -> String {
        format!("{}", data["main"]["pressure"])
    }

    pub fn print(&self, data: &Value, units: Units) {
        match self {
            Self::All => {
                let art = Ascii::from_id(data).art1();

                for i in art.iter().enumerate() {
                    println!(
                        "{} {}",
                        i.1,
                        match i.0 {
                            0 => self.location(data).bold().to_string(),
                            1 => self.datetime(data),
                            2 => self.weather_type(data),
                            3 => self.temperature(data, units.clone()),
                            4 => self.feels_like(data, units.clone()),
                            _ => "".to_string(),
                        }
                    );
                }
            }
            Self::WeatherType => println!("{}", self.weather_type(data)),
            Self::Temperature => println!("{}", self.temperature(data, units.clone())),
            Self::FeelsLike => println!("{}", self.feels_like(data, units.clone())),
            _ => {}
        }
    }
}
