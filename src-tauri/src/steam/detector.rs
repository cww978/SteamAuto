use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use winreg::enums::*;
use winreg::RegKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamInfo {
    pub steam_path: String,
    pub steam_exe: String,
    pub stplugin_path: String,
    pub depotcache_path: String,
    pub is_installed: bool,
    pub is_unlock_active: bool,
    pub total_unlocked_games: usize,
}

pub fn get_steam_path() -> Option<PathBuf> {
    // 1. Try HKCU\Software\Valve\Steam\SteamPath
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(steam_key) = hkcu.open_subkey("Software\\Valve\\Steam") {
        if let Ok(val) = steam_key.get_value::<String, _>("SteamPath") {
            let p = PathBuf::from(val.replace('/', "\\"));
            if p.join("steam.exe").exists() {
                return Some(p);
            }
        }
    }

    // 2. Try HKLM\SOFTWARE\WOW6432Node\Valve\Steam\InstallPath
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(steam_key) = hklm.open_subkey("SOFTWARE\\WOW6432Node\\Valve\\Steam") {
        if let Ok(val) = steam_key.get_value::<String, _>("InstallPath") {
            let p = PathBuf::from(val);
            if p.join("steam.exe").exists() {
                return Some(p);
            }
        }
    }

    // 3. Common fallback locations
    let common_paths = [
        "C:\\Program Files (x86)\\Steam",
        "C:\\Program Files\\Steam",
        "D:\\Steam",
        "E:\\Steam",
        "D:\\Program Files (x86)\\Steam",
        "D:\\Program Files\\Steam",
        "E:\\Program Files (x86)\\Steam",
        "E:\\Program Files\\Steam",
    ];

    for path_str in common_paths {
        let p = PathBuf::from(path_str);
        if p.join("steam.exe").exists() {
            return Some(p);
        }
    }

    None
}

pub fn ensure_directories(steam_path: &Path) -> std::io::Result<(PathBuf, PathBuf)> {
    let lua_dir = steam_path.join("config").join("lua");
    if !lua_dir.exists() {
        std::fs::create_dir_all(&lua_dir)?;
    }

    let stplugin_dir = steam_path.join("config").join("stplug-in");
    if !stplugin_dir.exists() {
        std::fs::create_dir_all(&stplugin_dir)?;
    }

    let depotcache_dir = steam_path.join("depotcache");
    if !depotcache_dir.exists() {
        std::fs::create_dir_all(&depotcache_dir)?;
    }

    Ok((lua_dir, depotcache_dir))
}

pub fn inspect_steam(custom_path: Option<&str>) -> SteamInfo {
    let steam_dir = custom_path
        .map(|p| PathBuf::from(p))
        .or_else(get_steam_path)
        .unwrap_or_else(|| PathBuf::from("C:\\Program Files (x86)\\Steam"));

    let steam_exe = steam_dir.join("steam.exe");
    let is_installed = steam_exe.exists();

    let lua_path = steam_dir.join("config").join("lua");
    let stplugin_path = steam_dir.join("config").join("stplug-in");
    let depotcache_path = steam_dir.join("depotcache");

    if is_installed {
        let _ = ensure_directories(&steam_dir);
    }

    let dll_active = steam_dir.join("OpenSteamTool.dll").exists() || steam_dir.join("xinput1_4.dll").exists() || steam_dir.join("dwmapi.dll").exists();

    let mut found_game_names = std::collections::HashSet::new();

    for check_dir in [&lua_path, &stplugin_path] {
        if check_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(check_dir) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.extension().map(|ext| ext.to_string_lossy().to_lowercase() == "lua").unwrap_or(false) {
                        if let Some(stem) = p.file_stem() {
                            found_game_names.insert(stem.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }

    SteamInfo {
        steam_path: steam_dir.to_string_lossy().to_string(),
        steam_exe: steam_exe.to_string_lossy().to_string(),
        stplugin_path: lua_path.to_string_lossy().to_string(),
        depotcache_path: depotcache_path.to_string_lossy().to_string(),
        is_installed,
        is_unlock_active: dll_active,
        total_unlocked_games: found_game_names.len(),
    }
}
