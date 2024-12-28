use std::sync::Arc;

use cron_jobs::application_business_rules::usecase::harvest_observation_weather_data::WeatherUsecase;
use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// mod application_business_rules {
//     pub mod usecase;
// }

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    // Define the log filter with specific levels for modules
    let filter = EnvFilter::new("info,aws_smithy_runtime=info,aws_sdk_s3=info,aws_types=info");

    // Initialize tracing with the defined filter
    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    let usecase = Arc::new(WeatherUsecase::new());
    match usecase.harvest_observation_weather_data().await {
        Ok(_) => {
            tracing::info!("Successfully harvested past weather data");
        }
        Err(e) => {
            tracing::error!("Failed to harvest past weather data: {:?}", e);
        }
    }
}
