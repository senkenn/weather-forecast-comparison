use crate::{
    enterprise_business_rules::domain::entity::jma_observation_data::JmaObservationData,
    interface_adaptors::handler::correct_weather_data::{WeatherDataPayload, WeatherDataType},
};
use anyhow::Result;

pub struct WeatherUsecase {}

impl WeatherUsecase {
    pub fn new() -> Self {
        WeatherUsecase {}
    }

    pub async fn correct_weather_data(
        &self,
        payload: WeatherDataPayload,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if payload.weather_data_type == WeatherDataType::JmaObservation {
            let csv_file_name = JmaObservationData::create_csv_file(payload.date).await?;

            let jma_observation_data = JmaObservationData::new();
            jma_observation_data
                .upload_to_s3(csv_file_name.as_str())
                .await?;
        } else {
            // let jma_forecast_data = JmaForecastData::new();
            // let csv_data = jma_forecast_data.create_csv_data();
            // jma_forecast_data.upload_to_s3();
        }

        tracing::info!("Corrected weather data");
        Ok(())
    }
}
