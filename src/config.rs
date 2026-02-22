use std::time::Duration;

/// 百度网盘服务器配置
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// 主服务器 URL
    pub pcs_url: String,
    /// PAN API URL
    pub pan_url: String,
    /// OpenAPI URL
    pub openapi_url: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            pcs_url: "https://d.pcs.baidu.com".to_string(),
            pan_url: "https://pan.baidu.com".to_string(),
            openapi_url: "https://openapi.baidu.com".to_string(),
        }
    }
}

/// SDK 配置
#[derive(Debug, Clone)]
pub struct Config {
    /// 访问令牌
    pub access_token: String,

    /// 服务器配置
    pub server: ServerConfig,

    /// 请求超时时间
    pub timeout: Duration,

    /// User-Agent
    pub user_agent: String,

    /// 是否启用调试模式
    pub debug: bool,
}

impl Config {
    /// 创建新的配置
    ///
    /// # 参数
    ///
    /// * `access_token` - 访问令牌
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
            server: ServerConfig::default(),
            timeout: Duration::from_secs(120),
            user_agent: format!("bd-sdk-rust/{}", env!("CARGO_PKG_VERSION")),
            debug: false,
        }
    }

    /// 设置服务器配置
    pub fn with_server(mut self, server: ServerConfig) -> Self {
        self.server = server;
        self
    }

    /// 设置超时时间
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 设置 User-Agent
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// 启用调试模式
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
}
