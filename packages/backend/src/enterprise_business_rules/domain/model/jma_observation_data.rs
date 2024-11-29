use std::path::Path;

use anyhow::Result;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_s3::{primitives::ByteStream, Client};
use csv::Writer;
use scraper::{Html, Selector};
use serde::Deserialize;

pub struct JmaObservationData {}

#[derive(Deserialize, Clone)]
pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

type CsvFileName = String;

impl JmaObservationData {
    pub fn new() -> Self {
        JmaObservationData {}
    }

    async fn fetch_html(date: Date) -> Result<String> {
        let prec_no = 44; // Tokyo
        let block_no = 47662; // Tokyo
        let time_period = "hourly";
        let pattern = "s1";
        let Date { year, month, day } = date;
        let url = format!(
            "https://www.data.jma.go.jp/obd/stats/etrn/view/{time_period}_{pattern}.php?prec_no={prec_no}&block_no={block_no}&year={year}&month={month}&day={day}&view="
        );
        println!("Fetching HTML from {}", url);
        let html = reqwest::get(url).await?.text().await?;
        Ok(html)
    }

    pub async fn create_csv_file(date: Date) -> Result<CsvFileName, Box<dyn std::error::Error>> {
        // fetch the web page
        let html = JmaObservationData::fetch_html(date.clone()).await?;

        // parse the HTML
        let document = Html::parse_document(&html);

        let row_selector = Selector::parse("tr.mtx[style='text-align:right;']")?;
        let csv_file_name = format!(
            "jma_observation_data_{}_{}_{}.csv",
            date.year, date.month, date.day
        );
        let mut wtr = Writer::from_path(csv_file_name.clone())?;
        for row in document.select(&row_selector) {
            let mut record = vec![];
            for cell in row.select(&Selector::parse("td").unwrap()) {
                record.push(cell.text().collect::<Vec<_>>().join(" "));
            }

            wtr.write_record(&record)?;
        }
        wtr.flush()?;
        tracing::info!("Created CSV file: {}", csv_file_name);

        Ok(csv_file_name)
    }

    pub async fn upload_to_s3(&self, csv_file_path: &str) -> Result<()> {
        // Initialize AWS configuration
        let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");
        let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
            .region(region_provider)
            .load()
            .await;
        let client = Client::new(&config);

        let bucket = "weather-forecast-comparison-data-store";
        let key = format!(
            "{}",
            Path::new(csv_file_path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        );

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
        tracing::info!("Uploaded CSV file to S3");

        Ok(())
    }
}
