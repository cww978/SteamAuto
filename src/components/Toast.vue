<template>
  <div class="toast-container">
    <transition-group name="toast-anim" tag="div" class="toast-list">
      <div
        v-for="toast in toasts"
        :key="toast.id"
        class="toast-item"
        :class="`toast-${toast.type}`"
      >
        <div class="toast-icon">
          <CheckCircle2 v-if="toast.type === 'success'" :size="18" />
          <AlertCircle v-else-if="toast.type === 'error'" :size="18" />
          <AlertTriangle v-else-if="toast.type === 'warning'" :size="18" />
          <Info v-else :size="18" />
        </div>
        <div class="toast-content">
          <div class="toast-title">{{ toast.title }}</div>
          <div v-if="toast.description" class="toast-desc">{{ toast.description }}</div>
        </div>
        <button class="toast-close" @click="removeToast(toast.id)">
          <X :size="14" />
        </button>
      </div>
    </transition-group>
  </div>
</template>

<script setup lang="ts">
import { CheckCircle2, AlertCircle, AlertTriangle, Info, X } from 'lucide-vue-next';
import type { ToastMessage } from '../types/steam';

defineProps<{
  toasts: ToastMessage[];
}>();

const emit = defineEmits<{
  (e: 'remove', id: string): void;
}>();

const removeToast = (id: string) => {
  emit('remove', id);
};
</script>

<style scoped>
.toast-container {
  position: fixed;
  bottom: 24px;
  right: 24px;
  z-index: 9999;
  pointer-events: none;
}

.toast-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.toast-item {
  pointer-events: auto;
  min-width: 300px;
  max-width: 420px;
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 12px 16px;
  background: rgba(18, 25, 40, 0.95);
  backdrop-filter: blur(16px);
  -webkit-backdrop-filter: blur(16px);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);
  color: var(--text-main);
  animation: slideIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.toast-success {
  border-color: rgba(16, 185, 129, 0.4);
  background: linear-gradient(135deg, rgba(16, 185, 129, 0.15) 0%, rgba(18, 25, 40, 0.95) 100%);
}
.toast-success .toast-icon {
  color: var(--accent-green);
}

.toast-error {
  border-color: rgba(239, 68, 68, 0.4);
  background: linear-gradient(135deg, rgba(239, 68, 68, 0.15) 0%, rgba(18, 25, 40, 0.95) 100%);
}
.toast-error .toast-icon {
  color: var(--accent-red);
}

.toast-warning {
  border-color: rgba(245, 158, 11, 0.4);
  background: linear-gradient(135deg, rgba(245, 158, 11, 0.15) 0%, rgba(18, 25, 40, 0.95) 100%);
}
.toast-warning .toast-icon {
  color: var(--accent-amber);
}

.toast-info {
  border-color: rgba(0, 242, 255, 0.4);
  background: linear-gradient(135deg, rgba(0, 242, 255, 0.15) 0%, rgba(18, 25, 40, 0.95) 100%);
}
.toast-info .toast-icon {
  color: var(--accent-cyan);
}

.toast-icon {
  flex-shrink: 0;
  margin-top: 2px;
}

.toast-content {
  flex: 1;
}

.toast-title {
  font-size: 13px;
  font-weight: 600;
  color: #ffffff;
}

.toast-desc {
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 2px;
  word-break: break-word;
}

.toast-close {
  background: transparent;
  border: none;
  color: var(--text-dim);
  cursor: pointer;
  padding: 2px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all 0.2s ease;
}
.toast-close:hover {
  color: var(--text-main);
  background: rgba(255, 255, 255, 0.1);
}

.toast-anim-enter-active,
.toast-anim-leave-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.toast-anim-enter-from {
  opacity: 0;
  transform: translateX(40px);
}

.toast-anim-leave-to {
  opacity: 0;
  transform: translateX(40px) scale(0.9);
}
</style>
