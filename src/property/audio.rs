use async_trait::async_trait;
use regex::Regex;
use std::process::Command;

use super::ShowBar;
use crate::{config::RsbrConfig, icons};

pub struct AudioProperty;

impl AudioProperty {
    fn get_muted(&self) -> Result<bool, anyhow::Error> {
        let output = Command::new("amixer").arg("get").arg("Master").output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let is_muted = output_str.contains("[off]");

        Ok(is_muted)
    }

    fn get_volume(&self) -> Result<u32, anyhow::Error> {
        let output = Command::new("amixer").arg("get").arg("Master").output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);

        let re = Regex::new(r"\d+%")?;

        if let Some(mat) = re.find(&output_str) {
            let volume_str = &mat.as_str()[..mat.as_str().len() - 1];

            if let Ok(volume) = volume_str.parse::<u32>() {
                return Ok(volume);
            }
        }

        Err(anyhow::anyhow!("Could not retrieve Master Volume!"))
    }

    fn get_audio(&self) -> Result<(u32, bool), anyhow::Error> {
        // nmcli -t -f active,ssid dev wifi | grep '^yes' | cut -c 5-

        let volume_perc = self.get_volume()?;
        let muted = self.get_muted()?;

        Ok((volume_perc, muted))
    }
}

#[async_trait]
impl ShowBar for AudioProperty {
    async fn show_bar(&self, config: &RsbrConfig, template: &str) -> String {
        let audio = match self.get_audio() {
            Ok((volume_percent, is_muted)) => {
                if is_muted {
                    icons::get_volume_icon(volume_percent, is_muted).to_string()
                } else {
                    format!(
                        "{} {}%",
                        icons::get_volume_icon(volume_percent, is_muted).to_string(),
                        volume_percent
                    )
                }
            }
            Err(x) => {
                eprintln!("{x}");
                "No Audio Device Found".to_string()
            }
        };

        template.replace(
            "{audio}",
            &format!(
                "^c{}^^b{}^ {}",
                &config.theme.get_color(&config.audio.fgcolor),
                &config.theme.get_color(&config.audio.bgcolor),
                audio.as_str()
            ),
        )
    }
}
