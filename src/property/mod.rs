pub mod battery;
pub mod datetime;
pub mod brightness;
pub mod audio;
pub mod network;

use async_trait::async_trait;

use crate::config::RsbrConfig;

#[async_trait]
pub trait ShowBar {
    async fn show_bar(&self, config: &RsbrConfig, template: &str) -> String;
}

pub struct ShowBars(Vec<Box<dyn ShowBar>>);

impl ShowBars {
    pub fn new(bar: Vec<Box<dyn ShowBar>>) -> Self {
        Self(bar)
    }

    pub async fn process(&self, config: RsbrConfig) -> String {
        let attributes = &self.0;
        let mut template = config.format.clone();

        for attribute in attributes {
            template = attribute.as_ref().show_bar(&config, &template).await.to_string();
        }

        template
    }
}
