use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use tracing::info;

use crate::{
    frameworks_drivers::date::date::DateWrapper,
    interface_adapters::csv_writer_interface::jma_observation::ICsvWriter,
};

#[derive(Deserialize)]
struct TimeSeries {
    #[serde(rename = "dateTime")]
    date_time: String,
}

#[derive(Deserialize)]
struct AreaTimeSeries {
    #[serde(rename = "timeDefines")]
    time_defines: Vec<TimeSeries>,
    weather: Vec<String>,
}

#[derive(Deserialize)]
struct Forecast {
    #[serde(rename = "areaTimeSeries")]
    area_time_series: AreaTimeSeries,
}

pub struct JmaForecastDailyCsvWriter;
impl JmaForecastDailyCsvWriter {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl ICsvWriter for JmaForecastDailyCsvWriter {
    async fn create_csv_file(&self, date: DateWrapper, json_str: String) -> Result<String> {
        let file_name = format!(
            "jma_forecast_daily_{}_{}_{}.csv",
            date.get_year(),
            date.get_month(),
            date.get_day()
        );

        // TODO: fix
        let forecast: Forecast = serde_json::from_str(&json_str)?;
        let mut wtr = csv::Writer::from_path(&file_name)?;

        wtr.write_record(&["dateTime", "weather"])?;

        for (i, time_define) in forecast.area_time_series.time_defines.iter().enumerate() {
            wtr.write_record(&[
                &time_define.date_time,
                &forecast.area_time_series.weather[i],
            ])?;
        }

        wtr.flush()?;

        info!("Created CSV file: {}", file_name);

        Ok(file_name)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::frameworks_drivers::date::date::DateWrapper;

    #[tokio::test]
    async fn test_create_csv_file() {
        let csv_writer = JmaForecastDailyCsvWriter {};
        let date = DateWrapper::new(2021, 1, 1).unwrap();
        let json_str = fs::read_to_string("src/tests/fixtures/jma_forecast_hourly.json").unwrap();
        let file_name = csv_writer.create_csv_file(date, json_str).await.unwrap();
        assert_eq!(file_name, "jma_forecast_daily_2021_1_1.csv");

        let csv = fs::read_to_string(file_name).unwrap();
        let rows: Vec<&str> = csv.split('\n').collect();
        let expected_rows = vec![
            "dateTime,weather",
            "2025-01-03T12:00:00+09:00,くもり",
            "2025-01-03T15:00:00+09:00,くもり",
            "2025-01-03T18:00:00+09:00,くもり",
            "2025-01-03T21:00:00+09:00,くもり",
            "2025-01-04T00:00:00+09:00,くもり",
            "2025-01-04T03:00:00+09:00,くもり",
            "2025-01-04T06:00:00+09:00,くもり",
            "2025-01-04T09:00:00+09:00,晴れ",
            "2025-01-04T12:00:00+09:00,晴れ",
            "2025-01-04T15:00:00+09:00,晴れ",
            "2025-01-04T18:00:00+09:00,くもり",
            "2025-01-04T21:00:00+09:00,くもり",
            "",
        ];
        for (row, expected_row) in rows.iter().zip(expected_rows.iter()) {
            assert_eq!(row, expected_row);
        }
    }
}
