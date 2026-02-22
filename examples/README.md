# 百度网盘 Rust SDK - 示例代码

这个目录包含了各种使用场景的示例代码。

## 📚 示例列表

### 1. 完整测试 Demo (`complete_demo.rs`)

这是最完整的示例，包含了所有主要功能的演示：

- ✅ OAuth 设备码授权流程
- ✅ 获取用户信息
- ✅ 查询存储配额
- ✅ 文件列表查询
- ✅ 创建文件夹
- ✅ 搜索文件
- ✅ 文件重命名和删除

**运行方式:**
```bash
# 1. 在代码中配置你的 AppKey 和 SecretKey
# 2. 运行示例
cargo run --example complete_demo
```

**使用前准备:**
1. 访问 [百度开放平台](https://pan.baidu.com/union/console/applist)
2. 创建应用获取 AppKey 和 SecretKey
3. 在代码中替换相应的配置

### 2. 文件上传示例 (`simple_upload.rs`)

演示如何上传文件到百度网盘。

**运行方式:**
```bash
cargo run --example simple_upload
```

**注意:**
- 需要提前获取 access_token
- 支持自动分片上传
- 支持秒传

### 3. 刷新 Token 示例 (`refresh_token.rs`)

演示如何使用 Refresh Token 刷新 Access Token。

**运行方式:**
```bash
cargo run --example refresh_token
```

**使用场景:**
- Access Token 过期后刷新
- 避免重复授权
- 长期使用场景

### 4. 多媒体文件 API 示例 (`multimedia_demo.rs`)

演示如何使用多媒体文件 API 获取文件元数据和下载链接。

**运行方式:**
```bash
# 需要先设置环境变量
export BAIDUPAN_ACCESS_TOKEN="your_access_token"
cargo run --example multimedia_demo
```

**功能演示:**
- ✅ 递归获取所有文件列表
- ✅ 获取文件下载链接 (dlink)
- ✅ 批量获取文件元数据
- ✅ 获取缩略图和额外信息

**使用场景:**
- 获取文件下载链接用于下载
- 批量处理文件信息
- 获取视频/图片的多媒体信息

## 🚀 快速开始

### 第一次使用

1. **运行完整示例获取凭证:**
```bash
cargo run --example complete_demo
```

2. **按照提示完成授权流程**
   - 获取设备码
   - 在浏览器中授权
   - 获取 Access Token 和 Refresh Token

3. **保存 Refresh Token**
   - 用于下次快速获取新的 Access Token

### 日常使用

使用保存的 Refresh Token 快速获取新 token:
```bash
cargo run --example refresh_token
```

## 📖 代码示例

### 获取用户信息

```rust
use baidupan_sdk_rust::{BaiduPanClient, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new("your_access_token");
    let client = BaiduPanClient::new(config);

    let user_info = client.get_user_info().await?;
    println!("用户: {:?}", user_info.baidu_name);

    Ok(())
}
```

### 上传文件

```rust
use baidupan_sdk_rust::{BaiduPanClient, Config};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new("your_access_token");
    let client = BaiduPanClient::new(config);

    let result = client.upload_file(
        Path::new("local_file.txt"),
        "/remote_file.txt",
        None
    ).await?;

    println!("上传成功! 文件 ID: {}", result.fs_id);

    Ok(())
}
```

### 文件管理

```rust
// 创建文件夹
client.create_dir("/my_folder").await?;

// 获取文件列表
let files = client.file_list("/", None, None, None, None).await?;

// 搜索文件
let results = client.file_search("keyword", Some("/"), Some(1)).await?;

// 删除文件
client.delete_files(&["/file1.txt", "/file2.txt"]).await?;

// 移动文件
client.move_files(&["/old_path"], &["/new_path"]).await?;
```

## ⚠️ 注意事项

1. **不要将凭证提交到版本控制**
   - AppKey / SecretKey
   - Access Token / Refresh Token

2. **Access Token 有效期**
   - 通常为 30 天
   - 过期后需要用 Refresh Token 刷新

3. **Refresh Token 管理**
   - 妥善保存 Refresh Token
   - 刷新后可能获得新的 Refresh Token
   - 及时更新保存的 Token

4. **错误处理**
   - 所有示例都包含完整的错误处理
   - 参考示例代码学习最佳实践

## 🔗 相关链接

- [百度网盘开放平台](https://pan.baidu.com/union/doc/)
- [API 文档](https://pan.baidu.com/union/doc/nksg0sbbo)
- [应用管理](https://pan.baidu.com/union/console/applist)

## 💡 提示

遇到问题？

1. 检查 AppKey 和 SecretKey 是否正确
2. 确认 Access Token 是否有效
3. 查看错误消息获取详细信息
4. 参考完整示例代码
