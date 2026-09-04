use std::fs;
use std::io::Write;
use std::path::Path;
use regex::Regex;
use crate::steam::parser::{import_zip_package, ZipImportResult};
use crate::steam::store_api;

const CLOUDFRONT_MANIFEST_URL: &str = "https://d41hvr6rtvs2p.cloudfront.net";

pub async fn download_and_install_manifest_by_appid(steam_path: &Path, appid: u32) -> Result<ZipImportResult, String> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("创建网络客户端失败: {}", e))?;

    let cf_url = format!("{}/{}.zip", CLOUDFRONT_MANIFEST_URL, appid);
    let proxy_url = format!(
        "https://remlua.com/proxy.php?url=https%3A%2F%2Fsteamgames554.s3.us-east-1.amazonaws.com%2F{}.zip",
        appid
    );

    let mut zip_bytes: Option<Vec<u8>> = None;

    // 1. Try direct CloudFront CDN mirror
    if let Ok(resp) = client.get(&cf_url).send().await {
        if resp.status().is_success() {
            if let Ok(b) = resp.bytes().await {
                zip_bytes = Some(b.to_vec());
            }
        }
    }

    // 2. If CloudFront returns 403/404 or fails, try Remlua proxy with Referer
    if zip_bytes.is_none() {
        if let Ok(resp) = client
            .get(&proxy_url)
            .header("Referer", "https://remlua.com/")
            .send()
            .await
        {
            if resp.status().is_success() {
                if let Ok(b) = resp.bytes().await {
                    zip_bytes = Some(b.to_vec());
                }
            }
        }
    }

    // 3. If pre-built zip exists, import manifests and lua
    if let Some(bytes) = zip_bytes {
        let temp_dir = std::env::temp_dir().join("steam_auto_manifests");
        let _ = fs::create_dir_all(&temp_dir);
        let temp_zip = temp_dir.join(format!("{}.zip", appid));

        let mut file = fs::File::create(&temp_zip).map_err(|e| format!("创建临时文件失败: {}", e))?;
        file.write_all(&bytes).map_err(|e| format!("写入临时清单包失败: {}", e))?;

        let import_res = import_zip_package(&temp_zip, steam_path)?;
        let _ = fs::remove_file(&temp_zip);

        return Ok(import_res);
    }

    // 4. Fallback: If cloud repository has no pre-packaged zip for this AppID (S3 returns 403/404 for missing keys),
    // fetch game metadata directly from Steam Store API, generate full Lua with all DLCs and deploy
    match store_api::fetch_steam_app_details(appid).await {
        Ok(store_details) => {
            let _ = crate::steam::parser::save_game_lua(steam_path, appid, &store_details.generated_lua)?;
            let dlc_count = store_details.dlcs.len();
            let msg = if dlc_count > 0 {
                format!(
                    "云端库暂无该游戏预打包清单，已自动从 Steam 官方生成完整入库与全套 DLC 脚本（含 {} 个 DLC）。客户端点击下载时，OpenSteamTool 将通过上游源自动拦截下载清单！",
                    dlc_count
                )
            } else {
                "云端库暂无该游戏预打包清单，已自动从 Steam 官方生成标准入库脚本。客户端点击下载时，OpenSteamTool 将通过上游源自动拦截下载清单！".to_string()
            };

            Ok(ZipImportResult {
                appid,
                name: store_details.name,
                lua_filename: format!("{}.lua", appid),
                extracted_manifests: Vec::new(),
                total_manifests: 0,
                message: msg,
            })
        }
        Err(_) => {
            Err(format!(
                "未在云端清单库或 Steam 官方商店中检索到 AppID 为 {} 的游戏数据，请核对游戏 ID 是否正确。",
                appid
            ))
        }
    }
}

pub fn get_ost_manifest_source(steam_path: &Path) -> String {
    let toml_path = steam_path.join("opensteamtool.toml");
    if toml_path.exists() {
        if let Ok(content) = fs::read_to_string(&toml_path) {
            let re = Regex::new(r#"(?m)^\s*url\s*=\s*["']([^"']+)["']"#).unwrap();
            if let Some(caps) = re.captures(&content) {
                return caps[1].to_string();
            }
        }
    }
    "opensteamtool".to_string()
}

pub fn set_ost_manifest_source(steam_path: &Path, source: &str) -> Result<String, String> {
    let toml_path = steam_path.join("opensteamtool.toml");
    let content = if toml_path.exists() {
        fs::read_to_string(&toml_path).unwrap_or_else(|_| crate::steam::unlocker::DEFAULT_OST_CONFIG.to_string())
    } else {
        crate::steam::unlocker::DEFAULT_OST_CONFIG.to_string()
    };

    let re = Regex::new(r#"(?m)^\s*url\s*=\s*["'][^"']*["']"#).unwrap();
    let updated_content = if re.is_match(&content) {
        re.replace(&content, format!(r#"url = "{}""#, source)).to_string()
    } else {
        format!("{}\n[manifest]\nurl = \"{}\"\n", content, source)
    };

    fs::write(&toml_path, updated_content).map_err(|e| format!("更新 opensteamtool.toml 失败: {}", e))?;
    Ok(format!("成功将 OpenSteamTool 清单上游源更新为: {}", source))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ost_manifest_source_toggle() {
        let temp_dir = std::env::temp_dir().join("test_ost_source");
        let _ = fs::create_dir_all(&temp_dir);
        let toml_path = temp_dir.join("opensteamtool.toml");
        let _ = fs::write(&toml_path, crate::steam::unlocker::DEFAULT_OST_CONFIG);

        assert_eq!(get_ost_manifest_source(&temp_dir), "opensteamtool");

        set_ost_manifest_source(&temp_dir, "wudrm").unwrap();
        assert_eq!(get_ost_manifest_source(&temp_dir), "wudrm");

        set_ost_manifest_source(&temp_dir, "steamrun").unwrap();
        assert_eq!(get_ost_manifest_source(&temp_dir), "steamrun");

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[tokio::test]
    async fn test_download_manifest_real_cf() {
        let temp_dir = std::env::temp_dir().join("test_steam_manifest");
        let _ = fs::create_dir_all(&temp_dir);

        // Test downloading Slay the Spire 2 (2868840)
        let res = download_and_install_manifest_by_appid(&temp_dir, 2868840).await;
        assert!(res.is_ok(), "Failed: {:?}", res);
        let item = res.unwrap();
        assert_eq!(item.appid, 2868840);
        assert!(temp_dir.join("config").join("lua").join("2868840.lua").exists() || temp_dir.join("config").join("stplug-in").join("2868840.lua").exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[tokio::test]
    async fn test_manifest_fallback_store_api() {
        let temp_dir = std::env::temp_dir().join("test_steam_manifest_fallback");
        let _ = fs::create_dir_all(&temp_dir);

        // Test with Portal (400)
        let res = download_and_install_manifest_by_appid(&temp_dir, 400).await;
        assert!(res.is_ok(), "Failed: {:?}", res);
        let item = res.unwrap();
        assert_eq!(item.appid, 400);
        assert!(temp_dir.join("config").join("lua").join("400.lua").exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }
}

