import { invoke } from '@tauri-apps/api/core';
import type {
  SteamInfo,
  SteamProcessStatus,
  GameItem,
  ZipImportResult,
  SteamStoreDetails,
  SteamSearchResultItem,
  SteamAccount,
} from '../types/steam';

export const isTauri = () => {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
};

// Steam Info & Status
export async function getSteamInfo(customPath?: string): Promise<SteamInfo> {
  return invoke<SteamInfo>('get_steam_info', { customPath: customPath || null });
}

export async function getSteamProcessStatus(): Promise<SteamProcessStatus> {
  return invoke<SteamProcessStatus>('get_steam_process_status');
}

// Unlock Mode Controls
export async function toggleUnlockMode(enable: boolean, customPath?: string): Promise<string> {
  return invoke<string>('toggle_unlock_mode', { enable, customPath: customPath || null });
}

export async function repairUnlockHook(customPath?: string): Promise<string> {
  return invoke<string>('repair_unlock_hook', { customPath: customPath || null });
}

// Games Management
export async function getInstalledGames(customPath?: string): Promise<GameItem[]> {
  return invoke<GameItem[]>('get_installed_games', { customPath: customPath || null });
}

export async function importZipFile(zipPath: string, customPath?: string): Promise<ZipImportResult> {
  return invoke<ZipImportResult>('import_zip_file', { zipPath, customPath: customPath || null });
}

export async function saveGameLua(appid: number, content: string, customPath?: string): Promise<string> {
  return invoke<string>('save_game_lua', { appid, content, customPath: customPath || null });
}

export async function removeGame(appid: number, deleteManifests: boolean, customPath?: string): Promise<string> {
  return invoke<string>('remove_game', { appid, deleteManifests, customPath: customPath || null });
}

export async function getGameCache(customPath?: string): Promise<Record<number, SteamStoreDetails>> {
  return invoke<Record<number, SteamStoreDetails>>('get_game_cache', { customPath: customPath || null });
}

export async function fetchGameFromStoreOrUrl(
  input: string,
  forceRefresh?: boolean,
  customPath?: string
): Promise<SteamStoreDetails> {
  return invoke<SteamStoreDetails>('fetch_game_from_store_or_url', {
    input,
    forceRefresh: forceRefresh || false,
    customPath: customPath || null,
  });
}

export async function crawlSteamCover(appid: number): Promise<string> {
  return invoke<string>('crawl_steam_cover', { appid });
}

export async function searchSteamGames(query: string): Promise<SteamSearchResultItem[]> {
  return invoke<SteamSearchResultItem[]>('search_steam_games', { query });
}

export async function getPopularSteamGames(): Promise<SteamSearchResultItem[]> {
  return invoke<SteamSearchResultItem[]>('get_popular_steam_games');
}


// Accounts Management
export async function getAccounts(customPath?: string): Promise<SteamAccount[]> {
  return invoke<SteamAccount[]>('get_accounts', { customPath: customPath || null });
}

export async function switchAccount(
  steamId: string,
  accountName: string,
  autoRestart: boolean,
  customPath?: string
): Promise<string> {
  return invoke<string>('switch_account', {
    steamId,
    accountName,
    autoRestart,
    customPath: customPath || null,
  });
}

export async function logoutAccount(customPath?: string): Promise<string> {
  return invoke<string>('logout_account', { customPath: customPath || null });
}

// Steam Process Controls
export async function restartSteamClient(customPath?: string): Promise<string> {
  return invoke<string>('restart_steam_client', { customPath: customPath || null });
}

export async function killSteamClient(): Promise<string> {
  return invoke<string>('kill_steam_client');
}

export async function startSteamClient(customPath?: string): Promise<string> {
  return invoke<string>('start_steam_client', { customPath: customPath || null });
}

export async function launchGameById(appid: number): Promise<string> {
  return invoke<string>('launch_game_by_id', { appid });
}

export async function openFolderInExplorer(folderPath: string): Promise<void> {
  return invoke<void>('open_folder_in_explorer', { folderPath });
}

export async function cleanAllDepotCache(customPath?: string): Promise<string> {
  return invoke<string>('clean_all_depot_cache', { customPath: customPath || null });
}

export async function downloadOnlineManifest(appid: number, customPath?: string): Promise<ZipImportResult> {
  return invoke<ZipImportResult>('download_online_manifest', { appid, steamPath: customPath || null });
}

export async function getManifestSource(customPath?: string): Promise<string> {
  return invoke<string>('get_manifest_source', { steamPath: customPath || null });
}

export async function setManifestSource(source: string, customPath?: string): Promise<string> {
  return invoke<string>('set_manifest_source', { source, steamPath: customPath || null });
}

