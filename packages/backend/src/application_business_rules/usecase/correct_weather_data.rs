use crate::{
    enterprise_business_rules::domain::entity::{self, jma_observation_data::JmaObservationData},
    interface_adaptors::handler::correct_weather_data::WeatherDataType,
};
use std::error::Error;

pub struct WeatherUsecase {
    // Add necessary fields and dependencies
}

impl WeatherUsecase {
    // pub fn new(repo: Arc<dyn IStudentRepository>) -> Self {
    //     StudentUsecase { repo }
    // }

    pub fn new(/* dependencies */) -> Self {
        WeatherUsecase {
            // Initialize dependencies
        }
    }

    pub async fn correct_weather_data(
        &self,
        weather_data_type: WeatherDataType,
    ) -> Result<(), Box<dyn Error>> {
        if weather_data_type == WeatherDataType::JmaObservation {
            // TODO: ここから始める
            // let jma_observation_data = entity::jma_observation_data::JmaObservationData::create_csv_data();
            // let csv_data = jma_observation_data.create_csv_data();
            // jma_observation_data.upload_to_s3();
        } else {
            // let jma_forecast_data = JmaForecastData::new();
            // let csv_data = jma_forecast_data.create_csv_data();
            // jma_forecast_data.upload_to_s3();
        }

        // Upload to S3
        Ok(())
    }
}
