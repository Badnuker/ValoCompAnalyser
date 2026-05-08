<script setup lang="ts">
import LanguageSwitcher from './components/LanguageSwitcher.vue';
import { invoke } from '@tauri-apps/api/core';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { useI18n } from 'vue-i18n';
import { ref } from 'vue';

const { t } = useI18n({ useScope: 'global' });

const showImportModal = ref(false);
const importCode = ref('');

const exportConfig = async () => {
  try {
    const encoded = await invoke<string>('get_data_packed');
    await writeText(encoded);
    alert(t('ui.alert_export_clipboard'));
  } catch (error) {
    console.error(t('ui.log_export_error'), error);
    alert(t('ui.alert_export_failed', { error }));
  }
};

const importConfig = () => {
  importCode.value = '';
  showImportModal.value = true;
};

const doImport = async () => {
  const encoded = importCode.value.trim();
  if (!encoded) return;
  try {
    await invoke('import_data_packed', { encoded });
    showImportModal.value = false;
    alert(t('ui.alert_import_success'));
    window.location.reload();
  } catch (error) {
    console.error(t('ui.log_import_error'), error);
    alert(t('ui.alert_import_failed', { error }));
  }
};

const resetConfig = async () => {
  if (confirm(t('ui.confirm_reset_config'))) {
    try {
      await invoke('reset_data');
      alert(t('ui.alert_reset_success'));
      window.location.reload();
    } catch (error) {
      alert(t('ui.alert_reset_failed', { error }));
    }
  }
};
</script>

<template>
  <div class="app-container">
    <nav class="navbar">
      <div class="nav-links">
        <router-link to="/tags" class="nav-item">{{ t('ui.nav_tags') }}</router-link>
        <router-link to="/" class="nav-item nav-main">{{ t('ui.nav_main') }}</router-link>
        <router-link to="/agents" class="nav-item">{{ t('ui.nav_agents') }}</router-link>
      </div>

      <div class="config-actions">
        <LanguageSwitcher />
        <button class="config-btn" @click="importConfig" :title="t('ui.title_import_config')">📥 {{
          t('ui.import_config') }}</button>
        <button class="config-btn" @click="exportConfig" :title="t('ui.title_export_config')">📤 {{
          t('ui.export_config') }}</button>
        <button class="config-btn reset-btn" @click="resetConfig" :title="t('ui.title_reset_config')">🔄 {{
          t('ui.reset_config') }}</button>
      </div>
    </nav>

    <main class="main-content">
      <router-view />
    </main>

    <div v-if="showImportModal" class="modal-overlay" @click.self="showImportModal = false">
      <div class="modal-box">
        <h3>{{ t('ui.title_import_config') }}</h3>
        <textarea v-model="importCode" :placeholder="t('ui.prompt_import_code')" class="import-textarea" rows="5"
          autofocus></textarea>
        <div class="modal-actions">
          <button class="config-btn" @click="showImportModal = false">{{ t('ui.btn_cancel') }}</button>
          <button class="config-btn" style="background:#ff4655;color:#fff;border-color:#ff4655" @click="doImport">{{
            t('ui.btn_ok') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
/* ===== 全局基础 ===== */
*,
*::before,
*::after {
  box-sizing: border-box;
}

:root {
  --status-bar-top: 0px;
  --nav-safe-bottom: 0px;
  --color-bg: #f5f5f5;
  --color-surface: #fff;
  --color-text: #333;
  --color-text-secondary: #888;
  --color-accent: #ff4655;
  --color-border: #eee;
  --font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
}

/* 移动端状态栏 + 底部导航栏安全区 */
@supports (padding-top: env(safe-area-inset-top)) {
  :root {
    --status-bar-top: env(safe-area-inset-top);
    --nav-safe-bottom: env(safe-area-inset-bottom);
  }
}

body {
  margin: 0;
  padding-top: var(--status-bar-top);
  font-family: var(--font-family);
  background: var(--color-bg);
  color: var(--color-text);
  -webkit-tap-highlight-color: transparent;
}

.app-container {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  min-height: 100dvh;
  padding-bottom: var(--nav-safe-bottom);
}

/* ===== 顶部导航栏 ===== */
.navbar {
  position: sticky;
  top: 0;
  z-index: 50;
  background: var(--color-surface);
  padding: clamp(8px, 1.2vw, 15px) clamp(10px, 1.6vw, 20px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  display: flex;
  justify-content: center;
  align-items: center;
}

.nav-links {
  display: flex;
  gap: clamp(8px, 1.6vw, 20px);
  align-items: center;
  flex-wrap: wrap;
  justify-content: center;
}

.nav-item {
  text-decoration: none;
  color: #666;
  font-weight: 500;
  padding: clamp(6px, 0.6vw, 8px) clamp(10px, 1.2vw, 16px);
  border-radius: 8px;
  transition: all 0.2s;
  font-size: clamp(0.85rem, 0.9vw + 0.5rem, 1rem);
  white-space: nowrap;
}

.nav-item:hover {
  background: #f0f0f0;
  color: var(--color-text);
}

.router-link-active {
  background: var(--color-accent) !important;
  color: #fff !important;
}

.nav-main {
  font-size: clamp(1rem, 1vw + 0.5rem, 1.1rem);
  font-weight: bold;
}

/* ===== 右上角控制栏 ===== */
.config-actions {
  position: absolute;
  right: clamp(8px, 1.6vw, 20px);
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  gap: clamp(4px, 0.6vw, 8px);
  flex-wrap: wrap;
  align-items: center;
}

.config-btn {
  background: #f5f5f5;
  border: 1px solid #ddd;
  padding: clamp(3px, 0.4vw, 4px) clamp(6px, 0.7vw, 8px);
  border-radius: 6px;
  font-size: clamp(0.7rem, 0.7vw + 0.3rem, 0.8rem);
  cursor: pointer;
  transition: 0.2s;
  color: #555;
  font-weight: bold;
  white-space: nowrap;
}

.config-btn:hover {
  background: #e0e0e0;
  border-color: #ccc;
}

.reset-btn:hover {
  background: #fff5f5;
  color: #c62828;
  border-color: #ef9a9a;
}

/* ===== 主内容区 ===== */
.main-content {
  flex: 1;
  padding: clamp(12px, 1.6vw, 20px);
  max-width: clamp(560px, 78vw, 1000px);
  margin: clamp(8px, 1.6vw, 20px) auto;
  width: calc(100% - clamp(16px, 3vw, 40px));
  background: var(--color-surface);
  border-radius: clamp(8px, 1vw, 12px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
}

/* ===== 导入模态框 ===== */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 16px;
  padding-top: calc(16px + var(--status-bar-top));
  padding-bottom: calc(16px + var(--nav-safe-bottom));
}

.modal-box {
  background: var(--color-surface);
  border-radius: clamp(8px, 1vw, 12px);
  padding: clamp(16px, 2vw, 24px);
  width: min(92vw, 500px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
}

.modal-box h3 {
  margin: 0 0 12px 0;
  font-size: clamp(1rem, 1vw + 0.3rem, 1.1rem);
}

.import-textarea {
  width: 100%;
  box-sizing: border-box;
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 10px;
  font-size: clamp(0.75rem, 0.8vw + 0.3rem, 0.85rem);
  font-family: monospace;
  resize: vertical;
}

.modal-actions {
  display: flex;
  gap: 10px;
  justify-content: flex-end;
  margin-top: 14px;
}

/* ===== 响应式断点 ===== */

/* 中小屏：导航栏换行，操作区移到下方 */
@media (max-width: 860px) {
  .navbar {
    flex-direction: column;
    gap: clamp(6px, 1vw, 10px);
  }

  .config-actions {
    position: static;
    transform: none;
    justify-content: center;
    width: 100%;
  }
}

/* 手机竖屏：紧凑布局 */
@media (max-width: 560px) {
  .navbar {
    padding: 10px 8px;
  }

  .nav-links {
    gap: 6px;
  }

  .nav-item {
    padding: 6px 10px;
  }

  .config-actions {
    gap: 4px;
  }

  .config-btn {
    flex: 1 1 auto;
    min-width: 70px;
    text-align: center;
  }

  .config-actions :deep(.language-switcher) {
    flex: 1 1 100%;
    display: flex;
    justify-content: center;
  }
}
</style>
