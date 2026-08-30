<template>
  <div class="settings-container">
    <!-- Section 1: Steam Directory -->
    <div class="settings-card glass-panel">
      <div class="card-header">
        <div class="header-icon-box bg-cyan-glow">
          <FolderCog :size="20" class="text-cyan" />
        </div>
        <div>
          <h3 class="card-title">Steam 安装目录与环境检测</h3>
          <p class="card-desc">自动检测自 Windows 注册表，若使用非默认路径可在此自定义配置</p>
        </div>
      </div>

      <div class="card-body">
        <div class="path-input-row">
          <input
            v-model="customPathInput"
            type="text"
            class="input-text font-mono path-input"
            placeholder="例如: C:\Program Files (x86)\Steam"
          />
          <div class="path-btn-group">
            <button class="btn btn-steam" @click="handleBrowseSteamPath" title="浏览本地文件夹">
              <FolderOpen :size="13" />
              <span>浏览选择</span>
            </button>
            <button class="btn btn-primary" @click="handleApplyPath" title="保存自定义 Steam 路径">
              <Check :size="13" />
              <span>保存路径</span>
            </button>
            <button class="btn btn-ghost" @click="handleAutoDetect" title="重新从系统注册表检测">
              <RefreshCw :size="13" />
              <span>自动检测</span>
            </button>
            <button class="btn btn-ghost" @click="$emit('open-folder', steamInfo.steam_path)" title="在资源管理器中打开">
              <Folder :size="13" />
              <span>打开 Steam 目录</span>
            </button>
          </div>
        </div>

        <div class="path-status-grid">
          <div class="path-status-item">
            <span class="status-key">Steam 主程序:</span>
            <span class="status-val font-mono" :class="steamInfo.is_installed ? 'text-emerald' : 'text-danger'">
              {{ steamInfo.is_installed ? '已检测到 (steam.exe)' : '未找到' }}
            </span>
          </div>

          <div class="path-status-item">
            <span class="status-key">Lua 脚本目录:</span>
            <span class="status-val font-mono text-cyan">
              config/lua & config/stplug-in
            </span>
          </div>

          <div class="path-status-item">
            <span class="status-key">清单缓存目录 (depotcache):</span>
            <span class="status-val font-mono text-cyan">
              {{ steamInfo.depotcache_path }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Section 2: Unlock Mode & OpenSteamTool Suite Manager -->
    <div class="settings-card glass-panel">
      <div class="card-header">
        <div class="header-icon-box" :class="steamInfo.is_unlock_active ? 'bg-emerald-glow' : 'bg-amber-glow'">
          <ShieldCheck v-if="steamInfo.is_unlock_active" :size="20" class="text-emerald" />
          <ShieldAlert v-else :size="20" class="text-amber" />
        </div>
        <div>
          <h3 class="card-title">解锁模式与 OpenSteamTool 核心套件管理</h3>
          <p class="card-desc">
            Steam 启动时加载系统引导加载器（xinput1_4.dll / dwmapi.dll）并调起 OpenSteamTool.dll 核心引擎以拦截清单与授权校验
          </p>
        </div>
      </div>

      <div class="card-body">
        <div class="dll-status-bar">
          <div class="dll-status-left">
            <span class="dll-tag" :class="steamInfo.is_unlock_active ? 'tag-active' : 'tag-disabled'">
              {{ steamInfo.is_unlock_active ? '解锁模式 已生效 (OpenSteamTool 核心套件已激活)' : '解锁模式 已停用 (原生模式)' }}
            </span>
          </div>

          <div class="dll-actions">
            <button
              class="btn"
              :class="steamInfo.is_unlock_active ? 'btn-danger' : 'btn-success'"
              @click="$emit('toggle-unlock')"
            >
              <Power :size="15" />
              <span>{{ steamInfo.is_unlock_active ? '关闭解锁模式' : '开启解锁模式' }}</span>
            </button>

            <button class="btn btn-ghost" @click="$emit('repair-dll')">
              <Wrench :size="15" />
              <span>一键修复 / 重置全部 DLL 钩子</span>
            </button>
          </div>
        </div>

        <!-- 4-File Visual Suite Grid -->
        <div class="dll-suite-grid">
          <div class="dll-suite-item" :class="{ active: steamInfo.is_unlock_active }">
            <div class="dll-indicator"></div>
            <div class="dll-item-info">
              <div class="dll-item-name font-mono">OpenSteamTool.dll</div>
              <div class="dll-item-desc">核心 Hook 引擎 (Detours / 密钥解密 / 清单拦截)</div>
            </div>
          </div>

          <div class="dll-suite-item" :class="{ active: steamInfo.is_unlock_active }">
            <div class="dll-indicator"></div>
            <div class="dll-item-info">
              <div class="dll-item-name font-mono">dwmapi.dll</div>
              <div class="dll-item-desc">DWMAPI 系统引导劫持加载器</div>
            </div>
          </div>

          <div class="dll-suite-item" :class="{ active: steamInfo.is_unlock_active }">
            <div class="dll-indicator"></div>
            <div class="dll-item-info">
              <div class="dll-item-name font-mono">xinput1_4.dll</div>
              <div class="dll-item-desc">XInput 系统引导劫持加载器</div>
            </div>
          </div>

          <div class="dll-suite-item" :class="{ active: steamInfo.is_unlock_active }">
            <div class="dll-indicator"></div>
            <div class="dll-item-info">
              <div class="dll-item-name font-mono">opensteamtool.toml</div>
              <div class="dll-item-desc">配置文件 (多路径 Lua 监听与清单服务配置)</div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Section 3: Directory Shortcuts & Cache Cleaner -->
    <div class="settings-card glass-panel">
      <div class="card-header">
        <div class="header-icon-box bg-purple-glow">
          <Layers :size="20" class="text-purple" />
        </div>
        <div>
          <h3 class="card-title">目录快捷打开与缓存管理</h3>
          <p class="card-desc">快速访问关键数据目录，或清理无用的 Manifest 清单缓存文件</p>
        </div>
      </div>

      <div class="card-body">
        <div class="quick-actions-row">
          <button class="btn btn-ghost" @click="$emit('open-folder', `${steamInfo.steam_path}\\config\\lua`)">
            <FolderArchive :size="15" class="text-cyan" />
            <span>打开 Lua 目录 (config/lua)</span>
          </button>

          <button class="btn btn-ghost" @click="$emit('open-folder', `${steamInfo.steam_path}\\config\\stplug-in`)">
            <FolderArchive :size="15" class="text-emerald" />
            <span>打开兼容目录 (stplug-in)</span>
          </button>

          <button class="btn btn-ghost" @click="$emit('open-folder', steamInfo.depotcache_path)">
            <FolderArchive :size="15" class="text-purple" />
            <span>打开清单目录 (depotcache)</span>
          </button>

          <button class="btn btn-danger" @click="$emit('clean-cache')">
            <Trash2 :size="15" />
            <span>一键清理全部 Manifest 缓存</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Section 4: Steam Client Process Controls -->
    <div class="settings-card glass-panel">
      <div class="card-header">
        <div class="header-icon-box bg-cyan-glow">
          <Terminal :size="20" class="text-cyan" />
        </div>
        <div>
          <h3 class="card-title">Steam 客户端进程控制</h3>
          <p class="card-desc">实时监控与快速拉起/停止 Steam 客户端</p>
        </div>
      </div>

      <div class="card-body">
        <div class="process-controls-row">
          <div class="process-status-text">
            <span>当前状态：</span>
            <span :class="steamStatus.is_running ? 'text-emerald' : 'text-dim'">
              {{ steamStatus.is_running ? `运行中 (PID: ${steamStatus.pid})` : '未运行' }}
            </span>
          </div>

          <div class="process-btns">
            <button class="btn btn-steam" :disabled="steamStatus.is_running" @click="$emit('start-steam')">
              <Play :size="15" />
              <span>启动 Steam</span>
            </button>

            <button class="btn btn-danger" :disabled="!steamStatus.is_running" @click="$emit('kill-steam')">
              <Square :size="15" />
              <span>结束 Steam 进程</span>
            </button>

            <button class="btn btn-primary" :disabled="isRestarting" @click="$emit('restart-steam')">
              <RotateCw :size="15" :class="{ 'animate-spin': isRestarting }" />
              <span>完全重启 Steam</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { open as openFileDialog } from '@tauri-apps/plugin-dialog';
import { isTauri } from '../api/tauri';
import {
  FolderCog,
  Check,
  RefreshCw,
  FolderOpen,
  Folder,
  ShieldCheck,
  ShieldAlert,
  Power,
  Wrench,
  Layers,
  FolderArchive,
  Trash2,
  Terminal,
  Play,
  Square,
  RotateCw,
} from 'lucide-vue-next';
import type { SteamInfo, SteamProcessStatus } from '../types/steam';

const props = defineProps<{
  steamInfo: SteamInfo;
  steamStatus: SteamProcessStatus;
  isRestarting: boolean;
}>();

const emit = defineEmits<{
  (e: 'update-path', path: string): void;
  (e: 'auto-detect-path'): void;
  (e: 'toggle-unlock'): void;
  (e: 'repair-dll'): void;
  (e: 'open-folder', path: string): void;
  (e: 'clean-cache'): void;
  (e: 'start-steam'): void;
  (e: 'kill-steam'): void;
  (e: 'restart-steam'): void;
}>();

const customPathInput = ref('');

watch(
  () => props.steamInfo.steam_path,
  (val) => {
    customPathInput.value = val || '';
  },
  { immediate: true }
);

const handleBrowseSteamPath = async () => {
  if (isTauri()) {
    try {
      const selected = await openFileDialog({
        title: '选择 Steam 根安装目录',
        directory: true,
        multiple: false,
        defaultPath: customPathInput.value || undefined,
      });
      if (selected && typeof selected === 'string') {
        customPathInput.value = selected;
        emit('update-path', selected);
      }
    } catch (err) {
      console.error('Failed to browse steam directory:', err);
    }
  }
};

const handleApplyPath = () => {
  if (customPathInput.value.trim()) {
    emit('update-path', customPathInput.value.trim());
  }
};

const handleAutoDetect = () => {
  emit('auto-detect-path');
};
</script>

<style scoped>
.settings-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 10px 14px;
  height: calc(100vh - 48px);
  overflow-y: auto;
}

.settings-card {
  display: flex;
  flex-direction: column;
  padding: 12px 14px;
  gap: 10px;
  border-radius: var(--radius-sm);
}

.card-header {
  display: flex;
  align-items: center;
  gap: 10px;
}

.header-icon-box {
  width: 32px;
  height: 32px;
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.bg-cyan-glow {
  background: rgba(0, 242, 255, 0.12);
  border: 1px solid rgba(0, 242, 255, 0.3);
}

.bg-emerald-glow {
  background: rgba(16, 185, 129, 0.12);
  border: 1px solid rgba(16, 185, 129, 0.3);
}

.bg-amber-glow {
  background: rgba(245, 158, 11, 0.12);
  border: 1px solid rgba(245, 158, 11, 0.3);
}

.bg-purple-glow {
  background: rgba(168, 85, 247, 0.12);
  border: 1px solid rgba(168, 85, 247, 0.3);
}

.text-cyan { color: var(--accent-cyan); }
.text-emerald { color: var(--accent-green); }
.text-amber { color: var(--accent-amber); }
.text-purple { color: var(--accent-purple); }
.text-danger { color: var(--accent-red); }

.card-title {
  font-size: 13px;
  font-weight: 700;
  color: #ffffff;
}

.card-desc {
  font-size: 11px;
  color: var(--text-dim);
  margin-top: 1px;
}

.card-body {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.path-input-row {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.path-input {
  flex: 1;
  min-width: 240px;
}

.path-btn-group {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-wrap: wrap;
}

.path-status-grid {
  display: flex;
  flex-direction: column;
  gap: 6px;
  background: rgba(10, 15, 24, 0.6);
  padding: 8px 12px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-subtle);
}

.path-status-item {
  display: flex;
  justify-content: space-between;
  font-size: 11px;
}

.status-key {
  color: var(--text-dim);
}

/* DLL Manager */
.dll-status-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: rgba(10, 15, 24, 0.6);
  padding: 8px 12px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-subtle);
}

.dll-tag {
  font-size: 11px;
  font-weight: 600;
  padding: 3px 8px;
  border-radius: 4px;
}

.tag-active {
  background: rgba(16, 185, 129, 0.15);
  color: #34d399;
  border: 1px solid rgba(16, 185, 129, 0.3);
}

.tag-disabled {
  background: rgba(245, 158, 11, 0.15);
  color: var(--accent-amber);
  border: 1px solid rgba(245, 158, 11, 0.3);
}

.dll-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.dll-suite-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 8px;
  margin-top: 2px;
}

.dll-suite-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 10px;
  background: rgba(10, 15, 24, 0.45);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  transition: all 0.2s ease;
}

.dll-suite-item.active {
  background: rgba(16, 185, 129, 0.06);
  border-color: rgba(16, 185, 129, 0.25);
}

.dll-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #f59e0b;
  box-shadow: 0 0 6px rgba(245, 158, 11, 0.5);
  flex-shrink: 0;
}

.dll-suite-item.active .dll-indicator {
  background: #10b981;
  box-shadow: 0 0 8px rgba(16, 185, 129, 0.8);
}

.dll-item-info {
  display: flex;
  flex-direction: column;
  gap: 1px;
  overflow: hidden;
}

.dll-item-name {
  font-size: 11.5px;
  font-weight: 700;
  color: #f1f5f9;
}

.dll-item-desc {
  font-size: 10px;
  color: var(--text-dim);
  white-space: nowrap;
  text-overflow: ellipsis;
  overflow: hidden;
}

/* Quick Actions */
.quick-actions-row {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

/* Process Controls */
.process-controls-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: rgba(10, 15, 24, 0.6);
  padding: 8px 12px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-subtle);
}

.process-status-text {
  font-size: 11px;
  font-weight: 600;
  display: flex;
  gap: 4px;
}

.process-btns {
  display: flex;
  align-items: center;
  gap: 8px;
}
</style>
