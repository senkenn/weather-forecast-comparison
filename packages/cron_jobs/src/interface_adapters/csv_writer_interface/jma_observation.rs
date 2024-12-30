use anyhow::Result;

use crate::frameworks_drivers::date::date::DateWrapper;

pub trait ICsvWriter {
    fn create_csv_file(&self, data: DateWrapper) -> Result<()>;
}
