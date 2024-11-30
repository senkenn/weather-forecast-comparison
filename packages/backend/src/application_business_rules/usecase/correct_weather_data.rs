use std::path::Path;

use crate::{
    enterprise_business_rules::domain::model::jma_observation_data::JmaObservationData,
    interface_adaptors::handler::correct_weather_data::{WeatherDataPayload, WeatherDataType},
};
use anyhow::Result;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_s3::{primitives::ByteStream, Client};

pub struct WeatherUsecase {}

impl WeatherUsecase {
    pub fn new() -> Self {
        WeatherUsecase {}
    }

    pub async fn correct_weather_data(
        &self,
        payload: WeatherDataPayload,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut csv_file_name = String::new();
        if payload.weather_data_type == WeatherDataType::JmaObservation {
            csv_file_name = JmaObservationData::create_csv_file(payload.date).await?;
        } else if payload.weather_data_type == WeatherDataType::JmaForecast {
            // let csv_file_name = JmaForecastData::create_csv_file(payload.date).await?;
        } else if payload.weather_data_type == WeatherDataType::WeatherNewsForecast {
            // let csv_file_name = WeatherNewsForecastData::create_csv_file(payload.date).await?;
        } else {
            // This should never happen
            panic!("Invalid weather data type");
        }

        upload_to_s3(&csv_file_name).await?;

        tracing::info!("Corrected weather data");
        Ok(())
    }
}

async fn upload_to_s3(csv_file_path: &str) -> Result<()> {
    // Initialize AWS configuration
    let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");
    let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .region(region_provider)
        .load()
        .await;
    let client = Client::new(&config);

    let bucket = "weather-forecast-comparison-data-store";
    let key = format!(
        "{}",
        Path::new(csv_file_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
    );

    // Read the CSV file into a byte stream
    let byte_stream = ByteStream::from_path(Path::new(csv_file_path)).await?;

    // Perform the upload
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(byte_stream)
        .send()
        .await?;
    tracing::info!("Uploaded CSV file to S3");

    Ok(())
}
