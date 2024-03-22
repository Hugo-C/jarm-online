import {defineConfig} from 'vite'
import vue from '@vitejs/plugin-vue'
import {sentryVitePlugin} from "@sentry/vite-plugin";
import {codecovVitePlugin} from "@codecov/vite-plugin";
import * as path from "path";


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
        codecovVitePlugin({
            enableBundleAnalysis: process.env.CODECOV_TOKEN !== undefined,
            bundleName: "jarm-online",
            uploadToken: process.env.CODECOV_TOKEN,
        }),
    ],
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "./src"),
        },
    },
    build: {
        chunkSizeWarningLimit: 800,
        sourcemap: true,
    },
})
