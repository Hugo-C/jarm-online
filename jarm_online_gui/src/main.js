import {createApp} from 'vue'

import {createMetaManager} from 'vue-meta'
import * as Sentry from "@sentry/vue"
import App from './App.vue'
import createVuetify from "./plugins/vuetify";

const app = createApp(App)

app.directive('autofocus', {
    mounted(el) {
        el.focus()
    }
})

// Set at build time through env var
// see https://cli.vuejs.org/guide/mode-and-env.html#using-env-variables-in-client-side-code
let sentry_dsn = process.env.VUE_APP_SENTRY_DSN
Sentry.init({
    app,
    dsn: sentry_dsn,
    integrations: [
        new Sentry.BrowserTracing(),
        new Sentry.Replay(),
    ],
    tracesSampleRate: 1.0,
    replaysSessionSampleRate: 0.1,
    replaysOnErrorSampleRate: 1.0,
});

app
    .use(createVuetify)
    .use(createMetaManager())
    .mount('#app')
