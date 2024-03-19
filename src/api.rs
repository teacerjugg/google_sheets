pub mod query;
pub mod request;
pub mod response;
pub mod values;

use request::*;
use reqwest::{Client, Result};
use response::*;

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
            Ok(_) => response.json::<BatchUpdateResponse>().await,
            Err(e) => Err(e),
        }
    }
}
