use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

/// https://developers.google.com/sheets/api/reference/rest/v4/ValueInputOption?hl=ja
#[allow(non_camel_case_types)]
pub enum ValueInputOption {
    INPUT_VALUE_OPTION_UNSPECIFIED,
    RAW,
    USER_ENTERED,
}

impl std::fmt::Display for ValueInputOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValueInputOption::INPUT_VALUE_OPTION_UNSPECIFIED => {
                write!(f, "INPUT_VALUE_OPTION_UNSPECIFIED")
            }
            ValueInputOption::RAW => write!(f, "RAW"),
            ValueInputOption::USER_ENTERED => write!(f, "USER_ENTERED"),
        }
    }
}

/// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/append?hl=ja#InsertDataOption
#[allow(non_camel_case_types)]
pub enum InsertDataOption {
    OVERWRITE,
    INSERT_ROWS,
}

impl std::fmt::Display for InsertDataOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InsertDataOption::OVERWRITE => write!(f, "OVERWRITE"),
            InsertDataOption::INSERT_ROWS => write!(f, "INSERT_ROWS"),
        }
    }
}

pub struct GoogleSheets {
    spreadsheet_id: String,
    access_token: String,
}

impl GoogleSheets {
    pub async fn get_values<T>(&self, client: &Client, range: T) -> Result<Value>
    where
        T: AsRef<str>,
    {
        let response = client
            .get(&format!(
                "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}",
                self.spreadsheet_id,
                range.as_ref(),
            ))
            .bearer_auth(&self.access_token)
            .send()
            .await?;

        let values = response
            .json::<Value>()
            .await?
            .get("values")
            .expect("values not found")
            .to_owned();

        Ok(values)
    }

    pub async fn append_values<T>(&self, client: &Client, range: T, values: Value) -> Result<()>
    where
        T: AsRef<str>,
    {
        let response = client
            .post(&format!(
                "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}:append?valueInputOption={}&insertDataOption={}",
                self.spreadsheet_id,
                range.as_ref(),
                ValueInputOption::USER_ENTERED.to_string(),
                InsertDataOption::INSERT_ROWS.to_string(),
            ))
            .bearer_auth(&self.access_token)
            .json(&values)
            .send()
            .await?;

        let _ = response.json::<Value>().await?;

        Ok(())
    }
}
