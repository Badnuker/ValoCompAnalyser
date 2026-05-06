<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
import type { Tag } from '../types';

const { t, te } = useI18n();
const tags = ref<Tag[]>([]);

const newTagName = ref('');
const newTagIsKey = ref(false);

onMounted(async () => {
    await fetchTags();
});

const fetchTags = async () => {
    tags.value = await invoke<Tag[]>('get_tags');
};

const getTagName = (tag: Tag) => {
    const i18nKey = `tags.${tag.id}`;
    return te(i18nKey) ? t(i18nKey) : tag.name;
};

const toggleKey = async (tag: Tag) => {
    try {
        await invoke('toggle_tag_key', { id: tag.id });
        tag.is_key = !tag.is_key;
    } catch (error) {
        console.error("切换失败:", error);
        alert("切换标签状态失败！");
    }
};

const addNewTag = async () => {
    const name = newTagName.value.trim();
    if (!name) {
        alert("标签名称不能为空！");
        return;
    }
    try {
        const newTag = await invoke<Tag>('add_tag', { name, isKey: newTagIsKey.value });
        tags.value.push(newTag);
        newTagName.value = '';
        newTagIsKey.value = false;
    } catch (error) {
        console.error("添加失败:", error);
        alert("添加标签失败！");
    }
};

// 新增：删除标签逻辑
const removeTag = async (id: string) => {
    // 加一个确认弹窗防误触
    if (!confirm("确定要删除这个标签吗？角色身上的该标签也会被一并移除。")) return;

    try {
        await invoke('delete_tag', { id });
        // 前端同步移除
        tags.value = tags.value.filter(t => t.id !== id);
    } catch (error) {
        console.error("删除失败:", error);
        alert("删除标签失败！");
    }
};
</script>

<template>
    <div class="page">
        <h2>{{ $t('ui.nav_tags') }}</h2>

        <div class="add-tag-panel">
            <input v-model="newTagName" type="text" class="input-name" placeholder="输入自定义标签名称..."
                @keyup.enter="addNewTag" />
            <label class="checkbox-label">
                <input v-model="newTagIsKey" type="checkbox" />
                关键标签 (Key)
            </label>
            <button class="btn-add" @click="addNewTag">{{ $t('ui.btn_add') }}</button>
        </div>

        <div class="tag-list">
            <div v-for="tag in tags" :key="tag.id" class="tag-item">
                <span class="tag-name">{{ getTagName(tag) }}</span>

                <div class="actions">
                    <button class="badge-btn" :class="tag.is_key ? 'key-badge' : 'normal-badge'" @click="toggleKey(tag)"
                        title="点击切换状态">
                        {{ tag.is_key ? 'Key' : 'Normal' }}
                    </button>
                    <!-- 新增删除按钮 -->
                    <button class="delete-btn" @click="removeTag(tag.id)" title="删除标签">×</button>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.add-tag-panel {
    display: flex;
    align-items: center;
    gap: 15px;
    background: #fdfdfd;
    padding: 15px;
    border-radius: 8px;
    border: 1px solid #ddd;
}

.input-name {
    flex: 1;
    padding: 8px 12px;
    border: 1px solid #ccc;
    border-radius: 4px;
    outline: none;
}

.input-name:focus {
    border-color: #ff4655;
}

.checkbox-label {
    display: flex;
    align-items: center;
    gap: 5px;
    cursor: pointer;
    font-size: 0.9rem;
}

.btn-add {
    padding: 8px 20px;
    background-color: #333;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: bold;
    transition: 0.2s;
}

.btn-add:hover {
    background-color: #ff4655;
}

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
    transition: 0.2s;
}

.tag-item:hover {
    border-color: #ccc;
}

.tag-name {
    font-weight: 600;
    font-size: 1rem;
}

.actions {
    display: flex;
    align-items: center;
    gap: 10px;
}

.badge-btn {
    font-size: 0.8rem;
    padding: 4px 12px;
    border-radius: 12px;
    font-weight: bold;
    border: none;
    cursor: pointer;
    transition: 0.2s;
    user-select: none;
}

.badge-btn:hover {
    transform: scale(1.05);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
}

.badge-btn:active {
    transform: scale(0.95);
}

.key-badge {
    background-color: #ff4655;
    color: white;
}

.normal-badge {
    background-color: #ccc;
    color: #333;
}

/* 删除按钮样式 */
.delete-btn {
    background: none;
    border: none;
    color: #999;
    font-size: 1.5rem;
    line-height: 1;
    cursor: pointer;
    padding: 0 5px;
    border-radius: 4px;
    transition: 0.2s;
}

.delete-btn:hover {
    color: #ff4655;
    background: #ffeeee;
}
</style>