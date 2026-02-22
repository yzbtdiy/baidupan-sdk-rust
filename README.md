# 百度网盘 Rust SDK

这是百度网盘开放平台的 Rust SDK,参考官方 Go SDK 进行重构,提供了完整的 API 封装。

[![Crates.io](https://img.shields.io/crates/v/baidupan-sdk-rust.svg)](https://crates.io/crates/baidupan-sdk-rust)
[![Documentation](https://docs.rs/baidupan-sdk-rust/badge.svg)](https://docs.rs/baidupan-sdk-rust)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.txt)

## 功能特性

- ✅ OAuth 认证(授权码模式、设备码模式、刷新令牌)
- ✅ 用户信息查询
- ✅ 文件列表查询与搜索
- ✅ 文件管理(创建文件夹、删除、移动、复制、重命名)
- ✅ 文件上传(支持分片上传和秒传)
- ✅ 多媒体文件 API(递归列表、文件元数据、下载链接)
- ✅ 完整的类型安全和错误处理
- ✅ 异步 API(基于 Tokio)
- ✅ 完整的示例代码

## 快速开始

### 1. 添加依赖

在 `Cargo.toml` 中添加:

```toml
[dependencies]
baidupan-sdk-rust = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

### 2. 获取 AppKey 和 SecretKey

访问 [百度开放平台](https://pan.baidu.com/union/console/applist) 创建应用:

1. 登录百度账号
2. 创建新应用
3. 获取 **AppKey** (client_id) 和 **SecretKey** (client_secret)

### 3. 运行示例程序

本项目提供了 3 个完整的示例程序:

#### 📱 完整功能演示

```bash
# 编辑 examples/complete_demo.rs 配置你的 AppKey 和 SecretKey
cargo run --example complete_demo
```

包含:
- OAuth 设备码授权流程
- 获取用户信息和配额
- 文件列表、搜索
- 创建/重命名/删除文件夹

#### 📤 文件上传示例

```bash
cargo run --example simple_upload
```

#### 🔄 刷新 Token 示例

```bash
cargo run --example refresh_token
```

更多示例详见 [examples/README.md](examples/README.md)

## 基本使用

```rust
use baidupan_sdk_rust::{BaiduPanClient, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 配置你的凭证
    let app_key = "your_app_key";
    let secret_key = "your_secret_key";

    // 1. 获取设备授权码
    let device_code = BaiduPanClient::oauth_token_device_code(
        app_key,
        "basic,netdisk"
    ).await?;

    println!("请访问: {}", device_code.verification_url);
    println!("输入用户码: {}", device_code.user_code);

    // 2. 等待用户授权后获取 access_token
    let token = BaiduPanClient::oauth_token_device_token(
        &device_code.device_code,
        app_key,
        secret_key
    ).await?;

    // 3. 创建客户端
    let config = Config::new(token.access_token);
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

### 多媒体文件 API

```rust
// 递归获取所有文件列表
let all_files = client.file_list_all(
    "/apps/myapp",     // 目录路径
    1,                 // 是否递归(0=否, 1=是)
    Some(0),           // 起始位置
    Some(100)          // 返回数量
).await?;

// 获取文件元数据和下载链接
let metas = client.file_metas(
    "[123456,789012]", // 文件 ID 列表
    Some(1),           // 返回下载链接(0=否, 1=是)
    Some("1"),         // 缩略图尺寸
    Some(1),           // 返回额外信息(0=否, 1=是)
    Some(1)            // 需要多媒体信息(0=否, 1=是)
).await?;

// 使用下载链接
for file in metas.list {
    if let Some(dlink) = file.dlink {
        println!("文件: {}", file.server_filename.unwrap_or_default());
        println!("下载链接: {}", dlink);
        // 下载时需要在请求中添加 access_token
    }
}
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
use baidupan_sdk_rust::Error;

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

## 示例程序

本项目在 `examples/` 目录下提供了完整的示例程序:

### 📱 完整功能演示 ([complete_demo.rs](examples/complete_demo.rs))

包含从授权到文件操作的完整流程:
- OAuth 设备码授权
- 获取用户信息和配额
- 文件列表、搜索
- 创建/重命名/删除文件夹

```bash
# 1. 编辑文件配置 AppKey 和 SecretKey
# 2. 运行
cargo run --example complete_demo
```

### 📤 文件上传 ([simple_upload.rs](examples/simple_upload.rs))

演示文件上传功能:
```bash
cargo run --example simple_upload
```

### 🔄 刷新 Token ([refresh_token.rs](examples/refresh_token.rs))

演示如何刷新 Access Token:
```bash
cargo run --example refresh_token
```

### 🎬 多媒体文件 API ([multimedia_demo.rs](examples/multimedia_demo.rs))

演示如何获取文件元数据和下载链接:
```bash
export BAIDUPAN_ACCESS_TOKEN="your_access_token"
cargo run --example multimedia_demo
```

包含:
- 递归获取所有文件列表
- 获取文件下载链接
- 批量获取文件元数据

详细说明请查看 [examples/README.md](examples/README.md)

## 项目结构

```
baidupan-sdk-rust/
├── src/
│   ├── lib.rs              # 库入口
│   ├── main.rs             # 主程序示例
│   ├── client.rs           # 核心客户端
│   ├── config.rs           # 配置管理
│   ├── error.rs            # 错误类型
│   ├── api/                # API 模块
│   │   ├── mod.rs
│   │   ├── auth.rs         # 认证 API
│   │   ├── fileinfo.rs     # 文件信息 API
│   │   ├── filemanager.rs  # 文件管理 API
│   │   ├── fileupload.rs   # 文件上传 API
│   │   └── userinfo.rs     # 用户信息 API
│   └── models/             # 数据模型
│       ├── mod.rs
│       ├── auth.rs         # 认证模型
│       ├── file.rs         # 文件模型
│       └── user.rs         # 用户模型
├── examples/               # 示例程序
│   ├── README.md
│   ├── complete_demo.rs    # 完整功能演示
│   ├── simple_upload.rs    # 文件上传示例
│   └── refresh_token.rs    # Token 刷新示例
├── Cargo.toml
├── LICENSE.txt
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

本项目使用了以下优质 Rust 库:

- `reqwest` - HTTP 客户端 (支持 async)
- `serde` / `serde_json` - 序列化/反序列化
- `tokio` - 异步运行时 (仅使用必需的 features)
- `thiserror` - 错误处理
- `url` - URL 解析
- `md5` - MD5 计算(用于文件上传)

## 开发

```bash
# 克隆项目
git clone https://github.com/yzbtdiy/baidupan-sdk-rust.git
cd baidupan-sdk-rust

# 构建项目
cargo build

# 运行测试
cargo test

# 运行示例
cargo run --example complete_demo

# 代码检查
cargo clippy

# 格式化代码
cargo fmt
```

## 常见问题

### 如何获取 AppKey 和 SecretKey?

1. 访问 [百度开放平台](https://pan.baidu.com/union/console/applist)
2. 登录百度账号
3. 创建新应用
4. 在应用详情中查看 **AppKey** 和 **SecretKey**

### Access Token 有效期是多久?

Access Token 通常有效期为 30 天。过期后可以使用 Refresh Token 刷新:

```rust
let new_token = BaiduPanClient::oauth_token_refresh(
    refresh_token,
    app_key,
    secret_key
).await?;
```

### 如何保存凭证?

**不要将凭证硬编码到代码中!** 建议使用:

1. **环境变量**
```rust
let app_key = std::env::var("BAIDU_APP_KEY")?;
let secret_key = std::env::var("BAIDU_SECRET_KEY")?;
```

2. **配置文件** (添加到 .gitignore)
```toml
# config.toml
[baidu]
app_key = "your_app_key"
secret_key = "your_secret_key"
```

3. **密钥管理服务** (生产环境推荐)

## 路线图

- [x] OAuth 认证
- [x] 用户信息 API
- [x] 文件信息 API
- [x] 文件管理 API
- [x] 文件上传 API
- [ ] 文件下载 API
- [ ] 分享管理 API
- [ ] 离线下载 API
- [ ] 更多示例和文档

## 贡献

欢迎提交 Issue 和 Pull Request!

1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启 Pull Request

## 许可证

本项目采用 MIT 许可证。详见 [LICENSE.txt](LICENSE.txt) 文件。
