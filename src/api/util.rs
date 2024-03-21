use crate::param::Dimension;
use serde_json::{json, Value};
use std::convert::TryInto::Error;

pub fn create_delete_duplicates_request<T>(
    sheet_id: T,
    start_index: T,
    end_index: T,
) -> Result<Value, Error>
where
    T: TryInto<u32>,
{
    json!({
        "deleteDuplicates": {
            "range": {
                "sheetId": sheet_id.try_into()?,
            },
            "comparisonColumns": [
                {
                    "sheetId": sheet_id.into(),
                    "dimension": Dimension::COLUMNS,
                    "startIndex": start_index.try_into()?,
                    "endIndex": end_index.try_into()?,
                }
            ],
        }
    })
}

pub fn create_sort_range_request<T>(
    sheet_id: T,
    column_index: T,
    sort_order: Dimension,
) -> Result<Value, Error>
where
    T: TryInto<u32>,
{
    json!({
        "sortRange": {
            "range": {
                "sheetId": sheet_id.try_into()?,
            },
            "sortSpecs": [
                {
                    "dimensionIndex": column_index.try_into()?,
                    "sortOrder": sort_order,
                },
            ],
        }
    })
}
