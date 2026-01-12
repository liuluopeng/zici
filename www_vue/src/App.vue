<script setup>
import { useRouter, useRoute } from 'vue-router';

const router = useRouter();
const route = useRoute();






</script>

<template>
  <div class="app-container">
    <!-- 路由导航 -->
    <nav class="nav">
      <router-link :to="{ path: '/', query: route.query }">生字</router-link>
      <router-link :to="{ path: '/pinyin', query: route.query }">拼音</router-link>
      <router-link :to="{ path: '/word', query: route.query }">生词</router-link>
      <router-link :to="{ path: '/dictation', query: route.query }">听写</router-link>
      <router-link :to="{ path: '/keyboard-layout', query: route.query }">键位</router-link>
    </nav>

    <!-- 路由出口 -->
    <router-view />
  </div>
</template>

<style scoped>
.app-container {
  width: 100%;
  margin: 0;
  padding: 20px;
}

.nav {
  margin-bottom: 20px;
  display: flex;
  gap: 10px;
}

.nav a {
  padding: 10px 20px;
  background-color: #4CAF50;
  color: white;
  text-decoration: none;
  border-radius: 4px;
  font-size: 16px;
  transition: background-color 0.3s;
}

.nav a:hover {
  background-color: #45a049;
}

.nav a.router-link-active {
  background-color: #3e8e41;
  font-weight: bold;
}

h1 {
  color: #333;
}

#draw-area {
  margin-top: 20px;
  padding: 20px;
  background-color: #f5f5f5;
  border-radius: 8px;
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 20px;
  justify-items: center;
}
</style>
