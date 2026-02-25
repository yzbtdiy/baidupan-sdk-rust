use crate::client::BaiduPanClient;
use crate::error::{Error, Result};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// 下载 User-Agent，百度网盘要求设置该值
const DOWNLOAD_USER_AGENT: &str = "pan.baidu.com";

impl BaiduPanClient {
    /// 下载文件到本地
    ///
    /// 文件下载流程：
    /// 1. 通过 `file_metas` 接口（dlink=1）获取文件的 dlink 下载链接
    /// 2. 调用本接口传入 dlink 进行下载
    ///
    /// # 参数
    /// - `dlink`: 文件下载链接，通过 `file_metas` 接口获取（dlink 有效期 8 小时）
    /// - `save_path`: 本地保存路径
    ///
    /// # 注意
    /// - dlink 有效期为 8 小时，过期后需重新获取
    /// - 下载时会自动跟随 302 跳转
    ///
    /// # 示例
    /// ```no_run
    /// # use baidupan_sdk_rust::BaiduPanClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BaiduPanClient::new("access_token");
    ///
    /// // 先获取下载链接
    /// let metas = client.file_metas("[123456]", Some(1), None, None, None).await?;
    /// if let Some(file) = metas.list.first() {
    ///     if let Some(dlink) = &file.dlink {
    ///         client.download_file(dlink, "/tmp/downloaded_file.txt").await?;
    ///         println!("文件下载完成");
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_file(&self, dlink: &str, save_path: impl AsRef<Path>) -> Result<u64> {
        self.download_file_range(dlink, save_path, None, None).await
    }

    /// 下载文件的指定字节范围（支持断点续传）
    ///
    /// # 参数
    /// - `dlink`: 文件下载链接，通过 `file_metas` 接口获取
    /// - `save_path`: 本地保存路径
    /// - `range_start`: 起始字节位置（包含），None 表示从文件开头
    /// - `range_end`: 结束字节位置（包含），None 表示到文件末尾
    ///
    /// # 示例
    /// ```no_run
    /// # use baidupan_sdk_rust::BaiduPanClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BaiduPanClient::new("access_token");
    ///
    /// // 下载文件的前 1MB
    /// client.download_file_range(
    ///     "https://d.pcs.baidu.com/...",
    ///     "/tmp/partial.dat",
    ///     Some(0),
    ///     Some(1024 * 1024 - 1),
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_file_range(
        &self,
        dlink: &str,
        save_path: impl AsRef<Path>,
        range_start: Option<u64>,
        range_end: Option<u64>,
    ) -> Result<u64> {
        let bytes: Vec<u8> = self.download_bytes(dlink, range_start, range_end).await?;
        let written = bytes.len() as u64;

        // 如果有断点续传起始位置，则以追加模式打开文件
        let mut file = if range_start.is_some() && range_start != Some(0) {
            tokio::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(save_path)
                .await
                .map_err(Error::IoError)?
        } else {
            File::create(save_path).await.map_err(Error::IoError)?
        };

        file.write_all(&bytes).await.map_err(Error::IoError)?;
        file.flush().await.map_err(Error::IoError)?;

        Ok(written)
    }

    /// 下载文件内容为字节数组
    ///
    /// 与 `download_file` 类似，但返回字节数组而不是写入磁盘，便于自定义处理。
    ///
    /// # 参数
    /// - `dlink`: 文件下载链接
    /// - `range_start`: 起始字节位置（包含），None 表示从文件开头
    /// - `range_end`: 结束字节位置（包含），None 表示到文件末尾
    ///
    /// # 示例
    /// ```no_run
    /// # use baidupan_sdk_rust::BaiduPanClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BaiduPanClient::new("access_token");
    /// let bytes = client.download_bytes("https://d.pcs.baidu.com/...", None, None).await?;
    /// println!("下载了 {} 字节", bytes.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_bytes(
        &self,
        dlink: &str,
        range_start: Option<u64>,
        range_end: Option<u64>,
    ) -> Result<Vec<u8>> {
        // 将 access_token 拼接到 dlink URL
        let mut url = url::Url::parse(dlink)?;
        url.query_pairs_mut()
            .append_pair("access_token", &self.config.access_token);

        if self.config.debug {
            eprintln!("DOWNLOAD {}", url);
        }

        // 构造请求，必须使用 pan.baidu.com 作为 User-Agent
        let mut request = self
            .http_client
            .get(url)
            .header("User-Agent", DOWNLOAD_USER_AGENT);

        // 添加 Range 头以支持断点续传
        if range_start.is_some() || range_end.is_some() {
            let range = build_range_header(range_start, range_end);
            request = request.header("Range", range);
        }

        let response = request.send().await.map_err(Error::HttpError)?;

        let status = response.status();
        if !status.is_success() && status.as_u16() != 206 {
            let err_text = response.text().await.unwrap_or_default();
            return Err(Error::ApiError {
                errno: status.as_u16() as i32,
                message: format!("下载失败: {}", err_text),
            });
        }

        let bytes = response.bytes().await.map_err(Error::HttpError)?;
        Ok(bytes.to_vec())
    }
}

/// 构建 Range 请求头字符串
///
/// - `Some(start), Some(end)` → `bytes=start-end`
/// - `Some(start), None`      → `bytes=start-`
/// - `None, Some(end)`        → `bytes=-end`
/// - `None, None`             → `bytes=0-`
fn build_range_header(start: Option<u64>, end: Option<u64>) -> String {
    match (start, end) {
        (Some(s), Some(e)) => format!("bytes={}-{}", s, e),
        (Some(s), None) => format!("bytes={}-", s),
        (None, Some(e)) => format!("bytes=-{}", e),
        (None, None) => "bytes=0-".to_string(),
    }
}
