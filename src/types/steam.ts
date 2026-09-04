export interface SteamInfo {
  steam_path: string;
  steam_exe: string;
  stplugin_path: string;
  depotcache_path: string;
  is_installed: bool;
  is_unlock_active: bool;
  total_unlocked_games: number;
}

export type bool = boolean;

export interface SteamProcessStatus {
  is_running: boolean;
  pid: number | null;
  memory_mb: number | null;
}

export interface GameItem {
  appid: number;
  name: string;
  lua_path: string;
  lua_content: string;
  dlcs: number[];
  manifest_count: number;
  manifest_files: string[];
  file_size_bytes: number;
  updated_at: string;
}

export interface ZipImportResult {
  appid: number;
  name: string;
  lua_filename: string;
  extracted_manifests: string[];
  total_manifests: number;
  message: string;
}

export interface SteamStoreDetails {
  appid: number;
  name: string;
  short_description: string;
  header_image: string;
  capsule_image: string;
  background: string;
  developers: string[];
  publishers: string[];
  dlcs: number[];
  genres: string[];
  release_date: string;
  generated_lua: string;
  is_official?: boolean;
}

export interface SteamSearchResultItem {
  id: number;
  name: string;
  header_image: string;
  price?: string | null;
}

export interface SteamAccount {
  steam_id64: string;
  account_name: string;
  persona_name: string;
  remember_password: boolean;
  auto_login: boolean;
  timestamp: number;
  last_login_formatted: string;
  is_active: boolean;
  avatar_url: string | null;
}

export type ActiveTab = 'library' | 'add' | 'accounts' | 'settings';

export interface ToastMessage {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  title: string;
  description?: string;
  duration?: number;
}

