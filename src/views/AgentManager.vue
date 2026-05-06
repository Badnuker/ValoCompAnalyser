<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Agent } from '../types';

const agents = ref<Agent[]>([]);

onMounted(async () => {
    agents.value = await invoke<Agent[]>('get_agents');
});
</script>

<template>
    <div class="page">
        <h2>{{ $t('ui.nav_agents') }}</h2>

        <div class="agent-grid">
            <div v-for="agent in agents" :key="agent.id" class="agent-card">
                <!-- 暂时没有图片，先用个占位圆圈 -->
                <div class="avatar-placeholder"></div>
                <div class="agent-name">{{ $t(`agents.${agent.id}`) }}</div>

                <div class="agent-tags">
                    <span v-for="tagId in agent.tags" :key="tagId" class="agent-tag-id">
                        {{ $t(`tags.${tagId}`) }}
                    </span>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.agent-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    gap: 20px;
    margin-top: 20px;
}

.agent-card {
    background: #f9f9f9;
    border: 1px solid #eee;
    border-radius: 12px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
}

.avatar-placeholder {
    width: 64px;
    height: 64px;
    background-color: #ddd;
    border-radius: 50%;
    margin-bottom: 12px;
}

.agent-name {
    font-weight: bold;
    margin-bottom: 8px;
}

.agent-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
    justify-content: center;
}

.agent-tag-id {
    font-size: 0.75rem;
    background: #e0e0e0;
    padding: 2px 6px;
    border-radius: 4px;
    color: #555;
}
</style>