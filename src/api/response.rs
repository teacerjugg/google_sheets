use serde::Deserialize;
use serde_json::Value;

/// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/clear?hl=ja#response-body
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearResponse {
    pub spreadsheet_id: String,
    pub cleared_range: String,
}

/// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values?hl=ja#ValueRange
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueRange {
    pub range: String,
    pub major_dimension: String,
    pub values: Vec<Vec<Value>>,
}
