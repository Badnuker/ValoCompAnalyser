<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from 'vue';
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

// --- 弹窗头像列数：保证短边至少 3 个 ---
const winW = ref(window.innerWidth);
const winH = ref(window.innerHeight);
const onResize = () => { winW.value = window.innerWidth; winH.value = window.innerHeight; };
onMounted(() => window.addEventListener('resize', onResize));
onBeforeUnmount(() => window.removeEventListener('resize', onResize));

const selectorColumns = computed(() => {
    const w = winW.value;
    const h = winH.value;
    // 取短边的 1/3 作为最小头像尺寸，看长边能放几个
    const minSide = Math.min(w, h);
    const minCell = Math.floor(minSide / 3);
    if (minCell < 50) return 3; // 极小屏保底
    const cols = Math.floor(w / minCell);
    return Math.max(3, Math.min(cols, 10));
});
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
                    <img :src="agent.avatar_url" :alt="agent.name" class="avatar-img" loading="lazy" />
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
                <div class="agent-grid" :style="{ gridTemplateColumns: `repeat(${selectorColumns}, minmax(0, 1fr))` }">
                    <div v-for="agent in allAgents" :key="agent.id" class="selector-card"
                        :class="{ 'is-disabled': isAgentSelected(agent) }" @click="selectAgent(agent)">
                        <img :src="agent.avatar_url" :alt="agent.name" class="avatar-img" loading="lazy" />
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
    gap: clamp(0.8rem, 1.2vw, 1.5rem);
}

/* ===== 5 槽位选人区 ===== */
.slots-container {
    display: grid;
    grid-template-columns: repeat(5, minmax(0, 1fr));
    gap: clamp(6px, 0.8vw, 10px);
    width: 100%;
}

.agent-slot {
    width: 100%;
    aspect-ratio: 1 / 1;
    border: 2px dashed #ccc;
    border-radius: clamp(6px, 0.6vw, 8px);
    cursor: pointer;
    transition: 0.2s;
    background: #fdfdfd;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
}

.agent-slot.is-filled {
    border-color: transparent;
    background: transparent;
}

.agent-slot:hover:not(.is-filled) {
    border-color: var(--color-accent, #ff4655);
}

.avatar-img {
    width: 100%;
    height: 100%;
    object-fit: contain;
    background-color: transparent;
    display: block;
}

.slot-empty {
    font-size: clamp(1.5rem, 3vw, 2rem);
    color: #ccc;
    line-height: 1;
    user-select: none;
}

.tip-text {
    text-align: center;
    font-size: clamp(0.75rem, 0.8vw + 0.3rem, 0.85rem);
    color: var(--color-text-secondary, #888);
    margin-top: clamp(-12px, -1vw, -10px);
}

/* ===== 分析面板三栏 ===== */
.analysis-panels {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    gap: clamp(10px, 1.2vw, 15px);
}

.panel {
    padding: clamp(10px, 1.2vw, 15px);
    border-radius: clamp(6px, 0.6vw, 8px);
    border: 1px solid #eee;
}

.panel h3 {
    margin-top: 0;
    font-size: clamp(0.85rem, 0.8vw + 0.3rem, 1rem);
    margin-bottom: clamp(8px, 1vw, 12px);
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
    gap: clamp(4px, 0.6vw, 8px);
}

.analysis-tag {
    padding: clamp(3px, 0.4vw, 4px) clamp(8px, 0.8vw, 10px);
    border-radius: 4px;
    font-size: clamp(0.75rem, 0.7vw + 0.3rem, 0.85rem);
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
    font-size: clamp(0.8rem, 0.7vw + 0.3rem, 0.9rem);
}

.perfect-text {
    color: #2e7d32;
    font-weight: bold;
}

/* ===== 角色选择器弹窗 ===== */
.modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
    padding: clamp(8px, 2vw, 16px);
    padding-top: calc(clamp(8px, 2vw, 16px) + var(--status-bar-top, 0px));
    padding-bottom: calc(clamp(8px, 2vw, 16px) + var(--nav-safe-bottom, 0px));
}

.modal-content {
    background: white;
    padding: clamp(14px, 1.6vw, 20px);
    border-radius: clamp(8px, 1vw, 12px);
    width: min(92vw, 980px);
    max-height: calc(85vh - var(--status-bar-top, 0px) - var(--nav-safe-bottom, 0px));
    display: flex;
    flex-direction: column;
}

.modal-content h3 {
    font-size: clamp(0.95rem, 0.9vw + 0.4rem, 1.1rem);
    margin: 0 0 clamp(8px, 1vw, 12px) 0;
}

.agent-grid {
    display: flex;
    flex-wrap: wrap;
    gap: clamp(6px, 1vw, 12px);
    margin-bottom: clamp(12px, 1.6vw, 20px);
    overflow-y: auto;
    overflow-x: hidden;
    max-height: min(56vh, 480px);
    padding-right: 4px;
    align-content: flex-start;
    justify-content: flex-start;
}

.selector-card {
    width: calc(4em + 4px);
    aspect-ratio: 1 / 1;
    background: #f5f5f5;
    border-radius: clamp(4px, 0.5vw, 6px);
    cursor: pointer;
    overflow: hidden;
    border: 2px solid transparent;
    transition: border-color 0.15s, opacity 0.15s;
    flex-shrink: 0;
    font-size: clamp(0.95rem, 0.9vw + 0.4rem, 1.1rem);
}

.selector-card:hover:not(.is-disabled) {
    border-color: var(--color-accent, #ff4655);
}

.selector-card.is-disabled {
    filter: grayscale(100%);
    opacity: 0.3;
    cursor: not-allowed;
}

.selector-card .avatar-img {
    width: 100%;
    height: 100%;
    object-fit: contain;
    background-color: transparent;
    display: block;
}

.close-btn {
    width: 100%;
    padding: clamp(10px, 1vw, 12px);
    border: none;
    background: #eee;
    border-radius: clamp(6px, 0.6vw, 8px);
    cursor: pointer;
    font-weight: bold;
    font-size: clamp(0.85rem, 0.8vw + 0.3rem, 1rem);
}

.close-btn:hover {
    background: #e0e0e0;
}

.agent-grid::-webkit-scrollbar {
    width: 6px;
}

.agent-grid::-webkit-scrollbar-thumb {
    background: #ccc;
    border-radius: 4px;
}

.agent-grid::-webkit-scrollbar-thumb:hover {
    background: #999;
}

/* ============ 扁宽屏：左右分栏布局 ============ */
/* 宽高比 > 1.6:1（如 2800×1272），高度 ≤ 800px */
@media (min-aspect-ratio: 16/10) and (max-height: 800px) {
    .analyzer-page {
        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: auto auto 1fr;
        grid-template-areas:
            "title  title"
            "slots  panels"
            "tip    panels";
        gap: clamp(8px, 1vw, 12px);
        align-items: start;
    }

    .analyzer-page>h2 {
        grid-area: title;
    }

    .slots-container {
        grid-area: slots;
        gap: clamp(4px, 0.6vw, 8px);
    }

    .tip-text {
        grid-area: tip;
        margin-top: -0.5rem;
        text-align: left;
    }

    .analysis-panels {
        grid-area: panels;
        grid-template-columns: 1fr;
        gap: clamp(6px, 0.8vw, 10px);
    }

    /* 分栏后槽位偏大，缩小 */
    .agent-slot {
        border-radius: clamp(4px, 0.5vw, 6px);
    }

    .slot-empty {
        font-size: clamp(1.2rem, 2vw, 1.6rem);
    }
}

/* ============ 普通响应式 ============ */
@media (max-width: 640px) {
    .analysis-panels {
        grid-template-columns: 1fr;
    }
}

@media (max-width: 560px) {
    .analyzer-page {
        gap: 0.8rem;
    }

    .slots-container {
        gap: 5px;
    }

    /* 列数由 JS selectorColumns 动态算，保证短边至少 3 个 */
    .agent-grid {
        max-height: min(48vh, 360px);
    }

    .tip-text {
        margin-top: -6px;
    }

    .modal-content {
        padding: 12px;
    }

    .slot-empty {
        font-size: 1.5rem;
    }
}
</style>
