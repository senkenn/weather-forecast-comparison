use std::sync::Arc;

use cron_jobs::application_business_rules::services_interface::IS3Service;
use cron_jobs::application_business_rules::usecase::harvest_observation_weather_data::WeatherUsecase;
use cron_jobs::interface_adapters::services::s3_service::S3Service;
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

    let s3_service: Arc<dyn IS3Service> = Arc::new(S3Service::new());
    let usecase = Arc::new(WeatherUsecase::new(s3_service));
    match usecase.harvest_observation_weather_data().await {
        Ok(_) => {
            tracing::info!("Successfully harvested past weather data");
        }
        Err(e) => {
            tracing::error!("Failed to harvest past weather data: {:?}", e);
        }
    }
}
