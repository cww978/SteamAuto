pub mod steam;
pub mod commands;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            get_steam_info,
            get_steam_process_status,
            toggle_unlock_mode,
            repair_unlock_hook,
            get_installed_games,
            import_zip_file,
            save_game_lua,
            remove_game,
            fetch_game_from_store_or_url,
            get_accounts,
            switch_account,
            logout_account,
            restart_steam_client,
            kill_steam_client,
            start_steam_client,
            launch_game_by_id,
            open_folder_in_explorer,
            clean_all_depot_cache,
        ])
        .run(tauri::generate_context!())
        .expect("error while running steam-auto tauri application");
}
