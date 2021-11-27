import { createApp } from 'vue'
import App from './App.vue'
import Router from './router/index'
import Store from './store/store'
import axios from 'axios'
import vSelect from "vue-select";

import "vue-select/dist/vue-select.css";

const app = createApp(App)
    .use(Router)
    .use(Store)
    .component("v-select", vSelect);

app.config.globalProperties.axios = axios

app.mount('#app');

