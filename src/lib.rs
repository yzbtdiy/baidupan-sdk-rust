//! 百度网盘 Rust SDK
//!
//! 这是百度网盘开放平台的 Rust SDK,提供了完整的 API 封装。
//!
//! # 主要功能
//!
//! - OAuth 认证
//! - 文件管理(上传、下载、删除、移动等)
//! - 文件信息查询
//! - 用户信息查询
//!
//! # 示例
//!
//! ```no_run
//! use bd_sdk_rust::{BaiduPanClient, Config};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::new("your_access_token");
//!     let client = BaiduPanClient::new(config);
//!
//!     // 使用客户端调用 API
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod client;
pub mod config;
pub mod error;
pub mod models;

pub use client::BaiduPanClient;
pub use config::Config;
pub use error::{Error, Result};
