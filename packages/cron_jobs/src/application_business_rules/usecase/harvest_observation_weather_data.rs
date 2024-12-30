use crate::application_business_rules::services_interface::s3_service::IS3Service;
use crate::frameworks_drivers::csv_writer::jma_observation::CsvWriter;
use crate::frameworks_drivers::date::date::DateWrapper;
use crate::frameworks_drivers::scraper::jma_observation::Scraper;
use crate::interface_adapters::s3_service::s3_service::S3Service;
use crate::interface_adapters::scraper_interface::jma_observation::IScraper;
use anyhow::Result;
use std::sync::Arc;
use tracing::info;

pub struct WeatherUsecase {
    scraper: Scraper,
    csv_writer: CsvWriter,
    service: Arc<S3Service>,
}

impl WeatherUsecase {
    pub fn new(scraper: Scraper, csv_writer: CsvWriter, service: Arc<S3Service>) -> Self {
        Self {
            service,
            scraper,
            csv_writer,
        }
    }

    pub async fn harvest_observation_weather_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        let yesterday = DateWrapper::new().get_yesterday();

        let html = self.scraper.fetch_data(yesterday.clone()).await?;

        let csv_file_name = self
            .csv_writer
            .create_csv_file(yesterday.clone(), html)
            .await?;

        self.service.upload_to_s3(csv_file_name).await?;

        info!("Corrected weather data");
        Ok(())
    }
}
