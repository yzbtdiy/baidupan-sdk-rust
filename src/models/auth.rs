use serde::{Deserialize, Serialize};

/// OAuth 令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokenResponse {
    /// 访问令牌
    pub access_token: String,

    /// 刷新令牌
    #[serde(default)]
    pub refresh_token: Option<String>,

    /// 过期时间(秒)
    pub expires_in: i64,

    /// 令牌类型
    #[serde(default)]
    pub token_type: Option<String>,

    /// 授权范围
    #[serde(default)]
    pub scope: Option<String>,
}

/// 设备授权码响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCodeResponse {
    /// 设备码
    pub device_code: String,

    /// 用户码
    pub user_code: String,

    /// 验证 URL
    pub verification_url: String,

    /// 二维码图片 URL
    pub qrcode_url: String,

    /// 过期时间(秒)
    pub expires_in: i64,

    /// 轮询间隔(秒)
    pub interval: i64,
}
