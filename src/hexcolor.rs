use anyhow;
use regex::Regex;
use serde::de;
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct HexColor(pub String);

impl fmt::Display for HexColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for HexColor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^#[0-9A-Fa-f]{6}$")?;

        if re.is_match(&s) {
            Ok(HexColor(s.to_string()))
        } else {
            Err(anyhow::anyhow!("{} is not a valid hex color", s))
        }
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        match HexColor::from_str(&s) {
            Ok(x) => Ok(x),
            Err(x) => Err(de::Error::custom(x)),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct Theme {
    pub black: HexColor,
    pub red: HexColor,
    pub green: HexColor,
    pub yellow: HexColor,
    pub blue: HexColor,
    pub magenta: HexColor,
    pub cyan: HexColor,
    pub white: HexColor,
    pub purple: HexColor,
    pub bright_black: HexColor,
    pub bright_red: HexColor,
    pub bright_green: HexColor,
    pub bright_yellow: HexColor,
    pub bright_blue: HexColor,
    pub bright_magenta: HexColor,
    pub bright_cyan: HexColor,
    pub bright_white: HexColor,
    pub bright_purple: HexColor,
    pub background: HexColor,
    pub foreground: HexColor,
}

impl Theme {
    pub fn get_color(&self, col: &ThemeColor) -> &HexColor {
        match col.0.to_lowercase().as_str() {
            "black" => &self.black,
            "red" => &self.red,
            "green" => &self.green,
            "yellow" => &self.yellow,
            "blue" => &self.blue,
            "magenta" => &self.magenta,
            "cyan" => &self.cyan,
            "white" => &self.white,
            "purple" => &self.purple,
            "bright_black" => &self.bright_black,
            "bright_red" => &self.bright_red,
            "bright_green" => &self.bright_green,
            "bright_yellow" => &self.bright_yellow,
            "bright_blue" => &self.bright_blue,
            "bright_magenta" => &self.bright_magenta,
            "bright_cyan" => &self.bright_cyan,
            "bright_white" => &self.bright_white,
            "bright_purple" => &self.bright_purple,
            "background" => &self.background,
            "foreground" => &self.foreground,
            _ => &self.black,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            black: HexColor("#000000".to_string()),
            red: HexColor("#FF0000".to_string()),
            green: HexColor("#33CC33".to_string()),
            yellow: HexColor("#FFFF00".to_string()),
            blue: HexColor("#3333FF".to_string()),
            magenta: HexColor("#CC33CC".to_string()),
            cyan: HexColor("#00CCCC".to_string()),
            white: HexColor("#CCCCCC".to_string()),
            purple: HexColor("#800080".to_string()),

            bright_black: HexColor("#666666".to_string()),
            bright_red: HexColor("#FF6666".to_string()),
            bright_green: HexColor("#66FF66".to_string()),
            bright_yellow: HexColor("#FFFF66".to_string()),
            bright_blue: HexColor("#6666FF".to_string()),
            bright_magenta: HexColor("#FF66FF".to_string()),
            bright_cyan: HexColor("#66FFFF".to_string()),
            bright_white: HexColor("#FFFFFF".to_string()),
            bright_purple: HexColor("#BF40BF".to_string()),

            background: HexColor("#000000".to_string()),
            foreground: HexColor("#FFFFFF".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThemeColor(pub String);

impl FromStr for ThemeColor {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re =
            Regex::new(r"^(black|green|white|grey|blue|red|orange|yellow|pink|darkblue)$").unwrap();
        if re.is_match(&s) {
            Ok(ThemeColor(s.to_string()))
        } else {
            Err(anyhow::anyhow!("{} is not a valid theme color", s))
        }
    }
}

impl<'de> Deserialize<'de> for ThemeColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        match ThemeColor::from_str(&s) {
            Ok(x) => Ok(x),
            Err(x) => Err(de::Error::custom(x)),
        }
    }
}
