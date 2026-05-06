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
        console.error("保存失败:", error);
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
                    <img :src="agent.avatar_url" :alt="agent.name" class="avatar-img" />
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
    color: #888;
    font-size: 0.9rem;
    margin-bottom: 15px;
}

/* 网格与卡片样式 (与之前相同) */
.agent-grid {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    gap: 15px;
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
    cursor: pointer;
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

/* 弹窗样式 */
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

.tag-selector {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    margin: 20px 0;
}

.tag-option {
    padding: 6px 12px;
    border: 1px solid #ddd;
    border-radius: 20px;
    font-size: 0.85rem;
    cursor: pointer;
    transition: 0.2s;
    user-select: none;
}

.tag-option:hover {
    border-color: #ff4655;
}

.tag-option.is-selected {
    background-color: #ff4655;
    color: white;
    border-color: #ff4655;
}

.modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 20px;
}

.btn-cancel,
.btn-save {
    padding: 8px 16px;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-weight: bold;
}

.btn-cancel {
    background: #eee;
    color: #333;
}

.btn-cancel:hover {
    background: #ddd;
}

.btn-save {
    background: #333;
    color: white;
}

.btn-save:hover {
    background: #ff4655;
}
</style>