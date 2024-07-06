import js from "@eslint/js";
import pluginVue from 'eslint-plugin-vue'

export default [
    js.configs.recommended,
    ...pluginVue.configs['flat/recommended'],
    {
        files: ["**/*.js", "**/*.ts", "**/vue.js"],
        ignores: [".gitignore"],
        languageOptions: {
            ecmaVersion: 2022,
        },
        rules: {}
    }
]