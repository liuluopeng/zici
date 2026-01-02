import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createPinia } from 'pinia'
import cnchar from 'cnchar-all'

// 设置CNCHAR资源基础路径
cnchar.setResourceBase('./')

const pinia = createPinia()

const app = createApp(App)
app.use(router)
app.use(pinia)
app.mount('#app')

