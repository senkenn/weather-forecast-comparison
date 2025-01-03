use crate::application_business_rules::services_interface::IS3Service;
use crate::frameworks_drivers::date::date::DateWrapper;
use crate::interface_adapters::csv_writer_interface::jma_observation::ICsvWriter;
use crate::interface_adapters::scraper_interface::jma_observation::IScraper;
use anyhow::Result;

pub struct ForecastUsecase {
    scrapers: Vec<Box<dyn IScraper>>,
    csv_writers: Vec<Box<dyn ICsvWriter>>,
    service: Box<dyn IS3Service>,
}

impl ForecastUsecase {
    pub fn new(
        scrapers: Vec<Box<dyn IScraper>>,
        csv_writers: Vec<Box<dyn ICsvWriter>>,
        service: Box<dyn IS3Service>,
    ) -> Self {
        Self {
            scrapers,
            csv_writers,
            service,
        }
    }

    pub async fn harvest_weather_forecast_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        for (scraper, csv_writer) in self.scrapers.iter().zip(self.csv_writers.iter()) {
            let now = DateWrapper::now();

            let html = scraper.fetch_data(None).await?;

            let csv_file_name = csv_writer.create_csv_file(now, html).await?;

            self.service.upload_to_s3(csv_file_name).await?;
        }

        Ok(())
    }
}
