use crate::models::file::FileOperationResponse;
use crate::{BaiduPanClient, Result};
use serde_json::json;
use std::collections::HashMap;

impl BaiduPanClient {
    /// 创建文件夹
    ///
    /// # 参数
    ///
    /// * `path` - 文件夹路径
    pub async fn create_dir(&self, path: &str) -> Result<FileOperationResponse> {
        let url = format!("{}/rest/2.0/xpan/file", self.config.server.pan_url);

        let mut params = HashMap::new();
        params.insert("method".to_string(), "create".to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        let body = json!({
            "path": path,
            "isdir": 1,
            "rtype": 0
        });

        let response = self.post(&url, params, Some(body)).await?;
        BaiduPanClient::parse_response(response).await
    }

    /// 删除文件
    ///
    /// # 参数
    ///
    /// * `paths` - 要删除的文件路径列表
    pub async fn delete_files(&self, paths: &[&str]) -> Result<FileOperationResponse> {
        let url = format!("{}/rest/2.0/xpan/file", self.config.server.pan_url);

        let mut params = HashMap::new();
        params.insert("method".to_string(), "filemanager".to_string());
        params.insert("opera".to_string(), "delete".to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        let file_list: Vec<_> = paths.iter().map(|p| json!({"path": p})).collect();

        let body = json!({
            "filelist": serde_json::to_string(&file_list)?
        });

        let response = self.post(&url, params, Some(body)).await?;
        BaiduPanClient::parse_response(response).await
    }

    /// 移动文件
    ///
    /// # 参数
    ///
    /// * `from_paths` - 源文件路径列表
    /// * `to_paths` - 目标文件路径列表
    pub async fn move_files(
        &self,
        from_paths: &[&str],
        to_paths: &[&str],
    ) -> Result<FileOperationResponse> {
        if from_paths.len() != to_paths.len() {
            return Err(crate::Error::ParamError(
                "源路径和目标路径数量不匹配".to_string(),
            ));
        }

        let url = format!("{}/rest/2.0/xpan/file", self.config.server.pan_url);

        let mut params = HashMap::new();
        params.insert("method".to_string(), "filemanager".to_string());
        params.insert("opera".to_string(), "move".to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        let file_list: Vec<_> = from_paths
            .iter()
            .zip(to_paths.iter())
            .map(|(from, to)| json!({"path": from, "dest": to, "newname": to}))
            .collect();

        let body = json!({
            "filelist": serde_json::to_string(&file_list)?
        });

        let response = self.post(&url, params, Some(body)).await?;
        BaiduPanClient::parse_response(response).await
    }

    /// 复制文件
    ///
    /// # 参数
    ///
    /// * `from_paths` - 源文件路径列表
    /// * `to_paths` - 目标文件路径列表
    pub async fn copy_files(
        &self,
        from_paths: &[&str],
        to_paths: &[&str],
    ) -> Result<FileOperationResponse> {
        if from_paths.len() != to_paths.len() {
            return Err(crate::Error::ParamError(
                "源路径和目标路径数量不匹配".to_string(),
            ));
        }

        let url = format!("{}/rest/2.0/xpan/file", self.config.server.pan_url);

        let mut params = HashMap::new();
        params.insert("method".to_string(), "filemanager".to_string());
        params.insert("opera".to_string(), "copy".to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        let file_list: Vec<_> = from_paths
            .iter()
            .zip(to_paths.iter())
            .map(|(from, to)| json!({"path": from, "dest": to, "newname": to}))
            .collect();

        let body = json!({
            "filelist": serde_json::to_string(&file_list)?
        });

        let response = self.post(&url, params, Some(body)).await?;
        BaiduPanClient::parse_response(response).await
    }

    /// 重命名文件
    ///
    /// # 参数
    ///
    /// * `path` - 文件路径
    /// * `new_name` - 新文件名
    pub async fn rename_file(&self, path: &str, new_name: &str) -> Result<FileOperationResponse> {
        let url = format!("{}/rest/2.0/xpan/file", self.config.server.pan_url);

        let mut params = HashMap::new();
        params.insert("method".to_string(), "filemanager".to_string());
        params.insert("opera".to_string(), "rename".to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        let file_list = vec![json!({"path": path, "newname": new_name})];

        let body = json!({
            "filelist": serde_json::to_string(&file_list)?
        });

        let response = self.post(&url, params, Some(body)).await?;
        BaiduPanClient::parse_response(response).await
    }
}
