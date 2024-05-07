//! Formats and outputs weather information to the terminal

use colored::Colorize;
use serde_json::Value;

use crate::ascii::Ascii;
use crate::conf::Units;

pub enum FmtMode {
    All,
    WeatherType,
    Temperature,
    FeelsLike,
    Pressure,
}

pub enum WeatherType {
    Thunderstorm,
    Drizzle,
    Rain,
    Snow,
    Clear,
    Clouds,
}

impl FmtMode {
    fn location(&self, data: &Value) -> String {
        format!(
            "{}/{}",
            data["sys"]["country"].to_string().replace('"', ""),
            data["name"].to_string().replace('"', "")
        )
    }

    fn weather_type(&self, data: &Value) -> String {
        format!(
            "{}",
            data["weather"][0]["main"].to_string().replace('"', "")
        )
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
                let art = Ascii::Rain.art1();

                for i in art.iter().enumerate() {
                    println!(
                        "{} {}",
                        i.1,
                        match i.0 {
                            0 => self.location(data).bold().to_string(),
                            1 => self.weather_type(data),
                            2 => self.temperature(data, units.clone()),
                            3 => self.feels_like(data, units.clone()),
                            _ => format!("2202 2062 5233 5406"),
                        }
                    );
                }
            }
            _ => {}
        }
    }
}
