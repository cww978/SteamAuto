<template>
  <div
    class="add-container"
    :class="{ 'is-dragging': isDragging }"
    @dragover.prevent="isDragging = true"
    @dragleave.prevent="isDragging = false"
    @drop.prevent="handleNativeDrop"
  >
    <!-- Drag Drop Global Overlay -->
    <div v-if="isDragging" class="drag-overlay">
      <div class="drag-box glass-card">
        <UploadCloud :size="48" class="text-cyan animate-bounce" />
        <div class="drag-title">松开鼠标以导入清单文件</div>
        <div class="drag-desc text-dim">支持 .zip 清单压缩包与 .lua 脚本文件</div>
      </div>
    </div>

    <!-- Hidden File Input for Local Import Button -->
    <input
      ref="fileInputRef"
      type="file"
      accept=".zip,.lua"
      style="display: none"
      @change="handleFileChange"
    />

    <!-- Top Action Toolbar -->
    <div class="toolbar glass-panel">
      <!-- Search Input Area -->
      <div class="search-section">
        <div class="search-input-box">
          <Search :size="16" class="search-icon" />
          <input
            v-model="searchQuery"
            type="text"
            class="search-input"
            placeholder="搜索 Steam 游戏名称（如：黑神话、艾尔登）或直接输入 AppID..."
            @keyup.enter="handleSearch"
          />
          <button v-if="searchQuery" class="clear-btn" @click="clearSearch" title="清空搜索">
            <X :size="14" />
          </button>
        </div>

        <button class="btn btn-primary btn-search" :disabled="searching" @click="handleSearch">
          <Search v-if="!searching" :size="15" />
          <RefreshCw v-else :size="15" class="animate-spin" />
          <span>{{ searching ? '搜索中...' : '搜索' }}</span>
        </button>
      </div>

      <!-- Right Action Tools -->
      <div class="toolbar-right">
        <!-- Upstream Source Picker -->
        <div class="source-picker" title="清单下载上游镜像源">
          <Globe2 :size="13" class="text-cyan flex-shrink-0" />
          <span class="source-label">清单源:</span>
          <select v-model="selectedSource" class="source-select" @change="handleChangeSource">
            <option value="wudrm">WUDRM</option>
            <option value="opensteamtool">OST</option>
            <option value="steamrun">SR</option>
          </select>
        </div>

        <!-- Local File Import Button -->
        <button class="btn btn-steam btn-local-import" @click="triggerFileInput" title="选择本地下载清单压缩包 (.zip) 或 Lua 脚本 (.lua)">
          <FolderOpen :size="15" class="text-cyan" />
          <span>本地导入文件</span>
        </button>
      </div>
    </div>

    <!-- Local File Import Banner (Appears when local file is selected/dropped) -->
    <transition name="slide-fade">
      <div v-if="selectedZipPath" class="local-import-bar glass-card">
        <div class="local-file-info">
          <FileCode2 v-if="isLuaFile" :size="24" class="text-cyan flex-shrink-0" />
          <FileArchive v-else :size="24" class="text-emerald flex-shrink-0" />
          <div class="local-file-texts">
            <div class="local-file-name">{{ selectedZipName }}</div>
            <div class="local-file-path text-dim" :title="selectedZipPath">{{ selectedZipPath }}</div>
          </div>
        </div>
        <div class="local-file-actions">
          <button class="btn btn-ghost btn-sm" @click="selectedZipPath = ''">
            <X :size="14" />
            <span>取消</span>
          </button>
          <button class="btn btn-success" :disabled="importing" @click="handleImportZip">
            <ArrowRight :size="15" />
            <span>{{ importing ? '正在入库并部署...' : '一键导入并部署' }}</span>
          </button>
        </div>
      </div>
    </transition>

    <!-- Category Filter Bar -->
    <div class="category-tabs">
      <div class="tabs-left">
        <button
          class="cat-tab"
          :class="{ active: currentCategory === 'popular' && !searchQuery.trim() }"
          @click="switchToPopular"
        >
          <Flame :size="15" class="cat-icon text-amber" />
          <span>🔥 热门畅销榜</span>
        </button>
        <button
          class="cat-tab"
          :class="{ active: currentCategory === 'classic' && !searchQuery.trim() }"
          @click="switchToClassic"
        >
          <Sparkles :size="15" class="cat-icon text-cyan" />
          <span>✨ 推荐精选</span>
        </button>
        <button
          v-if="searchedQuery"
          class="cat-tab active"
        >
          <Search :size="15" class="cat-icon text-purple" />
          <span>🔍 搜索：“{{ searchedQuery }}” ({{ displayedGames.length }})</span>
        </button>
      </div>
    </div>

    <!-- Games Card Grid Area -->
    <div v-if="searching || loadingPopular" class="loading-state">
      <RefreshCw :size="32" class="animate-spin text-cyan" />
      <div class="loading-text">{{ searching ? '正在从 Steam 商店检索匹配游戏...' : '正在同步 Steam 热门榜单...' }}</div>
    </div>

    <div v-else-if="displayedGames.length > 0" class="games-grid">
      <div
        v-for="game in displayedGames"
        :key="game.id"
        class="game-card glass-card"
        @click="openGameDetails(game.id)"
      >
        <!-- Header Banner Image -->
        <div class="game-cover-box">
          <img
            :src="game.header_image"
            class="game-cover-img"
            :alt="game.name"
            loading="lazy"
            @error="onImgError"
          />
          <div class="cover-overlay"></div>
          <div class="game-appid-tag">ID: {{ game.id }}</div>
          <div v-if="game.price" class="game-price-tag">{{ game.price }}</div>

          <!-- Quick Action Hover Button -->
          <div class="quick-view-action">
            <span class="view-pill">
              <Eye :size="13" />
              <span>查看详情 / 入库</span>
            </span>
          </div>
        </div>

        <!-- Card Bottom Info -->
        <div class="game-info">
          <div class="game-title" :title="game.name">{{ game.name }}</div>
          <div class="game-meta-row">
            <span class="game-id-text">AppID: {{ game.id }}</span>
            <span class="btn-detail-link">
              <span>一键入库</span>
              <ChevronRight :size="12" />
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty Search State -->
    <div v-else class="empty-state glass-panel">
      <Gamepad2 :size="48" class="text-dim mb-3" />
      <h3 class="empty-title">未搜索到相关 Steam 游戏</h3>
      <p class="empty-desc text-dim">
        请尝试输入更简短的游戏关键词（如“悟空”代替完整书名号），或直接在上方输入游戏的 <b>Steam AppID 数字编号</b> 进行直达。
      </p>
      <button class="btn btn-primary mt-3" @click="switchToPopular">
        <Flame :size="15" />
        <span>返回热门榜单</span>
      </button>
    </div>

    <!-- Game Details Modal (详情与一键入库) -->
    <transition name="modal-fade">
      <div v-if="selectedDetails" class="modal-overlay" @click.self="closeDetails">
        <div class="details-modal glass-card">
          <!-- Hero Header Image -->
          <div class="modal-hero">
            <img
              :src="selectedDetails.background || selectedDetails.header_image"
              class="hero-bg-img"
              :alt="selectedDetails.name"
              @error="onImgError"
            />
            <div class="hero-gradient"></div>

            <button class="close-modal-btn" @click="closeDetails" title="关闭详情">
              <X :size="18" />
            </button>

            <!-- Hero Content -->
            <div class="hero-content">
              <div class="hero-id-tag">AppID: {{ selectedDetails.appid }}</div>
              <h2 class="hero-title">{{ selectedDetails.name }}</h2>
              
              <!-- Quick Tags in Hero -->
              <div class="hero-tags">
                <span v-if="selectedDetails.release_date" class="hero-chip">
                  <Calendar :size="12" />
                  <span>发售于 {{ selectedDetails.release_date }}</span>
                </span>
                <span v-if="selectedDetails.developers?.length" class="hero-chip">
                  <User :size="12" />
                  <span>{{ selectedDetails.developers.join(', ') }}</span>
                </span>
                <span v-if="selectedDetails.dlcs?.length" class="hero-chip chip-purple">
                  <Layers :size="12" />
                  <span>含 {{ selectedDetails.dlcs.length }} 款官方 DLC</span>
                </span>
              </div>
            </div>
          </div>

          <!-- Modal Body Content -->
          <div class="modal-body">
            <!-- Genres -->
            <div v-if="selectedDetails.genres?.length" class="genres-row">
              <span v-for="genre in selectedDetails.genres" :key="genre" class="genre-tag">
                {{ genre }}
              </span>
            </div>

            <!-- Game Description -->
            <div class="desc-box">
              <div class="section-title">游戏简介</div>
              <div class="desc-text">{{ selectedDetails.short_description || '暂无官方中文简介。' }}</div>
            </div>

            <!-- Upstream Manifest Download Option Card -->
            <div class="manifest-card">
              <div class="manifest-card-header">
                <div class="manifest-title-group">
                  <div class="manifest-icon-box">
                    <Cpu :size="16" class="text-cyan" />
                  </div>
                  <div>
                    <div class="manifest-main-title">云端清单与密钥部署</div>
                    <div class="manifest-sub-desc">自动匹配并下载实体清单，支持 Steam 客户端直接完整下载</div>
                  </div>
                </div>

                <div class="source-select-pill">
                  <span class="source-pill-label">清单源:</span>
                  <select v-model="selectedSource" class="source-select" @change="handleChangeSource">
                    <option value="wudrm">WUDRM (推荐)</option>
                    <option value="opensteamtool">OST</option>
                    <option value="steamrun">SR</option>
                  </select>
                </div>
              </div>

              <div class="manifest-feature-tags">
                <span class="feature-tag">
                  <CheckCircle2 :size="12" class="text-emerald" />
                  <span>实体清单与解密密钥</span>
                </span>
                <span class="feature-tag">
                  <CheckCircle2 :size="12" class="text-emerald" />
                  <span>全 DLC 自动入库</span>
                </span>
                <span class="feature-tag">
                  <CheckCircle2 :size="12" class="text-emerald" />
                  <span>客户端直接下载</span>
                </span>
              </div>
            </div>
          </div>

          <!-- Modal Footer Actions -->
          <div class="modal-footer">
            <button class="btn btn-ghost" @click="openStoreUrl(selectedDetails.appid)">
              <ExternalLink :size="15" />
              <span>在 Steam 商店中查看</span>
            </button>

            <div class="footer-right">
              <button class="btn btn-ghost" @click="closeDetails">取消</button>
              <button
                class="btn btn-success btn-lg"
                :disabled="downloadingManifest"
                @click="handleInstallGame(selectedDetails.appid)"
              >
                <Download :size="17" :class="{ 'animate-spin': downloadingManifest }" />
                <span>{{ downloadingManifest ? '正在入库部署...' : '一键入库并部署' }}</span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue';
import {
  Search,
  X,
  RefreshCw,
  Globe2,
  FolderOpen,
  Flame,
  Sparkles,
  Gamepad2,
  Eye,
  ChevronRight,
  Calendar,
  User,
  Layers,
  ExternalLink,
  Download,
  UploadCloud,
  FileCode2,
  FileArchive,
  ArrowRight,
  Cpu,
  CheckCircle2,
} from 'lucide-vue-next';
import { getCurrentWebview } from '@tauri-apps/api/webview';
import * as api from '../api/tauri';
import type { SteamSearchResultItem, SteamStoreDetails } from '../types/steam';

const DEFAULT_COVER = '/default_cover.svg';

const props = defineProps<{
  importing?: boolean;
  downloadingManifest?: boolean;
  initialZipPath?: string;
}>();

const emit = defineEmits<{
  (e: 'import-zip', zipPath: string): void;
  (e: 'download-manifest', appid: number): void;
  (e: 'save-lua', appid: number, content: string): void;
  (e: 'update-manifest-source', source: string): void;
}>();

// State
const searchQuery = ref('');
const searchedQuery = ref('');
const searching = ref(false);
const loadingPopular = ref(false);
const currentCategory = ref<'popular' | 'classic'>('popular');
const selectedSource = ref('wudrm');

// Lists
const popularGames = ref<SteamSearchResultItem[]>([]);
const classicGames = ref<SteamSearchResultItem[]>([]);
const searchResults = ref<SteamSearchResultItem[]>([]);

// Details Modal State
const selectedDetails = ref<SteamStoreDetails | null>(null);

// Local Import State
const fileInputRef = ref<HTMLInputElement | null>(null);
const selectedZipPath = ref('');
const isDragging = ref(false);

const isLuaFile = computed(() => {
  return selectedZipPath.value.toLowerCase().endsWith('.lua');
});

const selectedZipName = computed(() => {
  if (!selectedZipPath.value) return '';
  const parts = selectedZipPath.value.replace(/\\/g, '/').split('/');
  return parts[parts.length - 1];
});

const displayedGames = computed(() => {
  if (searchedQuery.value.trim() && searchResults.value.length > 0) {
    return searchResults.value;
  }
  if (currentCategory.value === 'classic') {
    return classicGames.value;
  }
  return popularGames.value;
});

// Curated Classic Fallback Games
const curatedClassicList: SteamSearchResultItem[] = [
  { id: 2358720, name: '黑神话：悟空', header_image: 'https://cdn.akamai.steamstatic.com/steam/apps/2358720/header.jpg', price: '¥268.00' },
  { id: 1245620, name: '艾尔登法环 (ELDEN RING)', header_image: 'https://cdn.akamai.steamstatic.com/steam/apps/1245620/header.jpg', price: '¥298.00' },
  { id: 1091500, name: '赛博朋克 2077', header_image: 'https://cdn.akamai.steamstatic.com/steam/apps/1091500/header.jpg', price: '¥298.00' },
  { id: 2868840, name: '杀戮尖塔 2 (Slay the Spire 2)', header_image: 'https://cdn.akamai.steamstatic.com/steam/apps/2868840/header.jpg', price: '即将推出' },
  { id: 1623730, name: '幻兽帕鲁 (Palworld)', header_image: 'https://cdn.akamai.steamstatic.com/steam/apps/1623730/header.jpg', price: '¥108.00' },
  { id: 2246340, name: '怪物猎人：荒野 (Monster Hunter Wilds)', header_image: 'https://cdn.akamai.steamstatic.com/steam/apps/2246340/header.jpg', price: '¥368.00' },
  { id: 730, name: '反恐精英 2 (Counter-Strike 2)', header_image: 'https://cdn.akamai.steamstatic.com/steam/apps/730/header.jpg', price: '免费开玩' },
  { id: 1086940, name: '博德之门 3 (Baldur\'s Gate 3)', header_image: 'https://cdn.akamai.steamstatic.com/steam/apps/1086940/header.jpg', price: '¥298.00' },
  { id: 271590, name: 'Grand Theft Auto V', header_image: 'https://cdn.akamai.steamstatic.com/steam/apps/271590/header.jpg', price: '¥138.00' },
  { id: 1174180, name: '荒野大镖客：救赎 2', header_image: 'https://cdn.akamai.steamstatic.com/steam/apps/1174180/header.jpg', price: '¥279.00' },
  { id: 413150, name: '星露谷物语 (Stardew Valley)', header_image: 'https://cdn.akamai.steamstatic.com/steam/apps/413150/header.jpg', price: '¥48.00' },
  { id: 892970, name: 'Valheim: 英灵神殿', header_image: 'https://cdn.akamai.steamstatic.com/steam/apps/892970/header.jpg', price: '¥70.00' },
];

const loadPopular = async () => {
  loadingPopular.value = true;
  try {
    const list = await api.getPopularSteamGames();
    if (list && list.length > 0) {
      popularGames.value = list;
    } else {
      popularGames.value = curatedClassicList;
    }
  } catch (err) {
    console.warn('获取 Steam 热门游戏失败，加载精选列表:', err);
    popularGames.value = curatedClassicList;
  } finally {
    loadingPopular.value = false;
  }
};

const handleSearch = async () => {
  const q = searchQuery.value.trim();
  if (!q) {
    searchedQuery.value = '';
    searchResults.value = [];
    return;
  }

  searching.value = true;
  searchedQuery.value = q;
  try {
    const results = await api.searchSteamGames(q);
    searchResults.value = results;
  } catch (err) {
    console.warn('搜索失败:', err);
    searchResults.value = [];
  } finally {
    searching.value = false;
  }
};

const clearSearch = () => {
  searchQuery.value = '';
  searchedQuery.value = '';
  searchResults.value = [];
};

const switchToPopular = () => {
  clearSearch();
  currentCategory.value = 'popular';
};

const switchToClassic = () => {
  clearSearch();
  currentCategory.value = 'classic';
};

const openGameDetails = async (appid: number) => {
  try {
    const details = await api.fetchGameFromStoreOrUrl(String(appid));
    selectedDetails.value = details;
  } catch (err) {
    console.error('获取游戏详情失败:', err);
  }
};

const closeDetails = () => {
  selectedDetails.value = null;
};

const handleInstallGame = (appid: number) => {
  emit('download-manifest', appid);
};

const openStoreUrl = (appid: number) => {
  window.open(`https://store.steampowered.com/app/${appid}`, '_blank');
};

const onImgError = (event: Event) => {
  const img = event.target as HTMLImageElement;
  if (!img) return;
  img.onerror = null;
  img.src = DEFAULT_COVER;
};

const handleChangeSource = async () => {
  try {
    await api.setManifestSource(selectedSource.value);
    emit('update-manifest-source', selectedSource.value);
  } catch (e) {
    console.warn('修改清单源失败:', e);
  }
};

// Local File Import Logic
const triggerFileInput = () => {
  fileInputRef.value?.click();
};

const handleFileChange = (e: Event) => {
  const target = e.target as HTMLInputElement;
  if (target.files && target.files.length > 0) {
    const file = target.files[0];
    const path = (file as any).path || file.name;
    selectedZipPath.value = path;
  }
};

const handleNativeDrop = (e: DragEvent) => {
  isDragging.value = false;
  if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
    const file = e.dataTransfer.files[0];
    const path = (file as any).path || file.name;
    if (path.toLowerCase().endsWith('.zip') || path.toLowerCase().endsWith('.lua')) {
      selectedZipPath.value = path;
    }
  }
};

const handleImportZip = () => {
  if (!selectedZipPath.value) return;
  emit('import-zip', selectedZipPath.value);
};

watch(
  () => props.initialZipPath,
  (val) => {
    if (val) {
      selectedZipPath.value = val;
    }
  },
  { immediate: true }
);

let unlistenDragDrop: (() => void) | null = null;

onMounted(async () => {
  classicGames.value = curatedClassicList;
  await loadPopular();

  if (api.isTauri()) {
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
            const validPath = paths.find(
              (p) => p.toLowerCase().endsWith('.zip') || p.toLowerCase().endsWith('.lua')
            ) || paths[0];
            if (validPath) {
              selectedZipPath.value = validPath;
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
</script>

<style scoped>
.add-container {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 10px 14px;
  height: calc(100vh - 48px);
  overflow-y: auto;
  position: relative;
}

/* Drag overlay */
.drag-overlay {
  position: absolute;
  inset: 0;
  z-index: 999;
  background: rgba(13, 18, 28, 0.88);
  backdrop-filter: blur(10px);
  display: flex;
  align-items: center;
  justify-content: center;
  border: 2px dashed var(--accent-cyan);
  border-radius: var(--radius-sm);
}

.drag-box {
  padding: 30px 48px;
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
}

.drag-title {
  font-size: 18px;
  font-weight: 700;
  color: #ffffff;
}

/* Toolbar */
.toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  gap: 12px;
  border-radius: var(--radius-sm);
}

.search-section {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
  max-width: 580px;
}

.search-input-box {
  position: relative;
  flex: 1;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: 10px;
  color: var(--text-dim);
  pointer-events: none;
}

.search-input {
  width: 100%;
  padding: 7px 32px 7px 32px;
  background: rgba(0, 0, 0, 0.35);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  color: #ffffff;
  font-size: 13px;
  outline: none;
  transition: all 0.2s;
}

.search-input:focus {
  border-color: var(--accent-cyan);
  box-shadow: 0 0 10px rgba(0, 242, 255, 0.2);
}

.clear-btn {
  position: absolute;
  right: 8px;
  background: transparent;
  border: none;
  color: var(--text-dim);
  cursor: pointer;
  padding: 2px;
  display: flex;
  align-items: center;
}
.clear-btn:hover {
  color: #ffffff;
}

.btn-search {
  padding: 7px 16px;
  font-size: 13px;
  white-space: nowrap;
}

.toolbar-right {
  display: flex;
  align-items: center;
  gap: 10px;
}

.source-picker {
  display: flex;
  align-items: center;
  gap: 6px;
  background: rgba(0, 0, 0, 0.35);
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-subtle);
  font-size: 12px;
}

.source-label {
  color: var(--text-dim);
  font-size: 11px;
}

.source-select {
  background: transparent;
  border: none;
  color: var(--accent-cyan);
  font-weight: 700;
  font-size: 11.5px;
  outline: none;
  cursor: pointer;
}

.source-select option {
  background: #111726;
  color: #ffffff;
}

.btn-local-import {
  white-space: nowrap;
  font-size: 12.5px;
  padding: 6px 12px;
  background: rgba(0, 242, 255, 0.08);
  border-color: rgba(0, 242, 255, 0.3);
}
.btn-local-import:hover {
  background: rgba(0, 242, 255, 0.18);
  border-color: var(--accent-cyan);
}

/* Local import bar */
.local-import-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  background: rgba(16, 185, 129, 0.08);
  border: 1px solid rgba(16, 185, 129, 0.3);
  border-radius: var(--radius-sm);
  gap: 12px;
}

.local-file-info {
  display: flex;
  align-items: center;
  gap: 10px;
  overflow: hidden;
}

.local-file-texts {
  overflow: hidden;
}

.local-file-name {
  font-weight: 700;
  color: #ffffff;
  font-size: 13px;
}

.local-file-path {
  font-size: 11px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.local-file-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

/* Category tabs */
.category-tabs {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 2px 4px;
}

.tabs-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.cat-tab {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 5px 12px;
  border-radius: var(--radius-sm);
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid rgba(255, 255, 255, 0.08);
  color: var(--text-muted);
  font-size: 12.5px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.cat-tab:hover {
  color: #ffffff;
  background: rgba(255, 255, 255, 0.08);
}

.cat-tab.active {
  background: rgba(0, 242, 255, 0.12);
  border-color: var(--accent-cyan);
  color: #ffffff;
}

/* Loading state */
.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 0;
  gap: 12px;
}

.loading-text {
  font-size: 13.5px;
  color: var(--text-dim);
}

/* Games Grid */
.games-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: 12px;
  padding-bottom: 20px;
}

.game-card {
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border-radius: var(--radius-sm);
  background: #111726;
  border: 1px solid rgba(255, 255, 255, 0.08);
  cursor: pointer;
  transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1), border-color 0.2s, box-shadow 0.2s;
}

.game-card:hover {
  transform: translateY(-3px);
  border-color: rgba(0, 242, 255, 0.5);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5), 0 0 16px rgba(0, 242, 255, 0.15);
}

.game-cover-box {
  position: relative;
  width: 100%;
  aspect-ratio: 460 / 215;
  background: #090d15;
  overflow: hidden;
}

.game-cover-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.4s ease;
  font-size: 0;
}

.game-card:hover .game-cover-img {
  transform: scale(1.06);
}

.cover-overlay {
  position: absolute;
  inset: 0;
  background: linear-gradient(180deg, transparent 40%, rgba(17, 23, 38, 0.92) 100%);
}

.game-appid-tag {
  position: absolute;
  top: 6px;
  left: 6px;
  background: rgba(0, 0, 0, 0.75);
  backdrop-filter: blur(6px);
  border: 1px solid rgba(255, 255, 255, 0.15);
  padding: 1px 5px;
  border-radius: 3px;
  font-size: 9.5px;
  font-weight: 600;
  color: var(--text-secondary);
}

.game-price-tag {
  position: absolute;
  top: 6px;
  right: 6px;
  background: rgba(16, 185, 129, 0.85);
  padding: 1px 6px;
  border-radius: 3px;
  font-size: 10px;
  font-weight: 700;
  color: #ffffff;
}

.quick-view-action {
  position: absolute;
  bottom: 8px;
  right: 8px;
  opacity: 0;
  transform: translateY(4px);
  transition: all 0.2s;
}

.game-card:hover .quick-view-action {
  opacity: 1;
  transform: translateY(0);
}

.view-pill {
  display: flex;
  align-items: center;
  gap: 4px;
  background: rgba(0, 242, 255, 0.9);
  color: #0d121c;
  font-size: 10.5px;
  font-weight: 700;
  padding: 3px 8px;
  border-radius: 20px;
  box-shadow: 0 2px 10px rgba(0, 242, 255, 0.5);
}

.game-info {
  padding: 8px 10px;
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.game-title {
  font-size: 13px;
  font-weight: 600;
  color: #f1f5f9;
  line-height: 1.35;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  letter-spacing: -0.01em;
}

.game-meta-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: 2px;
}

.game-id-text {
  font-size: 11px;
  font-weight: 500;
  color: #94a3b8;
  letter-spacing: 0.01em;
}

.btn-detail-link {
  display: inline-flex;
  align-items: center;
  gap: 2px;
  font-size: 11px;
  font-weight: 600;
  color: var(--accent-cyan);
  background: rgba(0, 242, 255, 0.08);
  border: 1px solid rgba(0, 242, 255, 0.22);
  padding: 2px 7px;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.game-card:hover .btn-detail-link {
  background: rgba(0, 242, 255, 0.2);
  border-color: var(--accent-cyan);
  color: #ffffff;
}

/* Empty State */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 50px 20px;
  text-align: center;
  margin-top: 10px;
}

.empty-title {
  font-size: 16px;
  font-weight: 700;
  color: #ffffff;
  margin-bottom: 6px;
}

.empty-desc {
  font-size: 12.5px;
  max-width: 480px;
  line-height: 1.6;
}

/* Details Modal */
.modal-overlay {
  position: fixed;
  inset: 0;
  z-index: 1000;
  background: rgba(0, 0, 0, 0.78);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
}

.details-modal {
  width: 100%;
  max-width: 680px;
  background: #111726;
  border: 1px solid rgba(0, 242, 255, 0.3);
  box-shadow: 0 24px 60px rgba(0, 0, 0, 0.8), 0 0 30px rgba(0, 242, 255, 0.15);
  border-radius: var(--radius-md);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  max-height: 90vh;
}

.modal-hero {
  position: relative;
  width: 100%;
  height: 220px;
  overflow: hidden;
  background: #090d15;
}

.hero-bg-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.hero-gradient {
  position: absolute;
  inset: 0;
  background: linear-gradient(180deg, rgba(0,0,0,0.2) 0%, rgba(17, 23, 38, 0.95) 90%, #111726 100%);
}

.close-modal-btn {
  position: absolute;
  top: 12px;
  right: 12px;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(6px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  color: #ffffff;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s;
  z-index: 10;
}
.close-modal-btn:hover {
  background: rgba(239, 68, 68, 0.8);
  border-color: #ef4444;
}

.hero-content {
  position: absolute;
  bottom: 12px;
  left: 18px;
  right: 18px;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.hero-id-tag {
  display: inline-block;
  font-size: 11px;
  color: var(--accent-cyan);
  font-weight: 600;
}

.hero-title {
  font-size: 20px;
  font-weight: 800;
  color: #ffffff;
  line-height: 1.2;
}

.hero-tags {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.hero-chip {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 11px;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(6px);
  padding: 2px 8px;
  border-radius: 4px;
  color: #ffffff;
}

.chip-purple {
  background: rgba(168, 85, 247, 0.25);
  border: 1px solid rgba(168, 85, 247, 0.4);
  color: #e9d5ff;
}

.modal-body {
  padding: 16px 18px;
  display: flex;
  flex-direction: column;
  gap: 14px;
  overflow-y: auto;
  flex: 1;
}

.genres-row {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-wrap: wrap;
}

.genre-tag {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 4px;
  background: rgba(0, 242, 255, 0.1);
  border: 1px solid rgba(0, 242, 255, 0.25);
  color: var(--accent-cyan);
  font-weight: 500;
}

.desc-box {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.section-title {
  font-size: 12px;
  font-weight: 700;
  color: var(--text-dim);
  text-transform: uppercase;
}

.desc-text {
  font-size: 13px;
  color: #e2e8f0;
  line-height: 1.6;
}

.manifest-card {
  background: rgba(17, 24, 39, 0.6);
  border: 1px solid rgba(0, 242, 255, 0.16);
  border-radius: var(--radius-sm);
  padding: 12px 14px;
  display: flex;
  flex-direction: column;
  gap: 10px;
  transition: border-color 0.2s ease, box-shadow 0.2s ease;
}

.manifest-card:hover {
  border-color: rgba(0, 242, 255, 0.32);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
}

.manifest-card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.manifest-title-group {
  display: flex;
  align-items: center;
  gap: 10px;
}

.manifest-icon-box {
  width: 32px;
  height: 32px;
  border-radius: 6px;
  background: rgba(0, 242, 255, 0.1);
  border: 1px solid rgba(0, 242, 255, 0.25);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.manifest-main-title {
  font-size: 13px;
  font-weight: 600;
  color: #f8fafc;
}

.manifest-sub-desc {
  font-size: 11.5px;
  color: #94a3b8;
  margin-top: 1px;
}

.source-select-pill {
  display: flex;
  align-items: center;
  gap: 6px;
  background: rgba(0, 0, 0, 0.35);
  border: 1px solid rgba(255, 255, 255, 0.12);
  border-radius: 6px;
  padding: 3px 8px;
  flex-shrink: 0;
}

.source-pill-label {
  font-size: 11.5px;
  color: var(--text-dim);
}

.source-select {
  background: transparent;
  border: none;
  color: var(--accent-cyan);
  font-size: 12px;
  font-weight: 600;
  outline: none;
  cursor: pointer;
}

.source-select option {
  background: #111726;
  color: #ffffff;
}

.manifest-feature-tags {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
  padding-top: 8px;
  border-top: 1px solid rgba(255, 255, 255, 0.06);
}

.feature-tag {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 11px;
  color: #cbd5e1;
  background: rgba(255, 255, 255, 0.04);
  padding: 2px 7px;
  border-radius: 4px;
  border: 1px solid rgba(255, 255, 255, 0.06);
}

.modal-footer {
  padding: 12px 18px;
  background: rgba(0, 0, 0, 0.25);
  border-top: 1px solid var(--border-subtle);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.footer-right {
  display: flex;
  align-items: center;
  gap: 10px;
}

/* Transitions */
.slide-fade-enter-active,
.slide-fade-leave-active {
  transition: all 0.25s ease;
}
.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateY(-10px);
  opacity: 0;
}

.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: all 0.25s ease;
}
.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
  transform: scale(0.96);
}
</style>
