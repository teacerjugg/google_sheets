use serde::Deserialize;
use serde_json::Value;

/// https://developers.google.com/sheets/api/reference/rest/v4/Dimension?hl=ja
#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize)]
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
    pub major_dimension: Dimension,
    pub values: Vec<Vec<Value>>,
}
