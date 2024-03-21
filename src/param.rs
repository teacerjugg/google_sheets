use serde::{Deserialize, Serialize};

/// https://developers.google.com/sheets/api/reference/rest/v4/Dimension?hl=ja
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum Dimension {
    DIMENSION_UNSPECIFIED,
    ROWS,
    COLUMNS,
}

/// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/other?hl=ja#SortOrder
#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum SortOrder {
    ASCENDING,
    DESCENDING,
}
