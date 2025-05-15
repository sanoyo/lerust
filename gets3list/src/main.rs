use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Error};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let bucket_name = if args.len() > 1 {
        args[1].clone()
    } else {
        "sample".to_string()
    };

    // デフォルトの認証情報とリージョン設定を読み込む
    let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    println!("バケット '{}' が存在するか確認中...", bucket_name);
    
    match client.head_bucket().bucket(&bucket_name).send().await {
        Ok(_) => {
            println!("✅ バケット '{}' は存在します！", bucket_name);
            
            match client.get_bucket_location().bucket(&bucket_name).send().await {
                Ok(location) => {
                    let region = location
                        .location_constraint()
                        .map(|l| l.as_str())
                        .unwrap_or("不明");
                    println!("リージョン: {}", region);
                }
                Err(e) => println!("リージョン取得エラー: {}", e),
            }
            
            // バケット内のオブジェクト数を取得して表示
            match client.list_objects_v2().bucket(&bucket_name).send().await {
                Ok(objects) => {
                    let count = objects.key_count();
                    println!("オブジェクト数: {}", count);
                    
                    // オブジェクトの一部を表示（最大5個）
                    if count > 0 {
                        println!("\nオブジェクトの一部（最大5個）:");
                        if let Some(contents) = objects.contents() {
                            for (i, object) in contents.iter().take(5).enumerate() {
                                println!("{}. {}", i+1, object.key().unwrap_or_default());
                            }
                        }
                    }
                }
                Err(e) => println!("オブジェクト情報取得エラー: {}", e),
            }
        }
        Err(e) => {
            println!("❌ バケット '{}' は存在しないか、アクセスできません", bucket_name);
            println!("エラー詳細: {}", e);
            
            // バケットが存在するか確認方法2: すべてのバケットを一覧して確認する
            println!("\n全バケット一覧から検索中...");
            match client.list_buckets().send().await {
                Ok(buckets_resp) => {
                    let buckets = buckets_resp.buckets().unwrap_or_default();
                    let found = buckets.iter().any(|b| b.name().unwrap_or_default() == bucket_name);
                    
                    if found {
                        println!("✅ バケット '{}' は一覧に存在しますが、アクセス権限がないかもしれません", bucket_name);
                    } else {
                        println!("❌ バケット '{}' は全バケット一覧にも見つかりませんでした", bucket_name);
                        
                        // 利用可能なバケットを表示
                        println!("\n利用可能なバケット一覧:");
                        if buckets.is_empty() {
                            println!("  バケットが見つかりませんでした");
                        } else {
                            for bucket in buckets {
                                println!("  - {}", bucket.name().unwrap_or_default());
                            }
                        }
                    }
                }
                Err(e) => println!("バケット一覧取得エラー: {}", e),
            }
        }
    }

    Ok(())
}
