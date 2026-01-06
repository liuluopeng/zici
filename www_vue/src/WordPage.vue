<script setup>
import cnchar from 'cnchar-all';
import { onMounted, ref, watch } from 'vue';
import { useRouter } from 'vue-router';

// 导入wasm初始化函数和函数
import init, { my_console_log, get_new_words } from 'my-wasm';

// 标记wasm是否已初始化
const wasmInitialized = ref(false);

// 初始化wasm模块
onMounted(async () => {
  try {
    await init();
    wasmInitialized.value = true;
    console.log('wasm模块初始化成功');
    // WASM初始化完成后，自动加载和绘制词语
    loadAndDrawWords();
  } catch (error) {
    console.error('wasm模块初始化失败:', error);
  }
});

// Gallery相关状态
const currentIndex = ref(0);
const currentWord = ref('');
const currentPinyin = ref('');
const validWords = ref([]);

let wordsArray = [];

// 初始化路由
const router = useRouter();

// 上一个词语
const previousWord = () => {
  if (currentIndex.value > 0) {
    currentIndex.value--;
    updateCurrentWord();
  }
};

// 下一个词语
const nextWord = () => {
  if (currentIndex.value < validWords.value.length - 1) {
    currentIndex.value++;
    updateCurrentWord();
  }
};

// 跳转到指定词语
const goToWord = (index) => {
  currentIndex.value = index;
  updateCurrentWord();
};

// 更新当前显示的词语
const updateCurrentWord = () => {
  const word = validWords.value[currentIndex.value];
  currentWord.value = word;
  currentPinyin.value = cnchar.spell(word, 'tone');
};

// 加载和绘制词语的函数
const loadAndDrawWords = () => {
  if (!wasmInitialized.value) {
    console.warn('wasm模块未初始化，跳过词语加载');
    return;
  }

  // 从wasm获取词语
  try {
    wordsArray = get_new_words();
    console.log('从wasm获取的词语数组:', wordsArray);
  } catch (error) {
    console.error('调用wasm函数失败:', error);
  }

  // 验证所有词语都是中文且长度为2-4个字
  const isValidWord = (word) => {
    return /^[\u4e00-\u9fa5]{2,4}$/.test(word);
  };

  // 过滤出有效的中文词语
  validWords.value = Array.from(wordsArray).filter(isValidWord);
  console.log('有效中文词语:', validWords.value);

  // 初始化显示第一个词语
  if (validWords.value.length > 0) {
    currentIndex.value = 0;
    updateCurrentWord();
  } else {
    console.error('从wasm获取词语失败');
    currentWord.value = '加载失败';
    currentPinyin.value = '';
  }
};

// 发音指定的文本（词语）
const speakText = (text) => {
  if (text) {
    cnchar.voice.speak(text);
  }
};
</script>

<template>
  <div class="word-page">
    <h1>生词</h1>

    <!-- 词语列表 -->
    <div class="words-list">
      <h3>所有词语</h3>
      <div class="words-container">
        <div v-for="(word, index) in validWords" :key="index" class="word-item"
          :class="{ 'active': index === currentIndex }" @click="goToWord(index)">
          {{ word }}
        </div>
      </div>
    </div>

    <!-- Gallery视图 -->
    <div class="gallery-container">
      <!-- 导航按钮 -->
      <button class="nav-button prev-button" @click="previousWord" :disabled="currentIndex === 0">
        上一个
      </button>

      <!-- 当前词语展示区域 -->
      <div class="current-word">
        <div class="word-display">
          <h1 class="word">{{ currentWord }}</h1>
          <h2 class="pinyin" @click="speakText(currentPinyin)">{{ currentPinyin }}</h2>
        </div>
      </div>

      <!-- 导航按钮 -->
      <button class="nav-button next-button" @click="nextWord" :disabled="currentIndex === validWords.length - 1">
        下一个
      </button>
    </div>

    <!-- 进度指示器 -->
    <div class="progress-indicator">
      <span>{{ currentIndex + 1 }} / {{ validWords.length }}</span>
    </div>
  </div>
</template>

<style scoped>
.word-page h1 {
  color: #333;
  text-align: center;
}

.gallery-container {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 2rem;
  margin: 2rem 0;
  flex-wrap: wrap;
}

.nav-button {
  padding: 1rem 2rem;
  font-size: 1.5rem;
  border: none;
  border-radius: 0.5rem;
  background-color: #44f;
  color: white;
  cursor: pointer;
  transition: background-color 0.3s;
}

.nav-button:hover {
  background-color: #33d;
}

.nav-button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

.current-word {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2rem;
  padding: 2rem;
  background-color: white;
  border-radius: 1rem;
  box-shadow: 0 0.5rem 1rem rgba(0, 0, 0, 0.1);
  max-width: 800px;
  width: 100%;
}

.word-display {
  text-align: center;
}

.word-display .word {
  font-size: 6rem;
  margin: 0;
  color: #333;
}

.word-display .pinyin {
  font-size: 2rem;
  color: #666;
  margin: 0.5rem 0 0 0;
}

.progress-indicator {
  text-align: center;
  font-size: 1.2rem;
  color: #666;
  margin-top: 1rem;
}

/* 词语列表样式 */
.words-list {
  margin: 1.5rem 0;
  text-align: center;
}

.words-list h3 {
  font-size: 1.3rem;
  color: #333;
  margin-bottom: 0.8rem;
}

.words-container {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 0.5rem;
  max-width: 1200px;
  margin: 0 auto;
  padding: 0.8rem;
  background-color: #f5f5f5;
  border-radius: 0.8rem;
}

.word-item {
  padding: 0.5rem 1rem;
  font-size: 1.2rem;
  background-color: white;
  border: 1px solid #ddd;
  border-radius: 0.4rem;
  cursor: pointer;
  transition: all 0.3s;
  min-width: 80px;
  text-align: center;
}

.word-item:hover {
  border-color: #44f;
  background-color: #f0f0ff;
}

.word-item.active {
  border-color: #44f;
  background-color: #44f;
  color: white;
  font-weight: bold;
  box-shadow: 0 0 10px rgba(68, 68, 255, 0.5);
}
</style>
