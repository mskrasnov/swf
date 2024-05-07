pub mod ascii;
pub mod cli;
pub mod conf;
pub mod formatter;
pub mod weather;

// use anyhow::Result;
use clap::Parser;
use std::process::exit;

use cli::Cli;
use conf::Conf;
use formatter::FmtMode;
use weather::Weather;

fn get_data<'a>(cl: &'a Cli, cn: &'a Conf) -> Result<Weather<'a>, String> {
    // Oh shit, I'm sorry. But it works

    let loc = if let Some(loc) = &cl.location {
        loc
    } else {
        if let Some(loc) = &cn.location {
            loc
        } else {
            return Err("You must explicitly specify the location in \
                        the config (~/.config/swf.toml) or in the \
                        --location=... key"
                .to_string());
        }
    };

    let key = if let Some(key) = &cl.api_key {
        key
    } else {
        if let Some(key) = &cn.api_key {
            key
        } else {
            return Err("You must explicitly specify the API Key in \
                        the config (~/.config/swf.toml) or in the \
                        --api-key=... key"
                .to_string());
        }
    };

    let weather = Weather::new(loc, key);

    Ok(weather)
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
    let conf = Conf::parse("./swf.toml").await?;

    let weather = get_data(&cli, &conf).unwrap();
    let data = weather.get().await?;

    fmt_mode.print(&data, weather.units.clone());

    Ok(())
}
