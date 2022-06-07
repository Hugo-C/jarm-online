import {createApp} from 'vue'
import Equal from 'equal-vue'
import 'equal-vue/dist/style.css'
import { createMetaManager } from 'vue-meta'
import App from './App.vue'

const app = createApp(App)

app.directive('autofocus', {
    mounted(el) {
        el.focus()
    }
})

// app.component('hash-result', {
//     template: `<p>This is a hash jarm result<p>`
// })

app.component('todo-item', {
    props: ['todo'],
    template: `<li>{{ todo.text }}</li>`
})



app
    .use(Equal)
    .use(createMetaManager())
    .mount('#app')
