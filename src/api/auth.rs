use crate::models::auth::{DeviceCodeResponse, OAuthTokenResponse};
use crate::{BaiduPanClient, Result};
use std::collections::HashMap;

impl BaiduPanClient {
    /// 通过授权码获取访问令牌
    ///
    /// # 参数
    ///
    /// * `code` - 授权码
    /// * `client_id` - 应用 ID
    /// * `client_secret` - 应用密钥
    /// * `redirect_uri` - 回调地址
    pub async fn oauth_token_code2token(
        code: &str,
        client_id: &str,
        client_secret: &str,
        redirect_uri: &str,
    ) -> Result<OAuthTokenResponse> {
        let url = format!("{}/oauth/2.0/token", "https://openapi.baidu.com");

        let mut params = HashMap::new();
        params.insert("grant_type".to_string(), "authorization_code".to_string());
        params.insert("code".to_string(), code.to_string());
        params.insert("client_id".to_string(), client_id.to_string());
        params.insert("client_secret".to_string(), client_secret.to_string());
        params.insert("redirect_uri".to_string(), redirect_uri.to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        // 创建临时客户端(不需要 access_token)
        let client = reqwest::Client::new();
        let response = client.get(&url).query(&params).send().await?;

        BaiduPanClient::parse_response(response).await
    }

    /// 获取设备授权码
    ///
    /// # 参数
    ///
    /// * `client_id` - 应用 ID
    /// * `scope` - 授权范围
    pub async fn oauth_token_device_code(
        client_id: &str,
        scope: &str,
    ) -> Result<DeviceCodeResponse> {
        let url = format!("{}/oauth/2.0/device/code", "https://openapi.baidu.com");

        let mut params = HashMap::new();
        params.insert("response_type".to_string(), "device_code".to_string());
        params.insert("client_id".to_string(), client_id.to_string());
        params.insert("scope".to_string(), scope.to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        let client = reqwest::Client::new();
        let response = client.get(&url).query(&params).send().await?;

        BaiduPanClient::parse_response(response).await
    }

    /// 通过设备码获取访问令牌
    ///
    /// # 参数
    ///
    /// * `device_code` - 设备码
    /// * `client_id` - 应用 ID
    /// * `client_secret` - 应用密钥
    pub async fn oauth_token_device_token(
        device_code: &str,
        client_id: &str,
        client_secret: &str,
    ) -> Result<OAuthTokenResponse> {
        let url = format!("{}/oauth/2.0/token", "https://openapi.baidu.com");

        let mut params = HashMap::new();
        params.insert("grant_type".to_string(), "device_token".to_string());
        params.insert("code".to_string(), device_code.to_string());
        params.insert("client_id".to_string(), client_id.to_string());
        params.insert("client_secret".to_string(), client_secret.to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        let client = reqwest::Client::new();
        let response = client.get(&url).query(&params).send().await?;

        BaiduPanClient::parse_response(response).await
    }

    /// 刷新访问令牌
    ///
    /// # 参数
    ///
    /// * `refresh_token` - 刷新令牌
    /// * `client_id` - 应用 ID
    /// * `client_secret` - 应用密钥
    pub async fn oauth_token_refresh(
        refresh_token: &str,
        client_id: &str,
        client_secret: &str,
    ) -> Result<OAuthTokenResponse> {
        let url = format!("{}/oauth/2.0/token", "https://openapi.baidu.com");

        let mut params = HashMap::new();
        params.insert("grant_type".to_string(), "refresh_token".to_string());
        params.insert("refresh_token".to_string(), refresh_token.to_string());
        params.insert("client_id".to_string(), client_id.to_string());
        params.insert("client_secret".to_string(), client_secret.to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        let client = reqwest::Client::new();
        let response = client.get(&url).query(&params).send().await?;

        BaiduPanClient::parse_response(response).await
    }
}
