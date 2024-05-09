use clap::Parser;

static LONG_ABOUT: &str = "SWF is free software. You can thank the author for his work
by sending him a donation to the card: 2202 2062 5233 5406
(Россия, Сбербанк, Михаил Сергеевич).";

#[derive(Debug, Parser)]
#[command(version, about = None)]
#[command(long_about = LONG_ABOUT)]
pub struct Cli {
    /// Outputs the specified information to the terminal
    #[arg(short, long, default_value_t = String::from("all"))]
    pub display: String,

    /// Units in which to display information
    #[arg(short, long, default_value_t = crate::conf::Units::default().to_string())]
    pub units: String,

    /// API key for OpenWeatherMap
    #[arg(short, long)]
    pub api_key: Option<String>,

    /// Location where you want to display weather information
    #[arg(short, long)]
    pub location: Option<String>,
}
