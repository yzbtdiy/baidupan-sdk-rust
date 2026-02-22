use baidupan_sdk_rust::{BaiduPanClient, Config};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== 百度网盘多媒体 API 演示 ===\n");

    // 从环境变量获取 access_token
    let access_token = std::env::var("BAIDUPAN_ACCESS_TOKEN")
        .expect("请设置环境变量 BAIDUPAN_ACCESS_TOKEN");

    let config = Config::new(access_token);
    let client = BaiduPanClient::new(config);

    // 1. 递归获取文件列表
    println!("1. 递归获取文件列表");
    println!("   正在获取 /apps 目录下的所有文件（递归）...");
    match client.file_list_all("/apps", 1, Some(0), Some(100)).await {
        Ok(result) => {
            println!("   找到 {} 个文件/文件夹:\n", result.list.len());
            for (i, file) in result.list.iter().take(10).enumerate() {
                let file_type = if file.isdir == Some(1) {
                    "目录"
                } else {
                    "文件"
                };
                println!(
                    "   {}. [{}] {} (ID: {})",
                    i + 1,
                    file_type,
                    file.server_filename.as_deref().unwrap_or("未知"),
                    file.fs_id.unwrap_or(0)
                );
            }
            if result.list.len() > 10 {
                println!("   ... 还有 {} 个文件未显示", result.list.len() - 10);
            }

            // 2. 如果有文件，获取文件元数据和下载链接
            if let Some(first_file) = result
                .list
                .iter()
                .find(|f| f.isdir != Some(1) && f.fs_id.is_some())
            {
                let fs_id = first_file.fs_id.unwrap();
                println!("\n2. 获取文件元数据和下载链接");
                println!("   正在获取文件 {} 的详细信息...", fs_id);

                let fsids = format!("[{}]", fs_id);
                match client.file_metas(&fsids, Some(1), Some("1"), Some(1), Some(1)).await {
                    Ok(metas) => {
                        if let Some(file_meta) = metas.list.first() {
                            println!("   文件详细信息:");
                            println!("   - 文件名: {}", file_meta.server_filename.as_deref().unwrap_or("未知"));
                            println!("   - 文件大小: {} 字节", file_meta.size.unwrap_or(0));
                            println!("   - MD5: {}", file_meta.md5.as_deref().unwrap_or("未知"));
                            println!("   - 分类: {}", file_meta.category.unwrap_or(0));

                            if let Some(dlink) = &file_meta.dlink {
                                println!("   - 下载链接: {}...", &dlink[..50.min(dlink.len())]);
                                println!("\n   ✓ 获取下载链接成功！");
                                println!("   提示: 下载文件需要在请求中添加 access_token 参数或 Authorization 请求头");
                            } else {
                                println!("   ✗ 未返回下载链接");
                            }

                            if file_meta.thumbs.is_some() {
                                println!("   - 缩略图: 已获取");
                            }

                            if file_meta.extra.is_some() {
                                println!("   - 额外信息: 已获取");
                            }
                        }
                    }
                    Err(e) => {
                        println!("   ✗ 获取文件元数据失败: {}", e);
                    }
                }
            } else {
                println!("\n2. 没有找到可用于演示的文件");
            }
        }
        Err(e) => {
            println!("   ✗ 获取文件列表失败: {}", e);
        }
    }

    // 3. 批量获取文件元数据
    println!("\n3. 批量获取多个文件的元数据");
    println!("   正在获取根目录文件列表...");
    match client.file_list("/", None, None, Some(0), Some(5)).await {
        Ok(list_result) => {
            let file_ids: Vec<i64> = list_result
                .list
                .iter()
                .filter(|f| f.isdir != Some(1) && f.fs_id.is_some())
                .take(3)
                .map(|f| f.fs_id.unwrap())
                .collect();

            if !file_ids.is_empty() {
                let fsids = format!("[{}]", file_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(","));
                println!("   正在批量获取 {} 个文件的元数据...", file_ids.len());

                match client.file_metas(&fsids, Some(1), None, None, None).await {
                    Ok(metas) => {
                        println!("   成功获取 {} 个文件的元数据:\n", metas.list.len());
                        for (i, meta) in metas.list.iter().enumerate() {
                            println!(
                                "   {}. {} (大小: {} 字节)",
                                i + 1,
                                meta.server_filename.as_deref().unwrap_or("未知"),
                                meta.size.unwrap_or(0)
                            );
                            if let Some(dlink) = &meta.dlink {
                                println!("      下载链接: {}...", &dlink[..40.min(dlink.len())]);
                            }
                        }
                    }
                    Err(e) => {
                        println!("   ✗ 批量获取元数据失败: {}", e);
                    }
                }
            } else {
                println!("   没有找到可用于演示的文件");
            }
        }
        Err(e) => {
            println!("   ✗ 获取文件列表失败: {}", e);
        }
    }

    println!("\n=== 演示完成 ===");
    Ok(())
}
