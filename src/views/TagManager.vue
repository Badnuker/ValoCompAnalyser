<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
import type { Tag } from '../types';

const { t, te } = useI18n();
const tags = ref<Tag[]>([]);

// 页面加载时从 Rust 后端获取数据
onMounted(async () => {
    tags.value = await invoke<Tag[]>('get_tags');
});

// 获取显示名称
const getTagName = (tag: Tag) => {
    const i18nKey = `tags.${tag.id}`;
    return te(i18nKey) ? t(i18nKey) : tag.name;
};
</script>

<template>
    <div class="page">
        <h2>{{ $t('ui.nav_tags') }}</h2>

        <div class="tag-list">
            <div v-for="tag in tags" :key="tag.id" class="tag-item">
                <div class="tag-info">
                    <span class="tag-name">{{ getTagName(tag) }}</span>
                    <!-- 关键标签高亮显示 -->
                    <span v-if="tag.is_key" class="badge key-badge">Key</span>
                    <span v-else class="badge normal-badge">Normal</span>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.tag-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin-top: 20px;
}

.tag-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: #f9f9f9;
    border-radius: 8px;
    border: 1px solid #eee;
}

.tag-name {
    font-weight: 600;
    margin-right: 12px;
}

.badge {
    font-size: 0.8rem;
    padding: 2px 8px;
    border-radius: 12px;
    font-weight: bold;
}

.key-badge {
    background-color: #ff4655;
    color: white;
}

.normal-badge {
    background-color: #ccc;
    color: white;
}
</style>