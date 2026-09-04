<template>
  <div class="app-root">
    <!-- Header Navigation -->
    <HeaderNav
      :current-tab="currentTab"
      :is-unlock-active="steamInfo.is_unlock_active"
      :steam-status="steamStatus"
      :game-count="games.length"
      :is-restarting="isRestarting"
      @update:current-tab="currentTab = $event"
      @toggle-unlock="handleToggleUnlock"
      @restart-steam="handleRestartSteam"
    />

    <!-- Main View Content Area -->
    <main class="main-content">
      <transition name="fade-view" mode="out-in">
        <!-- 1. Library View -->
        <LibraryView
          v-if="currentTab === 'library'"
          :games="games"
          :is-unlock-active="steamInfo.is_unlock_active"
          :loading="loadingGames"
          @refresh="loadGames"
          @go-add="currentTab = 'add'"
          @edit-lua="openLuaEditor"
          @delete-game="confirmDeleteGame"
          @launch-game="handleLaunchGame"
        />

        <!-- 2. Add Game View -->
        <AddGameView
          v-else-if="currentTab === 'add'"
          :initial-zip-path="droppedZipPath"
          :importing="isImporting"
          :downloading-manifest="isDownloadingManifest"
          @import-zip="handleImportZip"
          @save-lua="handleSaveLua"
          @download-manifest="handleDownloadOnlineManifest"
        />

        <!-- 3. Accounts View -->
        <AccountsView
          v-else-if="currentTab === 'accounts'"
          :accounts="accounts"
          :loading="loadingAccounts"
          :switching-id="switchingAccountId"
          @refresh="loadAccounts"
          @logout="handleLogout"
          @switch-account="handleSwitchAccount"
        />

        <!-- 4. Settings View -->
        <SettingsView
          v-else-if="currentTab === 'settings'"
          :steam-info="steamInfo"
          :steam-status="steamStatus"
          :is-restarting="isRestarting"
          @update-path="handleUpdateCustomPath"
          @auto-detect-path="handleAutoDetectPath"
          @toggle-unlock="handleToggleUnlock"
          @repair-dll="handleRepairDll"
          @open-folder="handleOpenFolder"
          @clean-cache="confirmCleanCache"
          @start-steam="handleStartSteam"
          @kill-steam="handleKillSteam"
          @restart-steam="handleRestartSteam"
        />
      </transition>
    </main>

    <!-- Modals -->
    <!-- Lua Script Editor Modal -->
    <LuaEditorModal
      v-if="editingGame"
      :is-open="isLuaModalOpen"
      :appid="editingGame.appid"
      :game-name="editingGame.name"
      :lua-content="editingGame.lua_content"
      :saving="isSavingLua"
      @close="isLuaModalOpen = false"
      @save="saveEditedLua"
    />

    <!-- General Confirmation Modal -->
    <ConfirmModal
      :is-open="isConfirmModalOpen"
      :title="confirmModalTitle"
      :message="confirmModalMessage"
      :confirm-text="confirmModalBtnText"
      :danger="isConfirmModalDanger"
      :loading="isConfirmModalLoading"
      @confirm="executeConfirmAction"
      @cancel="isConfirmModalOpen = false"
    />

    <!-- Toast Notifications -->
    <Toast :toasts="toasts" @remove="removeToast" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import HeaderNav from './components/HeaderNav.vue';
import LibraryView from './views/LibraryView.vue';
import AddGameView from './views/AddGameView.vue';
import AccountsView from './views/AccountsView.vue';
import SettingsView from './views/SettingsView.vue';
import LuaEditorModal from './components/LuaEditorModal.vue';
import ConfirmModal from './components/ConfirmModal.vue';
import Toast from './components/Toast.vue';

import type {
  ActiveTab,
  SteamInfo,
  SteamProcessStatus,
  GameItem,
  SteamAccount,
  ToastMessage,
} from './types/steam';

import * as api from './api/tauri';
import { isTauri } from './api/tauri';
import { getCurrentWebview } from '@tauri-apps/api/webview';

// App State
const currentTab = ref<ActiveTab>('library');
const customSteamPath = ref<string>('');
const droppedZipPath = ref<string>('');
let unlistenGlobalDrag: (() => void) | null = null;

const steamInfo = ref<SteamInfo>({
  steam_path: 'C:\\Program Files (x86)\\Steam',
  steam_exe: 'C:\\Program Files (x86)\\Steam\\steam.exe',
  stplugin_path: 'C:\\Program Files (x86)\\Steam\\config\\stplug-in',
  depotcache_path: 'C:\\Program Files (x86)\\Steam\\depotcache',
  is_installed: true,
  is_unlock_active: false,
  total_unlocked_games: 0,
});

const steamStatus = ref<SteamProcessStatus>({
  is_running: false,
  pid: null,
  memory_mb: null,
});

const games = ref<GameItem[]>([]);
const accounts = ref<SteamAccount[]>([]);

const loadingGames = ref(false);
const loadingAccounts = ref(false);
const isRestarting = ref(false);
const isImporting = ref(false);
const isDownloadingManifest = ref(false);
const isSavingLua = ref(false);
const switchingAccountId = ref<string | null>(null);

// Lua Modal
const isLuaModalOpen = ref(false);
const editingGame = ref<GameItem | null>(null);

// Confirm Modal
const isConfirmModalOpen = ref(false);
const confirmModalTitle = ref('');
const confirmModalMessage = ref('');
const confirmModalBtnText = ref('确认');
const isConfirmModalDanger = ref(false);
const isConfirmModalLoading = ref(false);
let pendingConfirmAction: (() => Promise<void>) | null = null;

// Toast Notifications
const toasts = ref<ToastMessage[]>([]);

const addToast = (type: ToastMessage['type'], title: string, description?: string) => {
  const id = Math.random().toString(36).substring(2, 9);
  const toast: ToastMessage = { id, type, title, description, duration: 4000 };
  toasts.value.push(toast);

  setTimeout(() => {
    removeToast(id);
  }, toast.duration);
};

const removeToast = (id: string) => {
  toasts.value = toasts.value.filter((t) => t.id !== id);
};

// Data Fetching
const loadSteamInfo = async () => {
  try {
    const info = await api.getSteamInfo(customSteamPath.value);
    steamInfo.value = info;
  } catch (err: any) {
    console.error('Failed to get Steam Info:', err);
  }
};

const loadSteamStatus = async () => {
  try {
    const status = await api.getSteamProcessStatus();
    steamStatus.value = status;
  } catch (err: any) {
    console.error('Failed to get Steam Process Status:', err);
  }
};

const loadGames = async () => {
  loadingGames.value = true;
  try {
    const list = await api.getInstalledGames(customSteamPath.value);
    games.value = list;
    await loadSteamInfo();
  } catch (err: any) {
    addToast('error', '加载游戏列表失败', err.toString());
  } finally {
    loadingGames.value = false;
  }
};

const loadAccounts = async () => {
  loadingAccounts.value = true;
  try {
    const list = await api.getAccounts(customSteamPath.value);
    accounts.value = list;
  } catch (err: any) {
    addToast('error', '加载账号列表失败', err.toString());
  } finally {
    loadingAccounts.value = false;
  }
};

// Actions
const handleToggleUnlock = async () => {
  const target = !steamInfo.value.is_unlock_active;
  try {
    const msg = await api.toggleUnlockMode(target, customSteamPath.value);
    await loadSteamInfo();
    addToast('success', target ? '解锁模式已开启' : '解锁模式已关闭', msg);
  } catch (err: any) {
    addToast('error', '切换解锁模式失败', err.toString());
  }
};

const handleRepairDll = async () => {
  try {
    const msg = await api.repairUnlockHook(customSteamPath.value);
    await loadSteamInfo();
    addToast('success', 'DLL 修复成功', msg);
  } catch (err: any) {
    addToast('error', '修复失败', err.toString());
  }
};

const handleRestartSteam = async () => {
  isRestarting.value = true;
  try {
    const msg = await api.restartSteamClient(customSteamPath.value);
    addToast('success', 'Steam 重启成功', msg);
    setTimeout(loadSteamStatus, 2000);
  } catch (err: any) {
    addToast('error', '重启 Steam 失败', err.toString());
  } finally {
    isRestarting.value = false;
  }
};

const handleStartSteam = async () => {
  try {
    const msg = await api.startSteamClient(customSteamPath.value);
    addToast('success', '正在拉起 Steam', msg);
    setTimeout(loadSteamStatus, 2000);
  } catch (err: any) {
    addToast('error', '启动 Steam 失败', err.toString());
  }
};

const handleKillSteam = async () => {
  try {
    const msg = await api.killSteamClient();
    addToast('info', 'Steam 进程已停止', msg);
    await loadSteamStatus();
  } catch (err: any) {
    addToast('error', '结束 Steam 进程失败', err.toString());
  }
};

const handleImportZip = async (filePath: string) => {
  isImporting.value = true;
  try {
    const res = await api.importZipFile(filePath, customSteamPath.value);
    addToast('success', `成功入库: ${res.name || res.appid}`, res.message);
    await loadGames();
    currentTab.value = 'library';
  } catch (err: any) {
    addToast('error', '导入文件失败', err.toString());
  } finally {
    isImporting.value = false;
  }
};

const handleSaveLua = async (appid: number, content: string) => {
  try {
    const msg = await api.saveGameLua(appid, content, customSteamPath.value);
    addToast('success', '入库成功', msg);
    await loadGames();
    currentTab.value = 'library';
  } catch (err: any) {
    addToast('error', '保存 Lua 脚本失败', err.toString());
  }
};

const handleDownloadOnlineManifest = async (appid: number) => {
  isDownloadingManifest.value = true;
  try {
    const res = await api.downloadOnlineManifest(appid, customSteamPath.value);
    addToast('success', `成功入库: ${res.name || res.appid}`, res.message);
    await loadGames();
    currentTab.value = 'library';
  } catch (err: any) {
    addToast('error', '在线下载清单失败', err.toString());
  } finally {
    isDownloadingManifest.value = false;
  }
};

const openLuaEditor = (game: GameItem) => {
  editingGame.value = { ...game };
  isLuaModalOpen.value = true;
};

const saveEditedLua = async (content: string) => {
  if (!editingGame.value) return;
  isSavingLua.value = true;
  try {
    const msg = await api.saveGameLua(editingGame.value.appid, content, customSteamPath.value);
    addToast('success', '脚本保存成功', msg);
    isLuaModalOpen.value = false;
    await loadGames();
  } catch (err: any) {
    addToast('error', '保存失败', err.toString());
  } finally {
    isSavingLua.value = false;
  }
};

const confirmDeleteGame = (game: GameItem) => {
  confirmModalTitle.value = `移除游戏: ${game.name}`;
  confirmModalMessage.value = `确定要将 AppID: ${game.appid} 从假入库中移除吗？将同步删除相关的 Lua 脚本及 ${game.manifest_count} 个清单缓存文件。`;
  confirmModalBtnText.value = '确认移除';
  isConfirmModalDanger.value = true;
  pendingConfirmAction = async () => {
    const msg = await api.removeGame(game.appid, true, customSteamPath.value);
    addToast('info', '已移除游戏', msg);
    await loadGames();
  };
  isConfirmModalOpen.value = true;
};

const confirmCleanCache = () => {
  confirmModalTitle.value = '清理所有清单缓存';
  confirmModalMessage.value = '确定要清空 Steam depotcache 目录下的所有 Manifest 清单文件吗？已入库游戏的 Lua 脚本仍会保留。';
  confirmModalBtnText.value = '清空缓存';
  isConfirmModalDanger.value = true;
  pendingConfirmAction = async () => {
    const msg = await api.cleanAllDepotCache(customSteamPath.value);
    addToast('success', '缓存已清理', msg);
    await loadGames();
  };
  isConfirmModalOpen.value = true;
};

const executeConfirmAction = async () => {
  if (pendingConfirmAction) {
    isConfirmModalLoading.value = true;
    try {
      await pendingConfirmAction();
      isConfirmModalOpen.value = false;
    } catch (err: any) {
      addToast('error', '操作失败', err.toString());
    } finally {
      isConfirmModalLoading.value = false;
    }
  }
};

const handleSwitchAccount = async (account: SteamAccount, autoRestart: boolean) => {
  switchingAccountId.value = account.steam_id64;
  try {
    const msg = await api.switchAccount(
      account.steam_id64,
      account.account_name,
      autoRestart,
      customSteamPath.value
    );
    addToast('success', '账号切换成功', msg);
    await loadAccounts();
    if (autoRestart) {
      setTimeout(loadSteamStatus, 2500);
    }
  } catch (err: any) {
    addToast('error', '切换账号失败', err.toString());
  } finally {
    switchingAccountId.value = null;
  }
};

const handleLogout = async () => {
  try {
    const msg = await api.logoutAccount(customSteamPath.value);
    addToast('info', '已退出登录', msg);
    await loadAccounts();
  } catch (err: any) {
    addToast('error', '退出登录失败', err.toString());
  }
};

const handleLaunchGame = async (appid: number) => {
  try {
    const msg = await api.launchGameById(appid);
    addToast('success', '已发送启动请求', msg);
  } catch (err: any) {
    addToast('error', '拉起游戏失败', err.toString());
  }
};

const handleOpenFolder = async (folderPath: string) => {
  try {
    await api.openFolderInExplorer(folderPath);
  } catch (err: any) {
    addToast('error', '打开文件夹失败', err.toString());
  }
};

const handleUpdateCustomPath = async (path: string) => {
  customSteamPath.value = path;
  addToast('success', '已更新 Steam 路径', path);
  await loadSteamInfo();
  await loadGames();
  await loadAccounts();
};

const handleAutoDetectPath = async () => {
  customSteamPath.value = '';
  await loadSteamInfo();
  await loadGames();
  await loadAccounts();
  addToast('info', '自动检测路径完成', steamInfo.value.steam_path);
};

// Interval Polling for status
let timer: any = null;

onMounted(async () => {
  await loadSteamInfo();
  await loadSteamStatus();
  await loadGames();
  await loadAccounts();

  timer = setInterval(() => {
    loadSteamStatus();
  }, 4000);

  if (isTauri()) {
    try {
      unlistenGlobalDrag = await getCurrentWebview().onDragDropEvent((event) => {
        if (event.payload.type === 'drop') {
          const paths = event.payload.paths;
          if (paths && paths.length > 0) {
            const zipPath = paths.find((p) => p.toLowerCase().endsWith('.zip')) || paths[0];
            if (zipPath && zipPath.toLowerCase().endsWith('.zip')) {
              droppedZipPath.value = zipPath;
              currentTab.value = 'add';
              addToast('info', '已识别拖入的压缩包', zipPath);
            }
          }
        }
      });
    } catch (err) {
      console.warn('App global drag drop listener failed:', err);
    }
  }
});

onUnmounted(() => {
  if (timer) clearInterval(timer);
  if (unlistenGlobalDrag) {
    unlistenGlobalDrag();
    unlistenGlobalDrag = null;
  }
});
</script>

<style scoped>
.app-root {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  background-color: var(--bg-app);
}

.main-content {
  flex: 1;
  overflow: hidden;
  position: relative;
}

.fade-view-enter-active,
.fade-view-leave-active {
  transition: opacity 0.15s ease, transform 0.15s ease;
}

.fade-view-enter-from {
  opacity: 0;
  transform: translateY(4px);
}

.fade-view-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>
