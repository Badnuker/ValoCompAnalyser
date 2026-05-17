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
    tags.value = await invoke<Tag[]>('get_tags');
});

const getTagName = (tag: Tag) => {
    const i18nKey = `tags.${tag.id}`;
    return te(i18nKey) ? t(i18nKey) : tag.name;
};

const toggleKey = async (tag: Tag) => {
    try {
        await invoke('toggle_tag_key', { id: tag.id });
        tag.is_key = !tag.is_key;
    } catch (error) {
        console.error(t('ui.log_toggle_failed'), error);
        alert(t('ui.alert_toggle_failed'));
    }
};

const addNewTag = async () => {
    const name = newTagName.value.trim();
    if (!name) {
        alert(t('ui.alert_tag_name_required'));
        return;
    }
    try {
        const newTag = await invoke<Tag>('add_tag', { name, isKey: newTagIsKey.value });
        tags.value.push(newTag);
        newTagName.value = '';
        newTagIsKey.value = false;
    } catch (error) {
        console.error(t('ui.log_add_tag_failed'), error);
        alert(t('ui.alert_add_tag_failed'));
    }
};

const removeTag = async (id: string) => {
    if (!confirm(t('ui.confirm_delete_tag'))) return;
    try {
        await invoke('delete_tag', { id });
        tags.value = tags.value.filter(t => t.id !== id);
    } catch (error) {
        console.error(t('ui.log_delete_tag_failed'), error);
        alert(t('ui.alert_delete_tag_failed'));
    }
};
</script>

<template>
    <div class="page">
        <h2>{{ $t('ui.nav_tags') }}</h2>

        <div class="add-tag-panel">
            <input v-model="newTagName" type="text" class="input-name" :placeholder="$t('ui.placeholder_new_tag')"
                @keyup.enter="addNewTag" />
            <label class="checkbox-label">
                <input v-model="newTagIsKey" type="checkbox" />
                {{ $t('ui.label_key') }}
            </label>
            <button class="btn-add" @click="addNewTag">{{ $t('ui.btn_add') }}</button>
        </div>

        <div class="tag-list">
            <div v-for="tag in tags" :key="tag.id" class="tag-item">
                <span class="tag-name">{{ getTagName(tag) }}</span>
                <div class="actions">
                    <button class="badge-btn" :class="tag.is_key ? 'key-badge' : 'normal-badge'" @click="toggleKey(tag)"
                        :title="$t('ui.title_toggle_tag')">
                        {{ tag.is_key ? $t('ui.key_short') : $t('ui.normal_short') }}
                    </button>
                    <button class="delete-btn" @click="removeTag(tag.id)" :title="$t('ui.title_delete_tag')">×</button>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
/* ===== 添加标签面板 ===== */
.add-tag-panel {
    display: flex;
    align-items: center;
    gap: clamp(8px, 1.2vw, 15px);
    background: #fdfdfd;
    padding: clamp(10px, 1.2vw, 15px);
    border-radius: clamp(6px, 0.6vw, 8px);
    border: 1px solid #ddd;
    flex-wrap: wrap;
}

.input-name {
    flex: 1;
    min-width: 120px;
    padding: clamp(6px, 0.6vw, 8px) clamp(10px, 1vw, 12px);
    border: 1px solid #ccc;
    border-radius: 4px;
    outline: none;
    font-size: clamp(0.85rem, 0.8vw + 0.3rem, 1rem);
}

.input-name:focus {
    border-color: var(--color-accent, #ff4655);
}

.checkbox-label {
    display: flex;
    align-items: center;
    gap: 5px;
    cursor: pointer;
    font-size: clamp(0.82rem, 0.8vw + 0.3rem, 0.9rem);
    white-space: nowrap;
}

.btn-add {
    padding: clamp(7px, 0.6vw, 8px) clamp(16px, 1.6vw, 20px);
    background-color: var(--color-text, #333);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: bold;
    transition: 0.2s;
    font-size: clamp(0.85rem, 0.8vw + 0.3rem, 1rem);
    white-space: nowrap;
}

.btn-add:hover {
    background-color: var(--color-accent, #ff4655);
}

/* ===== 标签列表 ===== */
.tag-list {
    display: flex;
    flex-direction: column;
    gap: clamp(8px, 0.8vw, 10px);
    margin-top: clamp(12px, 1.6vw, 20px);
}

.tag-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: clamp(10px, 1vw, 12px) clamp(12px, 1.4vw, 16px);
    background: #f9f9f9;
    border-radius: clamp(6px, 0.6vw, 8px);
    border: 1px solid #eee;
    transition: 0.2s;
    gap: clamp(6px, 1vw, 12px);
}

.tag-item:hover {
    border-color: #ccc;
}

.tag-name {
    font-weight: 600;
    font-size: clamp(0.9rem, 0.8vw + 0.4rem, 1rem);
    word-break: break-word;
}

.actions {
    display: flex;
    align-items: center;
    gap: clamp(6px, 0.8vw, 10px);
    flex-shrink: 0;
}

.badge-btn {
    font-size: clamp(0.72rem, 0.65vw + 0.3rem, 0.8rem);
    padding: clamp(3px, 0.3vw, 4px) clamp(10px, 1vw, 12px);
    border-radius: 12px;
    font-weight: bold;
    border: none;
    cursor: pointer;
    transition: 0.2s;
    user-select: none;
    white-space: nowrap;
}

.badge-btn:hover {
    transform: scale(1.05);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
}

.badge-btn:active {
    transform: scale(0.95);
}

.key-badge {
    background-color: var(--color-accent, #ff4655);
    color: white;
}

.normal-badge {
    background-color: #ccc;
    color: var(--color-text, #333);
}

/* ===== 删除按钮 ===== */
.delete-btn {
    background: none;
    border: none;
    color: #999;
    font-size: clamp(1.3rem, 1.2vw + 0.5rem, 1.5rem);
    line-height: 1;
    cursor: pointer;
    padding: 0 5px;
    border-radius: 4px;
    transition: 0.2s;
}

.delete-btn:hover {
    color: var(--color-accent, #ff4655);
    background: #ffeeee;
}

/* ===== 手机竖屏 ===== */
@media (max-width: 480px) {
    .add-tag-panel {
        gap: 8px;
    }

    .input-name {
        min-width: 100%;
        flex-basis: 100%;
    }

    .checkbox-label {
        flex: 1;
    }

    .btn-add {
        flex: 1;
    }
}
</style>
