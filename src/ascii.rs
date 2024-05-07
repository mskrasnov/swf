use colored::Color;
// use colored::Colorize;
use colored::ColoredString;
use colored::Colorize;
use serde_json::Value;

pub enum Ascii {
    Sunny,
    Night,
    Rain,
    Thunder,
    Cloudy,
    // PartlyCloudy,
    Snow,
    Other,
}

pub struct Art {
    pub domination_color: Color,
    pub art: [ColoredString; 3],
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
            800 => Self::Sunny,
            801..=804 => Self::Cloudy,
            _ => Self::Other,
        }
    }

    pub fn art1(&self) -> [String; 5] {
        match self {
            Self::Sunny => [
                format!("   {}   {}   ", "\\".yellow(), "/".yellow()),
                format!(
                    "    {}{}{}    ",
                    ".".yellow().bold(),
                    "-".yellow().bold(),
                    ".".yellow().bold(),
                ),
                format!(
                    " {} {}   {} {} ",
                    "-".yellow(),
                    "(".yellow().bold(),
                    ")".yellow().bold(),
                    "-".yellow(),
                ),
                format!(
                    "    {}{}{}    ",
                    "`".yellow().bold(),
                    "-".yellow().bold(),
                    "'".yellow().bold(),
                ),
                format!("   {}   {}   ", "/".yellow(), "\\".yellow()),
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
            Self::Snow => [
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
                    "___".bright_black(),
                    "(".bright_black(),
                    "__".magenta(),
                    ")".white(),
                ),
                format!("     {}", "* * * *".bright_black()),
                format!("    {} ", "* * * *".white()),
            ],
            Self::Cloudy => [
                format!("            "),
                format!("     .--.   "),
                format!("  .-(    ). "),
                format!(" (___.__)__)"),
                format!("            "),
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
