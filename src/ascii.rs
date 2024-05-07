use chrono::prelude::*;
use colored::Colorize;
use serde_json::Value;

use crate::weather::get_location_time;

pub enum Ascii {
    Sunny,
    Night,
    Rain,
    Thunder,
    Cloudy,
    Snow,
    Other,
}

impl Ascii {
    pub fn from_id(data: &Value) -> Self {
        let id = data["weather"][0]["id"]
            .to_string()
            .parse::<usize>()
            .unwrap();

        match id {
            200..=232 => Self::Thunder,
            300..=321 | 500..=531 => Self::Rain,
            600..=622 => Self::Snow,
            800 => {
                let date = get_location_time(data).unwrap();

                match date.hour() {
                    7..=18 => Self::Sunny,
                    _ => Self::Night,
                }
            }
            801..=804 => Self::Cloudy,
            _ => Self::Other,
        }
    }

    pub fn art1(&self) -> [String; 5] {
        match self {
            Self::Sunny => [
                format!("    {}   {}   ", "\\".yellow(), "/".yellow()),
                format!(
                    "     {}{}{}    ",
                    ".".yellow().bold(),
                    "-".yellow().bold(),
                    ".".yellow().bold(),
                ),
                format!(
                    "  {} {}   {} {} ",
                    "-".yellow(),
                    "(".yellow().bold(),
                    ")".yellow().bold(),
                    "-".yellow(),
                ),
                format!(
                    "     {}{}{}    ",
                    "`".yellow().bold(),
                    "-".yellow().bold(),
                    "'".yellow().bold(),
                ),
                format!("    {}   {}   ", "/".yellow(), "\\".yellow()),
            ],
            Self::Rain => [
                format!(
                    " {}{}{}{}{}.-.   ",
                    "_".yellow().bold(),
                    "`".yellow(),
                    "/".yellow().bold(),
                    "\"".yellow(),
                    "\"".bright_yellow(),
                ),
                format!(
                    "  {}{}{}(   ){} ",
                    ",".yellow(),
                    "\\".yellow().bold(),
                    "_".bright_yellow(),
                    ".".bright_white()
                ),
                format!(
                    "   {}{}{}{}{}{}",
                    "/".yellow(),
                    "(".bright_black(),
                    "___".bright_blue(),
                    "(".bright_black(),
                    "__".magenta(),
                    ")".bright_black(),
                ),
                format!("     {}", ", , , ,".blue()),
                format!("    {} ", ", , , ,".bright_blue()),
            ],
            Self::Cloudy => [
                format!("            "),
                format!("     .--.   "),
                format!("  .-(    ). "),
                format!(" (___.__)__)"),
                format!("            "),
            ],
            Self::Snow => [
                format!("     .--.   "),
                format!("  .-(    ). "),
                format!(
                    " ({}{}{}){})",
                    "___".bright_black(),
                    ".".bright_white(),
                    "__".bright_black(),
                    "__".bright_black()
                ),
                format!(
                    "   {b} {w} {b} {w} {b}",
                    b = "*".bright_black(),
                    w = "*".bright_white()
                ),
                format!(
                    "  {w} * {b} * {w} ",
                    b = "*".bright_black(),
                    w = "*".bright_white()
                ),
            ],
            Self::Night => [
                format!("  x        {}", "*".red()),
                format!(
                    "  {}{}    {}   ",
                    "/".blue(),
                    "|".bright_blue(),
                    "*".magenta()
                ),
                format!(
                    " {} {}  {}  {}  ",
                    "|".bright_blue(),
                    "|".bright_white(),
                    "*".blue(),
                    "*".yellow().bold()
                ),
                format!("  {}{}   *    ", "\\".bright_white(), "|".bright_black()),
                format!("     *    x "),
            ],
            _ => [
                format!(""),
                format!(""),
                format!(""),
                format!(""),
                format!(""),
            ],
        }
    }
}
