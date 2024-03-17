use serde::Deserialize;
use serde_json::Value;
use super::request::ValueRange;

/// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/clear?hl=ja#response-body
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearValuesResponse {
    pub spreadsheet_id: String,
    pub cleared_range: String,
}

/// https://developers.google.com/sheets/api/reference/rest/v4/UpdateValuesResponse?hl=ja
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateValuesResponse {
    pub spreadsheet_id: String,
    pub updated_range: String,
    pub updated_rows: u32,
    pub updated_colums: u32,
    pub updated_cells: u64,
    pub updated_data: Option<ValueRange>,
}

/// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/append?hl=ja#response-body
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppendValuesResponse {
    pub spreadsheet_id: String,
    pub table_range: String,
    pub updates: UpdateValuesResponse,
}
