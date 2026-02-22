use serde::{Deserialize, Serialize};

/// 用户信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoResponse {
    /// 百度账号
    #[serde(default)]
    pub baidu_name: Option<String>,

    /// 网盘账号
    #[serde(default)]
    pub netdisk_name: Option<String>,

    /// 头像 URL
    #[serde(default)]
    pub avatar_url: Option<String>,

    /// VIP 类型(0: 普通用户, 1: 普通会员, 2: 超级会员)
    #[serde(default)]
    pub vip_type: Option<i32>,

    /// UK
    #[serde(default)]
    pub uk: Option<i64>,
}

/// 配额信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaResponse {
    /// 总空间(字节)
    pub total: i64,

    /// 已使用空间(字节)
    pub used: i64,

    /// 剩余空间(字节)
    #[serde(default)]
    pub free: Option<i64>,
}
