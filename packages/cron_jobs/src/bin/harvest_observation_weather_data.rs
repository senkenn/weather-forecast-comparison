use cron_jobs::frameworks_drivers::csv_writer::jma_observation::CsvWriter;
use cron_jobs::interface_adapters::s3_service::s3_service::S3Service;
use cron_jobs::{
    application_business_rules::usecase::harvest_observation_weather_data::WeatherUsecase,
    frameworks_drivers::scraper::jma_observation::Scraper,
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
    let scraper = Box::new(Scraper::new());
    let csv_writer = Box::new(CsvWriter::new());
    let usecase = Box::new(WeatherUsecase::new(scraper, csv_writer, s3_service));
    match usecase.harvest_observation_weather_data().await {
        Ok(_) => {
            tracing::info!("Successfully harvested past weather data");
        }
        Err(e) => {
            tracing::error!("Failed to harvest past weather data: {:?}", e);
        }
    }
}
