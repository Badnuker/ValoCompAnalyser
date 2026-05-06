import { createI18n } from 'vue-i18n';
import zh from './zh';
import en from './en';

const saved = typeof window !== 'undefined' ? localStorage.getItem('locale') : null;
const defaultLocale = saved || (typeof navigator !== 'undefined' && navigator.language?.startsWith('zh') ? 'zh' : 'en');

const i18n = createI18n({
    legacy: false,
    locale: defaultLocale,
    fallbackLocale: 'en',
    messages: { zh, en }
});

export default i18n;