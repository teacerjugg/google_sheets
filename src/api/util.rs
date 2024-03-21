use crate::param::Dimension;
use serde_json::{json, Value};

pub fn create_delete_duplicates_request<T>(sheet_id: T, start_index: T, end_index: T) -> Value
where
    T: Into<u32> + Copy,
{
    json!({
        "deleteDuplicates": {
            "range": {
                "sheetId": sheet_id.into()
            },
            "comparisonColumns": [
                {
                    "sheetId": sheet_id.into(),
                    "dimension": Dimension::COLUMNS,
                    "startIndex": start_index.into(),
                    "endIndex": end_index.into(),
                }
            ],
        }
    })
}

pub fn create_sort_range_request<T>(sheet_id: T, column_index: T, sort_order: Dimension) -> Value
where
    T: Into<u32> + Copy,
{
    json!({
        "sortRange": {
            "range": {
                "sheetId": sheet_id.into(),
            },
            "sortSpecs": [
                {
                    "dimensionIndex": column_index.into(),
                    "sortOrder": sort_order,
                },
            ],
        }
    })
}
