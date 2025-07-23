import { createMemoryHistory, createRouter } from 'vue-router'
import { createApp } from "vue";
import App from "./App.vue";
import Home from './pages/Home.vue'
import Internet from './pages/Internet.vue'

const routes = [
    { path: '/', redirect: '/internet' },
  { path: '/home', component: Home },
  { path: '/internet', component: Internet },
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

createApp(App).use(router).mount("#app");
