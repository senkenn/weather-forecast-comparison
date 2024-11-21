use std::sync::Arc;

use crate::application_business_rules::usecase::correct_weather_data::WeatherUsecase;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum WeatherDataType {
    JmaObservation,
    JmaForecast,
    WeatherNewsForecast,
}

#[derive(Deserialize)]
pub struct CsvUpload {
    pub weather_data_type: WeatherDataType,
}

pub struct WeatherHandler {
    pub usecase: Arc<WeatherUsecase>,
}

impl WeatherHandler {
    pub fn new(usecase: Arc<WeatherUsecase>) -> Self {
        WeatherHandler { usecase }
    }

    pub async fn correct_weather_data(
        State(handler): State<Arc<WeatherHandler>>,
        Json(payload): Json<CsvUpload>,
    ) -> StatusCode {
        // TODO: validate payload
        println!("called handler");

        handler
            .usecase
            .correct_weather_data(payload.weather_data_type)
            .await;
        StatusCode::CREATED
    }
}
