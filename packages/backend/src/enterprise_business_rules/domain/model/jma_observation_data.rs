use anyhow::Result;
use csv::Writer;
use scraper::{Html, Selector};
use serde::Deserialize;

use crate::enterprise_business_rules::domain::entity::jma_observation_data::JmaObservationData;

#[derive(Deserialize)]
pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

impl JmaObservationData {
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

    pub async fn create_csv_file(
        csv_file_name: String,
        date: Date,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // fetch the web page
        let html = JmaObservationData::fetch_html(date).await?;

        // parse the HTML
        let document = Html::parse_document(&html);

        let row_selector = Selector::parse("tr.mtx[style='text-align:right;']")?;
        let mut wtr = Writer::from_path(csv_file_name)?;
        for row in document.select(&row_selector) {
            let mut record = vec![];
            for cell in row.select(&Selector::parse("td").unwrap()) {
                record.push(cell.text().collect::<Vec<_>>().join(" "));
            }

            wtr.write_record(&record)?;
        }

        wtr.flush()?;

        Ok(())
    }

    pub fn upload_to_s3(csv_file_name: String) -> bool {
        println!("Uploading to S3...");
        // let region = Region::new("us-east-1");
        // let credentials =
        //     Credentials::new("your-access-key", "your-secret-key", None, None, "static");
        // let config = Config::builder()
        //     .region(region)
        //     .credentials_provider(credentials)
        //     .build();
        // let client = Client::from_conf(config);

        // let body = ByteStream::from_path(Path::new(csv_file_path)).await?;
        // client
        //     .put_object()
        //     .bucket("your-bucket-name")
        //     .key("output.csv")
        //     .body(body)
        //     .send()
        //     .await?;

        true
    }
}
