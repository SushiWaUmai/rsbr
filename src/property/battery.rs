use async_trait::async_trait;

use crate::{config::RsbrConfig, icons};

use super::ShowBar;

pub struct BatteryProperty;

impl BatteryProperty {
    fn get_battery(&self) -> Result<(f32, bool), anyhow::Error> {
        let manager = battery::Manager::new()?;

        let batteries = match manager.batteries() {
            Err(_) => vec![],
            Ok(bat) => bat.filter_map(Result::ok).collect(),
        };

        if batteries.len() == 0 {
            return Err(anyhow::anyhow!("No Battery found!"));
        }

        let sum: f32 = batteries
            .iter()
            .map(|x| x.state_of_charge().get::<battery::units::ratio::percent>())
            .sum();

        let battery_charging = batteries
            .iter()
            .map(|x| x.state())
            .any(|x| x == battery::State::Charging);

        Ok((sum / batteries.len() as f32, battery_charging))
    }
}

#[async_trait]
impl ShowBar for BatteryProperty {
    async fn show_bar(&self, config: &RsbrConfig, template: &str) -> String {
        let battery = match self.get_battery() {
            Ok((battery_charge, battery_status)) => format!(
                "{} {}",
                icons::get_battery_icon(battery_charge, battery_status),
                battery_charge.ceil()
            ),
            Err(x) => {
                eprintln!("{x}");
                "No Battery Found".to_string()
            }
        };

        template.replace(
            "{battery}",
            &format!(
                "^c{}^^b{}^ {}% ",
                &config.theme.get_color(&config.battery.fgcolor),
                &config.theme.get_color(&config.battery.bgcolor),
                battery.as_str()
            ),
        )
    }
}
