<script setup>
import { ref, onMounted } from 'vue';
import KeyboardAnimation from '@/components/keyboard-animation/KeyboardAnimation.vue';
import cnchar from 'cnchar-all';

const initialSounds = [
  { char: 'b', pinyin: 'b', example: '波' },
  { char: 'p', pinyin: 'p', example: '坡' },
  { char: 'm', pinyin: 'm', example: '摸' },
  { char: 'f', pinyin: 'f', example: '佛' },
  { char: 'd', pinyin: 'd', example: '的' },
  { char: 't', pinyin: 't', example: '特' },
  { char: 'n', pinyin: 'n', example: '呢' },
  { char: 'l', pinyin: 'l', example: '乐' },
  { char: 'g', pinyin: 'g', example: '哥' },
  { char: 'k', pinyin: 'k', example: '科' },
  { char: 'h', pinyin: 'h', example: '喝' },
  { char: 'j', pinyin: 'j', example: '鸡' },
  { char: 'q', pinyin: 'q', example: '七' },
  { char: 'x', pinyin: 'x', example: '希' },
  { char: 'zh', pinyin: 'zh', example: '知' },
  { char: 'ch', pinyin: 'ch', example: '吃' },
  { char: 'sh', pinyin: 'sh', example: '狮' },
  { char: 'r', pinyin: 'r', example: '日' },
  { char: 'z', pinyin: 'z', example: '资' },
  { char: 'c', pinyin: 'c', example: '疵' },
  { char: 's', pinyin: 's', example: '撕' },
  { char: 'y', pinyin: 'y', example: '医' },
  { char: 'w', pinyin: 'w', example: '乌' }
];

const finalSounds = [
  { char: 'a', pinyin: 'a', example: '啊' },
  { char: 'o', pinyin: 'o', example: '窝' },
  { char: 'e', pinyin: 'e', example: '鹅' },
  { char: 'i', pinyin: 'i', example: '衣' },
  { char: 'u', pinyin: 'u', example: '乌' },
  { char: 'ü', pinyin: 'ü', example: '迂' },
  { char: 'ai', pinyin: 'ai', example: '爱' },
  { char: 'ei', pinyin: 'ei', example: '飞' },
  { char: 'ui', pinyin: 'ui', example: '水' },
  { char: 'ao', pinyin: 'ao', example: '奥' },
  { char: 'ou', pinyin: 'ou', example: '欧' },
  { char: 'iu', pinyin: 'iu', example: '牛' },
  { char: 'ie', pinyin: 'ie', example: '姐' },
  { char: 'ue', pinyin: 'ue', example: '月' },
  { char: 'er', pinyin: 'er', example: '儿' },
  { char: 'an', pinyin: 'an', example: '安' },
  { char: 'en', pinyin: 'en', example: '恩' },
  { char: 'in', pinyin: 'in', example: '音' },
  { char: 'un', pinyin: 'un', example: '云' },
  { char: 'ang', pinyin: 'ang', example: '昂' },
  { char: 'eng', pinyin: 'eng', example: '风' },
  { char: 'ing', pinyin: 'ing', example: '英' },
  { char: 'ong', pinyin: 'ong', example: '东' }
];

const convertToKeyboardInput = (pinyin) => {
  return pinyin.replace(/ü/g, 'v');
};

const speak = (text) => {
  if (text) {
    cnchar.voice.speak(text);
  }
};
</script>

<template>
  <div class="keyboard-layout-page">
    <h1>拼音键位展示</h1>

    <div class="content-container">
      <div class="section">
        <h2>声母键位</h2>
        <div class="sounds-list">
          <div v-for="item in initialSounds" :key="item.pinyin" class="sound-item">
            <div class="pinyin-label" @click="speak(item.example)">
              {{ item.pinyin }}
              <span class="example-char">{{ item.example }}</span>
            </div>
            <KeyboardAnimation :input="convertToKeyboardInput(item.pinyin)" />
          </div>
        </div>
      </div>

      <div class="section">
        <h2>韵母键位</h2>
        <div class="sounds-list">
          <div v-for="item in finalSounds" :key="item.pinyin" class="sound-item">
            <div class="pinyin-label" @click="speak(item.example)">
              {{ item.pinyin }}
              <span class="example-char">{{ item.example }}</span>
            </div>
            <KeyboardAnimation :input="convertToKeyboardInput(item.pinyin)" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.keyboard-layout-page {
  padding: 2rem;
  max-width: 1600px;
  margin: 0 auto;
}

.keyboard-layout-page h1 {
  text-align: center;
  color: #333;
  margin-bottom: 2rem;
}

.content-container {
  display: flex;
  gap: 2rem;
  flex-wrap: wrap;
}

.section {
  flex: 1;
  min-width: 600px;
  background-color: white;
  border-radius: 1rem;
  padding: 1.5rem;
  box-shadow: 0 0.5rem 1rem rgba(0, 0, 0, 0.1);
}

.section h2 {
  color: #333;
  margin-bottom: 1rem;
  font-size: 1.5rem;
}

.sounds-list {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.sound-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  background-color: #fafafa;
  border-radius: 0.8rem;
  padding: 1rem;
  border: 1px solid #e0e0e0;
  border-bottom: 2px solid #e0e0e0;
}

.pinyin-label {
  font-size: 2.5rem;
  font-weight: bold;
  color: #333;
  text-align: center;
  padding: 0.8rem 1.5rem;
  background-color: #f0f0f0;
  border-radius: 0.5rem;
  min-width: 120px;
  cursor: pointer;
  transition: all 0.3s;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.3rem;
}

.pinyin-label:hover {
  background-color: #e0e0ff;
  transform: scale(1.05);
}

.example-char {
  font-size: 1.8rem;
  color: #666;
  font-weight: normal;
}
</style>
