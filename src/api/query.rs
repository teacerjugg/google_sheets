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
