use anyhow::Result;
use tracing::info;

use crate::{
    frameworks_drivers::date::date::DateWrapper,
    interface_adapters::scraper_interface::jma_observation::IScraper,
};

pub struct Scraper {}

impl Scraper {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl IScraper for Scraper {
    async fn fetch_data(&self, date: DateWrapper) -> Result<String> {
        let prec_no = 44; // Tokyo
        let block_no = 47662; // Tokyo
        let time_period = "hourly";
        let pattern = "s1";
        let year = date.get_year();
        let month = date.get_month();
        let day = date.get_day();
        let url = format!(
            "https://www.data.jma.go.jp/obd/stats/etrn/view/{time_period}_{pattern}.php?prec_no={prec_no}&block_no={block_no}&year={year}&month={month}&day={day}&view="
        );
        info!("Fetching HTML from {}", url);
        let html = reqwest::get(url).await?.text().await?;
        Ok(html)
    }
}
