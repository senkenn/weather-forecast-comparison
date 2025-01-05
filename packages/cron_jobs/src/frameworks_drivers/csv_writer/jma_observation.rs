use anyhow::Result;
use async_trait::async_trait;
use csv::Writer;
use scraper::{Html, Selector};
use tracing::info;

use crate::{
    frameworks_drivers::date::date::DateWrapper,
    interface_adapters::csv_writer_interface::jma_observation::ICsvWriter,
};

pub struct JmaObservationCsvWriter {}

impl JmaObservationCsvWriter {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl ICsvWriter for JmaObservationCsvWriter {
    async fn create_csv_file(&self, date: DateWrapper, html: String) -> Result<String> {
        // parse the HTML
        let document = Html::parse_document(&html);

        // ルート要素に <html> タグが存在するかを確認
        if !html.contains("<html") {
            return Err(anyhow::anyhow!("Input is missing <html> tag"));
        }

        let csv_file_name = format!(
            "jma_observation_{}_{}_{}.csv",
            date.get_year(),
            date.get_month(),
            date.get_day()
        );
        let mut wtr = Writer::from_path(csv_file_name.clone())?;

        // write the header
        let header = vec![
            "時",
            "気圧(現地)",
            "気圧(海面)",
            "降水量",
            "気温",
            "露点温度",
            "蒸気圧",
            "湿度",
            "風速",
            "風向",
            "日照時間",
            "全天日射量",
            "降雪",
            "積雪",
            "天気",
            "雲量",
            "視程",
        ];
        wtr.write_record(&header)?;

        // write the records
        let row_selector = Selector::parse("tr.mtx[style='text-align:right;']")
            .map_err(|e| anyhow::anyhow!("Failed to parse the selector: {:?}", e))?;
        for row in document.select(&row_selector) {
            let mut record = vec![];
            for cell in row.select(&Selector::parse("td").unwrap()) {
                // if the cell is img, get the alt attribute
                if let Some(img) = cell.select(&Selector::parse("img").unwrap()).next() {
                    record.push(img.value().attr("alt").unwrap().to_string());
                    continue;
                }

                record.push(cell.text().collect::<Vec<_>>().join(" "));
            }

            wtr.write_record(&record)?;
        }
        wtr.flush()?;
        info!("Created CSV file: {}", csv_file_name);

        Ok(csv_file_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frameworks_drivers::date::date::DateWrapper;
    use std::fs;

    #[tokio::test]
    async fn create_csv_file_ok() {
        let csv_writer = JmaObservationCsvWriter::new();
        let date = DateWrapper::new(2024, 11, 1).unwrap();
        let html =
            fs::read_to_string("./src/tests/fixtures/jma_observation_2024_11_1.html").unwrap();

        let csv_file_name = csv_writer.create_csv_file(date, html).await.unwrap();

        let csv = fs::read_to_string(csv_file_name.clone()).unwrap();
        assert_eq!(csv_file_name, "jma_observation_2024_11_1.csv");

        let rows: Vec<&str> = csv.split("\n").collect();
        let expected_rows: Vec<&str> = r#"時,気圧(現地),気圧(海面),降水量,気温,露点温度,蒸気圧,湿度,風速,風向,日照時間,全天日射量,降雪,積雪,天気,雲量,視程
1,1024.0,1026.9,--,14.4,11.2,13.3,81,1.8,西北西,,,×,×,,,
2,1023.2,1026.1,--,14.0,10.8,12.9,81,1.3,北北西,,,×,×,,,
3,1023.2,1026.1,--,13.4,10.8,12.9,84,1.4,北西,,,×,×,晴れ,7,30.0
4,1023.1,1026.0,--,13.2,10.2,12.4,82,1.1,北西,,,×,×,,,
5,1022.8,1025.7,--,13.3,10.1,12.4,81,1.4,北,,,×,×,,,
6,1022.9,1025.9,--,12.4,10.5,12.7,88,0.9,北,,0.00,×,×,薄曇,9,25.0
7,1023.0,1025.9,--,13.1,10.8,13.0,86,0.7,北北東,0.0,0.12,×,×,,,
8,1023.3,1026.2,--,15.7,11.7,13.7,77,1.7,北北西,0.7,0.70,×,×,,,
9,1022.7,1025.6,--,16.5,10.1,12.4,66,1.8,北,0.9,1.23,×,×,晴れ,6,25.0
10,1022.6,1025.5,--,17.9,10.5,12.7,62,1.8,北北西,0.8,1.46,×,×,,,
11,1021.5,1024.4,--,20.2,12.2,14.2,60,0.8,南西,1.0,2.20,×,×,,,
12,1020.2,1023.0,--,20.8,12.8,14.7,60,0.8,西北西,0.2,1.54,×,×,曇,10-,20.0
13,1019.7,1022.5,--,21.5,14.9,16.9,66,1.3,南南西,0.0,1.00,×,×,,,
14,1019.1,1021.9,--,21.0,15.6,17.7,71,0.6,西,0.0,0.58,×,×,,,
15,1019.2,1022.1,--,20.1,15.5,17.6,75,1.2,北,0.0,0.27,×,×,曇,10,20.0
16,1018.7,1021.6,--,19.6,14.6,16.7,73,1.4,北北西,0.0,0.14,×,×,,,
17,1018.6,1021.5,0.0,19.6,14.0,16.0,70,1.4,北北西,0.0,0.01,×,×,,,
18,1019.8,1022.7,0.0,19.3,12.8,14.8,66,2.2,北西,,0.00,×,×,曇,10,15.0
19,1020.1,1023.0,0.0,18.5,12.5,14.5,68,1.9,北西,,,×,×,,,
20,1019.3,1022.2,0.0,17.9,12.8,14.8,72,1.3,西,,,×,×,,,
21,1019.0,1021.9,0.0,16.5,14.2,16.1,86,2.0,西北西,,,×,×,雨,10,15.0
22,1018.0,1020.9,0.0,16.4,14.1,16.0,86,1.4,北東,,,×,×,,,
23,1017.7,1020.6,0.0,16.5,13.4,15.4,82,1.3,西北西,,,×,×,,,
24,1017.0,1019.9,0.5,16.0,14.2,16.2,89,1.7,北北西,,,×,×,,,
"#
        .split("\n")
        .collect();

        for (i, row) in rows.iter().enumerate() {
            assert_eq!(row, &expected_rows[i]);
        }
    }

    #[tokio::test]
    async fn create_csv_file_err_input_json() {
        let csv_writer = JmaObservationCsvWriter::new();
        let date = DateWrapper::new(2024, 11, 1).unwrap();
        let html = fs::read_to_string("./src/tests/fixtures/jma_forecast_daily.json").unwrap();

        let err = csv_writer.create_csv_file(date, html).await.unwrap_err();
        assert_eq!(err.to_string(), "Input is missing <html> tag");

        // let csv = fs::read_to_string(csv_file_name.clone()).unwrap();
    }
}
