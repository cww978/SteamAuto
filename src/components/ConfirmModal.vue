<template>
  <div v-if="isOpen" class="modal-overlay" @click.self="$emit('cancel')">
    <div class="confirm-card glass-panel">
      <div class="confirm-header">
        <div class="confirm-icon-box" :class="danger ? 'icon-danger' : 'icon-primary'">
          <AlertTriangle v-if="danger" :size="22" />
          <HelpCircle v-else :size="22" />
        </div>
        <div class="confirm-title-area">
          <h3 class="confirm-title">{{ title }}</h3>
          <p class="confirm-desc">{{ message }}</p>
        </div>
      </div>

      <div v-if="$slots.default" class="confirm-extra">
        <slot></slot>
      </div>

      <div class="confirm-actions">
        <button class="btn btn-ghost" @click="$emit('cancel')">
          {{ cancelText || '取消' }}
        </button>
        <button
          class="btn"
          :class="danger ? 'btn-danger' : 'btn-primary'"
          :disabled="loading"
          @click="$emit('confirm')"
        >
          <span v-if="loading">处理中...</span>
          <span v-else>{{ confirmText || '确认' }}</span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { AlertTriangle, HelpCircle } from 'lucide-vue-next';

defineProps<{
  isOpen: boolean;
  title: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
  danger?: boolean;
  loading?: boolean;
}>();

defineEmits<{
  (e: 'confirm'): void;
  (e: 'cancel'): void;
}>();
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(4, 7, 13, 0.75);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1100;
  padding: 24px;
}

.confirm-card {
  width: 100%;
  max-width: 440px;
  background: #0f1523;
  border: 1px solid rgba(255, 255, 255, 0.12);
  border-radius: var(--radius-lg);
  padding: 24px;
  box-shadow: 0 20px 50px rgba(0, 0, 0, 0.7);
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.confirm-header {
  display: flex;
  gap: 16px;
}

.confirm-icon-box {
  width: 44px;
  height: 44px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.icon-danger {
  background: rgba(239, 68, 68, 0.15);
  color: var(--accent-red);
  border: 1px solid rgba(239, 68, 68, 0.3);
}

.icon-primary {
  background: rgba(0, 242, 255, 0.15);
  color: var(--accent-cyan);
  border: 1px solid rgba(0, 242, 255, 0.3);
}

.confirm-title-area {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.confirm-title {
  font-size: 16px;
  font-weight: 700;
  color: #ffffff;
}

.confirm-desc {
  font-size: 13px;
  color: var(--text-muted);
  line-height: 1.5;
}

.confirm-extra {
  padding: 12px;
  background: rgba(0, 0, 0, 0.3);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-subtle);
}

.confirm-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}
</style>
