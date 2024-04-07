use crate::hexcolor::{Theme, ThemeColor};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use toml;

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct RsbrConfig {
    pub theme: Theme,
    pub format: String,
    pub datetime: RsbrDatetimeConfig,
    pub audio: RsbrAudioConfig,
    pub battery: RsbrBatteryConfig,
    pub brightness: RsbrBrightnessConfig,
    pub network: RsbrNetworkConfig,
}

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct RsbrDatetimeConfig {
    pub format: String,
    pub fgcolor: ThemeColor,
    pub bgcolor: ThemeColor,
}

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct RsbrBrightnessConfig {
    pub fgcolor: ThemeColor,
    pub bgcolor: ThemeColor,
}

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct RsbrBatteryConfig {
    pub fgcolor: ThemeColor,
    pub bgcolor: ThemeColor,
}

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct RsbrAudioConfig {
    pub fgcolor: ThemeColor,
    pub bgcolor: ThemeColor,
}

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct RsbrNetworkConfig {
    pub fgcolor: ThemeColor,
    pub bgcolor: ThemeColor,
}

impl Default for RsbrDatetimeConfig {
    fn default() -> Self {
        Self {
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            fgcolor: ThemeColor::from_str("white").unwrap(),
            bgcolor: ThemeColor::from_str("black").unwrap(),
        }
    }
}

impl Default for RsbrBrightnessConfig {
    fn default() -> Self {
        Self {
            fgcolor: ThemeColor::from_str("white").unwrap(),
            bgcolor: ThemeColor::from_str("black").unwrap(),
        }
    }
}

impl Default for RsbrBatteryConfig {
    fn default() -> Self {
        Self {
            fgcolor: ThemeColor::from_str("white").unwrap(),
            bgcolor: ThemeColor::from_str("black").unwrap(),
        }
    }
}

impl Default for RsbrAudioConfig {
    fn default() -> Self {
        Self {
            fgcolor: ThemeColor::from_str("white").unwrap(),
            bgcolor: ThemeColor::from_str("black").unwrap(),
        }
    }
}

impl Default for RsbrNetworkConfig {
    fn default() -> Self {
        Self {
            fgcolor: ThemeColor::from_str("white").unwrap(),
            bgcolor: ThemeColor::from_str("black").unwrap(),
        }
    }
}

impl Default for RsbrConfig {
    fn default() -> Self {
        RsbrConfig {
            format: "{brightness} {battery} {datetime}".to_string(),
            datetime: RsbrDatetimeConfig::default(),
            battery: RsbrBatteryConfig::default(),
            brightness: RsbrBrightnessConfig::default(),
            audio: RsbrAudioConfig::default(),
            network: RsbrNetworkConfig::default(),
            theme: Theme::default(),
        }
    }
}

impl FromStr for RsbrConfig {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let table: RsbrConfig = toml::from_str(s)?;
        Ok(table)
    }
}

pub fn read_config(config_path: &PathBuf) -> Result<RsbrConfig, anyhow::Error> {
    let config_content = fs::read_to_string(config_path)?;
    let config = RsbrConfig::from_str(config_content.as_str())?;

    Ok(config)
}
