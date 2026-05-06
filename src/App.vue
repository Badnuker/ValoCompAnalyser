<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

const exportConfig = async () => {
  try {
    await invoke('export_data');
    // 如果没有报错，说明用户成功保存了（或者正常取消了）
  } catch (error) {
    if (error !== "用户取消了导出") {
      alert("导出失败：" + error);
    }
  }
};

const importConfig = async () => {
  try {
    await invoke('import_data');
    alert("导入成功！程序将刷新以应用新配置。");
    window.location.reload();
  } catch (error) {
    if (error !== "用户取消了导入") {
      alert("导入失败：" + error);
    }
  }
};

const resetConfig = async () => {
  if (confirm("危险操作 ⚠️\n这将会清除你所有的自定义标签和改动，恢复到初始默认状态。确定要继续吗？")) {
    try {
      await invoke('reset_data');
      alert("已恢复默认配置！");
      window.location.reload();
    } catch (error) {
      alert("重置失败：" + error);
    }
  }
};
</script>

<template>
  <div class="app-container">
    <nav class="navbar">
      <div class="nav-links">
        <router-link to="/tags" class="nav-item">{{ $t('ui.nav_tags') }}</router-link>
        <router-link to="/" class="nav-item nav-main">{{ $t('ui.nav_main') }}</router-link>
        <router-link to="/agents" class="nav-item">{{ $t('ui.nav_agents') }}</router-link>
      </div>

      <div class="config-actions">
        <button class="config-btn" @click="importConfig" title="导入配置文件">📥 导入</button>
        <button class="config-btn" @click="exportConfig" title="导出配置文件">📤 导出</button>
        <button class="config-btn reset-btn" @click="resetConfig" title="恢复默认配置">🔄 重置</button>
      </div>
    </nav>

    <main class="main-content">
      <router-view />
    </main>
  </div>
</template>

<style>
/* 全局基础样式重置 */
body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  background-color: #f5f5f5;
  /* 浅灰背景色 */
  color: #333;
}

.app-container {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

/* 顶部导航栏 */
.navbar {
  position: relative;
  background-color: #fff;
  padding: 15px 20px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  display: flex;
  justify-content: center;
  /* 居中中间的路由导航 */
  align-items: center;
}

.nav-links {
  display: flex;
  gap: 20px;
  align-items: center;
  flex-wrap: wrap;
  justify-content: center;
}

.nav-item {
  text-decoration: none;
  color: #666;
  font-weight: 500;
  padding: 8px 16px;
  border-radius: 8px;
  transition: all 0.2s;
}

.nav-item:hover {
  background-color: #f0f0f0;
  color: #333;
}

/* 路由激活状态 */
.router-link-active {
  background-color: #ff4655;
  /* 无畏契约红 */
  color: #fff !important;
}

.nav-main {
  font-size: 1.1rem;
  font-weight: bold;
}

/* 右上角控制按钮区 */
.config-actions {
  position: absolute;
  right: 20px;
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.config-btn {
  background: #f5f5f5;
  border: 1px solid #ddd;
  padding: 4px 8px;
  border-radius: 6px;
  font-size: 0.8rem;
  cursor: pointer;
  transition: 0.2s;
  color: #555;
  font-weight: bold;
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

/* 主视图区域 */
.main-content {
  flex: 1;
  padding: 20px;
  max-width: 1000px;
  margin: 20px auto 20px auto;
  width: 100%;
  box-sizing: border-box;
  background: #fff;
  margin-top: 20px;
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
}

/* 移动端/小窗口响应式处理 */
@media (max-width: 800px) {
  .navbar {
    flex-direction: column;
    padding-bottom: 10px;
  }

  .config-actions {
    position: static;
    transform: none;
    justify-content: center;
    margin-top: 15px;
  }

  .main-content {
    margin: 12px auto;
    padding: 14px;
    border-radius: 10px;
  }
}

@media (max-width: 560px) {
  .navbar {
    padding: 12px 10px;
  }

  .nav-links {
    gap: 8px;
  }

  .nav-item {
    padding: 6px 10px;
    font-size: 0.9rem;
  }

  .nav-main {
    font-size: 1rem;
  }

  .config-actions {
    width: 100%;
    gap: 6px;
  }

  .config-btn {
    flex: 1 1 calc(33.33% - 6px);
    min-width: 90px;
    text-align: center;
  }

  .main-content {
    margin: 8px auto;
    padding: 12px;
    border-radius: 8px;
  }
}
</style>