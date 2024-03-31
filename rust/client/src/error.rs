pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
 
    
    #[error(transparent)]
    SystemTime(#[from] std::time::SystemTimeError),

   
    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),

    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),

    #[error("Invalid secret key")]
    SecretKey,

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
 
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error(transparent)]
    UrlParserError(#[from] url::ParseError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Qs(#[from] serde_qs::Error),
    #[error(transparent)]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),
 
    #[error("invalid listen key : {0}")]
    InvalidListenKey(String),
    #[error("unknown symbol {0}")]
    UnknownSymbol(String),
    #[error("{msg}")]
    InvalidOrderError { msg: String },
    #[error("invalid price")]
    InvalidPrice,
    #[error("invalid period {0}")]
    InvalidPeriod(String),
    #[error("internal server error")]
    InternalServerError,
    #[error("service unavailable")]
    ServiceUnavailable,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("{0}")]
    Msg(String),
}
