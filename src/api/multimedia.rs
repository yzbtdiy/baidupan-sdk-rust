use crate::client::BaiduPanClient;
use crate::error::Result;
use crate::models::file::{FileListResponse, FileMetasResponse};
use std::collections::HashMap;

impl BaiduPanClient {
    /// 递归获取所有文件列表
    ///
    /// # 参数
    /// - `path`: 目录路径
    /// - `recursion`: 是否递归 (0=不递归, 1=递归)
    /// - `start`: 起始位置，默认 0
    /// - `limit`: 返回数量，默认 1000
    ///
    /// # 示例
    /// ```no_run
    /// # use baidupan_sdk_rust::BaiduPanClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BaiduPanClient::new("access_token");
    /// let result = client.file_list_all("/apps/myapp", 1, None, None).await?;
    /// println!("Total files: {}", result.list.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn file_list_all(
        &self,
        path: &str,
        recursion: i32,
        start: Option<i32>,
        limit: Option<i32>,
    ) -> Result<FileListResponse> {
        let url = format!("{}/rest/2.0/xpan/multimedia", self.config.server.pan_url);

        let mut params = HashMap::new();
        params.insert("method".to_string(), "listall".to_string());
        params.insert("path".to_string(), path.to_string());
        params.insert("recursion".to_string(), recursion.to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        if let Some(s) = start {
            params.insert("start".to_string(), s.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }

        let response = self.get(&url, params).await?;
        BaiduPanClient::parse_response(response).await
    }

    /// 获取多媒体文件元数据
    ///
    /// 此接口可以获取文件的详细信息，包括：
    /// - 文件下载链接 (dlink)
    /// - 缩略图
    /// - 视频时长、分辨率
    /// - 图片尺寸、EXIF 信息
    ///
    /// # 参数
    /// - `fsids`: 文件 ID 列表，用逗号分隔，如 "[123456,789012]"
    /// - `dlink`: 是否返回下载链接 (0=否, 1=是)
    /// - `thumb`: 缩略图尺寸，如 "1" 表示获取缩略图
    /// - `extra`: 是否返回额外信息 (0=否, 1=是)
    /// - `needmedia`: 是否需要多媒体信息 (0=否, 1=是)
    ///
    /// # 示例
    /// ```no_run
    /// # use baidupan_sdk_rust::BaiduPanClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BaiduPanClient::new("access_token");
    /// // 获取文件下载链接
    /// let result = client.file_metas("[123456,789012]", Some(1), None, None, None).await?;
    /// for item in result.list {
    ///     if let Some(dlink) = item.dlink {
    ///         println!("Download: {}", dlink);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn file_metas(
        &self,
        fsids: &str,
        dlink: Option<i32>,
        thumb: Option<&str>,
        extra: Option<i32>,
        needmedia: Option<i32>,
    ) -> Result<FileMetasResponse> {
        let url = format!("{}/rest/2.0/xpan/multimedia", self.config.server.pan_url);

        let mut params = HashMap::new();
        params.insert("method".to_string(), "filemetas".to_string());
        params.insert("fsids".to_string(), fsids.to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        if let Some(d) = dlink {
            params.insert("dlink".to_string(), d.to_string());
        }
        if let Some(t) = thumb {
            params.insert("thumb".to_string(), t.to_string());
        }
        if let Some(e) = extra {
            params.insert("extra".to_string(), e.to_string());
        }
        if let Some(n) = needmedia {
            params.insert("needmedia".to_string(), n.to_string());
        }

        let response = self.get(&url, params).await?;
        BaiduPanClient::parse_response(response).await
    }
}
