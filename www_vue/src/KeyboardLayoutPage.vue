<script setup>
import { ref } from 'vue';
import KeyboardAnimation from '@/components/keyboard-animation/KeyboardAnimation.vue';
import cnchar from 'cnchar-all';

const initialSounds = [
  { char: 'b', pinyin: 'b', example: '波', mp3Path: '/b.mp3' },
  { char: 'p', pinyin: 'p', example: '坡', mp3Path: '/p.mp3' },
  { char: 'm', pinyin: 'm', example: '摸', mp3Path: '/m.mp3' },
  { char: 'f', pinyin: 'f', example: '佛', mp3Path: '/f.mp3' },
  { char: 'd', pinyin: 'd', example: '的', mp3Path: '/d.mp3' },
  { char: 't', pinyin: 't', example: '特', mp3Path: '/t.mp3' },
  { char: 'n', pinyin: 'n', example: '呢', mp3Path: '/n.mp3' },
  { char: 'l', pinyin: 'l', example: '乐', mp3Path: '/l.mp3' },
  { char: 'g', pinyin: 'g', example: '哥', mp3Path: '/g.mp3' },
  { char: 'k', pinyin: 'k', example: '科', mp3Path: '/k.mp3' },
  { char: 'h', pinyin: 'h', example: '喝', mp3Path: '/h.mp3' },
  { char: 'j', pinyin: 'j', example: '鸡', mp3Path: '/j.mp3' },
  { char: 'q', pinyin: 'q', example: '七', mp3Path: '/q.mp3' },
  { char: 'x', pinyin: 'x', example: '希', mp3Path: '/x.mp3' },
  { char: 'zh', pinyin: 'zh', example: '知', mp3Path: '/zh.mp3' },
  { char: 'ch', pinyin: 'ch', example: '吃', mp3Path: '/ch.mp3' },
  { char: 'sh', pinyin: 'sh', example: '狮', mp3Path: '/sh.mp3' },
  { char: 'r', pinyin: 'r', example: '日', mp3Path: '/r.mp3' },
  { char: 'z', pinyin: 'z', example: '资', mp3Path: '/z.mp3' },
  { char: 'c', pinyin: 'c', example: '疵', mp3Path: '/c.mp3' },
  { char: 's', pinyin: 's', example: '撕', mp3Path: '/s.mp3' },
  { char: 'y', pinyin: 'y', example: '医', mp3Path: '/y.mp3' },
  { char: 'w', pinyin: 'w', example: '乌', mp3Path: '/w.mp3' }
];

const finalSounds = [
  { char: 'a', pinyin: 'a', example: '啊', mp3Path: '/a.mp3' },
  { char: 'o', pinyin: 'o', example: '窝', mp3Path: '/o.mp3' },
  { char: 'e', pinyin: 'e', example: '鹅', mp3Path: '/e.mp3' },
  { char: 'i', pinyin: 'i', example: '衣', mp3Path: '/i.mp3' },
  { char: 'u', pinyin: 'u', example: '乌', mp3Path: '/u.mp3' },
  { char: 'ü', pinyin: 'ü', example: '迂', mp3Path: '/yu1.mp3' },
  { char: 'ai', pinyin: 'ai', example: '爱', mp3Path: '/ai.mp3' },
  { char: 'ei', pinyin: 'ei', example: '飞', mp3Path: '/ei.mp3' },
  { char: 'ui', pinyin: 'ui', example: '水', mp3Path: '/ui.mp3' },
  { char: 'ao', pinyin: 'ao', example: '奥', mp3Path: '/ao.mp3' },
  { char: 'ou', pinyin: 'ou', example: '欧', mp3Path: '/ou.mp3' },
  { char: 'iu', pinyin: 'iu', example: '牛', mp3Path: '/iu.mp3' },
  { char: 'ie', pinyin: 'ie', example: '姐', mp3Path: '/ie.mp3' },
  { char: 'ue', pinyin: 'ue', example: '月', mp3Path: '/yue1.mp3' },
  { char: 'er', pinyin: 'er', example: '儿', mp3Path: '/er.mp3' },
  { char: 'an', pinyin: 'an', example: '安', mp3Path: '/an.mp3' },
  { char: 'en', pinyin: 'en', example: '恩', mp3Path: '/en.mp3' },
  { char: 'in', pinyin: 'in', example: '音', mp3Path: '/in.mp3' },
  { char: 'un', pinyin: 'un', example: '云', mp3Path: '/un.mp3' },
  { char: 'ang', pinyin: 'ang', example: '昂', mp3Path: '/ang.mp3' },
  { char: 'eng', pinyin: 'eng', example: '风', mp3Path: '/eng.mp3' },
  { char: 'ing', pinyin: 'ing', example: '英', mp3Path: '/ing.mp3' },
  { char: 'ong', pinyin: 'ong', example: '东', mp3Path: '/ong.mp3' }
];

const convertToKeyboardInput = (pinyin) => {
  return pinyin.replace(/ü/g, 'v');
};

const speak = (text, mp3Path) => {
  if (mp3Path) {
    const audio = new Audio(mp3Path);
    audio.play().catch(error => {
      console.log('MP3播放失败，使用cnchar发音:', error);
      if (text) {
        cnchar.voice.speak(text);
      }
    });
  } else if (text) {
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
            <div class="pinyin-label" @click="speak(item.example, item.mp3Path)">
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
            <div class="pinyin-label" @click="speak(item.example, item.mp3Path)">
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
