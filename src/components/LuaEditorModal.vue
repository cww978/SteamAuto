<template>
  <div v-if="isOpen" class="modal-overlay" @click.self="$emit('close')">
    <div class="modal-card glass-panel">
      <!-- Modal Header -->
      <div class="modal-header">
        <div class="header-left">
          <div class="icon-chip">
            <FileCode2 :size="20" class="text-cyan" />
          </div>
          <div>
            <h2 class="modal-title">编辑 Lua 解锁脚本</h2>
            <p class="modal-subtitle font-mono">{{ gameName }} (AppID: {{ appid }}) - {{ filename }}</p>
          </div>
        </div>
        <button class="btn-close" @click="$emit('close')">
          <X :size="18" />
        </button>
      </div>

      <!-- Quick Toolbar -->
      <div class="modal-toolbar">
        <div class="toolbar-group">
          <button class="btn-tool" @click="insertTemplate('addappid')">
            <Plus :size="14" />
            <span>添加 DLC (addappid)</span>
          </button>
          <button class="btn-tool" @click="insertTemplate('manifest')">
            <Layers :size="14" />
            <span>添加 Manifest</span>
          </button>
        </div>
        <div class="toolbar-group">
          <button class="btn-tool" @click="copyCode">
            <Copy :size="14" />
            <span>{{ copied ? '已复制！' : '复制全部' }}</span>
          </button>
        </div>
      </div>

      <!-- Editor Body -->
      <div class="editor-container">
        <textarea
          ref="textareaRef"
          v-model="localContent"
          class="code-textarea font-mono"
          placeholder="-- 请输入 Lua 解锁脚本内容..."
          spellcheck="false"
        ></textarea>
      </div>

      <!-- Modal Footer -->
      <div class="modal-footer">
        <div class="footer-hint">
          <span>提示：保存后需重启 Steam 才能使新的授权或清单生效。</span>
        </div>
        <div class="footer-actions">
          <button class="btn btn-ghost" @click="$emit('close')">取消</button>
          <button class="btn btn-primary" :disabled="saving" @click="handleSave">
            <Save :size="15" />
            <span>{{ saving ? '保存中...' : '保存脚本' }}</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { FileCode2, X, Plus, Layers, Copy, Save } from 'lucide-vue-next';

const props = defineProps<{
  isOpen: boolean;
  appid: number;
  gameName: string;
  luaContent: string;
  saving?: boolean;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'save', content: string): void;
}>();

const localContent = ref('');
const copied = ref(false);
const textareaRef = ref<HTMLTextAreaElement | null>(null);

const filename = computed(() => `${props.appid}.lua`);

watch(
  () => props.luaContent,
  (val) => {
    localContent.value = val || '';
  },
  { immediate: true }
);

const insertTemplate = (type: 'addappid' | 'manifest') => {
  if (type === 'addappid') {
    localContent.value += `\naddappid(${props.appid + 1})`;
  } else if (type === 'manifest') {
    localContent.value += `\nsetManifestid(${props.appid}, "0000000000000000000")`;
  }
};

const copyCode = async () => {
  try {
    await navigator.clipboard.writeText(localContent.value);
    copied.value = true;
    setTimeout(() => {
      copied.value = false;
    }, 2000);
  } catch (err) {
    console.error('Copy failed:', err);
  }
};

const handleSave = () => {
  emit('save', localContent.value);
};
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(4, 7, 13, 0.8);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 24px;
  animation: fadeIn 0.2s ease;
}

.modal-card {
  width: 100%;
  max-width: 780px;
  height: 80vh;
  max-height: 680px;
  display: flex;
  flex-direction: column;
  background: #0d131f;
  border: 1px solid rgba(255, 255, 255, 0.12);
  border-radius: var(--radius-lg);
  box-shadow: 0 25px 60px rgba(0, 0, 0, 0.7), 0 0 30px rgba(0, 242, 255, 0.1);
  overflow: hidden;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-subtle);
  background: rgba(15, 23, 42, 0.6);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.icon-chip {
  width: 36px;
  height: 36px;
  border-radius: var(--radius-md);
  background: rgba(0, 242, 255, 0.12);
  border: 1px solid rgba(0, 242, 255, 0.25);
  display: flex;
  align-items: center;
  justify-content: center;
}

.text-cyan {
  color: var(--accent-cyan);
}

.modal-title {
  font-size: 15px;
  font-weight: 700;
  color: #ffffff;
}

.modal-subtitle {
  font-size: 11px;
  color: var(--text-dim);
}

.btn-close {
  background: transparent;
  border: none;
  color: var(--text-dim);
  cursor: pointer;
  padding: 6px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}
.btn-close:hover {
  color: #ffffff;
  background: rgba(255, 255, 255, 0.1);
}

.modal-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 16px;
  background: rgba(10, 15, 24, 0.95);
  border-bottom: 1px solid var(--border-subtle);
}

.toolbar-group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.btn-tool {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 5px 10px;
  font-size: 12px;
  color: var(--text-muted);
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all 0.2s;
}
.btn-tool:hover {
  color: #ffffff;
  background: rgba(255, 255, 255, 0.1);
  border-color: var(--border-strong);
}

.editor-container {
  flex: 1;
  display: flex;
  background: #090d15;
  position: relative;
  overflow: hidden;
}

.code-textarea {
  width: 100%;
  height: 100%;
  padding: 16px;
  background: transparent;
  color: #38bdf8;
  border: none;
  resize: none;
  font-size: 13px;
  line-height: 1.6;
  outline: none;
  font-family: 'JetBrains Mono', monospace;
  tab-size: 2;
  user-select: text;
}

.modal-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 20px;
  border-top: 1px solid var(--border-subtle);
  background: rgba(15, 23, 42, 0.7);
}

.footer-hint {
  font-size: 12px;
  color: var(--text-dim);
}

.footer-actions {
  display: flex;
  align-items: center;
  gap: 10px;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: scale(0.98);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}
</style>
