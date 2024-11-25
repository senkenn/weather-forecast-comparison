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

use std::sync::Arc;

use application_business_rules::usecase::correct_weather_data::WeatherUsecase;
use interface_adaptors::handler::correct_weather_data::WeatherHandler;

use axum::{
    routing::{get, post},
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
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
