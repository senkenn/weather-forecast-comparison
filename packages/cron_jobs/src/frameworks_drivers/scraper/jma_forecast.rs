use anyhow::Result;
use async_trait::async_trait;
use tracing::info;

use crate::{
    frameworks_drivers::date::date::DateWrapper,
    interface_adapters::scraper_interface::jma_observation::IScraper,
};

pub struct JmaForecastScraper {}

impl JmaForecastScraper {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IScraper for JmaForecastScraper {
    async fn fetch_data(&self, date: DateWrapper) -> Result<String> {
        let url = format!(
            "https://www.jma.go.jp/bosai/jmatile/data/wdist/VPFD/130010.json" // 東京都, 東京地方
        );
        info!("Fetching HTML from {}", url);
        let json = reqwest::get(url).await?.text().await?;
        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frameworks_drivers::date::date::DateWrapper;

    #[tokio::test]
    async fn test_fetch_data() {
        let scraper = JmaForecastScraper::new();
        let date = DateWrapper::new(2021, 1, 1).unwrap();
        let json = scraper.fetch_data(date).await.unwrap();
        println!("{}", json);
    }
}
