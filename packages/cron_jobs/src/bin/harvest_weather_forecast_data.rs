use cron_jobs::frameworks_drivers::csv_writer::jma_forecast_daily::JmaForecastDailyCsvWriter;
use cron_jobs::frameworks_drivers::csv_writer::jma_forecast_hourly::JmaForecastHourlyCsvWriter;
use cron_jobs::frameworks_drivers::scraper::jma_forecast::JmaForecastHourlyScraper;
use cron_jobs::interface_adapters::csv_writer_interface::jma_observation::ICsvWriter;
use cron_jobs::interface_adapters::s3_service::s3_service::S3Service;
use cron_jobs::interface_adapters::scraper_interface::jma_observation::IScraper;
use cron_jobs::{
    application_business_rules::usecase::harvest_forecast_data::ForecastUsecase,
    frameworks_drivers::scraper::jma_forecast::JmaForecastDailyScraper,
};
use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let filter = EnvFilter::new("info,aws_smithy_runtime=info,aws_sdk_s3=info,aws_types=info");
    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    let s3_service = Box::new(S3Service::new());
    let scrapers: Vec<Box<dyn IScraper>> = vec![
        Box::new(JmaForecastHourlyScraper::new()),
        // Box::new(JmaForecastDailyScraper::new()),
    ];
    let csv_writers: Vec<Box<dyn ICsvWriter>> = vec![
        Box::new(JmaForecastHourlyCsvWriter::new()),
        // Box::new(JmaForecastDailyCsvWriter::new()),
    ];
    let usecase = Box::new(ForecastUsecase::new(scrapers, csv_writers, s3_service));
    match usecase.harvest_weather_forecast_data().await {
        Ok(_) => {
            tracing::info!("Successfully harvested weather forecast data");
        }
        Err(e) => {
            tracing::error!("Failed to harvest weather forecast data: {:?}", e);
        }
    }
}
