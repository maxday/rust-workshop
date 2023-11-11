use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::Value;
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().json()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_ansi(false)
        .with_current_span(false)
        .without_time()
        .init();
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Response, Error> {
    tracing::info!("event: {:?}", event);
    Ok(Response {
        message: serde_json::to_string(&event.payload)?.to_string(),
    })
}