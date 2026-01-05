import { createRouter, createWebHistory } from 'vue-router'
import Home from '../HomePage.vue'
import DictationPage from '../DictationPage.vue'
import PinyinPage from '../PinyinPage.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'Home',
      component: Home
    },

    {
      path: '/dictation',
      name: 'DictationPage',
      component: DictationPage
    },

    {
      path: '/pinyin',
      name: 'Pinyin',
      component: PinyinPage
    }
  ]
})

export default router