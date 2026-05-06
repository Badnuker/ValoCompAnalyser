<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
import type { Tag, Agent } from '../types';

const { t, te } = useI18n();

const allTags = ref<Tag[]>([]);
const allAgents = ref<Agent[]>([]);
const selectedAgents = ref<(Agent | null)[]>([null, null, null, null, null]);
const showSelector = ref(false);
const currentSlotIndex = ref(0);

onMounted(async () => {
    allTags.value = await invoke<Tag[]>('get_tags');
    allAgents.value = await invoke<Agent[]>('get_agents');
});

const getTagName = (tagId: string) => {
    const i18nKey = `tags.${tagId}`;
    if (te(i18nKey)) return t(i18nKey);
    const tag = allTags.value.find(t => t.id === tagId);
    return tag ? tag.name : tagId;
};

// --- 核心分析逻辑 ---
const currentTagIds = computed(() => {
    const ids = new Set<string>();
    selectedAgents.value.forEach(agent => {
        if (agent) {
            agent.tags.forEach(tagId => ids.add(tagId));
        }
    });
    return ids;
});

const presentKeyTags = computed(() =>
    allTags.value.filter(tag => tag.is_key && currentTagIds.value.has(tag.id))
);
const missingKeyTags = computed(() =>
    allTags.value.filter(tag => tag.is_key && !currentTagIds.value.has(tag.id))
);
const presentNormalTags = computed(() =>
    allTags.value.filter(tag => !tag.is_key && currentTagIds.value.has(tag.id))
);

// --- 交互方法 ---
const openSelector = (index: number) => {
    currentSlotIndex.value = index;
    showSelector.value = true;
};

const isAgentSelected = (agent: Agent) => {
    return selectedAgents.value.some(a => a && a.id === agent.id);
};

const selectAgent = (agent: Agent) => {
    if (!isAgentSelected(agent)) {
        selectedAgents.value[currentSlotIndex.value] = agent;
        showSelector.value = false;
    }
};

const removeAgent = (index: number) => {
    selectedAgents.value[index] = null;
};
</script>

<template>
    <div class="page analyzer-page">
        <h2>{{ $t('ui.nav_main') }}</h2>

        <!-- 顶部：5个选人槽位 -->
        <div class="slots-container">
            <div v-for="(agent, index) in selectedAgents" :key="index" class="agent-slot"
                :class="{ 'is-filled': agent }" @click="!agent && openSelector(index)"
                @contextmenu.prevent="agent && removeAgent(index)">
                <template v-if="agent">
                    <img :src="agent.avatar_url" :alt="agent.name" class="avatar-img" />
                </template>
                <template v-else>
                    <div class="slot-empty">+</div>
                </template>
            </div>
        </div>
        <div class="tip-text">{{ $t('ui.tip_select_agent') }}</div>

        <!-- 底部：阵容分析面板 -->
        <div class="analysis-panels">
            <div class="panel warning-panel">
                <h3>{{ $t('analyzer.missing_key_tags') }}</h3>
                <div class="tags-wrap">
                    <span v-if="missingKeyTags.length === 0" class="perfect-text">{{ $t('ui.none') }}</span>
                    <span v-for="tag in missingKeyTags" :key="tag.id" class="analysis-tag tag-missing">
                        {{ getTagName(tag.id) }}
                    </span>
                </div>
            </div>

            <div class="panel good-panel">
                <h3>{{ $t('analyzer.present_key_tags') }}</h3>
                <div class="tags-wrap">
                    <span v-if="presentKeyTags.length === 0" class="empty-text">{{ $t('ui.empty') }}</span>
                    <span v-for="tag in presentKeyTags" :key="tag.id" class="analysis-tag tag-present">
                        {{ getTagName(tag.id) }}
                    </span>
                </div>
            </div>

            <div class="panel normal-panel">
                <h3>{{ $t('analyzer.present_normal_tags') }}</h3>
                <div class="tags-wrap">
                    <span v-if="presentNormalTags.length === 0" class="empty-text">{{ $t('ui.empty') }}</span>
                    <span v-for="tag in presentNormalTags" :key="tag.id" class="analysis-tag tag-normal">
                        {{ getTagName(tag.id) }}
                    </span>
                </div>
            </div>
        </div>

        <!-- 弹窗：选择角色 -->
        <div v-if="showSelector" class="modal-overlay" @click="showSelector = false">
            <div class="modal-content" @click.stop>
                <h3>{{ $t('analyzer.select_agent') }}</h3>
                <div class="agent-grid">
                    <div v-for="agent in allAgents" :key="agent.id" class="selector-card"
                        :class="{ 'is-disabled': isAgentSelected(agent) }" @click="selectAgent(agent)">
                        <img :src="agent.avatar_url" :alt="agent.name" class="avatar-img" />
                    </div>
                </div>
                <button class="close-btn" @click="showSelector = false">{{ $t('ui.btn_cancel') }}</button>
            </div>
        </div>
    </div>
</template>

<style scoped>
.analyzer-page {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}

.slots-container {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    gap: 12px;
    width: 100%;
}

.agent-slot {
    width: 100%;
    aspect-ratio: 1 / 1;
    border: 2px dashed #ccc;
    border-radius: 8px;
    position: relative;
    cursor: pointer;
    transition: 0.2s;
    background: #fdfdfd;
    overflow: hidden;
}

.agent-slot.is-filled {
    border-color: transparent;
    background: transparent;
}

.agent-slot:hover:not(.is-filled) {
    border-color: #ff4655;
}

.avatar-img {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
    background-color: transparent;
}

.slot-empty {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    font-size: 2rem;
    color: #ccc;
}

.tip-text {
    text-align: center;
    font-size: 0.85rem;
    color: #888;
    margin-top: -10px;
}

.analysis-panels {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    gap: 15px;
}

@media (max-width: 600px) {
    .analysis-panels {
        grid-template-columns: 1fr;
    }
}

.panel {
    padding: 15px;
    border-radius: 8px;
    border: 1px solid #eee;
}

.panel h3 {
    margin-top: 0;
    font-size: 1rem;
    margin-bottom: 12px;
}

.warning-panel {
    background: #fff5f5;
    border-color: #ffcdd2;
}

.good-panel {
    background: #f0fdf4;
    border-color: #c8e6c9;
}

.normal-panel {
    background: #f8f9fa;
    border-color: #e0e0e0;
}

.tags-wrap {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
}

.analysis-tag {
    padding: 4px 10px;
    border-radius: 4px;
    font-size: 0.85rem;
    font-weight: 500;
}

.tag-missing {
    background: #ff4655;
    color: white;
}

.tag-present {
    background: #4caf50;
    color: white;
}

.tag-normal {
    background: #9e9e9e;
    color: white;
}

.empty-text {
    color: #999;
    font-size: 0.9rem;
}

.perfect-text {
    color: #2e7d32;
    font-weight: bold;
}

.modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
}

.modal-content {
    background: white;
    padding: 20px;
    border-radius: 12px;
    width: 90%;
    max-width: 450px;
}

.agent-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 12px;
    margin-bottom: 20px;
}

.selector-card {
    width: 100%;
    aspect-ratio: 1 / 1;
    background: #f5f5f5;
    border-radius: 8px;
    cursor: pointer;
    overflow: hidden;
    border: 2px solid transparent;
    transition: all 0.2s;
    position: relative;
}

.selector-card:hover:not(.is-disabled) {
    border-color: #ff4655;
    transform: scale(1.05);
}

.selector-card.is-disabled {
    filter: grayscale(100%);
    opacity: 0.3;
    cursor: not-allowed;
}

.close-btn {
    width: 100%;
    padding: 12px;
    border: none;
    background: #eee;
    border-radius: 8px;
    cursor: pointer;
    font-weight: bold;
}

.close-btn:hover {
    background: #e0e0e0;
}
</style>