pub mod ascii;
pub mod cli;
pub mod conf;
pub mod formatter;
pub mod weather;

// use anyhow::Result;
use clap::Parser;
use std::{path::Path, process::exit, str::FromStr};

use cli::Cli;
use conf::{Conf, Units};
use formatter::FmtMode;
use weather::Weather;

fn get_data(cli: Option<String>, conf: Option<String>) -> Option<String> {
    if cli.is_none() && conf.is_none() {
        None
    } else if cli.is_some() {
        cli
    } else {
        conf
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let fmt_mode = match &cli.display as &str {
        "all" => FmtMode::All,
        "temperature" => FmtMode::Temperature,
        "weather-type" => FmtMode::WeatherType,
        "feels-like" => FmtMode::FeelsLike,
        "pressure" => FmtMode::Pressure,
        _ => {
            eprintln!("error: unknown mode {}", &cli.display);
            exit(1)
        }
    };
    let conf = Conf::parse(Path::new(&conf::home()?).join(".config").join("swf.toml")).await?;

    let location = match get_data(cli.location, conf.location) {
        Some(loc) => loc,
        None => {
            eprintln!(
                "You must explicitly specify the location in \
                the config (~/.config/swf.toml) or in the \
                --location=... key."
            );
            exit(1)
        }
    };

    let key = match get_data(cli.api_key, conf.api_key) {
        Some(key) => key,
        None => {
            eprintln!(
                "You must explicitly specify the API key in \
                the config (~/.config/swf.toml) or in the \
                --api-key=... key."
            );
            exit(1)
        }
    };

    let units = get_data(
        Some(cli.units),
        Some(conf.units.unwrap_or_default().to_string()),
    )
    .unwrap();

    let weather =
        Weather::new(&location, &key).set_units_type(Units::from_str(&units).unwrap_or_default());
    let data = weather.get().await?;

    fmt_mode.print(&data, weather.units.clone());

    Ok(())
}
