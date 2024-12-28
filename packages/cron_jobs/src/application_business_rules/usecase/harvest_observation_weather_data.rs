use std::path::Path;

use crate::{
    enterprise_business_rules::domain::model::jma_observation_data::JmaObservationData,
    // interface_adaptors::handler::correct_weather_data::{WeatherDataPayload, WeatherDataType},
};
use anyhow::Result;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_s3::{primitives::ByteStream, Client};
use tracing::info;

pub struct WeatherUsecase {}

impl WeatherUsecase {
    pub fn new() -> Self {
        WeatherUsecase {}
    }

    pub async fn harvest_observation_weather_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        let csv_file_name = JmaObservationData::create_csv_file().await?;
        upload_to_s3(&csv_file_name).await?;
        info!("Corrected weather data");
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

    let bucket = "weather-forecast-comparison";
    let key = format!("data/{}", csv_file_path);

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
    info!("Uploaded CSV file to S3");

    Ok(())
}
