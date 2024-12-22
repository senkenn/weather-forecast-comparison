use std::sync::Arc;

use crate::{
    application_business_rules::usecase::correct_weather_data::WeatherUsecase,
    enterprise_business_rules::domain::model::jma_observation_data::Date,
};
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
pub struct WeatherDataPayload {
    pub weather_data_type: WeatherDataType,
    pub date: Date,
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
        Json(payload): Json<WeatherDataPayload>,
    ) -> StatusCode {
        if let Err(e) = handler.usecase.correct_weather_data(payload).await {
            tracing::error!("Error correcting weather data: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        StatusCode::CREATED
    }
}
