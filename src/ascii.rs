use colored::Color;
// use colored::Colorize;
use colored::ColoredString;
use colored::Colorize;

pub enum Ascii {
    Sunny,
    Night,
    Rain,
    Thunder,
    Cloudy,
    PartlyCloudy,
    Other,
}

pub struct Art {
    pub domination_color: Color,
    pub art: [ColoredString; 3],
}

impl Ascii {
    pub fn art(&self) -> [&str; 5] {
        match self {
            Self::Sunny => ["   \\   /", "    .-.", " - (   ) -", "    `_,", "   /   \\"],
            Self::Night => ["", "", "", "", ""],
            Self::Rain => [
                " _`/\"\".-.",
                "  ,\\_(   ).",
                "   /(___(__)",
                "     , , , ,",
                "    , , , ,",
            ],
            Self::Thunder => ["", "", "", "", ""],
            Self::Cloudy => ["", "", "", "", ""],
            Self::PartlyCloudy => ["", "", "", "", ""],
            Self::Other => ["", "", "", "", ""],
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
