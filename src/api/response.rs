use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearResponse {
    pub spreadsheet_id: String,
    pub cleared_range: String,
}
