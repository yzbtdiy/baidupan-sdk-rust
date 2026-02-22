use baidupan_sdk_rust::BaiduPanClient;

/// 使用 Refresh Token 刷新 Access Token 示例
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("刷新 Access Token 示例\n");

    // 配置凭证
    let app_key = "your_app_key_here";
    let secret_key = "your_secret_key_here";
    let refresh_token = "your_refresh_token_here";

    if refresh_token == "your_refresh_token_here" {
        println!("❌ 请先配置:");
        println!("   - AppKey (从百度开放平台获取)");
        println!("   - SecretKey");
        println!("   - Refresh Token (首次授权后获得)\n");
        return Ok(());
    }

    println!("🔄 正在刷新 Access Token...\n");

    match BaiduPanClient::oauth_token_refresh(
        refresh_token,
        app_key,
        secret_key
    ).await {
        Ok(token_resp) => {
            println!("✅ 刷新成功!");
            println!("   新 Access Token: {}...", &token_resp.access_token[..50]);

            if let Some(new_refresh_token) = token_resp.refresh_token {
                println!("   新 Refresh Token: {}...", &new_refresh_token[..50]);
                println!("\n💡 请保存新的 Refresh Token 供下次使用");
            }

            println!("   过期时间: {} 秒", token_resp.expires_in);
        }
        Err(e) => {
            println!("❌ 刷新失败: {}", e);
        }
    }

    Ok(())
}
