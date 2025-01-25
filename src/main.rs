pub mod ascii;
pub mod cli;
pub mod conf;
pub mod formatter;
pub mod weather;

// use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use std::{process::exit, str::FromStr};

use cli::Cli;
use conf::{get_default_conf_path, Conf, Units};
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
    let conf = Conf::parse(get_default_conf_path()?).await?;

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
                the config ({}) or in the \
                --api-key=... key.",
                get_default_conf_path()?.display(),
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

    if data.get("cod") == Some(&serde_json::Value::String("404".to_string())) {
        eprintln!("{}: {}", "Error".red().bold(), data["message"].as_str().unwrap_or("Unknown error"));
        exit(1);
    }

    fmt_mode.print(&data, weather.units.clone());

    Ok(())
}
