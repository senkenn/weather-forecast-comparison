mod interface_adaptors {
    pub mod handler;
}
mod application_business_rules {
    pub mod usecase;
}
mod enterprise_business_rules {
    pub mod domain {
        pub mod entity;
        pub mod model;
    }
}

use std::{fs::File, io::Read, path::Path, sync::Arc};

use application_business_rules::usecase::correct_weather_data::WeatherUsecase;
use interface_adaptors::handler::correct_weather_data::{CsvUpload, WeatherHandler};

use axum::{
    routing::{get, post},
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use csv::Writer;
use scraper::{Html, Selector};
use std::error::Error;

fn html_table_to_csv(csv_file_path: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new("./test_data/jma_past_data_hourly.html");
    let mut file = File::open(&path)?;
    let mut html = String::new();
    file.read_to_string(&mut html)?;
    let document = Html::parse_document(html.as_str());

    let row_selector = Selector::parse("tr.mtx[style='text-align:right;']")?;
    let mut wtr = Writer::from_path(csv_file_path)?;
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

#[tokio::main]
async fn main() {
    // if let Err(e) = html_table_to_csv("output.csv") {
    //     eprintln!("Error generating CSV file: {}", e);
    //     return;
    // }
    // println!("CSV file generated successfully!");

    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let usecase = Arc::new(WeatherUsecase::new());
    let handler = Arc::new(WeatherHandler::new(usecase));

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route(
            "/api/correct-weather-data",
            post(WeatherHandler::correct_weather_data),
        )
        .with_state(handler);

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("Server listening on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    println!("hello world called");

    "Hello, World!"
}
