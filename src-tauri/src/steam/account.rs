use std::fs;
use std::path::Path;
use regex::Regex;
use serde::{Deserialize, Serialize};
use winreg::enums::*;
use winreg::RegKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamAccount {
    pub steam_id64: String,
    pub account_name: String,
    pub persona_name: String,
    pub remember_password: bool,
    pub auto_login: bool,
    pub timestamp: u64,
    pub last_login_formatted: String,
    pub is_active: bool,
    pub avatar_url: Option<String>,
}

pub fn get_active_registry_user() -> Option<String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(steam_key) = hkcu.open_subkey("Software\\Valve\\Steam") {
        if let Ok(val) = steam_key.get_value::<String, _>("AutoLoginUser") {
            if !val.trim().is_empty() {
                return Some(val.trim().to_string());
            }
        }
    }
    None
}

pub fn parse_loginusers_vdf(vdf_content: &str, active_reg_user: Option<&str>) -> Vec<SteamAccount> {
    let mut accounts = Vec::new();

    // Regex to match each user block: "76561198xxxx" { ... }
    let re_user_block = Regex::new(r#""(\d{15,20})"\s*\{([^}]+)\}"#).unwrap();
    let re_account_name = Regex::new(r#""AccountName"\s*"([^"]*)""#).unwrap();
    let re_persona_name = Regex::new(r#""PersonaName"\s*"([^"]*)""#).unwrap();
    let re_remember = Regex::new(r#""RememberPassword"\s*"([^"]*)""#).unwrap();
    let re_autologin = Regex::new(r#""AutoLogin"\s*"([^"]*)""#).unwrap();
    let re_timestamp = Regex::new(r#""Timestamp"\s*"([^"]*)""#).unwrap();

    for caps in re_user_block.captures_iter(vdf_content) {
        let steam_id64 = caps[1].to_string();
        let body = &caps[2];

        let account_name = re_account_name
            .captures(body)
            .map(|c| c[1].to_string())
            .unwrap_or_default();

        let persona_name = re_persona_name
            .captures(body)
            .map(|c| c[1].to_string())
            .unwrap_or_else(|| account_name.clone());

        let remember_password = re_remember
            .captures(body)
            .map(|c| c[1].to_string() == "1")
            .unwrap_or(false);

        let auto_login = re_autologin
            .captures(body)
            .map(|c| c[1].to_string() == "1")
            .unwrap_or(false);

        let timestamp = re_timestamp
            .captures(body)
            .and_then(|c| c[1].parse::<u64>().ok())
            .unwrap_or(0);

        let last_login_formatted = if timestamp > 0 {
            let dt = chrono::DateTime::from_timestamp(timestamp as i64, 0);
            dt.map(|d| {
                let local: chrono::DateTime<chrono::Local> = d.into();
                local.format("%Y-%m-%d %H:%M").to_string()
            })
            .unwrap_or_else(|| "-".to_string())
        } else {
            "-".to_string()
        };

        let is_active = match active_reg_user {
            Some(reg_u) => reg_u.eq_ignore_ascii_case(&account_name) || auto_login,
            None => auto_login,
        };

        accounts.push(SteamAccount {
            steam_id64,
            account_name,
            persona_name,
            remember_password,
            auto_login,
            timestamp,
            last_login_formatted,
            is_active,
            avatar_url: None,
        });
    }

    // Sort by timestamp desc (most recently logged in first)
    accounts.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    accounts
}

pub fn get_steam_accounts(steam_path: &Path) -> Result<Vec<SteamAccount>, String> {
    let loginusers_path = steam_path.join("config").join("loginusers.vdf");
    if !loginusers_path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&loginusers_path)
        .map_err(|e| format!("读取 loginusers.vdf 失败: {}", e))?;

    let active_reg_user = get_active_registry_user();
    let accounts = parse_loginusers_vdf(&content, active_reg_user.as_deref());

    Ok(accounts)
}

pub fn switch_steam_account(
    steam_path: &Path,
    target_steam_id: &str,
    target_account_name: &str,
) -> Result<String, String> {
    let loginusers_path = steam_path.join("config").join("loginusers.vdf");
    if !loginusers_path.exists() {
        return Err("未找到 Steam 账号配置文件 loginusers.vdf".to_string());
    }

    let content = fs::read_to_string(&loginusers_path)
        .map_err(|e| format!("读取 loginusers.vdf 失败: {}", e))?;

    // Regex match each user block and replace AutoLogin
    let re_user_block = Regex::new(r#""(\d{15,20})"\s*\{([^}]+)\}"#).unwrap();
    let re_autologin_field = Regex::new(r#""AutoLogin"\s*"[^"]*""#).unwrap();

    let new_content = re_user_block.replace_all(&content, |caps: &regex::Captures| {
        let steam_id = &caps[1];
        let body = &caps[2];

        let target_autologin = if steam_id == target_steam_id {
            r#""AutoLogin"		"1""#
        } else {
            r#""AutoLogin"		"0""#
        };

        let new_body = if re_autologin_field.is_match(body) {
            re_autologin_field.replace(body, target_autologin).to_string()
        } else {
            format!("{}\n\t\t{}", body.trim_end(), target_autologin)
        };

        format!("\"{}\"\n\t{{\n{}\n\t}}", steam_id, new_body.trim())
    });

    fs::write(&loginusers_path, new_content.as_ref())
        .map_err(|e| format!("更新 loginusers.vdf 失败: {}", e))?;

    // Update Registry AutoLoginUser
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok((steam_key, _)) = hkcu.create_subkey("Software\\Valve\\Steam") {
        let _ = steam_key.set_value("AutoLoginUser", &target_account_name);
        let _ = steam_key.set_value("RememberPassword", &1u32);
    }

    Ok(format!("成功切换至账号: {}", target_account_name))
}

pub fn clear_auto_login(steam_path: &Path) -> Result<String, String> {
    let loginusers_path = steam_path.join("config").join("loginusers.vdf");
    if loginusers_path.exists() {
        if let Ok(content) = fs::read_to_string(&loginusers_path) {
            let re_autologin_field = Regex::new(r#""AutoLogin"\s*"[^"]*""#).unwrap();
            let new_content = re_autologin_field.replace_all(&content, r#""AutoLogin"		"0""#);
            let _ = fs::write(&loginusers_path, new_content.as_ref());
        }
    }

    // Clear registry AutoLoginUser
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(steam_key) = hkcu.open_subkey_with_flags("Software\\Valve\\Steam", KEY_SET_VALUE) {
        let _ = steam_key.set_value("AutoLoginUser", &"");
    }

    Ok("已退出当前账号自动登录".to_string())
}
