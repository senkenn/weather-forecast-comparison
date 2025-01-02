use async_trait::async_trait;

use crate::{
    frameworks_drivers::date::date::DateWrapper,
    interface_adapters::csv_writer_interface::jma_observation::ICsvWriter,
};

pub struct JmaForecastCsvWriter;

#[async_trait]
impl ICsvWriter for JmaForecastCsvWriter {
    async fn create_csv_file(
        &self,
        date: DateWrapper,
        json: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let file_name = format!(
            "jma_forecast_{}_{}_{}.csv",
            date.get_year(),
            date.get_month(),
            date.get_day()
        );
        // let mut wtr = csv::Writer::from_path(file_name)?;
        // let forecast: Vec<
        //     crate::interface_adapters::csv_writer_interface::jma_observation::JmaForecast,
        // > = serde_json::from_str(&json)?;
        // for f in forecast {
        //     wtr.serialize(f)?;
        // }
        // wtr.flush()?;
        Ok(file_name)
    }
}
