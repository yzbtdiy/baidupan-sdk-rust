use serde::{Deserialize, Serialize};

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// 文件 ID
    #[serde(default)]
    pub fs_id: Option<i64>,

    /// 文件路径
    #[serde(default)]
    pub path: Option<String>,

    /// 服务器文件名
    #[serde(default)]
    pub server_filename: Option<String>,

    /// 文件大小(字节)
    #[serde(default)]
    pub size: Option<i64>,

    /// 是否为目录(0: 否, 1: 是)
    #[serde(default)]
    pub isdir: Option<i32>,

    /// 文件分类
    #[serde(default)]
    pub category: Option<i32>,

    /// 创建时间
    #[serde(default)]
    pub ctime: Option<i64>,

    /// 修改时间
    #[serde(default)]
    pub mtime: Option<i64>,

    /// MD5 值
    #[serde(default)]
    pub md5: Option<String>,

    /// 文件来源类型
    #[serde(default)]
    pub from_type: Option<i32>,
}

/// 文件列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileListResponse {
    /// 文件列表
    pub list: Vec<FileInfo>,

    /// 游标,用于分页
    #[serde(default)]
    pub cursor: Option<String>,

    /// 是否有更多数据 (0=否, 1=是)
    #[serde(default)]
    pub has_more: Option<i32>,
}

/// 文件预创建响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePrecreateResponse {
    /// 上传 ID
    pub uploadid: String,

    /// 返回类型(0: 预创建成功, 1: 秒传成功)
    #[serde(rename = "return_type")]
    pub return_type: i32,

    /// 分片信息(秒传失败时返回)
    #[serde(default)]
    pub block_list: Option<Vec<i32>>,

    /// 文件信息(秒传成功时返回)
    #[serde(flatten)]
    pub file_info: Option<FileInfo>,
}

/// 文件创建响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCreateResponse {
    /// 文件 ID
    pub fs_id: i64,

    /// 文件路径
    pub path: String,

    /// 服务器文件名
    pub server_filename: String,

    /// 文件大小
    pub size: i64,

    /// MD5 值
    #[serde(default)]
    pub md5: Option<String>,

    /// 创建时间
    pub ctime: i64,

    /// 修改时间
    pub mtime: i64,

    /// 是否为目录
    pub isdir: i32,

    /// 文件分类
    pub category: i32,
}

/// 文件操作响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperationResponse {
    /// 任务 ID
    #[serde(default)]
    pub taskid: Option<i64>,

    /// 操作信息
    #[serde(default)]
    pub info: Option<Vec<FileOperationInfo>>,
}

/// 文件操作信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperationInfo {
    /// 文件路径
    #[serde(default)]
    pub path: Option<String>,

    /// 文件 ID
    #[serde(default)]
    pub fs_id: Option<i64>,
}

/// 多媒体文件元数据信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetaInfo {
    /// 文件 ID
    #[serde(default)]
    pub fs_id: Option<i64>,

    /// 文件路径
    #[serde(default)]
    pub path: Option<String>,

    /// 服务器文件名
    #[serde(default)]
    pub server_filename: Option<String>,

    /// 文件大小(字节)
    #[serde(default)]
    pub size: Option<i64>,

    /// 是否为目录(0: 否, 1: 是)
    #[serde(default)]
    pub isdir: Option<i32>,

    /// 文件分类
    #[serde(default)]
    pub category: Option<i32>,

    /// 创建时间
    #[serde(default)]
    pub ctime: Option<i64>,

    /// 修改时间
    #[serde(default)]
    pub mtime: Option<i64>,

    /// MD5 值
    #[serde(default)]
    pub md5: Option<String>,

    /// 下载链接
    #[serde(default)]
    pub dlink: Option<String>,

    /// 缩略图地址
    #[serde(default)]
    pub thumbs: Option<serde_json::Value>,

    /// 额外信息(包含多媒体信息)
    #[serde(default)]
    pub extra: Option<serde_json::Value>,

    /// 文件来源类型
    #[serde(default)]
    pub from_type: Option<i32>,
}

/// 多媒体文件元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetasResponse {
    /// 文件元数据列表
    pub list: Vec<FileMetaInfo>,
}
