import { createApp } from 'vue'
import App from './App.vue'
import Router from './router/index'
import Store from './store/store'

createApp(App)
    .use(Router)
    .use(Store)
    .mount('#app')