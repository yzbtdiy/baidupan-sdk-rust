use thiserror::Error;

/// SDK 错误类型
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP 请求错误
    #[error("HTTP 请求失败: {0}")]
    HttpError(#[from] reqwest::Error),

    /// JSON 序列化/反序列化错误
    #[error("JSON 处理失败: {0}")]
    JsonError(#[from] serde_json::Error),

    /// API 返回错误
    #[error("API 错误 (errno: {errno}): {message}")]
    ApiError { errno: i32, message: String },

    /// 参数错误
    #[error("参数错误: {0}")]
    ParamError(String),

    /// URL 解析错误
    #[error("URL 解析失败: {0}")]
    UrlParseError(#[from] url::ParseError),

    /// IO 错误
    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    /// 其他错误
    #[error("其他错误: {0}")]
    Other(String),
}

/// SDK Result 类型
pub type Result<T> = std::result::Result<T, Error>;
