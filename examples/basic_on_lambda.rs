use google_sheets::api::GoogleSheets;
use google_sheets::auth::read_credential_from_string;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct Request {}

async fn function_handler(_event: LambdaEvent<Request>) -> Result<(), anyhow::Error> {
    // Extract some useful information from the request
    let client = Client::new();
    let credential_json = std::env::var("GOOGLE_CREDENTIAL").expect("GOOGLE_CREDENTIAL is not set");

    let credential = read_credential_from_string(credential_json);
    let access_token = credential.get_access_token(&client).await?;
    tracing::info!("access_token: {}", &access_token);

    let sheets = GoogleSheets::new(access_token).await;

    let values = sheets.get_values(&client, "A1:C").await?;
    tracing::info!("values: {:?}", values);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
