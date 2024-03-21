use serde::Serialize;
use serde_json::Value;

use crate::param::SortOrder;

/// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/batchUpdate?hl=ja#request-body
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpdateRequest {
    pub requests: Vec<Value>,
    pub include_spreadsheet_in_response: bool,
    pub response_ranges: Option<Vec<String>>,
    pub response_include_grid_data: Option<bool>,
}

/// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/other?hl=ja#ExtendedValue
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ExtendedValue {
    NumberValue(f64),
    StringValue(String),
    BoolValue(bool),
    FormulaValue(String),
    ErrorValue(Value),
}

// /// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/cells?hl=ja#CellFormat
// #[derive(Debug, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct CellFormat {
//     number_format: Option<NumberFormat>,
//     background_color: Option<Color>,
//     borders: Option<Borders>,
//     padding: Option<Padding>,
//     horizontal_alignment: Option<String>,
//     vertical_alignment: Option<String>,
//     wrap_strategy: Option<String>,
//     text_direction: Option<String>,
//     text_format: Option<TextFormat>,
//     hyperlink_display_type: Option<String>,
// }

/// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/cells?hl=ja#CellData
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CellData {
    pub user_entered_value: ExtendedValue,
    pub effective_value: Option<ExtendedValue>,
    pub formatted_value: Option<String>,
    pub user_entered_format: Option<Value>,
    pub effective_format: Option<Value>,
    pub hyperlink: Option<String>,
    pub note: Option<String>,
    pub text_format_runs: Option<Vec<Value>>,
    pub data_validation: Option<Value>,
    pub pivot_table: Option<Value>,
    pub data_source_table: Option<Value>,
    pub data_source_formula: Option<Value>,
}

/// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/other?hl=ja#GridRange
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridRange {
    pub sheet_id: u32,
    pub start_row_index: Option<u32>,
    pub end_row_index: Option<u32>,
    pub start_column_index: Option<u32>,
    pub end_column_index: Option<u32>,
}

/// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/other?hl=ja#SortSpec
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SortSpec {
    pub sort_order: SortOrder,
    pub foreground_color_style: Option<Value>,
    pub background_color_style: Option<Value>,
    pub dimension_index: u32,
    pub data_source_column_reference: Option<Value>,
}
