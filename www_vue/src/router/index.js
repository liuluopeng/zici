import { createRouter, createWebHistory } from 'vue-router'
import Home from '../HomePage.vue'
import DictationPage from '../DictationPage.vue'
import PinyinPage from '../PinyinPage.vue'
import WordPage from '../WordPage.vue'
import KeyboardLayoutPage from '../KeyboardLayoutPage.vue'

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
    },

    {
      path: '/word',
      name: 'Word',
      component: WordPage
    },

    {
      path: '/keyboard-layout',
      name: 'KeyboardLayout',
      component: KeyboardLayoutPage
    }
  ]
})

export default router