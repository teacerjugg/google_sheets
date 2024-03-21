use crate::param::Dimension;
use super::query::ValueInputOption;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
