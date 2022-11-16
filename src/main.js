import { createApp } from "vue";
import "./style.css";
import { createRouter, createWebHistory } from 'vue-router'
import { plugin, defaultConfig } from '@formkit/vue'

import routes from '~pages'
import App from "./App.vue";
const router = createRouter({
    history: createWebHistory(),
    routes,
})


createApp(App).use(router).use(plugin, defaultConfig).mount("#app");
