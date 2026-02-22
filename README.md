# 百度网盘 Rust SDK

这是百度网盘开放平台的 Rust SDK,参考官方 Go SDK 进行重构,提供了完整的 API 封装。

## 功能特性

- ✅ OAuth 认证(授权码模式、设备码模式、刷新令牌)
- ✅ 用户信息查询
- ✅ 文件列表查询与搜索
- ✅ 文件管理(创建文件夹、删除、移动、复制、重命名)
- ✅ 文件上传(支持分片上传和秒传)
- ✅ 完整的类型安全和错误处理
- ✅ 异步 API(基于 Tokio)

## 快速开始

### 添加依赖

在 `Cargo.toml` 中添加:

```toml
[dependencies]
bd-sdk-rust = "0.1"
tokio = { version = "1", features = ["full"] }
```

### 基本使用

```rust
use bd_sdk_rust::{BaiduPanClient, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 获取设备授权码
    let device_code_response = BaiduPanClient::oauth_token_device_code(
        "your_client_id",
        "basic,netdisk"
    ).await?;

    println!("请访问: {}", device_code_response.verification_url);
    println!("输入用户码: {}", device_code_response.user_code);

    // 2. 轮询获取 access_token
    let token_response = BaiduPanClient::oauth_token_device_token(
        &device_code_response.device_code,
        "your_client_id",
        "your_client_secret"
    ).await?;

    // 3. 创建客户端
    let config = Config::new(token_response.access_token);
    let client = BaiduPanClient::new(config);

    // 4. 获取用户信息
    let user_info = client.get_user_info().await?;
    println!("欢迎, {:?}!", user_info.baidu_name);

    // 5. 获取文件列表
    let files = client.file_list("/", None, None, None, None).await?;
    for file in files.list {
        println!("  - {}", file.server_filename.unwrap_or_default());
    }

    Ok(())
}
```

## API 文档

### 认证 API

```rust
// 获取设备授权码
let device_code = BaiduPanClient::oauth_token_device_code(
    client_id,
    scope
).await?;

// 通过设备码获取访问令牌
let token = BaiduPanClient::oauth_token_device_token(
    &device_code.device_code,
    client_id,
    client_secret
).await?;

// 通过授权码获取访问令牌
let token = BaiduPanClient::oauth_token_code2token(
    code,
    client_id,
    client_secret,
    redirect_uri
).await?;

// 刷新访问令牌
let new_token = BaiduPanClient::oauth_token_refresh(
    refresh_token,
    client_id,
    client_secret
).await?;
```

### 用户信息 API

```rust
// 获取用户信息
let user_info = client.get_user_info().await?;

// 获取配额信息
let quota = client.get_quota().await?;
println!("总空间: {} 字节", quota.total);
println!("已使用: {} 字节", quota.used);
```

### 文件信息 API

```rust
// 获取文件列表
let files = client.file_list(
    "/path",                // 目录路径
    Some("time"),          // 排序字段(name/time/size)
    Some(1),               // 降序(0:升序,1:降序)
    Some(0),               // 起始位置
    Some(100)              // 返回数量
).await?;

// 搜索文件
let results = client.file_search(
    "关键字",
    Some("/搜索目录"),
    Some(1)                // 是否递归
).await?;

// 获取图片列表
let images = client.file_image_list().await?;

// 获取文档列表
let docs = client.file_doc_list().await?;
```

### 文件管理 API

```rust
// 创建文件夹
client.create_dir("/新文件夹").await?;

// 删除文件
client.delete_files(&["/文件1.txt", "/文件2.txt"]).await?;

// 移动文件
client.move_files(
    &["/源路径/文件.txt"],
    &["/目标路径/文件.txt"]
).await?;

// 复制文件
client.copy_files(
    &["/源路径/文件.txt"],
    &["/目标路径/文件.txt"]
).await?;

// 重命名文件
client.rename_file("/旧名称.txt", "新名称.txt").await?;
```

### 文件上传 API

```rust
use std::path::Path;

// 上传文件(自动处理分片上传和秒传)
let result = client.upload_file(
    Path::new("本地文件.txt"),
    "/远程路径/文件.txt",
    None  // 可选: 分片大小(默认 4MB)
).await?;

println!("文件 ID: {}", result.fs_id);
println!("文件路径: {}", result.path);
```

## 配置选项

```rust
use std::time::Duration;

let config = Config::new("access_token")
    .with_timeout(Duration::from_secs(60))    // 设置请求超时
    .with_user_agent("MyApp/1.0")             // 设置 User-Agent
    .with_debug(true);                        // 启用调试模式

let client = BaiduPanClient::new(config);
```

## 错误处理

SDK 提供了完整的错误类型:

```rust
use bd_sdk_rust::Error;

match client.get_user_info().await {
    Ok(user_info) => {
        // 处理成功情况
    }
    Err(Error::HttpError(e)) => {
        eprintln!("HTTP 请求失败: {}", e);
    }
    Err(Error::ApiError { errno, message }) => {
        eprintln!("API 错误 {}: {}", errno, message);
    }
    Err(e) => {
        eprintln!("其他错误: {}", e);
    }
}
```

## 项目结构

```
bd-sdk-rust/
├── src/
│   ├── lib.rs           # 库入口
│   ├── client.rs        # 客户端核心
│   ├── config.rs        # 配置管理
│   ├── error.rs         # 错误类型
│   ├── api/             # API 模块
│   │   ├── auth.rs      # 认证 API
│   │   ├── fileinfo.rs  # 文件信息 API
│   │   ├── filemanager.rs  # 文件管理 API
│   │   ├── fileupload.rs   # 文件上传 API
│   │   └── userinfo.rs     # 用户信息 API
│   └── models/          # 数据模型
│       ├── auth.rs      # 认证相关模型
│       ├── file.rs      # 文件相关模型
│       └── user.rs      # 用户相关模型
├── Cargo.toml
└── README.md
```

## 与 Go SDK 的对比

| 特性 | Go SDK | Rust SDK |
|------|--------|----------|
| 类型安全 | ✅ | ✅✅ (更强的类型系统) |
| 错误处理 | error | Result<T, Error> |
| 异步支持 | 同步 | async/await |
| 内存安全 | GC | 零成本抽象 |
| 代码行数 | ~6800 | ~2000 |

## 依赖

- `reqwest` - HTTP 客户端
- `serde` - 序列化/反序列化
- `tokio` - 异步运行时
- `thiserror` - 错误处理
- `md5` - MD5 计算

## 开发

```bash
# 克隆项目
git clone https://github.com/yourusername/bd-sdk-rust.git
cd bd-sdk-rust

# 构建项目
cargo build

# 运行测试
cargo test

# 运行示例
cargo run
```

## 许可证

MIT License

## 参考

- [百度网盘开放平台文档](https://pan.baidu.com/union/doc/)
- [官方 Go SDK](https://github.com/tickstep/cloudpan189-go)

## 贡献

欢迎提交 Issue 和 Pull Request!
