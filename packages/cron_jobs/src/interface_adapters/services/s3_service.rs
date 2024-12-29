use anyhow::Result;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_s3::{primitives::ByteStream, Client};
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use tracing::info;

use crate::application_business_rules::services_interface::IS3Service;

pub struct S3Service;

impl S3Service {
    pub fn new() -> Self {
        S3Service
    }
}

impl IS3Service for S3Service {
    fn upload_to_s3(
        &self,
        csv_file_path: String,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        Box::pin(async move {
            let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");
            let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
                .region(region_provider)
                .load()
                .await;
            let client = Client::new(&config);

            let bucket = "weather-forecast-comparison";
            let key = format!("data/{}", &csv_file_path);

            let byte_stream = ByteStream::from_path(Path::new(&csv_file_path.to_string())).await?;

            client
                .put_object()
                .bucket(bucket)
                .key(key)
                .body(byte_stream)
                .send()
                .await?;
            info!("Uploaded CSV file to S3");

            Ok(())
        })
    }
}
