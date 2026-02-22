use crate::{Config, Error, Result};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 百度网盘客户端
#[derive(Debug, Clone)]
pub struct BaiduPanClient {
    /// HTTP 客户端
    pub(crate) http_client: Client,

    /// 配置
    pub(crate) config: Config,
}

/// API 响应基础结构
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    /// 错误码,0 表示成功
    #[serde(default)]
    pub errno: i32,

    /// 错误消息
    #[serde(default)]
    pub errmsg: Option<String>,

    /// 响应数据
    #[serde(flatten)]
    pub data: Option<T>,
}

impl BaiduPanClient {
    /// 创建新的客户端
    ///
    /// # 参数
    ///
    /// * `config` - SDK 配置
    pub fn new(config: Config) -> Self {
        let http_client = Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            http_client,
            config,
        }
    }

    /// 获取配置的引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 发送 GET 请求
    pub(crate) async fn get(&self, url: &str, params: HashMap<String, String>) -> Result<Response> {
        let mut url = url::Url::parse(url)?;

        // 添加 access_token
        let mut query_params = params;
        query_params.insert("access_token".to_string(), self.config.access_token.clone());

        for (key, value) in query_params {
            url.query_pairs_mut().append_pair(&key, &value);
        }

        if self.config.debug {
            eprintln!("GET {}", url);
        }

        let response = self.http_client.get(url).send().await?;

        Ok(response)
    }

    /// 发送 POST 请求
    pub(crate) async fn post<T: Serialize>(
        &self,
        url: &str,
        params: HashMap<String, String>,
        body: Option<T>,
    ) -> Result<Response> {
        let mut url = url::Url::parse(url)?;

        // 添加 access_token
        let mut query_params = params;
        query_params.insert("access_token".to_string(), self.config.access_token.clone());

        for (key, value) in query_params {
            url.query_pairs_mut().append_pair(&key, &value);
        }

        if self.config.debug {
            eprintln!("POST {}", url);
        }

        let mut request = self.http_client.post(url);

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await?;

        Ok(response)
    }

    /// 发送 POST 表单请求 (application/x-www-form-urlencoded)
    pub(crate) async fn post_form(
        &self,
        url: &str,
        params: HashMap<String, String>,
        form_data: HashMap<String, String>,
    ) -> Result<Response> {
        let mut url = url::Url::parse(url)?;

        // 添加 access_token
        let mut query_params = params;
        query_params.insert("access_token".to_string(), self.config.access_token.clone());

        for (key, value) in query_params {
            url.query_pairs_mut().append_pair(&key, &value);
        }

        if self.config.debug {
            eprintln!("POST (form) {}", url);
            eprintln!("Form data: {:?}", form_data);
        }

        // 手动构建 form-urlencoded 字符串
        let form_body: String = form_data
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let response = self.http_client
            .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_body)
            .send()
            .await?;

        Ok(response)
    }

    /// 解析 API 响应
    pub(crate) async fn parse_response<T: DeserializeOwned>(response: Response) -> Result<T> {
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            return Err(Error::ApiError {
                errno: status.as_u16() as i32,
                message: text,
            });
        }

        // 尝试解析为 ApiResponse
        if let Ok(api_response) = serde_json::from_str::<ApiResponse<T>>(&text) {
            if api_response.errno != 0 {
                return Err(Error::ApiError {
                    errno: api_response.errno,
                    message: api_response
                        .errmsg
                        .unwrap_or_else(|| "Unknown error".to_string()),
                });
            }

            if let Some(data) = api_response.data {
                return Ok(data);
            }
        }

        // 直接解析为目标类型
        serde_json::from_str(&text).map_err(Error::from)
    }
}
