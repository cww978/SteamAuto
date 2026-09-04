<template>
  <div class="add-container">
    <!-- Sub-tab Navigation -->
    <div class="mode-tabs glass-panel">
      <button
        class="mode-tab-btn"
        :class="{ active: activeMode === 'zip' }"
        @click="activeMode = 'zip'"
      >
        <Archive :size="16" />
        <span>本地导入</span>
      </button>

      <button
        class="mode-tab-btn"
        :class="{ active: activeMode === 'online' }"
        @click="activeMode = 'online'"
      >
        <Download :size="16" />
        <span>在线下载</span>
      </button>

      <button
        class="mode-tab-btn"
        :class="{ active: activeMode === 'custom' }"
        @click="activeMode = 'custom'"
      >
        <Code2 :size="16" />
        <span>自定义脚本</span>
      </button>
    </div>

    <!-- Mode 1: Zip Package or Lua Script Import -->
    <div v-if="activeMode === 'zip'" class="tab-content">
      <div
        class="drop-zone glass-card"
        :class="{ dragging: isDragging, 'has-file': !!selectedZipPath }"
        @dragover.prevent="isDragging = true"
        @dragleave.prevent="isDragging = false"
        @drop.prevent="handleDrop"
        @click="triggerFileInput"
      >
        <input
          ref="fileInputRef"
          type="file"
          accept=".zip,.lua"
          style="display: none"
          @change="handleFileChange"
        />
        
        <div class="drop-icon-box" :class="{ 'icon-active': !!selectedZipPath }">
          <FileCode2 v-if="selectedZipPath && isLuaFile" :size="36" class="text-cyan" />
          <FileArchive v-else-if="selectedZipPath" :size="36" class="text-emerald" />
          <UploadCloud v-else :size="36" class="text-cyan" />
        </div>

        <h3 class="drop-title">
          {{ selectedZipPath ? selectedZipName : '拖拽下载清单压缩包 (.zip) 或 Lua 脚本 (.lua) 到此处，或点击浏览选择' }}
        </h3>
        <p class="drop-desc">
          {{ selectedZipPath ? (isLuaFile ? '已锁定本地 Lua 脚本文件，确认后点击下方一键入库并部署' : '已锁定本地清单压缩包，确认后点击下方一键入库并部署') : '支持 OpenSteamTool / Walftech 压缩包（含 .manifest 部署清单）及独立 .lua 脚本' }}
        </p>

        <div v-if="selectedZipPath" class="selected-file-badge font-mono" @click.stop>
          <CheckCircle2 :size="14" class="text-emerald flex-shrink-0" />
          <span class="path-text" :title="selectedZipPath">{{ selectedZipPath }}</span>
          <button class="badge-action-btn" title="复制路径" @click.stop="copyPath">
            <Check v-if="copiedPath" :size="12" class="text-emerald" />
            <Copy v-else :size="12" />
          </button>
          <button class="badge-action-btn" title="清除已选" @click.stop="clearSelectedZip">
            <X :size="12" />
          </button>
        </div>

        <div class="drop-actions">
          <button class="btn btn-steam" @click.stop="triggerFileInput">
            <FolderOpen :size="15" />
            <span>{{ selectedZipPath ? '重新选择文件' : '选择本地 ZIP / LUA 文件' }}</span>
          </button>
        </div>
      </div>

      <!-- Action Button -->
      <div v-if="selectedZipPath" class="import-action-bar glass-panel">
        <div class="action-info">
          <FileCode2 v-if="isLuaFile" :size="22" class="text-cyan" />
          <FileArchive v-else :size="22" class="text-emerald" />
          <div>
            <div class="action-name">{{ selectedZipName }}</div>
            <div class="action-tip font-mono">{{ selectedZipPath }}</div>
          </div>
        </div>

        <button class="btn btn-success btn-lg" :disabled="importing" @click="handleImportZip">
          <ArrowRight :size="16" />
          <span>{{ importing ? '正在解析并入库...' : '一键入库并部署' }}</span>
        </button>
      </div>
    </div>

    <!-- Mode 2: Online Manifest & Key Download by AppID -->
    <div v-else-if="activeMode === 'online'" class="tab-content">
      <!-- Tip Banner -->
      <div class="tip-banner glass-panel">
        <Sparkles :size="18" class="text-cyan flex-shrink-0" />
        <div class="tip-content">
          <div class="tip-title">输入 AppID 在线下载清单与密钥</div>
          <div class="tip-desc">
            输入 Steam 游戏 AppID，软件将自动从云端检索下载包含 <b>.manifest 实体清单</b> 与 <b>解密密钥</b> 的完整包，并一键解压部署至 Steam，直接支持客户端内点击下载。
          </div>
        </div>
      </div>

      <div class="input-card glass-panel">
        <div class="input-card-header">
          <label class="input-label">输入 Steam 游戏 AppID：</label>
          <div class="source-picker">
            <Globe2 :size="13" class="text-cyan" />
            <span class="source-picker-label">清单上游源:</span>
            <select v-model="selectedSource" class="source-select font-mono" @change="handleChangeSource">
              <option value="wudrm">WUDRM</option>
              <option value="opensteamtool">OST</option>
              <option value="steamrun">SR</option>
            </select>
          </div>
        </div>

        <div class="url-input-row">
          <input
            v-model="appidInput"
            type="text"
            class="input-text url-input font-mono"
            placeholder="例如: 1245620 (艾尔登法环) 或 2868840 (杀戮尖塔2)"
            @keyup.enter="handleDownloadManifestDirect"
          />
          <button
            class="btn btn-success"
            :disabled="downloadingManifest || !appidInput.trim()"
            @click="handleDownloadManifestDirect"
            title="自动从高速清单源下载完整 .manifest 实体文件与密钥并一键入库部署"
          >
            <Download :size="15" :class="{ 'animate-spin': downloadingManifest }" />
            <span>{{ downloadingManifest ? '正在入库...' : '一键入库' }}</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Mode 3: Custom Lua Script -->
    <div v-else-if="activeMode === 'custom'" class="tab-content">
      <div class="custom-card glass-panel">
        <div class="custom-header">
          <div class="input-field">
            <label class="field-label">主游戏 AppID：</label>
            <input
              v-model.number="customAppid"
              type="number"
              class="input-text font-mono"
              placeholder="例如: 1245620"
            />
          </div>
          <div class="input-field" style="flex: 2;">
            <label class="field-label">游戏名称（可选）：</label>
            <input
              v-model="customName"
              type="text"
              class="input-text"
              placeholder="例如: Elden Ring"
            />
          </div>
        </div>

        <div class="custom-editor-area">
          <div class="lua-editor-header">
            <span class="editor-title font-mono">自定义 Lua 内容</span>
            <div class="editor-toolbar">
              <button class="btn-tool" @click="insertCustom('addappid')">+ addappid</button>
              <button class="btn-tool" @click="insertCustom('manifest')">+ setManifestid</button>
            </div>
          </div>
          <textarea
            v-model="customLua"
            class="code-textarea font-mono"
            placeholder="addappid(1245620)&#10;setManifestid(1245621, &quot;6005357081270866877&quot;)"
            spellcheck="false"
          ></textarea>
        </div>

        <div class="custom-footer">
          <button
            class="btn btn-primary btn-lg"
            :disabled="!customAppid || !customLua.trim() || savingCustom"
            @click="handleSaveCustom"
          >
            <Save :size="16" />
            <span>{{ savingCustom ? '保存中...' : '保存并添加入库' }}</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
import { open as openFileDialog } from '@tauri-apps/plugin-dialog';
import { getCurrentWebview } from '@tauri-apps/api/webview';
import { isTauri } from '../api/tauri';
import {
  Archive,
  Code2,
  UploadCloud,
  FolderOpen,
  Sparkles,
  FileArchive,
  FileCode2,
  ArrowRight,
  CheckCircle2,
  Save,
  X,
  Copy,
  Check,
  Download,
  Globe2,
} from 'lucide-vue-next';
import * as api from '../api/tauri';

const emit = defineEmits<{
  (e: 'import-zip', zipPath: string): void;
  (e: 'save-lua', appid: number, content: string): void;
  (e: 'download-manifest', appid: number): void;
  (e: 'update-manifest-source', source: string): void;
}>();

const props = defineProps<{
  importing?: boolean;
  downloadingManifest?: boolean;
  initialZipPath?: string;
}>();

const activeMode = ref<'zip' | 'online' | 'custom'>('zip');
const isDragging = ref(false);
const selectedZipPath = ref('');
const fileInputRef = ref<HTMLInputElement | null>(null);
const copiedPath = ref(false);

const appidInput = ref('');
const selectedSource = ref('wudrm');

// Custom Lua fields
const customAppid = ref<number | null>(null);
const customName = ref('');
const customLua = ref('');
const savingCustom = ref(false);

watch(
  () => props.initialZipPath,
  (val) => {
    if (val) {
      selectedZipPath.value = val;
      activeMode.value = 'zip';
    }
  },
  { immediate: true }
);

const isLuaFile = computed(() => {
  return selectedZipPath.value.toLowerCase().endsWith('.lua');
});

const selectedZipName = computed(() => {
  if (!selectedZipPath.value) return '';
  const parts = selectedZipPath.value.replace(/\\/g, '/').split('/');
  return parts[parts.length - 1];
});

let unlistenDragDrop: (() => void) | null = null;

onMounted(async () => {
  if (isTauri()) {
    try {
      const src = await api.getManifestSource();
      if (src) {
        selectedSource.value = src;
      }
    } catch (e) {
      console.warn('Failed to get manifest source:', e);
    }

    try {
      unlistenDragDrop = await getCurrentWebview().onDragDropEvent((event) => {
        if (event.payload.type === 'enter' || event.payload.type === 'over') {
          isDragging.value = true;
        } else if (event.payload.type === 'leave') {
          isDragging.value = false;
        } else if (event.payload.type === 'drop') {
          isDragging.value = false;
          const paths = event.payload.paths;
          if (paths && paths.length > 0) {
            const validPath = paths.find((p) => p.toLowerCase().endsWith('.zip') || p.toLowerCase().endsWith('.lua')) || paths[0];
            if (validPath) {
              selectedZipPath.value = validPath;
              activeMode.value = 'zip';
            }
          }
        }
      });
    } catch (err) {
      console.warn('Registering onDragDropEvent failed:', err);
    }
  }
});

onUnmounted(() => {
  if (unlistenDragDrop) {
    unlistenDragDrop();
    unlistenDragDrop = null;
  }
});

const triggerFileInput = async () => {
  if (isTauri()) {
    try {
      const selected = await openFileDialog({
        title: '选择清单压缩包 (.zip) 或 Lua 脚本 (.lua)',
        multiple: false,
        filters: [
          {
            name: '清单压缩包 / Lua 脚本 (*.zip, *.lua)',
            extensions: ['zip', 'lua'],
          },
          {
            name: 'ZIP 压缩包 (*.zip)',
            extensions: ['zip'],
          },
          {
            name: 'Lua 脚本 (*.lua)',
            extensions: ['lua'],
          },
        ],
      });
      if (selected && typeof selected === 'string') {
        selectedZipPath.value = selected;
      }
      return;
    } catch (err) {
      console.warn('Tauri open dialog error, falling back to input:', err);
    }
  }
  fileInputRef.value?.click();
};

const handleFileChange = (e: Event) => {
  const target = e.target as HTMLInputElement;
  if (target.files && target.files.length > 0) {
    const file = target.files[0];
    const p = (file as any).path || file.name;
    selectedZipPath.value = p;
  }
};

const handleDrop = (e: DragEvent) => {
  isDragging.value = false;
  if (e.dataTransfer && e.dataTransfer.files.length > 0) {
    const file = e.dataTransfer.files[0];
    const p = (file as any).path || file.name;
    if (p) {
      selectedZipPath.value = p;
    }
  }
};

const clearSelectedZip = () => {
  selectedZipPath.value = '';
  if (fileInputRef.value) {
    fileInputRef.value.value = '';
  }
};

const copyPath = async () => {
  if (selectedZipPath.value) {
    try {
      await navigator.clipboard.writeText(selectedZipPath.value);
      copiedPath.value = true;
      setTimeout(() => {
        copiedPath.value = false;
      }, 2000);
    } catch (e) {
      console.error('Failed to copy path:', e);
    }
  }
};

const handleImportZip = () => {
  if (selectedZipPath.value) {
    emit('import-zip', selectedZipPath.value);
  }
};

const extractAppId = (input: string): number | null => {
  const trimmed = input.trim();
  if (/^\d+$/.test(trimmed)) return parseInt(trimmed, 10);
  const match = trimmed.match(/\/app\/(\d+)/);
  if (match) return parseInt(match[1], 10);
  const digits = trimmed.match(/\b\d{4,8}\b/);
  return digits ? parseInt(digits[0], 10) : null;
};

const handleChangeSource = async () => {
  try {
    await api.setManifestSource(selectedSource.value);
    emit('update-manifest-source', selectedSource.value);
  } catch (e) {
    console.error('Failed to set manifest source:', e);
  }
};

const handleDownloadManifestDirect = () => {
  if (!appidInput.value.trim() || props.downloadingManifest) return;
  const appid = extractAppId(appidInput.value);
  if (appid) {
    emit('download-manifest', appid);
  }
};

const insertCustom = (type: 'addappid' | 'manifest') => {
  const id = customAppid.value || 123456;
  if (type === 'addappid') {
    customLua.value += `\naddappid(${id})`;
  } else if (type === 'manifest') {
    customLua.value += `\nsetManifestid(${id}, "0000000000000000000")`;
  }
};

const handleSaveCustom = () => {
  if (customAppid.value && customLua.value.trim()) {
    let script = customLua.value.trim();
    if (customName.value.trim() && !script.includes('-- Name:')) {
      script = `-- Name: ${customName.value.trim()}\n-- AppID: ${customAppid.value}\n\n` + script;
    }
    emit('save-lua', customAppid.value, script);
  }
};
</script>

<style scoped>
.add-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 10px 14px;
  height: calc(100vh - 48px);
  overflow-y: auto;
}

/* Mode Tabs */
.mode-tabs {
  display: flex;
  align-items: center;
  padding: 3px;
  gap: 4px;
  background: rgba(13, 18, 28, 0.7);
  border-radius: var(--radius-sm);
}

.mode-tab-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 6px 12px;
  font-size: 12px;
  font-weight: 600;
  color: var(--text-muted);
  background: transparent;
  border: 1px solid transparent;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all 0.2s ease;
}

.mode-tab-btn:hover {
  color: var(--text-main);
  background: rgba(255, 255, 255, 0.05);
}

.mode-tab-btn.active {
  color: #ffffff;
  background: linear-gradient(135deg, rgba(0, 242, 255, 0.15) 0%, rgba(26, 159, 255, 0.2) 100%);
  border-color: rgba(0, 242, 255, 0.3);
  box-shadow: 0 0 12px rgba(0, 242, 255, 0.15);
}

.tab-content {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

/* Drop Zone */
.drop-zone {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 30px 16px;
  border: 2px dashed rgba(0, 242, 255, 0.25);
  border-radius: var(--radius-lg);
  text-align: center;
  gap: 8px;
  cursor: pointer;
  background: rgba(13, 18, 28, 0.5);
  transition: all 0.3s ease;
}

.drop-zone:hover,
.drop-zone.dragging {
  border-color: var(--accent-cyan);
  background: rgba(0, 242, 255, 0.06);
  box-shadow: 0 0 20px rgba(0, 242, 255, 0.15);
}

.drop-zone.has-file {
  border-color: rgba(16, 185, 129, 0.4);
  background: rgba(16, 185, 129, 0.04);
}

.drop-icon-box {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  background: rgba(0, 242, 255, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 2px;
  transition: all 0.3s ease;
}

.drop-icon-box.icon-active {
  background: rgba(16, 185, 129, 0.15);
  box-shadow: 0 0 16px rgba(16, 185, 129, 0.2);
}

.drop-title {
  font-size: 13px;
  font-weight: 700;
  color: #ffffff;
}

.drop-desc {
  font-size: 11px;
  color: var(--text-muted);
  max-width: 480px;
}

.selected-file-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  max-width: 90%;
  padding: 4px 10px;
  border-radius: var(--radius-full);
  background: rgba(16, 185, 129, 0.12);
  border: 1px solid rgba(16, 185, 129, 0.35);
  color: #a7f3d0;
  font-size: 11px;
  margin-top: 4px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}

.path-text {
  max-width: 520px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.badge-action-btn {
  background: rgba(255, 255, 255, 0.1);
  border: none;
  color: #a7f3d0;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s;
  padding: 0;
  flex-shrink: 0;
}

.badge-action-btn:hover {
  background: rgba(255, 255, 255, 0.25);
  color: #ffffff;
}

.drop-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 8px;
}

/* Import Action Bar */
.import-action-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
}

.action-info {
  display: flex;
  align-items: center;
  gap: 10px;
}

.action-name {
  font-size: 13px;
  font-weight: 700;
  color: #ffffff;
}

.action-tip {
  font-size: 11px;
  color: var(--text-dim);
}

.btn-lg {
  padding: 8px 16px;
  font-size: 12px;
}

/* Mode 2: URL Input */
.input-card {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 20px;
}

.input-card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
}

.source-picker {
  display: flex;
  align-items: center;
  gap: 6px;
  background: rgba(15, 23, 42, 0.7);
  padding: 4px 10px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-subtle);
}

.source-picker-label {
  font-size: 11px;
  color: var(--text-dim);
}

.source-select {
  background: transparent;
  border: none;
  color: var(--accent-cyan);
  font-size: 11.5px;
  font-weight: 600;
  outline: none;
  cursor: pointer;
  font-family: inherit;
}
.source-select option {
  background: #0f172a;
  color: #ffffff;
}

.input-label {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary);
}

.url-input-row {
  display: flex;
  gap: 12px;
}

.url-input {
  flex: 1;
}

/* Custom Mode */
.custom-card {
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.custom-header {
  display: flex;
  gap: 16px;
}

.input-field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  flex: 1;
}

.field-label {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
}

.custom-editor-area {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.lua-editor-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.editor-title {
  font-size: 12px;
  color: var(--accent-cyan);
  font-weight: 600;
}

.code-textarea {
  width: 100%;
  height: 220px;
  background: #090d15;
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  padding: 12px;
  color: #38bdf8;
  font-size: 12px;
  line-height: 1.6;
  resize: vertical;
  outline: none;
}

.code-textarea:focus {
  border-color: var(--accent-cyan);
}

.editor-toolbar {
  display: flex;
  gap: 6px;
}

.btn-tool {
  padding: 3px 8px;
  font-size: 11px;
  background: rgba(255, 255, 255, 0.06);
  border: 1px solid var(--border-subtle);
  color: var(--text-muted);
  border-radius: 4px;
  cursor: pointer;
}
.btn-tool:hover {
  color: #ffffff;
  background: rgba(255, 255, 255, 0.12);
}

.custom-footer {
  display: flex;
  justify-content: flex-end;
}

/* Tip Banner */
.tip-banner {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 14px 18px;
  background: rgba(0, 242, 255, 0.04);
  border: 1px solid rgba(0, 242, 255, 0.2);
  border-radius: var(--radius-lg);
}

.tip-content {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.tip-title {
  font-size: 13px;
  font-weight: 700;
  color: var(--accent-cyan);
}

.tip-desc {
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.6;
}

.tip-desc b {
  color: #ffffff;
}
</style>
