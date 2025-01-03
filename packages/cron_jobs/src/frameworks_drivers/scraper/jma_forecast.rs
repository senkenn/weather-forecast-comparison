use anyhow::Result;
use async_trait::async_trait;
use tracing::info;

use crate::{
    frameworks_drivers::date::date::DateWrapper,
    interface_adapters::scraper_interface::jma_observation::IScraper,
};

pub struct JmaForecastHourlyScraper;
pub struct JmaForecastDailyScraper;

impl JmaForecastHourlyScraper {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IScraper for JmaForecastHourlyScraper {
    async fn fetch_data(&self, date: Option<DateWrapper>) -> Result<String> {
        if date.is_some() {
            return Err(anyhow::anyhow!("Date must be None"));
        }

        let url = format!(
            "https://www.jma.go.jp/bosai/jmatile/data/wdist/VPFD/130010.json" // 東京都, 東京地方
        );
        info!("Fetching HTML from {}", url);
        let json = reqwest::get(url).await?.text().await?;
        Ok(json)
    }
}

#[async_trait]
impl IScraper for JmaForecastDailyScraper {
    async fn fetch_data(&self, date: Option<DateWrapper>) -> Result<String> {
        if date.is_some() {
            return Err(anyhow::anyhow!("Date must be None"));
        }

        let url = format!(
            "https://www.jma.go.jp/bosai/forecast/data/forecast/130000.json" // 東京都, 東京地方
        );
        info!("Fetching HTML from {}", url);
        let json = reqwest::get(url).await?.text().await?;
        Ok(json)
    }
}
