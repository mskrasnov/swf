//! Parsing of SWF configuration file

use anyhow::Result;
use serde::Deserialize;
use std::path::Path;
use std::str::FromStr;
use tokio::fs::read_to_string;
use toml;

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
