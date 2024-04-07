use super::ShowBar;
use crate::{icons, config::RsbrConfig};

use async_trait::async_trait;
use brightness::Brightness;
use futures_util::stream::StreamExt;

pub struct BrightnessProperty;

impl BrightnessProperty {
    async fn get_brightness(&self) -> Result<u32, anyhow::Error> {
        match brightness::brightness_devices().next().await {
            Some(x) => match x {
                Ok(x) => Ok(x.get().await? + 1),
                Err(x) => Err(anyhow::anyhow!(x)),
            },
            None => Err(anyhow::anyhow!("No brightness devices found!")),
        }
    }
}

#[async_trait]
impl ShowBar for BrightnessProperty {
    async fn show_bar(&self, config: &RsbrConfig, template: &str) -> String {
        let brightness = match self.get_brightness().await {
            Ok(x) => format!("{} {}", icons::get_brightness_icon(x), x),
            Err(x) => {
                eprintln!("{x}");
                "No Brightness Device".to_string()
            }
        };

        template.replace(
            "{brightness}",
            &format!(
                "^c{}^^b{}^ {}% ",
                &config.theme.get_color(&config.brightness.fgcolor),
                &config.theme.get_color(&config.brightness.bgcolor),
                brightness.as_str()
            ),
        )
    }
}
