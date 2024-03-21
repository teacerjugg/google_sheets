pub mod error;
pub mod query;
pub mod request;
pub mod response;
pub mod values;
pub mod util;

use error::{Error, Result};
use request::*;
use reqwest::{Client, StatusCode};
use response::*;
use serde_json::Value;

pub struct GoogleSheets {
    pub spreadsheet_id: String,
    pub access_token: String,
}

impl GoogleSheets {
    /// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/batchUpdate?hl=ja
    pub async fn batch_update(
        &self,
        client: &Client,
        batch_update_request: BatchUpdateRequest,
    ) -> Result<BatchUpdateResponse> {
        let response = client
            .post(&format!(
                "https://sheets.googleapis.com/v4/spreadsheets/{}:batchUpdate",
                self.spreadsheet_id,
            ))
            .bearer_auth(&self.access_token)
            .json(&batch_update_request)
            .send()
            .await?;
        let status_ref = response.error_for_status_ref();

        match status_ref {
            Ok(_) => match response.status() {
                StatusCode::OK => Ok(response.json::<BatchUpdateResponse>().await?),
                _ => Err(Error::Unknown),
            },
            Err(e) => match response.status() {
                StatusCode::BAD_REQUEST => {
                    let error = response
                        .json::<Value>()
                        .await?
                        .get("error")
                        .unwrap()
                        .to_owned();
                    Err(Error::BadRequest {
                        code: error.get("code").unwrap().as_u64().unwrap(),
                        message: error.get("message").unwrap().as_str().unwrap().to_owned(),
                    })
                },
                _ => Err(Error::Other(e)),
            }
        }
    }
}
