use crate::models::file::{FileCreateResponse, FilePrecreateResponse};
use crate::{BaiduPanClient, Result};
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

impl BaiduPanClient {
    /// 预创建文件
    ///
    /// # 参数
    ///
    /// * `path` - 上传路径
    /// * `size` - 文件大小
    /// * `block_list` - MD5 列表
    pub async fn file_precreate(
        &self,
        path: &str,
        size: i64,
        block_list: &[String],
    ) -> Result<FilePrecreateResponse> {
        let url = format!("{}/rest/2.0/xpan/file", self.config.server.pan_url);

        let mut params = HashMap::new();
        params.insert("method".to_string(), "precreate".to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        let body = json!({
            "path": path,
            "size": size,
            "isdir": 0,
            "autoinit": 1,
            "rtype": 1,
            "block_list": serde_json::to_string(block_list)?
        });

        let response = self.post(&url, params, Some(body)).await?;
        BaiduPanClient::parse_response(response).await
    }

    /// 创建文件
    ///
    /// # 参数
    ///
    /// * `path` - 上传路径
    /// * `size` - 文件大小
    /// * `uploadid` - 上传 ID
    /// * `block_list` - MD5 列表
    pub async fn file_create(
        &self,
        path: &str,
        size: i64,
        uploadid: &str,
        block_list: &[String],
    ) -> Result<FileCreateResponse> {
        let url = format!("{}/rest/2.0/xpan/file", self.config.server.pan_url);

        let mut params = HashMap::new();
        params.insert("method".to_string(), "create".to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        let body = json!({
            "path": path,
            "size": size,
            "isdir": 0,
            "uploadid": uploadid,
            "rtype": 1,
            "block_list": serde_json::to_string(block_list)?
        });

        let response = self.post(&url, params, Some(body)).await?;
        BaiduPanClient::parse_response(response).await
    }

    /// 分片上传
    ///
    /// # 参数
    ///
    /// * `path` - 上传路径
    /// * `uploadid` - 上传 ID
    /// * `partseq` - 分片序号(从 0 开始)
    /// * `data` - 分片数据
    pub async fn upload_slice(
        &self,
        path: &str,
        uploadid: &str,
        partseq: i32,
        data: &[u8],
    ) -> Result<String> {
        let url = format!("{}/rest/2.0/pcs/superfile2", self.config.server.pcs_url);

        let mut params = HashMap::new();
        params.insert("method".to_string(), "upload".to_string());
        params.insert("type".to_string(), "tmpfile".to_string());
        params.insert("path".to_string(), path.to_string());
        params.insert("uploadid".to_string(), uploadid.to_string());
        params.insert("partseq".to_string(), partseq.to_string());
        params.insert("openapi".to_string(), "xpansdk".to_string());

        // 构建 multipart 表单
        let form = reqwest::multipart::Form::new()
            .part("file", reqwest::multipart::Part::bytes(data.to_vec()));

        let mut url = url::Url::parse(&url)?;
        for (key, value) in params {
            url.query_pairs_mut().append_pair(&key, &value);
        }
        url.query_pairs_mut()
            .append_pair("access_token", &self.config.access_token);

        let response = self.http_client.post(url).multipart(form).send().await?;

        BaiduPanClient::parse_response(response).await
    }

    /// 上传文件(辅助方法)
    ///
    /// # 参数
    ///
    /// * `local_path` - 本地文件路径
    /// * `remote_path` - 远程文件路径
    /// * `chunk_size` - 分片大小(默认 4MB)
    pub async fn upload_file(
        &self,
        local_path: impl AsRef<Path>,
        remote_path: &str,
        chunk_size: Option<usize>,
    ) -> Result<FileCreateResponse> {
        let chunk_size = chunk_size.unwrap_or(4 * 1024 * 1024); // 4MB

        // 读取文件
        let mut file = File::open(local_path.as_ref()).await?;
        let file_size = file.metadata().await?.len() as i64;

        // 计算分片 MD5
        let mut block_list = Vec::new();
        let mut buffer = vec![0u8; chunk_size];
        let mut offset = 0;

        while offset < file_size as usize {
            let n = file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }

            let md5 = format!("{:x}", md5::compute(&buffer[..n]));
            block_list.push(md5);
            offset += n;
        }

        // 预创建
        let precreate_resp = self
            .file_precreate(remote_path, file_size, &block_list)
            .await?;

        // 如果秒传成功
        if precreate_resp.return_type == 1 {
            if let Some(file_info) = precreate_resp.file_info {
                return Ok(FileCreateResponse {
                    fs_id: file_info.fs_id.unwrap_or(0),
                    path: file_info.path.unwrap_or_default(),
                    server_filename: file_info.server_filename.unwrap_or_default(),
                    size: file_info.size.unwrap_or(0),
                    md5: file_info.md5,
                    ctime: file_info.ctime.unwrap_or(0),
                    mtime: file_info.mtime.unwrap_or(0),
                    isdir: file_info.isdir.unwrap_or(0),
                    category: file_info.category.unwrap_or(0),
                });
            }
        }

        // 重新打开文件进行分片上传
        let mut file = File::open(local_path.as_ref()).await?;
        let mut offset = 0;
        let mut partseq = 0;

        while offset < file_size as usize {
            let mut buffer = vec![0u8; chunk_size];
            let n = file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }

            buffer.truncate(n);
            self.upload_slice(remote_path, &precreate_resp.uploadid, partseq, &buffer)
                .await?;

            offset += n;
            partseq += 1;
        }

        // 创建文件
        self.file_create(
            remote_path,
            file_size,
            &precreate_resp.uploadid,
            &block_list,
        )
        .await
    }
}
