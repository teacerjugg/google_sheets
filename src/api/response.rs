use serde::Deserialize;
use serde_json::Value;

/// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/batchUpdate?hl=ja#response-body
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpdateResponse {
    pub spreadsheet_id: String,
    pub replies: Option<Vec<Value>>,
    pub updated_spreadsheet: Option<Value>,
}
