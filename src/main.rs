use baidupan_sdk_rust::{BaiduPanClient, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("百度网盘 Rust SDK 示例");
    println!("====================\n");

    // 注意: 在实际使用前,请先获取 access_token
    // 可以使用以下方法之一:
    // 1. 授权码模式: BaiduPanClient::oauth_token_code2token()
    // 2. 设备码模式: BaiduPanClient::oauth_token_device_code()

    println!("示例 1: 获取设备授权码");
    println!("---------------------");
    let client_id = "your_client_id";
    let scope = "basic,netdisk";

    match BaiduPanClient::oauth_token_device_code(client_id, scope).await {
        Ok(response) => {
            println!("设备码: {}", response.device_code);
            println!("用户码: {}", response.user_code);
            println!("验证 URL: {}", response.verification_url);
            println!("二维码 URL: {}", response.qrcode_url);
            println!("有效期: {} 秒", response.expires_in);
        }
        Err(e) => {
            println!("获取设备码失败: {}", e);
        }
    }

    println!("\n示例 2: 使用 access_token 创建客户端");
    println!("------------------------------------");
    let access_token = "your_access_token_here";
    let config = Config::new(access_token).with_debug(true);

    let _client = BaiduPanClient::new(config);

    println!("客户端已创建");

    println!("\n示例 3: 获取用户信息");
    println!("--------------------");
    // 取消注释以下代码来测试(需要有效的 access_token):
    // match client.get_user_info().await {
    //     Ok(user_info) => {
    //         println!("百度账号: {:?}", user_info.baidu_name);
    //         println!("网盘账号: {:?}", user_info.netdisk_name);
    //         println!("VIP 类型: {:?}", user_info.vip_type);
    //     }
    //     Err(e) => {
    //         println!("获取用户信息失败: {}", e);
    //     }
    // }

    println!("\n示例 4: 获取配额信息");
    println!("--------------------");
    // match client.get_quota().await {
    //     Ok(quota) => {
    //         println!("总空间: {} 字节", quota.total);
    //         println!("已使用: {} 字节", quota.used);
    //         if let Some(free) = quota.free {
    //             println!("剩余空间: {} 字节", free);
    //         }
    //     }
    //     Err(e) => {
    //         println!("获取配额失败: {}", e);
    //     }
    // }

    println!("\n示例 5: 获取文件列表");
    println!("--------------------");
    // match client.file_list("/", Some("time"), Some(1), Some(0), Some(10)).await {
    //     Ok(file_list) => {
    //         println!("文件数量: {}", file_list.list.len());
    //         for file in file_list.list {
    //             println!("  - {}", file.server_filename.unwrap_or_default());
    //         }
    //     }
    //     Err(e) => {
    //         println!("获取文件列表失败: {}", e);
    //     }
    // }

    println!("\n示例 6: 创建文件夹");
    println!("------------------");
    // match client.create_dir("/test_folder").await {
    //     Ok(_) => {
    //         println!("文件夹创建成功");
    //     }
    //     Err(e) => {
    //         println!("创建文件夹失败: {}", e);
    //     }
    // }

    println!("\n示例 7: 上传文件");
    println!("----------------");
    // use std::path::Path;
    // let local_path = Path::new("test.txt");
    // let remote_path = "/test.txt";
    //
    // match client.upload_file(local_path, remote_path, None).await {
    //     Ok(response) => {
    //         println!("文件上传成功");
    //         println!("文件 ID: {}", response.fs_id);
    //         println!("文件路径: {}", response.path);
    //         println!("文件大小: {} 字节", response.size);
    //     }
    //     Err(e) => {
    //         println!("上传文件失败: {}", e);
    //     }
    // }

    println!("\n更多示例请参考文档。");
    println!("注意: 所有 API 调用都需要有效的 access_token。");

    Ok(())
}
