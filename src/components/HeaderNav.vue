<template>
  <header class="header-container glass-panel">
    <!-- Brand / Logo Area -->
    <div class="brand-area">
      <div class="logo-box">
        <img v-if="!showFallbackIcon" src="/icon.png" class="logo-img" alt="SteamAuto" @error="showFallbackIcon = true" />
        <Gamepad2 v-else class="logo-icon text-cyan" :size="18" />
        <div class="logo-glow"></div>
      </div>
      <div class="brand-info">
        <h1 class="brand-title">SteamAuto</h1>
      </div>
    </div>

    <!-- Navigation Tabs -->
    <nav class="nav-tabs">
      <button
        class="nav-tab-btn"
        :class="{ active: currentTab === 'library' }"
        @click="$emit('update:currentTab', 'library')"
      >
        <Sparkles :size="14" />
        <span>已入库游戏</span>
        <span v-if="gameCount > 0" class="nav-badge">{{ gameCount }}</span>
      </button>

      <button
        class="nav-tab-btn"
        :class="{ active: currentTab === 'add' }"
        @click="$emit('update:currentTab', 'add')"
      >
        <PlusCircle :size="14" />
        <span>添加游戏</span>
      </button>

      <button
        class="nav-tab-btn"
        :class="{ active: currentTab === 'accounts' }"
        @click="$emit('update:currentTab', 'accounts')"
      >
        <Users :size="14" />
        <span>账号管理</span>
      </button>

      <button
        class="nav-tab-btn"
        :class="{ active: currentTab === 'settings' }"
        @click="$emit('update:currentTab', 'settings')"
      >
        <Settings :size="14" />
        <span>设置与工具</span>
      </button>
    </nav>

    <!-- Right Controls Area -->
    <div class="header-controls">
      <!-- Live Steam Process Indicator -->
      <div class="steam-status-chip" :class="steamStatus.is_running ? 'running' : 'stopped'">
        <span class="status-dot"></span>
        <span class="status-text">
          {{ steamStatus.is_running ? `Steam 运行中 ${steamStatus.memory_mb ? '(' + Math.round(steamStatus.memory_mb) + 'M)' : ''}` : 'Steam 未运行' }}
        </span>
      </div>

      <!-- Unlock Mode Switch -->
      <div
        class="unlock-switch-box"
        :class="{ 'is-active': isUnlockActive }"
        @click="$emit('toggle-unlock')"
        :title="isUnlockActive ? '点击关闭解锁模式（切换至官方纯净模式）' : '点击开启解锁模式（激活假入库与DLC）'"
      >
        <div class="switch-glow"></div>
        <div class="switch-icon-area">
          <ShieldCheck v-if="isUnlockActive" :size="14" class="text-emerald" />
          <ShieldAlert v-else :size="14" class="text-muted" />
        </div>
        <div class="switch-label-area">
          <span class="switch-title">{{ isUnlockActive ? '解锁模式 已开启' : '解锁模式 已关闭' }}</span>
        </div>
        <div class="mini-toggle" :class="{ active: isUnlockActive }">
          <span class="mini-thumb"></span>
        </div>
      </div>

      <!-- Quick Restart Steam Button -->
      <button
        class="btn btn-steam btn-restart"
        :disabled="isRestarting"
        @click="$emit('restart-steam')"
        title="快速重启 Steam 客户端"
      >
        <RotateCw :size="13" :class="{ 'animate-spin': isRestarting }" />
        <span>{{ isRestarting ? '重启中...' : '重启 Steam' }}</span>
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import {
  Gamepad2,
  Sparkles,
  PlusCircle,
  Users,
  Settings,
  ShieldCheck,
  ShieldAlert,
  RotateCw,
} from 'lucide-vue-next';
import type { ActiveTab, SteamProcessStatus } from '../types/steam';

const showFallbackIcon = ref(false);

defineProps<{
  currentTab: ActiveTab;
  isUnlockActive: boolean;
  steamStatus: SteamProcessStatus;
  gameCount: number;
  isRestarting: boolean;
}>();

defineEmits<{
  (e: 'update:currentTab', tab: ActiveTab): void;
  (e: 'toggle-unlock'): void;
  (e: 'restart-steam'): void;
}>();
</script>

<style scoped>
.header-container {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 14px;
  background: rgba(13, 18, 28, 0.9);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-bottom: 1px solid var(--border-subtle);
  border-radius: 0;
  z-index: 50;
  gap: 12px;
  min-height: 48px;
}

/* Brand */
.brand-area {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

.logo-box {
  position: relative;
  width: 30px;
  height: 30px;
  border-radius: var(--radius-sm);
  background: linear-gradient(135deg, #ffffff 0%, #e2e8f0 100%);
  border: 1px solid rgba(255, 255, 255, 0.9);
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3), 0 0 12px rgba(255, 255, 255, 0.2);
}

.logo-img {
  width: 24px;
  height: 24px;
  object-fit: contain;
  border-radius: 4px;
}

.logo-icon {
  color: var(--accent-cyan);
  filter: drop-shadow(0 0 6px rgba(0, 242, 255, 0.6));
}

.brand-info {
  display: flex;
  flex-direction: column;
}

.brand-title {
  font-size: 14px;
  font-weight: 800;
  letter-spacing: -0.3px;
  background: linear-gradient(135deg, #ffffff 0%, #00f2ff 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  line-height: 1.1;
}

.brand-subtitle {
  font-size: 9.5px;
  color: var(--text-dim);
  font-weight: 500;
}

/* Navigation Tabs */
.nav-tabs {
  display: flex;
  align-items: center;
  background: rgba(15, 23, 42, 0.6);
  padding: 3px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-subtle);
  gap: 2px;
}

.nav-tab-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 5px 10px;
  font-size: 12px;
  font-weight: 600;
  color: var(--text-muted);
  background: transparent;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s ease;
  white-space: nowrap;
}

.nav-tab-btn:hover {
  color: var(--text-main);
  background: rgba(255, 255, 255, 0.06);
}

.nav-tab-btn.active {
  color: #ffffff;
  background: linear-gradient(135deg, rgba(0, 242, 255, 0.2) 0%, rgba(26, 159, 255, 0.25) 100%);
  border: 1px solid rgba(0, 242, 255, 0.35);
  box-shadow: 0 0 10px rgba(0, 242, 255, 0.15);
}

.nav-badge {
  font-size: 9.5px;
  padding: 1px 5px;
  border-radius: 9999px;
  background: var(--accent-cyan);
  color: #080b10;
  font-weight: 700;
  line-height: 1.2;
}

/* Header Controls */
.header-controls {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

/* Steam Status Chip */
.steam-status-chip {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 9px;
  border-radius: var(--radius-full);
  font-size: 11px;
  font-weight: 600;
  background: rgba(15, 23, 42, 0.6);
  border: 1px solid var(--border-subtle);
  white-space: nowrap;
}

.status-dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
}

.steam-status-chip.running .status-dot {
  background: var(--accent-green);
  box-shadow: 0 0 6px var(--accent-green);
  animation: pulse-glow 2s infinite ease-in-out;
}
.steam-status-chip.running {
  color: #a7f3d0;
  border-color: rgba(16, 185, 129, 0.3);
}

.steam-status-chip.stopped .status-dot {
  background: var(--text-dim);
}
.steam-status-chip.stopped {
  color: var(--text-dim);
}

/* Unlock Switch Box */
.unlock-switch-box {
  display: flex;
  align-items: center;
  gap: 7px;
  padding: 4px 10px;
  border-radius: var(--radius-sm);
  background: rgba(15, 23, 42, 0.7);
  border: 1px solid var(--border-subtle);
  cursor: pointer;
  transition: all 0.25s ease;
  user-select: none;
  white-space: nowrap;
}

.unlock-switch-box:hover {
  border-color: var(--border-strong);
  background: rgba(26, 36, 56, 0.8);
}

.unlock-switch-box.is-active {
  background: linear-gradient(135deg, rgba(16, 185, 129, 0.15) 0%, rgba(6, 78, 59, 0.25) 100%);
  border-color: rgba(16, 185, 129, 0.4);
  box-shadow: 0 0 12px rgba(16, 185, 129, 0.2);
}

.switch-title {
  font-size: 11px;
  font-weight: 700;
  color: var(--text-main);
}

.unlock-switch-box.is-active .switch-title {
  color: #34d399;
}

.mini-toggle {
  width: 28px;
  height: 15px;
  border-radius: 9999px;
  background: rgba(255, 255, 255, 0.15);
  position: relative;
  transition: all 0.25s ease;
}

.mini-toggle.active {
  background: var(--accent-green);
  box-shadow: 0 0 6px var(--accent-green);
}

.mini-thumb {
  position: absolute;
  top: 1.5px;
  left: 2px;
  width: 12px;
  height: 12px;
  background: #ffffff;
  border-radius: 50%;
  transition: transform 0.25s ease;
}

.mini-toggle.active .mini-thumb {
  transform: translateX(12px);
}

.text-emerald {
  color: var(--accent-green);
}

.btn-restart {
  font-size: 11px;
  padding: 4px 10px;
  gap: 5px;
}
</style>
