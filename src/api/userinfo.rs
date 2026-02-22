use crate::models::user::{QuotaResponse, UserInfoResponse};
use crate::{BaiduPanClient, Result};
use std::collections::HashMap;

impl BaiduPanClient {
    /// 获取用户信息
    pub async fn get_user_info(&self) -> Result<UserInfoResponse> {
        let url = format!("{}/rest/2.0/xpan/nas", self.config.server.pan_url);

        let mut params = HashMap::new();
        params.insert("method".to_string(), "uinfo".to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        let response = self.get(&url, params).await?;
        BaiduPanClient::parse_response(response).await
    }

    /// 获取配额信息
    pub async fn get_quota(&self) -> Result<QuotaResponse> {
        let url = format!("{}/api/quota", self.config.server.pan_url);

        let mut params = HashMap::new();
        params.insert("checkfree".to_string(), "1".to_string());
        params.insert("checkexpire".to_string(), "1".to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        let response = self.get(&url, params).await?;
        BaiduPanClient::parse_response(response).await
    }
}
