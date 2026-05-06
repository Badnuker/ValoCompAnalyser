<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { ref, computed, watch, onMounted, onBeforeUnmount } from 'vue';

const { locale, t } = useI18n({ useScope: 'global' });

const langs = [
    { code: 'zh', labelKey: 'ui.locale_zh' },
    { code: 'en', labelKey: 'ui.locale_en' },
];

function normalize(code: string | undefined) {
    if (!code) return 'en';
    if (code.startsWith('zh')) return 'zh';
    if (code.startsWith('en')) return 'en';
    return code;
}

const open = ref(false);
const rootRef = ref<HTMLElement | null>(null);

const currentLang = computed(() => {
    return langs.find((l) => l.code === normalize(locale.value)) || langs[1];
});

const currentLangLabel = computed(() => t(currentLang.value.labelKey));

function toggle() {
    open.value = !open.value;
}

function selectLocale(code: string) {
    const val = normalize(code);
    locale.value = val;
    try { localStorage.setItem('locale', val); } catch (e) { }
    open.value = false;
}

watch(locale, (val) => {
    try { localStorage.setItem('locale', val); } catch (e) { }
});

function onDocClick(e: MouseEvent) {
    const el = rootRef.value;
    if (!el) return;
    const target = e.target as Node;
    if (!el.contains(target)) open.value = false;
}

onMounted(() => document.addEventListener('click', onDocClick));
onBeforeUnmount(() => document.removeEventListener('click', onDocClick));
</script>

<template>
    <div class="language-switcher" ref="rootRef">
        <button class="ls-btn" type="button" @click="toggle" :aria-expanded="open">
            <span class="flag-icon">
                <svg v-if="currentLang.code === 'zh'" width="18" height="12" viewBox="0 0 18 12"
                    xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                    <rect width="18" height="12" fill="#de2910" />
                    <g fill="#ffde00">
                        <path d="M2 2.8l1.2.4-.2-1 1-.6-.8-.6L2 1.6 1 1.0l-.8.6 1 .6-.2 1L2 2.8z"
                            transform="translate(1,1) scale(0.9)" />
                    </g>
                </svg>
                <svg v-else width="18" height="12" viewBox="0 0 19 12" xmlns="http://www.w3.org/2000/svg"
                    aria-hidden="true">
                    <rect width="19" height="12" fill="#b22234" />
                    <rect width="8" height="6" fill="#3c3b6e" />
                    <g fill="#fff">
                        <rect y="2" width="19" height="1" />
                        <rect y="4" width="19" height="1" />
                        <rect y="6" width="19" height="1" />
                        <rect y="8" width="19" height="1" />
                    </g>
                </svg>
            </span>
            <span class="ls-label">{{ currentLangLabel }}</span>
            <span class="ls-caret">▾</span>
        </button>

        <ul v-show="open" class="ls-list" role="menu">
            <li v-for="l in langs" :key="l.code" class="ls-item" role="menuitem" @click.prevent="selectLocale(l.code)">
                <span class="flag-icon small">
                    <svg v-if="l.code === 'zh'" width="16" height="10" viewBox="0 0 18 12"
                        xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                        <rect width="18" height="12" fill="#de2910" />
                        <g fill="#ffde00">
                            <path d="M2 2.8l1.2.4-.2-1 1-.6-.8-.6L2 1.6 1 1.0l-.8.6 1 .6-.2 1L2 2.8z"
                                transform="translate(1,1) scale(0.9)" />
                        </g>
                    </svg>
                    <svg v-else-if="l.code === 'en'" width="16" height="10" viewBox="0 0 19 12"
                        xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                        <rect width="19" height="12" fill="#b22234" />
                        <rect width="7" height="5" fill="#3c3b6e" />
                        <g fill="#fff">
                            <rect y="2" width="19" height="1" />
                            <rect y="4" width="19" height="1" />
                            <rect y="6" width="19" height="1" />
                        </g>
                    </svg>
                </span>
                <span class="ls-item-label">{{ t(l.labelKey) }}</span>
            </li>
        </ul>
    </div>
</template>

<style scoped>
.language-switcher {
    position: relative;
    display: inline-block;
}

.ls-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    border: 1px solid var(--border-color, #ddd);
    background: var(--card-bg, #fff);
    padding: 4px 8px;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 600;
}

.flag-icon {
    display: inline-flex;
    align-items: center;
}

.flag-icon.small {
    width: 16px;
    height: 10px;
    margin-right: 6px;
}

.ls-label {
    margin-right: 6px;
}

.ls-caret {
    font-size: 12px;
    color: #666;
}

.ls-list {
    position: absolute;
    right: 0;
    margin-top: 6px;
    background: var(--card-bg, #fff);
    border: 1px solid var(--border-color, #ddd);
    border-radius: 6px;
    box-shadow: 0 6px 18px rgba(0, 0, 0, 0.08);
    list-style: none;
    padding: 6px 6px;
    z-index: 50;
    min-width: 140px;
}

.ls-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 8px;
    border-radius: 4px;
    cursor: pointer;
}

.ls-item:hover {
    background: rgba(0, 0, 0, 0.04);
}

.ls-item-label {
    flex: 1;
}
</style>
