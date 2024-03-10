import {defineConfig} from 'vite'
import vue from '@vitejs/plugin-vue'
import {sentryVitePlugin} from "@sentry/vite-plugin";

const path = require("path");


// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        vue({
            template: {
                compilerOptions: {
                    isCustomElement: (tag) => tag === "Head",
                }
            }
        }),
        sentryVitePlugin({
            org: process.env.SENTRY_ORG,
            project: process.env.SENTRY_PROJECT,
            authToken: process.env.SENTRY_AUTH_TOKEN,
        }),
    ],
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "./src"),
        },
    },
    build: {
        chunkSizeWarningLimit: 750,
        sourcemap: true,
    },
})
