import {createApp} from 'vue'

import { createHead } from '@unhead/vue'
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
let sentry_dsn = import.meta.env.VUE_APP_SENTRY_DSN
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

const head = createHead()
app
    .use(createVuetify)
    .use(head)
    .mount('#app')
