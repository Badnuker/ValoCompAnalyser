<script setup lang="ts">
import LanguageSwitcher from './components/LanguageSwitcher.vue';
</script>

<template>
  <div class="app-container">
    <!-- 导航栏 -->
    <nav class="navbar">
      <div class="nav-links">
        <!-- 标签管理 -->
        <router-link to="/tags" class="nav-item">{{ $t('ui.nav_tags') }}</router-link>

        <!-- 阵容分析 -->
        <router-link to="/" class="nav-item nav-main">{{ $t('ui.nav_main') }}</router-link>

        <!-- 角色管理 -->
        <router-link to="/agents" class="nav-item">{{ $t('ui.nav_agents') }}</router-link>
      </div>
      <div class="nav-extra">
        <LanguageSwitcher />
      </div>
    </nav>

    <!-- 页面内容渲染区 -->
    <main class="main-content">
      <router-view />
    </main>
  </div>
</template>

<style>
/* 基础样式 */
body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  background-color: #f5f5f7;
  color: #333;
}

.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.navbar {
  background-color: #ffffff;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  padding: 1rem 0;
}

.navbar {
  position: relative;
}

.nav-links {
  display: flex;
  justify-content: center;
  align-items: center;
  /* 确保文字大小不同步时能垂直居中 */
  gap: 3rem;
  /* 稍微拉开一点间距 */
}

.nav-item {
  text-decoration: none;
  color: #666;
  font-weight: 600;
  font-size: 1rem;
  padding-bottom: 4px;
  transition: all 0.3s;
}

/* 主界面特殊样式 */
.nav-main {
  font-size: 1.4rem;
  font-weight: 800;
}

.nav-item:hover {
  color: #ff4655;
}

/* 路由激活时的样式 */
.nav-item.router-link-active {
  color: #ff4655;
  border-bottom: 2px solid #ff4655;
}

.main-content {
  flex: 1;
  padding: 2rem;
  overflow-y: auto;
}

.page {
  max-width: 800px;
  margin: 0 auto;
  background: white;
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.03);
}

/* 简单的移动端响应式 */
@media (max-width: 600px) {
  .nav-links {
    gap: 1.5rem;
  }

  .nav-item {
    font-size: 0.9rem;
  }

  .nav-main {
    font-size: 1.2rem;
  }

  .main-content {
    padding: 1rem;
  }

  .page {
    padding: 1rem;
  }
}

.nav-extra {
  position: absolute;
  right: 1rem;
  top: 50%;
  transform: translateY(-50%);
}
</style>