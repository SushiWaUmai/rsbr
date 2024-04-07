use async_trait::async_trait;
use chrono::Local;

use crate::config::RsbrConfig;

use super::ShowBar;

pub struct DatetimeProperty;

#[async_trait]
impl ShowBar for DatetimeProperty {
    async fn show_bar(&self, config: &RsbrConfig, template: &str) -> String {
        let datetime = Local::now().format(&config.datetime.format).to_string();

        template.replace(
            "{datetime}",
            &format!(
                "^c{}^^b{}^ {} ",
                &config.theme.get_color(&config.datetime.fgcolor),
                &config.theme.get_color(&config.datetime.bgcolor),
                datetime,
            ),
        )
    }
}
