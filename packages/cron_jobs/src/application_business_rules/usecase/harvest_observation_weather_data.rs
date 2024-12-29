use crate::application_business_rules::services_interface::s3_service::IS3Service;
use crate::enterprise_business_rules::domain::model::jma_observation_data::JmaObservationData;
use anyhow::Result;
use std::sync::Arc;
use tracing::info;

pub struct WeatherUsecase {
    service: Arc<dyn IS3Service>,
}

impl WeatherUsecase {
    pub fn new(service: Arc<dyn IS3Service>) -> Self {
        Self { service }
    }

    pub async fn harvest_observation_weather_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        let csv_file_name = JmaObservationData::create_csv_file().await?;

        self.service.upload_to_s3(csv_file_name).await?;

        info!("Corrected weather data");
        Ok(())
    }
}
