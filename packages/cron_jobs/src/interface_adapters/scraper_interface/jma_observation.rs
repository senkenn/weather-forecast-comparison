use anyhow::Result;

use crate::frameworks_drivers::date::date::DateWrapper;

#[async_trait::async_trait]
pub trait IScraper {
    async fn fetch_data(&self, date: DateWrapper) -> Result<String>;
}
