use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::query::ValueInputOption;

/// https://developers.google.com/sheets/api/reference/rest/v4/Dimension?hl=ja
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum Dimension {
    DIMENSION_UNSPECIFIED,
    ROWS,
    COLUMNS,
}

/// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values?hl=ja#ValueRange
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueRange {
    pub range: String,
    pub major_dimension: Option<Dimension>,
    pub values: Vec<Value>,
}

/// https://developers.google.com/sheets/api/reference/rest/v4/ValueRenderOption?hl=ja
#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum ValueRenderOption {
    FORMATTED_VALUE,
    UNFORMATTED_VALUE,
    FORMULA,
}

/// https://developers.google.com/sheets/api/reference/rest/v4/DateTimeRenderOption?hl=ja
#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum DateTimeRenderOption {
    SERIAL_NUMBER,
    FORMATTED_STRING,
}

/// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/batchUpdate?hl=ja#request-body
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpdateValuesRequest {
    pub value_input_option: ValueInputOption,
    pub data: Vec<ValueRange>,
    pub include_values_in_response: bool,
    pub response_value_render_option: ValueRenderOption,
    pub response_date_time_render_option: DateTimeRenderOption,
}

/// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/batchUpdate?hl=ja#request-body
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpdateRequest {
    pub requests: Vec<Value>,
    pub include_spreadsheet_in_response: bool,
    pub response_ranges: Option<Vec<String>>,
    pub response_include_grid_data: Option<bool>,
}
