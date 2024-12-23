use anyhow::Result;
use chrono::{DateTime, Datelike, Local};
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
    async fn fetch_html(date: DateTime<Local>) -> Result<String> {
        let prec_no = 44; // Tokyo
        let block_no = 47662; // Tokyo
        let time_period = "hourly";
        let pattern = "s1";
        let year = date.year();
        let month = date.month();
        let day = date.day();
        let url = format!(
            "https://www.data.jma.go.jp/obd/stats/etrn/view/{time_period}_{pattern}.php?prec_no={prec_no}&block_no={block_no}&year={year}&month={month}&day={day}&view="
        );
        println!("Fetching HTML from {}", url);
        let html = reqwest::get(url).await?.text().await?;
        Ok(html)
    }

    pub async fn create_csv_file() -> Result<CsvFileName, Box<dyn std::error::Error>> {
        let now = Local::now();
        let yesterday = now - chrono::Duration::days(1);

        // fetch the web page
        let html = JmaObservationData::fetch_html(yesterday).await?;

        // parse the HTML
        let document = Html::parse_document(&html);

        let row_selector = Selector::parse("tr.mtx[style='text-align:right;']")?;
        let csv_file_name = format!(
            "jma_observation_data_{}_{}_{}.csv",
            yesterday.year(),
            yesterday.month(),
            yesterday.day()
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
}
