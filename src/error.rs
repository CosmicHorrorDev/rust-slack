#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("utf8 error, slack responses should be valid utf8")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("`serde_json::error::Error`")]
    Serialize(#[from] serde_json::error::Error),
    #[error("`rustc_serialize::hex::FromHexError`")]
    FromHex(#[from] hex::FromHexError),
    #[error("`reqwest::Error`")]
    Reqwest(#[from] reqwest::Error),
    #[error("`url::ParseError`")]
    Url(#[from] url::ParseError),
    #[error("`std::io::Error`")]
    Io(#[from] std::io::Error),
    #[error("slack service error: {0}")]
    Slack(String),
    #[error("hex color parsing error: {0}")]
    HexColor(String),
}

pub type Result<T> = std::result::Result<T, Error>;
