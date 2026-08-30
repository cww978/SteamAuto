<template>
  <div class="library-container">
    <!-- Stats Top Banner -->
    <div class="stats-grid">
      <div class="stat-card glass-card">
        <div class="stat-icon-box bg-cyan-glow">
          <Gamepad2 :size="20" class="text-cyan" />
        </div>
        <div class="stat-content">
          <div class="stat-value">{{ games.length }}</div>
          <div class="stat-label">已假入库游戏</div>
        </div>
      </div>

      <div class="stat-card glass-card">
        <div class="stat-icon-box bg-purple-glow">
          <Layers :size="20" class="text-purple" />
        </div>
        <div class="stat-content">
          <div class="stat-value">{{ totalDlcs }}</div>
          <div class="stat-label">已解锁 DLC 数量</div>
        </div>
      </div>

      <div class="stat-card glass-card">
        <div class="stat-icon-box bg-emerald-glow">
          <FileCheck2 :size="20" class="text-emerald" />
        </div>
        <div class="stat-content">
          <div class="stat-value">{{ totalManifests }}</div>
          <div class="stat-label">已部署 Manifest 清单</div>
        </div>
      </div>

      <div class="stat-card glass-card">
        <div class="stat-icon-box" :class="isUnlockActive ? 'bg-emerald-glow' : 'bg-amber-glow'">
          <ShieldCheck v-if="isUnlockActive" :size="20" class="text-emerald" />
          <ShieldAlert v-else :size="20" class="text-amber" />
        </div>
        <div class="stat-content">
          <div class="stat-value" :class="isUnlockActive ? 'text-emerald' : 'text-amber'">
            {{ isUnlockActive ? '解锁已生效' : '原生纯净模式' }}
          </div>
          <div class="stat-label">当前运行模式</div>
        </div>
      </div>
    </div>

    <!-- Filter & Search Toolbar -->
    <div class="toolbar glass-panel">
      <div class="search-box">
        <Search :size="16" class="search-icon" />
        <input
          v-model="searchQuery"
          type="text"
          class="input-text search-input"
          placeholder="搜索游戏名称、AppID 或 DLC..."
        />
        <button v-if="searchQuery" class="clear-search" @click="searchQuery = ''">
          <X :size="14" />
        </button>
      </div>

      <div class="filter-tabs">
        <button
          class="filter-btn"
          :class="{ active: filterType === 'all' }"
          @click="filterType = 'all'"
        >
          全部 ({{ games.length }})
        </button>
        <button
          class="filter-btn"
          :class="{ active: filterType === 'manifest' }"
          @click="filterType = 'manifest'"
        >
          含清单文件 ({{ manifestGameCount }})
        </button>
        <button
          class="filter-btn"
          :class="{ active: filterType === 'dlc' }"
          @click="filterType = 'dlc'"
        >
          含 DLC ({{ dlcGameCount }})
        </button>
      </div>

      <div class="toolbar-actions">
        <!-- Grid / List Switch -->
        <div class="view-switch">
          <button
            class="view-btn"
            :class="{ active: viewMode === 'grid' }"
            @click="viewMode = 'grid'"
            title="网格视图"
          >
            <LayoutGrid :size="16" />
          </button>
          <button
            class="view-btn"
            :class="{ active: viewMode === 'list' }"
            @click="viewMode = 'list'"
            title="列表视图"
          >
            <List :size="16" />
          </button>
        </div>

        <button class="btn btn-ghost btn-refresh" :disabled="loading" @click="$emit('refresh')">
          <RefreshCw :size="15" :class="{ 'animate-spin': loading }" />
          <span>刷新</span>
        </button>

        <button class="btn btn-primary" @click="$emit('go-add')">
          <Plus :size="15" />
          <span>添加游戏</span>
        </button>
      </div>
    </div>

    <!-- Games Grid View -->
    <div v-if="filteredGames.length > 0 && viewMode === 'grid'" class="games-grid">
      <div
        v-for="game in filteredGames"
        :key="game.appid"
        class="game-card glass-card"
      >
        <!-- Header / Banner Image -->
        <div class="game-cover-box">
          <img
            :src="`https://cdn.akamai.steamstatic.com/steam/apps/${game.appid}/header.jpg`"
            class="game-cover-img"
            :alt="game.name"
            loading="lazy"
            @error="onImageError($event, game.appid)"
          />
          <div class="cover-gradient"></div>
          <div class="game-appid-tag font-mono">ID: {{ game.appid }}</div>
          
          <!-- Launch Quick Overlay Button -->
          <button
            class="quick-launch-btn"
            @click="$emit('launch-game', game.appid)"
            title="直接通过 Steam 启动游戏"
          >
            <Play :size="18" fill="currentColor" />
          </button>
        </div>

        <!-- Card Content -->
        <div class="game-info">
          <div class="game-title" :title="game.name">{{ game.name }}</div>

          <div class="game-meta-tags">
            <span v-if="game.dlcs.length > 0" class="badge badge-purple" :title="`已解锁 ${game.dlcs.length} 个 DLC 扩展包`">
              <Layers :size="11" />
              <span>{{ game.dlcs.length }} DLC</span>
            </span>
            <span v-if="game.manifest_count > 0" class="badge badge-green" :title="`已部署 ${game.manifest_count} 个 Manifest 清单文件与解密密钥，Steam 客户端内可直接完整下载`">
              <FileCheck2 :size="11" />
              <span>{{ game.manifest_count }} 清单 (可下载)</span>
            </span>
            <span v-else class="badge badge-amber" title="当前仅包含基础所有权入库脚本，缺少 Manifest 清单文件与解密密钥，如需 Steam 下载请导入下载清单压缩包">
              <Code2 :size="11" />
              <span>仅入库 (需清单)</span>
            </span>
          </div>

          <div class="game-footer">
            <span class="game-date">{{ game.updated_at }}</span>
            <div class="game-actions">
              <button
                class="icon-action-btn"
                @click="$emit('edit-lua', game)"
                title="查看/编辑 Lua 脚本"
              >
                <Edit3 :size="15" />
              </button>
              <button
                class="icon-action-btn"
                @click="openSteamStore(game.appid)"
                title="在浏览器中查看 Steam 商店页面"
              >
                <ExternalLink :size="15" />
              </button>
              <button
                class="icon-action-btn btn-action-delete"
                @click="$emit('delete-game', game)"
                title="从假入库中移除"
              >
                <Trash2 :size="15" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Games List View -->
    <div v-else-if="filteredGames.length > 0 && viewMode === 'list'" class="games-list-view glass-panel">
      <table class="games-table">
        <thead>
          <tr>
            <th style="width: 80px;">封面</th>
            <th>游戏名称</th>
            <th style="width: 110px;">AppID</th>
            <th style="width: 140px;">扩展 / 清单</th>
            <th style="width: 150px;">更新时间</th>
            <th style="width: 180px; text-align: right;">操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="game in filteredGames" :key="game.appid" class="table-row">
            <td>
              <img
                :src="`https://cdn.akamai.steamstatic.com/steam/apps/${game.appid}/capsule_sm_120.jpg`"
                class="table-thumb"
                :alt="game.name"
                @error="onImageError($event, game.appid)"
              />
            </td>
            <td>
              <div class="table-game-name">{{ game.name }}</div>
            </td>
            <td>
              <span class="font-mono table-appid">{{ game.appid }}</span>
            </td>
            <td>
              <div class="table-tags">
                <span v-if="game.dlcs.length > 0" class="badge badge-purple">{{ game.dlcs.length }} DLC</span>
                <span v-if="game.manifest_count > 0" class="badge badge-green">{{ game.manifest_count }} 清单 (可下载)</span>
                <span v-else class="badge badge-amber">仅入库 (需清单)</span>
              </div>
            </td>
            <td class="text-dim text-sm">{{ game.updated_at }}</td>
            <td style="text-align: right;">
              <div class="table-actions">
                <button
                  class="btn btn-steam btn-sm"
                  @click="$emit('launch-game', game.appid)"
                >
                  <Play :size="13" fill="currentColor" />
                  <span>启动</span>
                </button>
                <button class="icon-action-btn" @click="$emit('edit-lua', game)" title="编辑脚本">
                  <Edit3 :size="14" />
                </button>
                <button class="icon-action-btn" @click="openSteamStore(game.appid)" title="商店">
                  <ExternalLink :size="14" />
                </button>
                <button class="icon-action-btn btn-action-delete" @click="$emit('delete-game', game)" title="删除">
                  <Trash2 :size="14" />
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Empty State -->
    <div v-else class="empty-state glass-panel">
      <div class="empty-icon-box">
        <Sparkles :size="36" class="text-cyan" />
      </div>
      <h3 class="empty-title">
        {{ searchQuery ? '未找到匹配的游戏' : '假入库暂无游戏' }}
      </h3>
      <p class="empty-desc">
        {{ searchQuery ? '请尝试更换搜索关键词' : '可通过导入下载清单压缩包（.zip）或粘贴 Steam 商店链接快速入库' }}
      </p>
      <button v-if="!searchQuery" class="btn btn-primary" @click="$emit('go-add')">
        <Plus :size="16" />
        <span>立即入库第一款游戏</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import {
  Gamepad2,
  Layers,
  FileCheck2,
  ShieldCheck,
  ShieldAlert,
  Search,
  X,
  LayoutGrid,
  List,
  RefreshCw,
  Plus,
  Play,
  Edit3,
  ExternalLink,
  Trash2,
  Code2,
  Sparkles,
} from 'lucide-vue-next';
import type { GameItem } from '../types/steam';

const props = defineProps<{
  games: GameItem[];
  isUnlockActive: boolean;
  loading: boolean;
}>();

defineEmits<{
  (e: 'refresh'): void;
  (e: 'go-add'): void;
  (e: 'edit-lua', game: GameItem): void;
  (e: 'delete-game', game: GameItem): void;
  (e: 'launch-game', appid: number): void;
}>();

const searchQuery = ref('');
const filterType = ref<'all' | 'manifest' | 'dlc'>('all');
const viewMode = ref<'grid' | 'list'>('grid');

const totalDlcs = computed(() => {
  return props.games.reduce((acc, g) => acc + g.dlcs.length, 0);
});

const totalManifests = computed(() => {
  return props.games.reduce((acc, g) => acc + g.manifest_count, 0);
});

const manifestGameCount = computed(() => {
  return props.games.filter((g) => g.manifest_count > 0).length;
});

const dlcGameCount = computed(() => {
  return props.games.filter((g) => g.dlcs.length > 0).length;
});

const filteredGames = computed(() => {
  return props.games.filter((game) => {
    // Search query
    const q = searchQuery.value.trim().toLowerCase();
    const matchesSearch =
      !q ||
      game.name.toLowerCase().includes(q) ||
      game.appid.toString().includes(q) ||
      game.dlcs.some((d) => d.toString().includes(q));

    if (!matchesSearch) return false;

    // Filter type
    if (filterType.value === 'manifest') {
      return game.manifest_count > 0;
    } else if (filterType.value === 'dlc') {
      return game.dlcs.length > 0;
    }

    return true;
  });
});

const onImageError = (event: Event, appid: number) => {
  const img = event.target as HTMLImageElement;
  img.src = `https://cdn.akamai.steamstatic.com/steam/apps/${appid}/capsule_616x353.jpg`;
};

const openSteamStore = (appid: number) => {
  window.open(`https://store.steampowered.com/app/${appid}`, '_blank');
};
</script>

<style scoped>
.library-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 10px 14px;
  height: calc(100vh - 48px);
  overflow-y: auto;
}

/* Stats Grid */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 10px;
}

.stat-card {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 12px;
  border-radius: var(--radius-sm);
}

.stat-icon-box {
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

.bg-purple-glow {
  background: rgba(168, 85, 247, 0.12);
  border: 1px solid rgba(168, 85, 247, 0.3);
}

.bg-emerald-glow {
  background: rgba(16, 185, 129, 0.12);
  border: 1px solid rgba(16, 185, 129, 0.3);
}

.bg-amber-glow {
  background: rgba(245, 158, 11, 0.12);
  border: 1px solid rgba(245, 158, 11, 0.3);
}

.text-cyan { color: var(--accent-cyan); }
.text-purple { color: var(--accent-purple); }
.text-emerald { color: var(--accent-green); }
.text-amber { color: var(--accent-amber); }

.stat-value {
  font-size: 15px;
  font-weight: 800;
  color: #ffffff;
  line-height: 1.2;
}

.stat-label {
  font-size: 10.5px;
  color: var(--text-dim);
  margin-top: 1px;
}

/* Toolbar */
.toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 12px;
  gap: 10px;
  border-radius: var(--radius-sm);
}

.search-box {
  position: relative;
  width: 220px;
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
  padding-left: 28px;
  padding-right: 26px;
  height: 28px;
  font-size: 11.5px;
}

.clear-search {
  position: absolute;
  right: 8px;
  background: transparent;
  border: none;
  color: var(--text-dim);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

.filter-tabs {
  display: flex;
  align-items: center;
  background: rgba(15, 23, 42, 0.6);
  padding: 2px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-subtle);
  gap: 2px;
}

.filter-btn {
  background: transparent;
  border: none;
  padding: 3px 8px;
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all 0.2s;
  white-space: nowrap;
}
.filter-btn:hover {
  color: var(--text-main);
}
.filter-btn.active {
  background: rgba(255, 255, 255, 0.1);
  color: #ffffff;
}

.toolbar-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.view-switch {
  display: flex;
  align-items: center;
  background: rgba(15, 23, 42, 0.6);
  padding: 2px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-subtle);
}

.view-btn {
  background: transparent;
  border: none;
  padding: 4px 6px;
  color: var(--text-dim);
  border-radius: var(--radius-sm);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}
.view-btn.active {
  background: rgba(255, 255, 255, 0.12);
  color: #ffffff;
}

/* Games Grid */
.games-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(210px, 1fr));
  gap: 10px;
  padding-bottom: 14px;
}

.game-card {
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border-radius: var(--radius-sm);
  background: #111726;
  border: 1px solid rgba(255, 255, 255, 0.08);
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
}

.game-card:hover .game-cover-img {
  transform: scale(1.05);
}

.cover-gradient {
  position: absolute;
  inset: 0;
  background: linear-gradient(180deg, transparent 40%, rgba(17, 23, 38, 0.9) 100%);
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

.quick-launch-btn {
  position: absolute;
  right: 8px;
  bottom: 8px;
  width: 28px;
  height: 28px;
  border-radius: 50%;
  background: linear-gradient(135deg, #00f2ff 0%, #0077ff 100%);
  border: none;
  color: #0d121c;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  box-shadow: 0 2px 10px rgba(0, 242, 255, 0.5);
  opacity: 0;
  transform: scale(0.8);
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

.game-card:hover .quick-launch-btn {
  opacity: 1;
  transform: scale(1);
}

.quick-launch-btn:hover {
  transform: scale(1.1) !important;
  box-shadow: 0 4px 14px rgba(0, 242, 255, 0.7);
}

.game-info {
  padding: 8px 10px;
  display: flex;
  flex-direction: column;
  gap: 6px;
  flex: 1;
}

.game-title {
  font-size: 12px;
  font-weight: 700;
  color: #ffffff;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.game-meta-tags {
  display: flex;
  align-items: center;
  gap: 4px;
  flex-wrap: wrap;
}

.game-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: auto;
  padding-top: 6px;
  border-top: 1px solid var(--border-subtle);
}

.game-date {
  font-size: 10px;
  color: var(--text-dim);
}

.game-actions {
  display: flex;
  align-items: center;
  gap: 2px;
}

.icon-action-btn {
  background: transparent;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}
.icon-action-btn:hover {
  color: #ffffff;
  background: rgba(255, 255, 255, 0.1);
}

.btn-action-delete:hover {
  color: var(--accent-red);
  background: rgba(239, 68, 68, 0.15);
}

/* List View Table */
.games-list-view {
  overflow-x: auto;
}

.games-table {
  width: 100%;
  border-collapse: collapse;
  text-align: left;
}

.games-table th {
  padding: 12px 16px;
  font-size: 12px;
  color: var(--text-dim);
  font-weight: 600;
  border-bottom: 1px solid var(--border-subtle);
}

.games-table td {
  padding: 10px 16px;
  font-size: 13px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.04);
}

.table-row:hover td {
  background: rgba(255, 255, 255, 0.03);
}

.table-thumb {
  width: 64px;
  height: 30px;
  object-fit: cover;
  border-radius: 4px;
  background: #090d15;
}

.table-game-name {
  font-weight: 600;
  color: #ffffff;
}

.table-appid {
  font-size: 12px;
  color: var(--text-muted);
}

.table-tags {
  display: flex;
  align-items: center;
  gap: 6px;
}

.table-actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 6px;
}

.btn-sm {
  padding: 5px 10px;
  font-size: 12px;
}

.text-sm {
  font-size: 12px;
}

/* Empty State */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  text-align: center;
  gap: 12px;
}

.empty-icon-box {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  background: rgba(0, 242, 255, 0.1);
  border: 1px solid rgba(0, 242, 255, 0.25);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 8px;
}

.empty-title {
  font-size: 16px;
  font-weight: 700;
  color: #ffffff;
}

.empty-desc {
  font-size: 13px;
  color: var(--text-muted);
  max-width: 400px;
}
</style>
