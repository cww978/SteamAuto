use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameItem {
    pub appid: u32,
    pub name: String,
    pub lua_path: String,
    pub lua_content: String,
    pub dlcs: Vec<u32>,
    pub manifest_count: usize,
    pub manifest_files: Vec<String>,
    pub file_size_bytes: u64,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZipImportResult {
    pub appid: u32,
    pub name: String,
    pub lua_filename: String,
    pub extracted_manifests: Vec<String>,
    pub total_manifests: usize,
    pub message: String,
}

pub fn parse_lua_details(content: &str, default_appid_hint: Option<u32>) -> (u32, String, Vec<u32>, Vec<String>) {
    let mut detected_appid = default_appid_hint.unwrap_or(0);
    let mut detected_name = String::new();
    let mut dlcs = Vec::new();
    let mut manifest_depots = Vec::new();

    // Regex for explicit name tags: --Gamename ..., -- Name: ..., -- Title: ...
    let re_name_tag = Regex::new(r"(?i)^--\s*(?:Game\s*name|Gamename|Game|Name|Title)\s*[:=]?\s*(.+)").unwrap();
    let re_appid_tag = Regex::new(r"(?i)^--\s*AppID:?\s*(\d+)").unwrap();
    let re_inline_comment = Regex::new(r#"(?i)addappid\s*\(\s*(\d+)[^)]*\)\s*--(?:Main\s*appid|Mainappid)?\s*(.+)"#).unwrap();

    for line in content.lines() {
        // Strip zero-width unicode characters
        let clean_line: String = line.chars().filter(|&c| {
            c != '\u{200B}' && c != '\u{200C}' && c != '\u{200D}' && c != '\u{FEFF}' && c != '\u{2060}'
        }).collect();
        let trimmed = clean_line.trim();

        if trimmed.starts_with("--") {
            if let Some(caps) = re_name_tag.captures(trimmed) {
                let candidate = caps[1].trim();
                if detected_name.is_empty() && candidate.chars().any(|c| c.is_alphanumeric()) {
                    detected_name = candidate.to_string();
                }
            } else if let Some(caps) = re_appid_tag.captures(trimmed) {
                if detected_appid == 0 {
                    if let Ok(id) = caps[1].parse::<u32>() {
                        detected_appid = id;
                    }
                }
            } else if detected_name.is_empty() {
                // Check if this is a general title comment, e.g. "-- Slay the Spire 2"
                let comment_text = trimmed.trim_start_matches(|c| c == '-' || c == '[' || c == ']').trim();
                let lower = comment_text.to_lowercase();
                let has_alphanumeric = comment_text.chars().any(|c| c.is_alphanumeric());
                let is_noise = lower.starts_with("appid")
                    || lower.starts_with("generated")
                    || lower.starts_with("note")
                    || lower.starts_with("main application")
                    || lower.starts_with("content depot")
                    || lower.starts_with("descargado")
                    || lower.starts_with("generador")
                    || lower.starts_with("discord")
                    || lower.starts_with("fecha")
                    || lower.starts_with("http")
                    || lower.starts_with("depot")
                    || lower.starts_with("addappid")
                    || lower.starts_with("setmanifest");

                if has_alphanumeric && !is_noise {
                    detected_name = comment_text.to_string();
                }
            }
        }
    }

    // Fallback: check inline comments on addappid calls if detected_name is still empty
    if detected_name.is_empty() {
        for cap in re_inline_comment.captures_iter(content) {
            let candidate = cap[2].trim();
            if candidate.chars().any(|c| c.is_alphanumeric()) {
                detected_name = candidate.to_string();
                break;
            }
        }
    }

    // Regex for addappid: addappid(123456) or addappid(123456, 0, "...")
    let re_addappid = Regex::new(r#"addappid\s*\(\s*(\d+)(?:\s*,\s*[^,)]+)?(?:\s*,\s*"[^"]*")?\s*\)"#).unwrap();
    for cap in re_addappid.captures_iter(content) {
        if let Ok(id) = cap[1].parse::<u32>() {
            if detected_appid == 0 {
                detected_appid = id;
            } else if id != detected_appid && !dlcs.contains(&id) {
                dlcs.push(id);
            }
        }
    }

    // Regex for setManifestid: setManifestid(123456, "...") or setManifestid(123456, 123456789)
    let re_manifest = Regex::new(r#"setManifestid\s*\(\s*(\d+)\s*,\s*["']?(\d+)["']?"#).unwrap();
    for cap in re_manifest.captures_iter(content) {
        let depot = &cap[1];
        let manifest = &cap[2];
        let manifest_file = format!("{}_{}.manifest", depot, manifest);
        manifest_depots.push(manifest_file);
    }

    if detected_name.is_empty() {
        if detected_appid != 0 {
            detected_name = format!("Steam App {}", detected_appid);
        } else {
            detected_name = "未知游戏".to_string();
        }
    }

    (detected_appid, detected_name, dlcs, manifest_depots)
}

pub fn get_installed_games(steam_path: &Path) -> Result<Vec<GameItem>, String> {
    let lua_dir = steam_path.join("config").join("lua");
    let stplugin_dir = steam_path.join("config").join("stplug-in");
    let depotcache_dir = steam_path.join("depotcache");

    let mut games_map = std::collections::HashMap::<u32, GameItem>::new();
    let official_cache = crate::steam::store_api::load_game_cache(steam_path);

    for check_dir in [&lua_dir, &stplugin_dir] {
        if !check_dir.exists() {
            continue;
        }

        if let Ok(entries) = fs::read_dir(check_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext.to_string_lossy().to_lowercase() == "lua" {
                            let file_stem = path.file_stem().unwrap_or_default().to_string_lossy();
                            let hint_appid = file_stem.parse::<u32>().ok();

                            if let Ok(content) = fs::read_to_string(&path) {
                                let (appid, name, dlcs, expected_manifests) = parse_lua_details(&content, hint_appid);
                                let final_appid = if appid != 0 { appid } else { hint_appid.unwrap_or(0) };

                                if final_appid == 0 {
                                    continue;
                                }

                                // Check manifest files
                                let mut found_manifests = Vec::new();
                                for m in &expected_manifests {
                                    if depotcache_dir.join(m).exists() {
                                        found_manifests.push(m.clone());
                                    }
                                }

                                // Also search for any manifest in depotcache starting with {final_appid}_
                                if let Ok(depot_entries) = fs::read_dir(&depotcache_dir) {
                                    for de in depot_entries.flatten() {
                                        let fname = de.file_name().to_string_lossy().to_string();
                                        if fname.starts_with(&format!("{}_", final_appid)) && !found_manifests.contains(&fname) {
                                            found_manifests.push(fname);
                                        }
                                    }
                                }

                                let metadata = entry.metadata().ok();
                                let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
                                let updated_at = metadata
                                    .and_then(|m| m.modified().ok())
                                    .map(|time| {
                                        let dt: chrono::DateTime<chrono::Local> = time.into();
                                        dt.format("%Y-%m-%d %H:%M").to_string()
                                    })
                                    .unwrap_or_else(|| "-".to_string());

                                let official_name = if let Some(cached) = official_cache.get(&final_appid) {
                                    if !cached.name.is_empty() && cached.is_official {
                                        cached.name.clone()
                                    } else if !name.is_empty() {
                                        name
                                    } else {
                                        format!("Steam 游戏 (AppID: {})", final_appid)
                                    }
                                } else if !name.is_empty() {
                                    name
                                } else {
                                    format!("Steam 游戏 (AppID: {})", final_appid)
                                };

                                let item = GameItem {
                                    appid: final_appid,
                                    name: official_name,
                                    lua_path: path.to_string_lossy().to_string(),
                                    lua_content: content,
                                    dlcs,
                                    manifest_count: found_manifests.len(),
                                    manifest_files: found_manifests,
                                    file_size_bytes: size,
                                    updated_at,
                                };

                                // Insert or upgrade if has more manifests/content
                                if let Some(existing) = games_map.get(&final_appid) {
                                    if item.manifest_count > existing.manifest_count || item.lua_content.len() > existing.lua_content.len() {
                                        games_map.insert(final_appid, item);
                                    }
                                } else {
                                    games_map.insert(final_appid, item);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut games: Vec<GameItem> = games_map.into_values().collect();
    games.sort_by(|a, b| b.appid.cmp(&a.appid));
    Ok(games)
}

pub fn import_zip_package(zip_path: &Path, steam_path: &Path) -> Result<ZipImportResult, String> {
    let zip_str = zip_path.to_string_lossy().trim().to_string();
    let zip_clean_path = Path::new(&zip_str);

    if !zip_clean_path.exists() {
        return Err(format!("压缩包文件不存在: {}", zip_clean_path.display()));
    }

    let file = File::open(zip_clean_path).map_err(|e| format!("打开压缩包失败: {}", e))?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("解析 ZIP 压缩包失败: {}", e))?;

    let lua_dir = steam_path.join("config").join("lua");
    let stplugin_dir = steam_path.join("config").join("stplug-in");
    let depotcache_dir = steam_path.join("depotcache");
    fs::create_dir_all(&lua_dir).map_err(|e| format!("创建 Lua 目录失败: {}", e))?;
    fs::create_dir_all(&stplugin_dir).map_err(|e| format!("创建插件目录失败: {}", e))?;
    fs::create_dir_all(&depotcache_dir).map_err(|e| format!("创建清单目录失败: {}", e))?;

    let mut main_appid = 0;
    let mut game_name = String::new();
    let mut lua_filename = String::new();
    let mut extracted_manifests = Vec::new();

    // First pass: extract lua files to BOTH config/lua and config/stplug-in
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| format!("读取压缩包条目失败: {}", e))?;
        if entry.is_dir() {
            continue;
        }

        let entry_name = entry.name().to_string();
        let fname = Path::new(&entry_name).file_name().unwrap_or_default().to_string_lossy().to_string();

        if fname.to_lowercase().ends_with(".lua") {
            let mut buffer = Vec::new();
            entry.read_to_end(&mut buffer).map_err(|e| format!("读取 Lua 文件内容失败: {}", e))?;
            let content = String::from_utf8_lossy(&buffer).to_string();

            let stem = Path::new(&fname).file_stem().unwrap_or_default().to_string_lossy().to_string();
            let hint_appid = stem.parse::<u32>().ok();
            let (parsed_appid, parsed_name, _, _) = parse_lua_details(&content, hint_appid);

            if main_appid == 0 {
                main_appid = if parsed_appid != 0 { parsed_appid } else { hint_appid.unwrap_or(0) };
                game_name = parsed_name;
                lua_filename = fname.clone();
            }

            // Write to config/lua
            let target_path_lua = lua_dir.join(&fname);
            let _ = fs::write(&target_path_lua, content.as_bytes());

            // Write to config/stplug-in
            let target_path_st = stplugin_dir.join(&fname);
            let _ = fs::write(&target_path_st, content.as_bytes());
        }
    }

    // Second pass: extract manifest files to Steam/depotcache/
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| format!("读取压缩包条目失败: {}", e))?;
        if entry.is_dir() {
            continue;
        }

        let entry_name = entry.name().to_string();
        let fname = Path::new(&entry_name).file_name().unwrap_or_default().to_string_lossy().to_string();

        if fname.to_lowercase().ends_with(".manifest") {
            let target_path = depotcache_dir.join(&fname);
            let mut out_file = File::create(&target_path).map_err(|e| format!("写入 Manifest 清单失败: {}", e))?;
            std::io::copy(&mut entry, &mut out_file).map_err(|e| format!("保存 Manifest 清单失败: {}", e))?;
            extracted_manifests.push(fname);
        }
    }

    if lua_filename.is_empty() {
        return Err("压缩包内未找到任何 .lua 脚本文件，非标准的 OpenSteamTool 下载清单包".to_string());
    }

    let count = extracted_manifests.len();
    Ok(ZipImportResult {
        appid: main_appid,
        name: game_name,
        lua_filename,
        extracted_manifests,
        total_manifests: count,
        message: format!("成功导入游戏 AppID: {}，已同步写入 Lua 脚本并部署 {} 个清单文件", main_appid, count),
    })
}

pub fn import_file_or_package(file_path: &Path, steam_path: &Path) -> Result<ZipImportResult, String> {
    let file_str = file_path.to_string_lossy().trim().to_string();
    let clean_path = Path::new(&file_str);

    if !clean_path.exists() {
        return Err(format!("文件不存在: {}", clean_path.display()));
    }

    let ext = clean_path.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();

    if ext == "lua" {
        // Direct Lua file import
        let content = fs::read_to_string(clean_path).map_err(|e| format!("读取 Lua 文件失败: {}", e))?;
        let fname = clean_path.file_name().unwrap_or_default().to_string_lossy().to_string();
        let stem = clean_path.file_stem().unwrap_or_default().to_string_lossy().to_string();
        let hint_appid = stem.parse::<u32>().ok();
        let (parsed_appid, parsed_name, _, expected_manifests) = parse_lua_details(&content, hint_appid);
        let final_appid = if parsed_appid != 0 { parsed_appid } else { hint_appid.unwrap_or(0) };

        if final_appid == 0 {
            return Err("无法从 Lua 文件内容或文件名中解析出有效的 AppID".to_string());
        }

        let lua_dir = steam_path.join("config").join("lua");
        let stplugin_dir = steam_path.join("config").join("stplug-in");
        let depotcache_dir = steam_path.join("depotcache");
        fs::create_dir_all(&lua_dir).map_err(|e| format!("创建 Lua 目录失败: {}", e))?;
        fs::create_dir_all(&stplugin_dir).map_err(|e| format!("创建插件目录失败: {}", e))?;
        fs::create_dir_all(&depotcache_dir).map_err(|e| format!("创建清单目录失败: {}", e))?;

        let target_fname = if fname.to_lowercase().ends_with(".lua") { fname.clone() } else { format!("{}.lua", final_appid) };
        let _ = fs::write(lua_dir.join(&target_fname), content.as_bytes());
        let _ = fs::write(stplugin_dir.join(&target_fname), content.as_bytes());

        // Check if any manifests exist in depotcache
        let mut found_manifests = Vec::new();
        for m in &expected_manifests {
            if depotcache_dir.join(m).exists() {
                found_manifests.push(m.clone());
            }
        }

        let count = found_manifests.len();
        Ok(ZipImportResult {
            appid: final_appid,
            name: parsed_name,
            lua_filename: target_fname,
            extracted_manifests: found_manifests,
            total_manifests: count,
            message: format!("成功导入 Lua 脚本: {} (AppID: {})，已同步部署到 config/lua 与 config/stplug-in", fname, final_appid),
        })
    } else if ext == "zip" {
        import_zip_package(clean_path, steam_path)
    } else {
        Err(format!("不支持的文件格式: .{}，请选择 .zip 清单压缩包或 .lua 脚本文件", ext))
    }
}

pub fn save_game_lua(steam_path: &Path, appid: u32, content: &str) -> Result<String, String> {
    let lua_dir = steam_path.join("config").join("lua");
    let stplugin_dir = steam_path.join("config").join("stplug-in");
    let _ = fs::create_dir_all(&lua_dir);
    let _ = fs::create_dir_all(&stplugin_dir);

    let lua_path1 = lua_dir.join(format!("{}.lua", appid));
    let lua_path2 = stplugin_dir.join(format!("{}.lua", appid));

    let _ = fs::write(&lua_path1, content);
    let _ = fs::write(&lua_path2, content);

    Ok(format!("游戏 {} 的 Lua 脚本已成功同步保存至 config/lua 与 config/stplug-in", appid))
}

pub fn delete_game(steam_path: &Path, appid: u32, delete_manifests: bool) -> Result<String, String> {
    let lua_dir = steam_path.join("config").join("lua");
    let stplugin_dir = steam_path.join("config").join("stplug-in");
    let depotcache_dir = steam_path.join("depotcache");

    let lua_path1 = lua_dir.join(format!("{}.lua", appid));
    let lua_path2 = stplugin_dir.join(format!("{}.lua", appid));
    let mut deleted_manifests_count = 0;
    let mut found_lua = false;

    if lua_path1.exists() {
        let _ = fs::remove_file(&lua_path1);
        found_lua = true;
    }
    if lua_path2.exists() {
        let _ = fs::remove_file(&lua_path2);
        found_lua = true;
    }

    if delete_manifests {
        // Also remove any manifest starting with appid_
        if let Ok(entries) = fs::read_dir(&depotcache_dir) {
            for entry in entries.flatten() {
                let fname = entry.file_name().to_string_lossy().to_string();
                if fname.starts_with(&format!("{}_", appid)) {
                    let _ = fs::remove_file(entry.path());
                    deleted_manifests_count += 1;
                }
            }
        }
    }

    if !found_lua {
        return Err(format!("未找到 AppID 为 {} 的 Lua 脚本", appid));
    }

    Ok(format!("已成功移除游戏 {} (并清理了 {} 个清单文件)", appid, deleted_manifests_count))
}
