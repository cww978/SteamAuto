use std::path::{Path, PathBuf};
use std::process::Command;
use crate::steam::account::{self, SteamAccount};
use crate::steam::detector::{self, SteamInfo};
use crate::steam::parser::{self, GameItem, ZipImportResult};
use crate::steam::process::{self, SteamProcessStatus};
use crate::steam::store_api::{self, SteamStoreDetails};
use crate::steam::unlocker;

fn resolve_steam_path(custom_path: Option<String>) -> PathBuf {
    if let Some(p) = custom_path {
        if !p.trim().is_empty() {
            return PathBuf::from(p);
        }
    }
    detector::get_steam_path().unwrap_or_else(|| PathBuf::from("C:\\Program Files (x86)\\Steam"))
}

#[tauri::command]
pub fn get_steam_info(custom_path: Option<String>) -> SteamInfo {
    detector::inspect_steam(custom_path.as_deref())
}

#[tauri::command]
pub fn get_steam_process_status() -> SteamProcessStatus {
    process::check_steam_status()
}

#[tauri::command]
pub fn toggle_unlock_mode(enable: bool, custom_path: Option<String>) -> Result<String, String> {
    let path = resolve_steam_path(custom_path);
    if enable {
        unlocker::enable_unlock_mode(&path)
    } else {
        unlocker::disable_unlock_mode(&path)
    }
}

#[tauri::command]
pub fn repair_unlock_hook(custom_path: Option<String>) -> Result<String, String> {
    let path = resolve_steam_path(custom_path);
    unlocker::repair_unlock_dll(&path)
}

#[tauri::command]
pub fn get_installed_games(custom_path: Option<String>) -> Result<Vec<GameItem>, String> {
    let path = resolve_steam_path(custom_path);
    parser::get_installed_games(&path)
}

#[tauri::command]
pub fn import_zip_file(zip_path: String, custom_path: Option<String>) -> Result<ZipImportResult, String> {
    let steam_path = resolve_steam_path(custom_path);
    parser::import_file_or_package(Path::new(&zip_path), &steam_path)
}

#[tauri::command]
pub fn save_game_lua(appid: u32, content: String, custom_path: Option<String>) -> Result<String, String> {
    let steam_path = resolve_steam_path(custom_path);
    parser::save_game_lua(&steam_path, appid, &content)
}

#[tauri::command]
pub fn remove_game(appid: u32, delete_manifests: bool, custom_path: Option<String>) -> Result<String, String> {
    let steam_path = resolve_steam_path(custom_path);
    parser::delete_game(&steam_path, appid, delete_manifests)
}

#[tauri::command]
pub async fn fetch_game_from_store_or_url(input: String) -> Result<SteamStoreDetails, String> {
    let appid = store_api::extract_appid_from_input(&input)
        .ok_or("无法从输入中识别有效的 Steam AppID 或商店链接")?;
    store_api::fetch_steam_app_details(appid).await
}

#[tauri::command]
pub fn get_accounts(custom_path: Option<String>) -> Result<Vec<SteamAccount>, String> {
    let steam_path = resolve_steam_path(custom_path);
    account::get_steam_accounts(&steam_path)
}

#[tauri::command]
pub fn switch_account(
    steam_id: String,
    account_name: String,
    auto_restart: bool,
    custom_path: Option<String>,
) -> Result<String, String> {
    let steam_path = resolve_steam_path(custom_path);
    let msg = account::switch_steam_account(&steam_path, &steam_id, &account_name)?;
    if auto_restart {
        let _ = process::restart_steam(&steam_path);
    }
    Ok(msg)
}

#[tauri::command]
pub fn logout_account(custom_path: Option<String>) -> Result<String, String> {
    let steam_path = resolve_steam_path(custom_path);
    account::clear_auto_login(&steam_path)
}

#[tauri::command]
pub fn restart_steam_client(custom_path: Option<String>) -> Result<String, String> {
    let steam_path = resolve_steam_path(custom_path);
    process::restart_steam(&steam_path)
}

#[tauri::command]
pub fn kill_steam_client() -> Result<String, String> {
    process::kill_steam()?;
    Ok("Steam 进程已停止".to_string())
}

#[tauri::command]
pub fn start_steam_client(custom_path: Option<String>) -> Result<String, String> {
    let steam_path = resolve_steam_path(custom_path);
    process::start_steam(&steam_path)?;
    Ok("Steam 客户端已启动".to_string())
}

#[tauri::command]
pub fn launch_game_by_id(appid: u32) -> Result<String, String> {
    process::launch_steam_game(appid)?;
    Ok(format!("已向 Steam 发送启动游戏指令: {}", appid))
}

#[tauri::command]
pub fn open_folder_in_explorer(folder_path: String) -> Result<(), String> {
    let p = Path::new(&folder_path);
    if !p.exists() {
        let _ = std::fs::create_dir_all(p);
    }
    Command::new("explorer")
        .arg(&folder_path)
        .spawn()
        .map_err(|e| format!("打开文件夹失败: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn clean_all_depot_cache(custom_path: Option<String>) -> Result<String, String> {
    let steam_path = resolve_steam_path(custom_path);
    let depotcache = steam_path.join("depotcache");
    if !depotcache.exists() {
        return Ok("清单缓存目录为空".to_string());
    }

    let mut count = 0;
    if let Ok(entries) = std::fs::read_dir(&depotcache) {
        for entry in entries.flatten() {
            if entry.path().is_file() {
                if let Ok(_) = std::fs::remove_file(entry.path()) {
                    count += 1;
                }
            }
        }
    }

    Ok(format!("已成功清理 {} 个清单缓存文件", count))
}
