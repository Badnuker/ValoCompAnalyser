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
                <div class="avatar-container">
                    <img :src="agent.avatar_url" :alt="agent.name" class="avatar-img" />
                </div>

                <div class="agent-info">
                    <div class="agent-name">{{ $t(`agents.${agent.id}`) }}</div>
                    <div class="agent-tags">
                        <span v-for="tagId in agent.tags" :key="tagId" class="agent-tag-id">
                            {{ $t(`tags.${tagId}`) }}
                        </span>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.agent-grid {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    gap: 15px;
    margin-top: 20px;
}

@media (max-width: 768px) {
    .agent-grid {
        grid-template-columns: repeat(4, 1fr);
    }
}

@media (max-width: 500px) {
    .agent-grid {
        grid-template-columns: repeat(3, 1fr);
    }
}

.agent-card {
    background: #fdfdfd;
    border: 1px solid #eee;
    border-radius: 8px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    transition: transform 0.2s, box-shadow 0.2s;
}

.agent-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
    border-color: #ff4655;
}

.avatar-container {
    width: 100%;
    aspect-ratio: 1 / 1;
    background-color: transparent;
}

.avatar-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
}

.agent-info {
    padding: 10px;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    flex: 1;
}

.agent-name {
    font-size: 0.9rem;
    font-weight: bold;
    margin-bottom: 8px;
    color: #333;
}

.agent-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
    justify-content: center;
}

.agent-tag-id {
    font-size: 0.7rem;
    background: #f0f0f0;
    padding: 2px 6px;
    border-radius: 4px;
    color: #666;
}
</style>