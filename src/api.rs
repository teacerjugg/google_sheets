use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

pub mod query;
use query::*;

pub mod response;
use response::*;

pub mod request;
use request::*;

pub struct GoogleSheets {
    pub spreadsheet_id: String,
    pub access_token: String,
}

impl GoogleSheets {
    /// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/get?hl=ja
    pub async fn get_values<T>(&self, client: &Client, range: T) -> Result<ValueRange>
    where
        T: AsRef<str>,
    {
        let response = client
            .get(&format!(
                "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}",
                self.spreadsheet_id,
                range.as_ref(),
            ))
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        let status_ref = response.error_for_status_ref();

        match status_ref {
            Ok(_) => {
                match response.json::<ValueRange>().await {
                    Ok(clear_response) => Ok(clear_response),
                    Err(e) => Err(anyhow::anyhow!("failed to get values: {}", e)),
                }
            },
            Err(e) => Err(anyhow::anyhow!("failed to get values: {}", e)),
        }
    }

    /// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/append?hl=ja
    pub async fn append_values<T>(&self, client: &Client, range: T, values: Value) -> Result<AppendValuesResponse>
    where
        T: AsRef<str>,
    {
        let response = client
            .post(&format!(
                "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}:append?valueInputOption={}&insertDataOption={}",
                self.spreadsheet_id,
                range.as_ref(),
                ValueInputOption::USER_ENTERED.to_string(),
                InsertDataOption::INSERT_ROWS.to_string(),
            ))
            .bearer_auth(&self.access_token)
            .json(&values)
            .send()
            .await?;
        let status_ref = response.error_for_status_ref();

        match status_ref {
            Ok(_) => {
                match response.json::<AppendValuesResponse>().await {
                    Ok(clear_response) => Ok(clear_response),
                    Err(e) => Err(anyhow::anyhow!("failed to append values: {}", e)),
                }
            },
            Err(e) => Err(anyhow::anyhow!("failed to append values: {}", e)),
        }
    }

    /// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/clear?hl=ja
    pub async fn clear_values<T>(&self, client: &Client, range: T) -> Result<ClearValuesResponse>
    where
        T: AsRef<str>,
    {
        let response = client
            .post(&format!(
                "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}:clear",
                self.spreadsheet_id, range.as_ref(),
            ))
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        let status_ref = response.error_for_status_ref();

        match status_ref {
            Ok(_) => {
                match response.json::<ClearValuesResponse>().await {
                    Ok(clear_response) => Ok(clear_response),
                    Err(e) => Err(anyhow::anyhow!("failed to clear values: {}", e)),
                }
            },
            Err(e) => Err(anyhow::anyhow!("failed to clear values: {}", e)),
        }
    }

    /// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/batchUpdate?hl=ja
    pub async fn batch_update() -> Result<BatchUpdateResponse> {
        let response = client
            .post(&format!(
                "https://sheets.googleapis.com/v4/spreadsheets/{}/values:batchUpdate",
                self.spreadsheet_id,
            ))
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        let status_ref = response.error_for_status_ref();

        match status_ref {
            Ok(_) => {
                match response.json::<BatchUpdateResponse>().await {
                    Ok(response) => Ok(response),
                    Err(e) => Err(anyhow::anyhow!("failed to batch update: {}", e)),
                }
            },
            Err(e) => Err(anyhow::anyhow!("failed to batch update: {}", e)),
        }
    }
}
