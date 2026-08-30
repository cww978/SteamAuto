<template>
  <div class="accounts-container">
    <!-- Header Summary -->
    <div class="summary-card glass-panel">
      <div class="summary-left">
        <div class="avatar-ring">
          <UserCheck :size="28" class="text-cyan" />
        </div>
        <div>
          <div class="summary-title">Steam 多账号快捷切换</div>
          <div class="summary-desc">
            从 Steam 本地凭据配置文件（loginusers.vdf）中读取已保存的账号，一键免密切换。
          </div>
        </div>
      </div>

      <div class="summary-actions">
        <label class="auto-restart-label">
          <input v-model="autoRestart" type="checkbox" class="checkbox-custom" />
          <span>切换后自动重启 Steam 生效</span>
        </label>

        <button class="btn btn-ghost" @click="$emit('refresh')">
          <RefreshCw :size="14" :class="{ 'animate-spin': loading }" />
          <span>刷新列表</span>
        </button>

        <button class="btn btn-danger" @click="$emit('logout')">
          <LogOut :size="14" />
          <span>退出当前账号自动登录</span>
        </button>
      </div>
    </div>

    <!-- Accounts Grid -->
    <div v-if="accounts.length > 0" class="accounts-grid">
      <div
        v-for="account in accounts"
        :key="account.steam_id64"
        class="account-card glass-card"
        :class="{ 'active-account': account.is_active }"
      >
        <!-- Top Status Banner -->
        <div class="account-card-header">
          <div class="avatar-box">
            <div class="avatar-inner font-mono">
              {{ account.persona_name ? account.persona_name[0].toUpperCase() : 'U' }}
            </div>
            <div v-if="account.is_active" class="active-badge-dot"></div>
          </div>

          <div class="account-names">
            <div class="persona-name" :title="account.persona_name">
              {{ account.persona_name || 'Steam 用户' }}
            </div>
            <div class="account-name font-mono">
              账号: {{ account.account_name }}
            </div>
          </div>

          <div v-if="account.is_active" class="badge badge-green">
            <CheckCircle2 :size="12" />
            <span>当前登录</span>
          </div>
        </div>

        <!-- Details Info -->
        <div class="account-details">
          <div class="detail-item">
            <span class="detail-label">SteamID64:</span>
            <span class="detail-val font-mono">{{ account.steam_id64 }}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">最后登录:</span>
            <span class="detail-val">{{ account.last_login_formatted }}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">记住密码:</span>
            <span class="detail-val" :class="account.remember_password ? 'text-emerald' : 'text-dim'">
              {{ account.remember_password ? '已保存凭据' : '未保存' }}
            </span>
          </div>
        </div>

        <!-- Footer Action -->
        <div class="account-footer">
          <button
            v-if="!account.is_active"
            class="btn btn-primary btn-switch"
            :disabled="switchingId === account.steam_id64"
            @click="handleSwitch(account)"
          >
            <Repeat :size="15" :class="{ 'animate-spin': switchingId === account.steam_id64 }" />
            <span>{{ switchingId === account.steam_id64 ? '切换中...' : '切换至此账号' }}</span>
          </button>
          
          <div v-else class="active-status-notice">
            <Check :size="14" class="text-emerald" />
            <span class="text-emerald font-semibold">此账号正在使用中</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="empty-state glass-panel">
      <Users :size="48" class="text-dim" />
      <h3 class="empty-title">未检测到已保存的 Steam 账号</h3>
      <p class="empty-desc">请先使用 Steam 官方客户端登录并勾选“记住密码”。</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import {
  UserCheck,
  RefreshCw,
  LogOut,
  CheckCircle2,
  Repeat,
  Check,
  Users,
} from 'lucide-vue-next';
import type { SteamAccount } from '../types/steam';

const props = defineProps<{
  accounts: SteamAccount[];
  loading: boolean;
  switchingId?: string | null;
}>();

const emit = defineEmits<{
  (e: 'refresh'): void;
  (e: 'logout'): void;
  (e: 'switch-account', account: SteamAccount, autoRestart: boolean): void;
}>();

const autoRestart = ref(true);

const handleSwitch = (account: SteamAccount) => {
  emit('switch-account', account, autoRestart.value);
};
</script>

<style scoped>
.accounts-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 10px 14px;
  height: calc(100vh - 48px);
  overflow-y: auto;
}

/* Summary Card */
.summary-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  gap: 12px;
  border-radius: var(--radius-sm);
}

.summary-left {
  display: flex;
  align-items: center;
  gap: 10px;
}

.avatar-ring {
  width: 34px;
  height: 34px;
  border-radius: 50%;
  background: rgba(0, 242, 255, 0.1);
  border: 1px solid rgba(0, 242, 255, 0.25);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.text-cyan {
  color: var(--accent-cyan);
}

.summary-title {
  font-size: 13px;
  font-weight: 700;
  color: #ffffff;
}

.summary-desc {
  font-size: 11px;
  color: var(--text-dim);
  margin-top: 1px;
}

.summary-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.auto-restart-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
  color: var(--text-secondary);
  cursor: pointer;
  background: rgba(15, 23, 42, 0.6);
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-subtle);
}

.checkbox-custom {
  accent-color: var(--accent-cyan);
  cursor: pointer;
}

/* Accounts Grid */
.accounts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 10px;
}

.account-card {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 12px 14px;
  background: #111726;
  border-radius: var(--radius-sm);
  border: 1px solid rgba(255, 255, 255, 0.08);
  position: relative;
}

.account-card.active-account {
  background: linear-gradient(135deg, rgba(16, 185, 129, 0.1) 0%, #111726 100%);
  border-color: rgba(16, 185, 129, 0.4);
  box-shadow: 0 0 20px rgba(16, 185, 129, 0.15);
}

.account-card-header {
  display: flex;
  align-items: center;
  gap: 12px;
}

.avatar-box {
  position: relative;
  width: 44px;
  height: 44px;
}

.avatar-inner {
  width: 100%;
  height: 100%;
  border-radius: 50%;
  background: linear-gradient(135deg, #1b2838 0%, #2a475e 100%);
  border: 1px solid rgba(102, 192, 244, 0.3);
  color: #66c0f4;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  font-weight: 800;
}

.active-badge-dot {
  position: absolute;
  bottom: 0;
  right: 0;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: var(--accent-green);
  border: 2px solid #111726;
  box-shadow: 0 0 8px var(--accent-green);
}

.account-names {
  flex: 1;
  overflow: hidden;
}

.persona-name {
  font-size: 14px;
  font-weight: 700;
  color: #ffffff;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.account-name {
  font-size: 12px;
  color: var(--text-dim);
}

.account-details {
  display: flex;
  flex-direction: column;
  gap: 6px;
  background: rgba(10, 15, 24, 0.6);
  padding: 10px 12px;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-subtle);
}

.detail-item {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
}

.detail-label {
  color: var(--text-dim);
}

.detail-val {
  color: var(--text-secondary);
}

.text-emerald {
  color: var(--accent-green);
}

.account-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  margin-top: auto;
}

.btn-switch {
  width: 100%;
}

.active-status-notice {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  width: 100%;
  padding: 8px;
  font-size: 12px;
}

.font-semibold {
  font-weight: 600;
}

/* Empty */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  text-align: center;
  gap: 12px;
}

.empty-title {
  font-size: 16px;
  font-weight: 700;
  color: #ffffff;
}

.empty-desc {
  font-size: 13px;
  color: var(--text-dim);
}
</style>
