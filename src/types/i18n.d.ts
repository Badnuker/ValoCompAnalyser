import 'vue-i18n';

declare module 'vue-i18n' {
    // 使用默认语言文件的结构作为消息 schema，保证 $t 的键名自动补全与类型检查
    type MessageSchema = typeof import('../locales/zh').default;

    interface DefineLocaleMessage extends MessageSchema { }
}
