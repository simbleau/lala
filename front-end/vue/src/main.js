import { createApp } from 'vue'
import App from './App.vue'
import Router from './router/index'
import Store from './store/store'
import axios from 'axios'

const app = createApp(App)
    .use(Router)
    .use(Store);

app.config.globalProperties.axios = axios

app.mount('#app');

