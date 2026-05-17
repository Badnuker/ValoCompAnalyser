<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
import type { Agent, Tag } from '../types';

const { t, te } = useI18n();
const agents = ref<Agent[]>([]);
const allTags = ref<Tag[]>([]);

// 弹窗状态
const editingAgent = ref<Agent | null>(null);
// 用于在弹窗中双向绑定的临时标签列表
const tempTags = ref<string[]>([]);

onMounted(async () => {
    agents.value = await invoke<Agent[]>('get_agents');
    allTags.value = await invoke<Tag[]>('get_tags');
});

// 获取标签名称的辅助函数
const getTagName = (tagId: string) => {
    const i18nKey = `tags.${tagId}`;
    if (te(i18nKey)) return t(i18nKey);
    const tag = allTags.value.find(t => t.id === tagId);
    return tag ? tag.name : tagId;
};

// 打开编辑弹窗
const openEditModal = (agent: Agent) => {
    editingAgent.value = agent;
    tempTags.value = [...agent.tags]; // 拷贝一份当前标签
};

// 切换标签选中状态
const toggleTagSelection = (tagId: string) => {
    const index = tempTags.value.indexOf(tagId);
    if (index > -1) {
        tempTags.value.splice(index, 1);
    } else {
        tempTags.value.push(tagId);
    }
};

// 保存修改
const saveAgentTags = async () => {
    if (!editingAgent.value) return;
    try {
        await invoke('update_agent_tags', {
            agentId: editingAgent.value.id,
            newTags: tempTags.value
        });
        // 更新本地前端数据
        editingAgent.value.tags = [...tempTags.value];
        editingAgent.value = null; // 关闭弹窗
    } catch (error) {
        console.error(t('ui.log_save_agent_failed'), error);
        alert(t('ui.alert_save_agent_failed'));
    }
};
</script>

<template>
    <div class="page">
        <h2>{{ $t('ui.nav_agents') }}</h2>
        <p class="tip-text">{{ $t('ui.tip_click_agent_card') }}</p>

        <div class="agent-grid">
            <div v-for="agent in agents" :key="agent.id" class="agent-card" @click="openEditModal(agent)">
                <div class="avatar-container">
                    <img :src="agent.avatar_url" :alt="agent.name" class="avatar-img" loading="lazy" />
                </div>
                <div class="agent-info">
                    <div class="agent-name">{{ $t(`agents.${agent.id}`) }}</div>
                    <div class="agent-tags">
                        <span v-for="tagId in agent.tags" :key="tagId" class="agent-tag-id">
                            {{ getTagName(tagId) }}
                        </span>
                    </div>
                </div>
            </div>
        </div>

        <!-- 编辑角色标签的弹窗 -->
        <div v-if="editingAgent" class="modal-overlay" @click="editingAgent = null">
            <div class="modal-content" @click.stop>
                <h3>{{ $t('ui.edit_agent_tags', { name: $t(`agents.${editingAgent.id}`) }) }}</h3>

                <div class="tag-selector">
                    <div v-for="tag in allTags" :key="tag.id" class="tag-option"
                        :class="{ 'is-selected': tempTags.includes(tag.id) }" @click="toggleTagSelection(tag.id)">
                        {{ getTagName(tag.id) }}
                    </div>
                </div>

                <div class="modal-actions">
                    <button class="btn-cancel" @click="editingAgent = null">{{ $t('ui.btn_cancel') }}</button>
                    <button class="btn-save" @click="saveAgentTags">{{ $t('ui.btn_save') }}</button>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.tip-text {
    color: var(--color-text-secondary, #888);
    font-size: clamp(0.8rem, 0.8vw + 0.3rem, 0.9rem);
    margin-bottom: clamp(10px, 1.2vw, 15px);
}

/* ===== 角色卡片网格 ===== */
.agent-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(clamp(75px, 7vw, 100px), 1fr));
    gap: clamp(6px, 0.8vw, 10px);
}

.agent-card {
    background: #fdfdfd;
    border: 1px solid #eee;
    border-radius: clamp(6px, 0.6vw, 8px);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    transition: box-shadow 0.2s;
    cursor: pointer;
}

.agent-card:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
    border-color: var(--color-accent, #ff4655);
}

.avatar-container {
    width: 100%;
    padding: clamp(2px, 0.3vw, 4px);
    padding-bottom: 0;
    box-sizing: border-box;
    aspect-ratio: 1 / 1;
    background-color: transparent;
    display: flex;
    align-items: center;
    justify-content: center;
}

.avatar-img {
    width: 100%;
    height: 100%;
    object-fit: contain;
    display: block;
}

.agent-info {
    padding: clamp(4px, 0.4vw, 6px);
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    flex: 1;
}

.agent-name {
    font-size: clamp(0.72rem, 0.65vw + 0.3rem, 0.8rem);
    font-weight: bold;
    margin-bottom: clamp(3px, 0.3vw, 4px);
    color: var(--color-text, #333);
    word-break: break-word;
}

.agent-tags {
    display: flex;
    flex-wrap: wrap;
    gap: clamp(2px, 0.3vw, 4px);
    justify-content: center;
}

.agent-tag-id {
    font-size: clamp(0.65rem, 0.55vw + 0.3rem, 0.7rem);
    background: #f0f0f0;
    padding: 2px 5px;
    border-radius: 4px;
    color: #666;
}

/* ===== 标签编辑弹窗 ===== */
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
    width: min(94vw, 450px);
    max-height: calc(85vh - var(--status-bar-top, 0px) - var(--nav-safe-bottom, 0px));
    display: flex;
    flex-direction: column;
}

.modal-content h3 {
    font-size: clamp(0.95rem, 0.9vw + 0.4rem, 1.1rem);
    margin: 0 0 clamp(6px, 0.8vw, 10px) 0;
}

.tag-selector {
    display: flex;
    flex-wrap: wrap;
    gap: clamp(6px, 0.8vw, 10px);
    margin: clamp(8px, 1vw, 12px) 0 clamp(10px, 1.2vw, 16px);
    overflow-y: auto;
    max-height: min(46vh, 320px);
    padding-right: 4px;
    align-content: flex-start;
}

.tag-selector::-webkit-scrollbar {
    width: 6px;
}

.tag-selector::-webkit-scrollbar-thumb {
    background: #ccc;
    border-radius: 4px;
}

.tag-selector::-webkit-scrollbar-thumb:hover {
    background: #999;
}

.tag-option {
    padding: clamp(5px, 0.5vw, 6px) clamp(10px, 1vw, 12px);
    border: 1px solid #ddd;
    border-radius: 20px;
    font-size: clamp(0.78rem, 0.7vw + 0.3rem, 0.85rem);
    cursor: pointer;
    transition: 0.2s;
    user-select: none;
}

.tag-option:hover {
    border-color: var(--color-accent, #ff4655);
}

.tag-option.is-selected {
    background-color: var(--color-accent, #ff4655);
    color: white;
    border-color: var(--color-accent, #ff4655);
}

.modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: clamp(6px, 0.8vw, 10px);
    margin-top: auto;
    padding-top: clamp(6px, 0.6vw, 8px);
}

.btn-cancel,
.btn-save {
    padding: clamp(8px, 0.8vw, 10px) clamp(12px, 1.2vw, 16px);
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-weight: bold;
    font-size: clamp(0.85rem, 0.8vw + 0.3rem, 1rem);
}

.btn-cancel {
    background: #eee;
    color: var(--color-text, #333);
}

.btn-cancel:hover {
    background: #ddd;
}

.btn-save {
    background: var(--color-text, #333);
    color: white;
}

.btn-save:hover {
    background: var(--color-accent, #ff4655);
}

@media (max-width: 560px) {
    .modal-actions {
        gap: 8px;
    }

    .btn-cancel,
    .btn-save {
        flex: 1;
    }
}
</style>
