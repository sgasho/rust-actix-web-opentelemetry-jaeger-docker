mod handler;
mod model;

use tokio;
use actix_web::{App, HttpServer, web};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{trace, Resource, runtime};
use sqlx::{Error, MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{ Registry, EnvFilter };
use tracing_opentelemetry::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn setup_tracer() {
    let tracer = match opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://jaeger:4317"),
        )
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "backend",
            )])),
        )
        .install_batch(runtime::Tokio) {
        Err(e) => {
            eprintln!("failed to initialize tracer: {e}");
            std::process::exit(1);
        },
        Ok(t) => t
    };

    Registry::default()
        .with(layer().with_tracer(tracer))
        .with(EnvFilter::new("DEBUG")) // in order not to send noisy TRACE logs
        .init();
}

async fn setup_mysql_pool() -> Result<Pool<MySql>, Error> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    setup_tracer();

    let mysql_pool = match setup_mysql_pool().await {
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        },
        Ok(pool) => pool,
    };

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web_opentelemetry::RequestTracing::new())
            .app_data(web::Data::new(mysql_pool.clone()))
            .configure(handler::config)
            .wrap(TracingLogger::default())
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}
