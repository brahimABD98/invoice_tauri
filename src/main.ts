import { createApp } from "vue";
import "./style.css";
import { createRouter, createWebHistory } from 'vue-router'

import routes from '~pages'
import App from "./App.vue";
const router = createRouter({
    history: createWebHistory(),
    routes,
})


createApp(App).use(router).mount("#app");
