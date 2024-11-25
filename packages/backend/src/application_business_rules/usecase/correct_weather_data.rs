use crate::{
    enterprise_business_rules::domain::{
        entity::jma_observation_data::JmaObservationData, model::jma_observation_data::Date,
    },
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
        let Date { year, month, day } = payload.date;
        if payload.weather_data_type == WeatherDataType::JmaObservation {
            let csv_file_name = format!("jma_past_data_hourly_{year}_{month}_{day}.csv");
            JmaObservationData::create_csv_file(csv_file_name.clone(), payload.date).await?;
            println!("Created CSV file: {}", csv_file_name);
        } else {
            // let jma_forecast_data = JmaForecastData::new();
            // let csv_data = jma_forecast_data.create_csv_data();
            // jma_forecast_data.upload_to_s3();
        }

        // Upload to S3
        Ok(())
    }
}
