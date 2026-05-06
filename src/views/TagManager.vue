<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
import type { Tag } from '../types';

const { t, te } = useI18n();
const tags = ref<Tag[]>([]);

// 表单状态
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

// 交互：切换关键标签属性
const toggleKey = async (tag: Tag) => {
    try {
        // 1. 调用后端接口修改数据
        await invoke('toggle_tag_key', { id: tag.id });
        // 2. 前端本地状态同步更新（避免重新请求整个列表，提升性能）
        tag.is_key = !tag.is_key;
    } catch (error) {
        console.error("切换失败:", error);
        alert("切换标签状态失败！");
    }
};

// 交互：新增标签
const addNewTag = async () => {
    const name = newTagName.value.trim();
    if (!name) {
        alert("标签名称不能为空！");
        return;
    }

    try {
        // 调用后端接口
        const newTag = await invoke<Tag>('add_tag', {
            name: name,
            isKey: newTagIsKey.value
        });

        // 将后端返回的新标签推入前端列表
        tags.value.push(newTag);

        // 清空表单
        newTagName.value = '';
        newTagIsKey.value = false;
    } catch (error) {
        console.error("添加失败:", error);
        alert("添加标签失败！");
    }
};
</script>

<template>
    <div class="page">
        <h2>{{ $t('ui.nav_tags') }}</h2>

        <!-- 新增标签控制台 -->
        <div class="add-tag-panel">
            <input v-model="newTagName" type="text" class="input-name" placeholder="输入自定义标签名称..."
                @keyup.enter="addNewTag" />
            <label class="checkbox-label">
                <input v-model="newTagIsKey" type="checkbox" />
                关键标签 (Key)
            </label>
            <button class="btn-add" @click="addNewTag">{{ $t('ui.btn_add') }}</button>
        </div>

        <!-- 标签列表 -->
        <div class="tag-list">
            <div v-for="tag in tags" :key="tag.id" class="tag-item">
                <span class="tag-name">{{ getTagName(tag) }}</span>

                <!-- 变成可点击的 Toggle 按钮 -->
                <button class="badge-btn" :class="tag.is_key ? 'key-badge' : 'normal-badge'" @click="toggleKey(tag)"
                    title="点击切换状态">
                    {{ tag.is_key ? 'Key' : 'Normal' }}
                </button>
            </div>
        </div>
    </div>
</template>

<style scoped>
/* 新增控制台样式 */
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

/* 列表样式 */
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

/* 可点击徽章样式 */
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
</style>