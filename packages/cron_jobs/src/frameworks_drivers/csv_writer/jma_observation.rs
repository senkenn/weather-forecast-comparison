use anyhow::Result;
use csv::Writer;
use scraper::{Html, Selector};

use crate::frameworks_drivers::date::date::DateWrapper;

pub struct CsvWriter {}

impl CsvWriter {
    pub fn new() -> Self {
        Self {}
    }
}

type CsvFileName = String;

impl CsvWriter {
    pub async fn create_csv_file(
        &self,
        date: DateWrapper,
        html: String,
    ) -> Result<CsvFileName, Box<dyn std::error::Error>> {
        // parse the HTML
        let document = Html::parse_document(&html);

        let row_selector = Selector::parse("tr.mtx[style='text-align:right;']")?;
        let csv_file_name = format!(
            "jma_observation_data_{}_{}_{}.csv",
            date.get_year(),
            date.get_month(),
            date.get_day()
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
