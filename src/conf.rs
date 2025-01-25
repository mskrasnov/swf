//! Parsing of SWF configuration file

use anyhow::Result;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tokio::fs::read_to_string;
use toml;

pub fn home() -> Result<PathBuf> {
    match home::home_dir() {
        Some(home) => Ok(home),
        None => Err(anyhow::anyhow!("Failed to get home directory!"))
    }
}

pub fn get_default_conf_path() -> Result<PathBuf> {
    if cfg!(target_family="unix") {
        Ok(home()?.join(".config").join("swf.toml"))
    } else if cfg!(target_family="windows") {
        Ok(home()?.join("AppData").join("Roaming").join("swf.toml"))
    } else {
        Err(anyhow::anyhow!("Your OS doesn't supported yet"))
    }
}

#[derive(Debug, Deserialize)]
pub struct Conf {
    pub location: Option<String>,
    pub units: Option<Units>,
    pub api_key: Option<String>,
}

impl Conf {
    pub async fn parse<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = read_to_string(&path).await?;
        Ok(toml::from_str(&contents)?)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum Units {
    #[serde(rename(deserialize = "metric"))]
    Metric,

    #[serde(rename(deserialize = "imperial"))]
    Imperial,
}

impl Default for Units {
    fn default() -> Self {
        Self::Metric
    }
}

impl ToString for Units {
    fn to_string(&self) -> String {
        match self {
            Self::Metric => "metric",
            Self::Imperial => "imperial",
        }
        .to_string()
    }
}

impl FromStr for Units {
    type Err = String;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s {
            "metric" => Ok(Self::Metric),
            "imperial" => Ok(Self::Imperial),
            _ => Err(format!("Unknown units type: {s}")),
        }
    }
}
