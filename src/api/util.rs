use crate::{api::{GridRange, SortSpec}, param::{Dimension, SortOrder}};
use serde_json::{json, Value};

pub fn create_delete_duplicates_request<T>(sheet_id: T, start_index: T, end_index: T) -> Value
where
    T: Into<u32> + Copy,
{
    json!({
        "deleteDuplicates": {
            "range": GridRange {
                sheet_id: sheet_id.into(),
                start_row_index: None,
                end_row_index: None,
                start_column_index: None,
                end_column_index: None,
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

pub fn create_sort_range_request<T>(sheet_id: T, start_row_index: T, column_index: T, sort_order: SortOrder) -> Value
where
    T: Into<u32> + Copy,
{
    json!({
        "sortRange": {
            "range": GridRange {
                sheet_id: sheet_id.into(),
                start_row_index: Some(start_row_index.into()),
                end_row_index: None,
                start_column_index: None,
                end_column_index: None,
            },
            "sortSpecs": [
                SortSpec {
                    sort_order: sort_order,
                    foreground_color_style: None,
                    background_color_style: None,
                    dimension_index: column_index.into(),
                    data_source_column_reference: None,
                }
            ],
        }
    })
}
