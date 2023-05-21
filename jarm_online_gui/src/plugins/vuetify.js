// Vuetify
import 'vuetify/styles'
import {createVuetify} from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import {aliases, md} from 'vuetify/iconsets/md'

const myCustomLightTheme = {
    dark: false,
    colors: {
        primary: '#2196F3',
        secondary: '#F37F21',
        error: '#F44336',
        info: '#232323',
        success: '#4CAF50',
    },
}

export default createVuetify({
    components,
    directives,
    theme: {
        defaultTheme: 'myCustomLightTheme',
        variations: {
            colors: ['primary', 'secondary'],
            lighten: 1,
            darken: 1,
        },
        themes: {
            myCustomLightTheme,
        },
    },
    icons: {
        defaultSet: 'md',
        aliases,
        sets: {
            md,
        },
    },
})