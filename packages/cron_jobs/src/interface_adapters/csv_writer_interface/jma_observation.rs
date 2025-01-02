use anyhow::Result;
use async_trait::async_trait;

use crate::frameworks_drivers::date::date::DateWrapper;

type FileName = String;
#[async_trait]
pub trait ICsvWriter {
    async fn create_csv_file(
        &self,
        date: DateWrapper,
        html: String,
    ) -> Result<FileName, Box<dyn std::error::Error>>;
}
