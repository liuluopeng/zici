import { createRouter, createWebHistory, createWebHashHistory } from 'vue-router'
import Home from '../HomePage.vue'
import DictationPage from '../DictationPage.vue'
import WordPage from '../WordPage.vue'
import KeyboardLayoutPage from '../KeyboardLayoutPage.vue'
import PinyinPickerPage from '../PinyinPickerPage.vue'

const router = createRouter({
  history: createWebHashHistory(),
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
      path: '/word',
      name: 'Word',
      component: WordPage
    },

    {
      path: '/keyboard-layout',
      name: 'KeyboardLayout',
      component: KeyboardLayoutPage
    },

    {
      path: '/pinyin-picker',
      name: 'PinyinPicker',
      component: PinyinPickerPage
    }
  ]
})

export default router