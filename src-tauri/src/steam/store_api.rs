use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamStoreDetails {
    pub appid: u32,
    pub name: String,
    pub short_description: String,
    pub header_image: String,
    pub capsule_image: String,
    pub background: String,
    pub developers: Vec<String>,
    pub publishers: Vec<String>,
    pub dlcs: Vec<u32>,
    pub genres: Vec<String>,
    pub release_date: String,
    pub generated_lua: String,
    pub is_official: bool,
}

pub fn extract_appid_from_input(input: &str) -> Option<u32> {
    let trimmed = input.trim();
    if let Ok(id) = trimmed.parse::<u32>() {
        return Some(id);
    }

    // Match steam store url: https://store.steampowered.com/app/1245620/...
    let re = Regex::new(r"store\.steampowered\.com/app/(\d+)").unwrap();
    if let Some(caps) = re.captures(trimmed) {
        if let Ok(id) = caps[1].parse::<u32>() {
            return Some(id);
        }
    }

    // Match steam://rungameid/1245620
    let re_run = Regex::new(r"steam://(?:rungameid|app|launch)/(\d+)").unwrap();
    if let Some(caps) = re_run.captures(trimmed) {
        if let Ok(id) = caps[1].parse::<u32>() {
            return Some(id);
        }
    }

    // Match general digits in string
    let re_digits = Regex::new(r"\b(\d{4,8})\b").unwrap();
    if let Some(caps) = re_digits.captures(trimmed) {
        if let Ok(id) = caps[1].parse::<u32>() {
            return Some(id);
        }
    }

    None
}

pub async fn fetch_steam_app_details(appid: u32) -> Result<SteamStoreDetails, String> {
    let endpoints = [
        format!("https://store.steampowered.com/api/appdetails?appids={}&cc=cn&l=schinese", appid),
        format!("https://store.steampowered.com/api/appdetails?appids={}&l=schinese", appid),
        format!("https://store.steampowered.com/api/appdetails?appids={}", appid),
    ];

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::from_secs(6))
        .build()
        .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

    let mut body_json: Option<serde_json::Value> = None;

    for url in &endpoints {
        if let Ok(resp) = client.get(url).send().await {
            if resp.status().is_success() {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    let app_key = appid.to_string();
                    if json.get(&app_key).and_then(|d| d.get("success")).and_then(|s| s.as_bool()).unwrap_or(false) {
                        body_json = Some(json);
                        break;
                    }
                }
            }
        }
    }

    // If online fetching succeeded and returned data
    if let Some(json) = body_json {
        let app_key = appid.to_string();
        if let Some(app_data) = json.get(&app_key) {
            if let Some(data) = app_data.get("data") {
                let name = data.get("name").and_then(|n| n.as_str()).unwrap_or("未知游戏").to_string();
                let short_desc = data.get("short_description").and_then(|s| s.as_str()).unwrap_or("").to_string();
                let header_image = data.get("header_image").and_then(|h| h.as_str()).unwrap_or("").to_string();
                let capsule_image = data.get("capsule_image").and_then(|c| c.as_str()).unwrap_or("").to_string();
                let background = data.get("background").and_then(|b| b.as_str()).unwrap_or("").to_string();

                let developers: Vec<String> = data
                    .get("developers")
                    .and_then(|d| d.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_default();

                let publishers: Vec<String> = data
                    .get("publishers")
                    .and_then(|p| p.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_default();

                let dlcs: Vec<u32> = data
                    .get("dlc")
                    .and_then(|d| d.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_u64().map(|id| id as u32)).collect())
                    .unwrap_or_default();

                let genres: Vec<String> = data
                    .get("genres")
                    .and_then(|g| g.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.get("description").and_then(|d| d.as_str()).map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();

                let release_date = data
                    .get("release_date")
                    .and_then(|r| r.get("date"))
                    .and_then(|d| d.as_str())
                    .unwrap_or("未知")
                    .to_string();

                // Generate standard Lua script
                let mut lua_lines = Vec::new();
                lua_lines.push(format!("-- Name: {}", name));
                lua_lines.push(format!("-- AppID: {}", appid));
                if !developers.is_empty() {
                    lua_lines.push(format!("-- Developer: {}", developers.join(", ")));
                }
                lua_lines.push(String::new());
                lua_lines.push(format!("addappid({})", appid));

                if !dlcs.is_empty() {
                    lua_lines.push(String::new());
                    lua_lines.push(format!("-- DLCs ({})", dlcs.len()));
                    for dlc_id in &dlcs {
                        lua_lines.push(format!("addappid({})", dlc_id));
                    }
                }

                let generated_lua = lua_lines.join("\n");

                return Ok(SteamStoreDetails {
                    appid,
                    name,
                    short_description: short_desc,
                    header_image,
                    capsule_image,
                    background,
                    developers,
                    publishers,
                    dlcs,
                    genres,
                    release_date,
                    generated_lua,
                    is_official: true,
                });
            }
        }
    }

    // Graceful Fallback: If network is blocked by GFW / ISP or game has no public page
    let name = format!("Steam 游戏 (AppID: {})", appid);
    let header = format!("https://cdn.akamai.steamstatic.com/steam/apps/{}/header.jpg", appid);
    let capsule = format!("https://cdn.akamai.steamstatic.com/steam/apps/{}/capsule_616x353.jpg", appid);
    let generated_lua = format!("-- Name: {}\n-- AppID: {}\n\naddappid({})\n", name, appid, appid);

    Ok(SteamStoreDetails {
        appid,
        name,
        short_description: "由于当前网络无法直连 Steam 商店接口（或游戏需登录访问），已自动为您离线解析该 AppID 并生成基础入库脚本。可直接点击下方一键入库并部署。".to_string(),
        header_image: header,
        capsule_image: capsule,
        background: String::new(),
        developers: Vec::new(),
        publishers: Vec::new(),
        dlcs: Vec::new(),
        genres: Vec::new(),
        release_date: "未知".to_string(),
        generated_lua,
        is_official: false,
    })
}

pub fn load_game_cache(steam_path: &Path) -> HashMap<u32, SteamStoreDetails> {
    let cache_file = steam_path.join("config").join("game_cache.json");
    if cache_file.exists() {
        if let Ok(content) = std::fs::read_to_string(&cache_file) {
            if let Ok(map) = serde_json::from_str::<HashMap<u32, SteamStoreDetails>>(&content) {
                return map;
            }
        }
    }
    HashMap::new()
}

pub fn save_game_cache(steam_path: &Path, cache: &HashMap<u32, SteamStoreDetails>) {
    let config_dir = steam_path.join("config");
    let _ = std::fs::create_dir_all(&config_dir);
    let cache_file = config_dir.join("game_cache.json");
    if let Ok(json) = serde_json::to_string_pretty(cache) {
        let _ = std::fs::write(&cache_file, json);
    }
}

pub fn get_all_cached_game_details(steam_path: &Path) -> HashMap<u32, SteamStoreDetails> {
    load_game_cache(steam_path)
}

pub async fn get_or_fetch_game_details(
    appid: u32,
    steam_path: &Path,
    force_refresh: bool,
) -> Result<SteamStoreDetails, String> {
    if !force_refresh {
        let cache = load_game_cache(steam_path);
        if let Some(cached) = cache.get(&appid) {
            if cached.is_official {
                return Ok(cached.clone());
            }
        }
    }

    let details = fetch_steam_app_details(appid).await?;
    if details.is_official {
        let mut cache = load_game_cache(steam_path);
        cache.insert(appid, details.clone());
        save_game_cache(steam_path, &cache);
    }

    Ok(details)
}

pub async fn crawl_steam_cover(appid: u32) -> Result<String, String> {
    let url = format!("https://store.steampowered.com/api/appdetails?appids={}&filters=basic", appid);
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    let header = json[&appid.to_string()]["data"]["header_image"]
        .as_str()
        .ok_or_else(|| "未找到官方封面".to_string())?;

    Ok(header.to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamSearchResultItem {
    pub id: u32,
    pub name: String,
    pub header_image: String,
    pub price: Option<String>,
}

pub async fn search_steam_store(query: &str) -> Result<Vec<SteamSearchResultItem>, String> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(6))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client
        .get("https://store.steampowered.com/api/storesearch/")
        .query(&[("term", trimmed), ("l", "schinese"), ("cc", "cn")])
        .send()
        .await
        .map_err(|e| format!("搜索 Steam 游戏失败: {}", e))?;

    let json: serde_json::Value = resp.json().await.map_err(|e| format!("解析搜索结果失败: {}", e))?;

    let mut results = Vec::new();
    if let Some(items) = json.get("items").and_then(|v| v.as_array()) {
        for item in items {
            if let Some(id) = item.get("id").and_then(|v| v.as_u64()).map(|v| v as u32) {
                let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("未知游戏").to_string();
                let header_image = format!("https://cdn.akamai.steamstatic.com/steam/apps/{}/header.jpg", id);
                let price = item.get("price").and_then(|p| p.get("final")).and_then(|f| f.as_i64()).map(|cents| {
                    if cents == 0 {
                        "免费开玩".to_string()
                    } else {
                        format!("¥{:.2}", (cents as f64) / 100.0)
                    }
                });

                results.push(SteamSearchResultItem {
                    id,
                    name,
                    header_image,
                    price,
                });
            }
        }
    }

    // If searching a direct numeric AppID and storesearch didn't return it
    if results.is_empty() {
        if let Ok(id) = trimmed.parse::<u32>() {
            if let Ok(details) = fetch_steam_app_details(id).await {
                results.push(SteamSearchResultItem {
                    id,
                    name: details.name,
                    header_image: details.header_image,
                    price: None,
                });
            }
        }
    }

    Ok(results)
}

pub async fn get_featured_popular_games() -> Result<Vec<SteamSearchResultItem>, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(6))
        .build()
        .map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    let mut seen_ids = std::collections::HashSet::new();

    if let Ok(resp) = client
        .get("https://store.steampowered.com/api/featuredcategories?cc=cn&l=schinese")
        .send()
        .await
    {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            // Collect from top_sellers
            if let Some(items) = json.get("top_sellers").and_then(|c| c.get("items")).and_then(|i| i.as_array()) {
                for item in items {
                    if let Some(id) = item.get("id").and_then(|v| v.as_u64()).map(|v| v as u32) {
                        if seen_ids.insert(id) {
                            let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("未知游戏").to_string();
                            let header = item.get("header_image").and_then(|v| v.as_str()).map(|s| s.to_string())
                                .unwrap_or_else(|| format!("https://cdn.akamai.steamstatic.com/steam/apps/{}/header.jpg", id));
                            let price = item.get("final_price").and_then(|f| f.as_i64()).map(|cents| {
                                if cents == 0 { "免费开玩".to_string() } else { format!("¥{:.2}", (cents as f64) / 100.0) }
                            });
                            results.push(SteamSearchResultItem { id, name, header_image: header, price });
                        }
                    }
                }
            }
            // Also collect from specials
            if let Some(items) = json.get("specials").and_then(|c| c.get("items")).and_then(|i| i.as_array()) {
                for item in items {
                    if let Some(id) = item.get("id").and_then(|v| v.as_u64()).map(|v| v as u32) {
                        if seen_ids.insert(id) {
                            let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("未知游戏").to_string();
                            let header = item.get("header_image").and_then(|v| v.as_str()).map(|s| s.to_string())
                                .unwrap_or_else(|| format!("https://cdn.akamai.steamstatic.com/steam/apps/{}/header.jpg", id));
                            let price = item.get("final_price").and_then(|f| f.as_i64()).map(|cents| {
                                if cents == 0 { "免费开玩".to_string() } else { format!("¥{:.2}", (cents as f64) / 100.0) }
                            });
                            results.push(SteamSearchResultItem { id, name, header_image: header, price });
                        }
                    }
                }
            }
        }
    }

    // If online list is empty or sparse, supplement with curated top hits
    if results.len() < 8 {
        let curated = [
            (2358720, "黑神话：悟空"),
            (1245620, "艾尔登法环"),
            (1091500, "赛博朋克 2077"),
            (2868840, "杀戮尖塔 2"),
            (1623730, "幻兽帕鲁"),
            (2246340, "怪物猎人：荒野"),
            (730, "反恐精英 2"),
            (1086940, "博德之门 3"),
            (271590, "侠盗猎车手 5"),
            (1174180, "荒野大镖客：救赎 2"),
            (413150, "星露谷物语"),
            (892970, "英灵神殿"),
        ];

        for (id, name) in curated {
            if seen_ids.insert(id) {
                results.push(SteamSearchResultItem {
                    id,
                    name: name.to_string(),
                    header_image: format!("https://cdn.akamai.steamstatic.com/steam/apps/{}/header.jpg", id),
                    price: None,
                });
            }
        }
    }

    Ok(results)
}

