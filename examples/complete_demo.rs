use baidupan_sdk_rust::{BaiduPanClient, Config};
use std::io::{self, Write};
use tokio::time::{sleep, Duration};

/// 完整的百度网盘 SDK 测试 Demo
///
/// 使用前请先配置你的 AppKey 和 SecretKey
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("  百度网盘 Rust SDK - 完整测试 Demo");
    println!("===========================================\n");

    // ==================== 配置凭证 ====================
    // 请替换为你在百度开放平台申请的凭证
    // 申请地址: https://pan.baidu.com/union/console/applist
    let app_key = "your_app_key_here";        // 替换为你的 AppKey
    let secret_key = "your_secret_key_here";  // 替换为你的 SecretKey

    if app_key == "your_app_key_here" || secret_key == "your_secret_key_here" {
        println!("❌ 错误: 请先在代码中配置你的 AppKey 和 SecretKey");
        println!("   申请地址: https://pan.baidu.com/union/console/applist\n");
        return Ok(());
    }

    // ==================== 步骤 1: 设备码授权 ====================
    println!("📱 步骤 1: 获取设备授权码");
    println!("─────────────────────────────────────────");

    let device_code_resp = match BaiduPanClient::oauth_token_device_code(
        app_key,
        "basic,netdisk"
    ).await {
        Ok(resp) => {
            println!("✅ 设备码获取成功!");
            println!("   设备码: {}", resp.device_code);
            println!("   用户码: {}", resp.user_code);
            println!("   验证URL: {}", resp.verification_url);
            println!("   二维码: {}", resp.qrcode_url);
            println!("   有效期: {} 秒", resp.expires_in);
            println!("   轮询间隔: {} 秒\n", resp.interval);
            resp
        }
        Err(e) => {
            println!("❌ 获取设备码失败: {}", e);
            return Err(Box::new(e) as Box<dyn std::error::Error>);
        }
    };

    // ==================== 步骤 2: 等待用户授权 ====================
    println!("🔐 步骤 2: 等待用户授权");
    println!("─────────────────────────────────────────");
    println!("请在浏览器中打开以下链接:");
    println!("👉 {}", device_code_resp.verification_url);
    println!("\n然后输入用户码: {}\n", device_code_resp.user_code);

    print!("按回车键开始轮询授权状态...");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    // ==================== 步骤 3: 轮询获取 Access Token ====================
    println!("\n⏳ 步骤 3: 轮询获取 Access Token");
    println!("─────────────────────────────────────────");

    let mut token_resp = None;
    let max_attempts = 30;
    let interval = device_code_resp.interval as u64;

    for attempt in 1..=max_attempts {
        print!("尝试 {}/{} ... ", attempt, max_attempts);
        io::stdout().flush().ok();

        match BaiduPanClient::oauth_token_device_token(
            &device_code_resp.device_code,
            app_key,
            secret_key
        ).await {
            Ok(resp) => {
                println!("✅ 授权成功!");
                println!("\n   Access Token: {}...", &resp.access_token[..50]);
                if let Some(ref refresh_token) = resp.refresh_token {
                    println!("   Refresh Token: {}...", &refresh_token[..50]);
                }
                println!("   过期时间: {} 秒", resp.expires_in);
                token_resp = Some(resp);
                break;
            }
            Err(e) => {
                println!("等待中... ({})", e);
                if attempt < max_attempts {
                    sleep(Duration::from_secs(interval)).await;
                }
            }
        }
    }

    let token = match token_resp {
        Some(t) => t,
        None => {
            println!("\n❌ 获取 Access Token 超时，请重试");
            return Ok(());
        }
    };

    // ==================== 步骤 4: 创建客户端 ====================
    println!("\n🔧 步骤 4: 创建 API 客户端");
    println!("─────────────────────────────────────────");

    let config = Config::new(token.access_token.clone())
        .with_debug(false);

    let client = BaiduPanClient::new(config);
    println!("✅ 客户端创建成功\n");

    // ==================== 步骤 5: 获取用户信息 ====================
    println!("👤 步骤 5: 获取用户信息");
    println!("─────────────────────────────────────────");

    match client.get_user_info().await {
        Ok(user_info) => {
            println!("✅ 用户信息:");
            println!("   百度账号: {:?}", user_info.baidu_name);
            println!("   网盘账号: {:?}", user_info.netdisk_name);
            println!("   VIP 类型: {:?}", user_info.vip_type);
            println!("   用户 UK: {:?}", user_info.uk);
        }
        Err(e) => {
            println!("❌ 获取用户信息失败: {}", e);
        }
    }

    // ==================== 步骤 6: 获取配额信息 ====================
    println!("\n💾 步骤 6: 获取存储配额");
    println!("─────────────────────────────────────────");

    match client.get_quota().await {
        Ok(quota) => {
            let total_gb = quota.total as f64 / (1024.0 * 1024.0 * 1024.0);
            let used_gb = quota.used as f64 / (1024.0 * 1024.0 * 1024.0);
            let free_gb = total_gb - used_gb;

            println!("✅ 存储配额:");
            println!("   总空间: {:.2} GB", total_gb);
            println!("   已使用: {:.2} GB ({:.1}%)", used_gb, (used_gb / total_gb) * 100.0);
            println!("   剩余空间: {:.2} GB", free_gb);
        }
        Err(e) => {
            println!("❌ 获取配额信息失败: {}", e);
        }
    }

    // ==================== 步骤 7: 获取文件列表 ====================
    println!("\n📁 步骤 7: 获取根目录文件列表");
    println!("─────────────────────────────────────────");

    match client.file_list("/", Some("time"), Some(1), Some(0), Some(10)).await {
        Ok(file_list) => {
            println!("✅ 文件列表 (最多显示 10 个):");
            if file_list.list.is_empty() {
                println!("   (空目录)");
            } else {
                for (index, file) in file_list.list.iter().enumerate() {
                    let name = file.server_filename.as_deref().unwrap_or("未命名");
                    let size = file.size.unwrap_or(0);
                    let is_dir = file.isdir.unwrap_or(0) == 1;

                    let type_icon = if is_dir { "📁" } else { "📄" };
                    let size_str = if is_dir {
                        "文件夹".to_string()
                    } else {
                        format_size(size)
                    };

                    println!("   {}. {} {} ({})", index + 1, type_icon, name, size_str);
                }
            }
        }
        Err(e) => {
            println!("❌ 获取文件列表失败: {}", e);
        }
    }

    // ==================== 步骤 8: 创建测试文件夹 ====================
    println!("\n📂 步骤 8: 创建测试文件夹");
    println!("─────────────────────────────────────────");

    let test_folder = "/SDK_TEST_DEMO";
    match client.create_dir(test_folder).await {
        Ok(_) => {
            println!("✅ 文件夹创建成功: {}", test_folder);
        }
        Err(e) => {
            println!("⚠️  创建文件夹失败 (可能已存在): {}", e);
        }
    }

    // ==================== 步骤 9: 搜索文件 ====================
    println!("\n🔍 步骤 9: 搜索文件");
    println!("─────────────────────────────────────────");

    match client.file_search("TEST", Some("/"), Some(1)).await {
        Ok(search_result) => {
            println!("✅ 搜索结果 (关键字: TEST):");
            if search_result.list.is_empty() {
                println!("   未找到匹配的文件");
            } else {
                for (index, file) in search_result.list.iter().take(5).enumerate() {
                    let name = file.server_filename.as_deref().unwrap_or("未命名");
                    let path = file.path.as_deref().unwrap_or("");
                    println!("   {}. {} ({})", index + 1, name, path);
                }
            }
        }
        Err(e) => {
            println!("⚠️  搜索失败: {}", e);
        }
    }

    // ==================== 步骤 10: 文件重命名 ====================
    println!("\n✏️  步骤 10: 文件操作测试");
    println!("─────────────────────────────────────────");

    match client.rename_file(test_folder, "SDK_TEST_RENAMED").await {
        Ok(_) => {
            println!("✅ 文件夹重命名成功: {} -> /SDK_TEST_RENAMED", test_folder);

            // 删除测试文件夹
            match client.delete_files(&["/SDK_TEST_RENAMED"]).await {
                Ok(_) => {
                    println!("✅ 测试文件夹已删除");
                }
                Err(e) => {
                    println!("⚠️  删除失败: {}", e);
                }
            }
        }
        Err(e) => {
            println!("⚠️  重命名失败: {}", e);
        }
    }

    // ==================== 完成 ====================
    println!("\n===========================================");
    println!("  ✅ 测试完成!");
    println!("===========================================");

    // 保存 refresh_token 提示
    if let Some(refresh_token) = token.refresh_token {
        println!("\n💡 提示:");
        println!("   请保存以下 Refresh Token 以便下次使用:");
        println!("   {}", refresh_token);
        println!("\n   下次可以使用以下代码刷新 token:");
        println!("   BaiduPanClient::oauth_token_refresh(");
        println!("       &refresh_token,");
        println!("       app_key,");
        println!("       secret_key");
        println!("   ).await?;");
    }

    println!("\n🎉 所有功能测试完成！");

    Ok(())
}

/// 格式化文件大小
fn format_size(bytes: i64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let size = bytes as f64;

    if size >= GB {
        format!("{:.2} GB", size / GB)
    } else if size >= MB {
        format!("{:.2} MB", size / MB)
    } else if size >= KB {
        format!("{:.2} KB", size / KB)
    } else {
        format!("{} B", bytes)
    }
}
