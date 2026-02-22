use crate::models::file::FileListResponse;
use crate::{BaiduPanClient, Result};
use std::collections::HashMap;

impl BaiduPanClient {
    /// 获取文件列表
    ///
    /// # 参数
    ///
    /// * `dir` - 目录路径
    /// * `order` - 排序字段(name/time/size)
    /// * `desc` - 是否降序(0: 升序, 1: 降序)
    /// * `start` - 起始位置
    /// * `limit` - 返回数量
    pub async fn file_list(
        &self,
        dir: &str,
        order: Option<&str>,
        desc: Option<i32>,
        start: Option<i32>,
        limit: Option<i32>,
    ) -> Result<FileListResponse> {
        let url = format!("{}/rest/2.0/xpan/file", self.config.server.pan_url);

        let mut params = HashMap::new();
        params.insert("method".to_string(), "list".to_string());
        params.insert("dir".to_string(), dir.to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        if let Some(order) = order {
            params.insert("order".to_string(), order.to_string());
        }
        if let Some(desc) = desc {
            params.insert("desc".to_string(), desc.to_string());
        }
        if let Some(start) = start {
            params.insert("start".to_string(), start.to_string());
        }
        if let Some(limit) = limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        let response = self.get(&url, params).await?;
        BaiduPanClient::parse_response(response).await
    }

    /// 搜索文件
    ///
    /// # 参数
    ///
    /// * `key` - 搜索关键字
    /// * `dir` - 搜索目录(可选)
    /// * `recursion` - 是否递归搜索
    pub async fn file_search(
        &self,
        key: &str,
        dir: Option<&str>,
        recursion: Option<i32>,
    ) -> Result<FileListResponse> {
        let url = format!("{}/rest/2.0/xpan/file", self.config.server.pan_url);

        let mut params = HashMap::new();
        params.insert("method".to_string(), "search".to_string());
        params.insert("key".to_string(), key.to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        if let Some(dir) = dir {
            params.insert("dir".to_string(), dir.to_string());
        }
        if let Some(recursion) = recursion {
            params.insert("recursion".to_string(), recursion.to_string());
        }

        let response = self.get(&url, params).await?;
        BaiduPanClient::parse_response(response).await
    }

    /// 获取图片列表
    pub async fn file_image_list(&self) -> Result<FileListResponse> {
        let url = format!("{}/rest/2.0/xpan/file", self.config.server.pan_url);

        let mut params = HashMap::new();
        params.insert("method".to_string(), "imagelist".to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        let response = self.get(&url, params).await?;
        BaiduPanClient::parse_response(response).await
    }

    /// 获取文档列表
    pub async fn file_doc_list(&self) -> Result<FileListResponse> {
        let url = format!("{}/rest/2.0/xpan/file", self.config.server.pan_url);

        let mut params = HashMap::new();
        params.insert("method".to_string(), "doclist".to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        let response = self.get(&url, params).await?;
        BaiduPanClient::parse_response(response).await
    }
}
