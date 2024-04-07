use async_trait::async_trait;
use std::process::Command;

use super::ShowBar;
use crate::{config::RsbrConfig, icons};

pub struct NetworkProperty;

impl NetworkProperty {
    fn get_network(&self) -> Result<String, anyhow::Error> {
        let output = Command::new("nmcli")
            .arg("-t")
            .arg("-f")
            .arg("active,ssid")
            .arg("dev")
            .arg("wifi")
            .output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);

        for line in output_str.lines() {
            if line.starts_with("yes:") {
                return Ok(line[4..].to_string());
            }
        }

        Err(anyhow::anyhow!("Network Information"))
    }
}

#[async_trait]
impl ShowBar for NetworkProperty {
    async fn show_bar(&self, config: &RsbrConfig, template: &str) -> String {
        let network = match self.get_network() {
            Ok(network_ssid) => {
                if network_ssid != "" {
                    format!("{} {}", icons::get_wifi_icon(&network_ssid), network_ssid)
                } else {
                    format!("{}", icons::get_wifi_icon(&network_ssid))
                }
            }
            Err(x) => {
                eprintln!("{x}");
                "No Wifi Found".to_string()
            }
        };

        template.replace(
            "{network}",
            &format!(
                "^c{}^^b{}^ {}",
                &config.theme.get_color(&config.network.fgcolor),
                &config.theme.get_color(&config.network.bgcolor),
                network.as_str()
            ),
        )
    }
}
