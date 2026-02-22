use baidupan_sdk_rust::{BaiduPanClient, Config};
use std::path::Path;

/// 简单的文件上传示例
///
/// 使用前请先通过其他方式获取 access_token
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("百度网盘文件上传示例\n");

    // 配置 access_token (需要提前获取)
    let access_token = "your_access_token_here";

    if access_token == "your_access_token_here" {
        println!("❌ 请先配置有效的 access_token");
        println!("   可以运行 complete_demo 获取 token\n");
        return Ok(());
    }

    // 创建客户端
    let config = Config::new(access_token);
    let client = BaiduPanClient::new(config);

    // 要上传的本地文件
    let local_file = Path::new("test.txt");
    let remote_path = "/test_upload.txt";

    // 如果文件不存在，创建一个测试文件
    if !local_file.exists() {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(local_file)?;
        file.write_all(b"Hello, Baidu Pan!\nThis is a test file from Rust SDK.")?;
        println!("✅ 创建测试文件: {:?}\n", local_file);
    }

    println!("📤 开始上传文件...");
    println!("   本地路径: {:?}", local_file);
    println!("   远程路径: {}\n", remote_path);

    match client.upload_file(local_file, remote_path, None).await {
        Ok(response) => {
            println!("✅ 上传成功!");
            println!("   文件 ID: {}", response.fs_id);
            println!("   文件路径: {}", response.path);
            println!("   文件大小: {} 字节", response.size);
            if let Some(md5) = response.md5 {
                println!("   MD5: {}", md5);
            }
        }
        Err(e) => {
            println!("❌ 上传失败: {}", e);
        }
    }

    Ok(())
}
