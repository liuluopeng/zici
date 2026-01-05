<script setup>
import cnchar from 'cnchar-all';
import { onMounted, ref, watch } from 'vue';
import { useRouter } from 'vue-router';
// 导入键盘遮罩组件
import KeyboardMask from '@/components/keyboard-mask/KeyboardMask.vue';



// 导入wasm函数和初始化函数
import init, { my_console_log, get_new_chars } from 'my-wasm';

// 标记wasm是否已初始化
const wasmInitialized = ref(false);

// 初始化wasm模块
onMounted(async () => {
  try {
    await init();
    wasmInitialized.value = true;
    console.log('wasm模块初始化成功');
    // WASM初始化完成后，自动加载和绘制汉字
    loadAndDrawCharacters();
  } catch (error) {
    console.error('wasm模块初始化失败:', error);
  }
});





// 学期选择
const currentGrade = ref(6);
const currentTerm = ref(2);

// Gallery相关状态
const currentIndex = ref(0);
const currentCharacter = ref('');
const currentPinyin = ref('');
const currentWords = ref([]);
const validCharacters = ref([]);

let charactersArray = [];

// 初始化路由
const router = useRouter();

// 移除了不再需要的键盘遮罩打开和关闭函数
// 现在直接在组件中传递当前字符

// 上一个汉字
const previousChar = () => {
  if (currentIndex.value > 0) {
    currentIndex.value--;
    updateCurrentCharacter();
  }
};

// 下一个汉字
const nextChar = () => {
  if (currentIndex.value < validCharacters.value.length - 1) {
    currentIndex.value++;
    updateCurrentCharacter();
  }
};

// 跳转到指定汉字
const goToCharacter = (index) => {
  currentIndex.value = index;
  updateCurrentCharacter();
};

// 更新当前显示的汉字
const updateCurrentCharacter = () => {
  const char = validCharacters.value[currentIndex.value];
  currentCharacter.value = char;
  currentPinyin.value = cnchar.spell(char, 'tone');

  // 获取组词
  const words = cnchar.words(char);
  if (words && words.length > 0) {
    // 先打乱词语数组，然后取前10个
    const shuffledWords = [...words];
    for (let i = shuffledWords.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [shuffledWords[i], shuffledWords[j]] = [shuffledWords[j], shuffledWords[i]];
    }
    currentWords.value = shuffledWords.slice(0, 10);
  } else {
    currentWords.value = [];
  }

  // 重绘当前汉字的笔画
  drawCurrentCharacter();
};

// 绘制当前汉字的笔画
const drawCurrentCharacter = () => {
  const drawArea = document.getElementById('draw-area');
  if (!drawArea) return;

  // 清空绘制区域
  drawArea.innerHTML = '';

  // 创建新的绘制容器
  const charContainer = document.createElement('div');
  charContainer.style.textAlign = 'center';
  charContainer.style.padding = '1rem';
  drawArea.appendChild(charContainer);

  var option = {
    clear: false,
    el: charContainer,
    style: {
      radicalColor: '#44f',
      backgroundColor: '#eee',
      length: 150
    },
    type: cnchar.draw.TYPE.ANIMATION,
    animation: {
      strokeAnimationSpeed: 2.5,
      delayBetweenStrokes: 1,
      loopAnimate: true,
    }
  };

  try {
    cnchar.draw(currentCharacter.value, option);
  } catch (error) {
    console.error('绘制字符失败:', currentCharacter.value, error);
  }
};

// 学期选择变化时的处理函数
const handleTermChange = (event) => {
  // 从选中的值解析出年级和学期
  const [grade, term] = event.target.value.split('-').map(Number);
  currentGrade.value = grade;
  currentTerm.value = term;
  loadAndDrawCharacters();
};

// 监听年级或学期变化，更新路由参数
watch([currentGrade, currentTerm], ([newGrade, newTerm]) => {
  // 更新当前页面的路由参数
  router.replace({
    query: {
      term: `${newGrade}-${newTerm}`
    }
  });
});

// 加载和绘制汉字的函数
const loadAndDrawCharacters = () => {
  if (!wasmInitialized.value) {
    console.warn('wasm模块未初始化，跳过汉字加载');
    return;
  }

  // 从wasm获取汉字
  try {
    charactersArray = get_new_chars(currentGrade.value, currentTerm.value);
    console.log('从wasm获取的汉字数组:', charactersArray);
  } catch (error) {
    console.error('调用wasm函数失败:', error);
  }

  // 验证所有字符都是中文
  const isValidChinese = (char) => {
    return /^[\u4e00-\u9fa5]$/.test(char);
  };

  // 过滤出有效的中文字符
  validCharacters.value = Array.from(charactersArray).filter(isValidChinese);
  console.log('有效中文字符:', validCharacters.value);

  // 初始化显示第一个汉字
  if (validCharacters.value.length > 0) {
    currentIndex.value = 0;
    updateCurrentCharacter();
  } else {
    console.error('从wasm获取汉字失败');
    currentCharacter.value = '加载失败';
    currentPinyin.value = '';
    currentWords.value = [];
    drawCurrentCharacter();
  }
};

// 发音指定的文本（单个字符或词语）
const speakText = (text) => {
  if (text) {
    cnchar.voice.speak(text);
  }
};

onMounted(() => {
  // 页面加载时从路由参数获取学期选择
  const routeTerm = router.currentRoute.value.query.term;
  if (routeTerm) {
    const [grade, term] = String(routeTerm).split('-').map(Number);
    currentGrade.value = grade;
    currentTerm.value = term;
  }
  // 不再在此处直接调用loadAndDrawCharacters()，改为在WASM初始化完成后调用
});
</script>

<template>
  <div class="home-page">
    <h1>{{ currentGrade }}年级{{ currentTerm === 1 ? '上' : '下' }}学期 生字</h1>

    <!-- 学期选择框 -->
    <div class="term-selector">
      <select :value="`${currentGrade}-${currentTerm}`" @change="handleTermChange">
        <option value="1-1">1年级上学期</option>
        <option value="1-2">1年级下学期</option>
        <option value="2-1">2年级上学期</option>
        <option value="2-2">2年级下学期</option>
        <option value="3-1">3年级上学期</option>
        <option value="3-2">3年级下学期</option>
        <option value="4-1">4年级上学期</option>
        <option value="4-2">4年级下学期</option>
        <option value="5-1">5年级上学期</option>
        <option value="5-2">5年级下学期</option>
        <option value="6-1">6年级上学期</option>
        <option value="6-2">6年级下学期</option>
      </select>
    </div>

    <!-- 汉字列表 -->
    <div class="characters-list">
      <h3>所有汉字</h3>
      <div class="characters-container">
        <div v-for="(char, index) in validCharacters" :key="index" class="character-item"
          :class="{ 'active': index === currentIndex }" @click="goToCharacter(index)">
          {{ char }}
        </div>
      </div>
    </div>

    <!-- Gallery视图 -->
    <div class="gallery-container">
      <!-- 导航按钮 -->
      <button class="nav-button prev-button" @click="previousChar" :disabled="currentIndex === 0">
        上一个
      </button>

      <!-- 当前汉字展示区域 -->
      <div class="current-character">
        <div class="char-display">
          <h1 class="char">{{ currentCharacter }}</h1>
          <h2 class="pinyin" @click="speakText(currentPinyin)">{{ currentPinyin }}</h2>
        </div>

        <!-- 笔画和键盘并排展示 -->
        <div class="stroke-keyboard-container">
          <!-- 笔画展示 -->
          <div class="stroke-display">
            <h3>笔画演示</h3>
            <div id="draw-area"></div>
          </div>

          <!-- 键盘展示 -->
          <div class="keyboard-display">
            <div class="keyboard-header">
              <h3>拼音键盘</h3>
            </div>
            <KeyboardMask :show="true" :char="currentCharacter" :click-x="0" :click-y="0" />
          </div>
        </div>

        <!-- 组词展示 -->
        <div class="word-groups">
          <h3>组词</h3>
          <div class="words-container">
            <div v-for="(word, index) in currentWords" :key="index" class="word-item" @click="speakText(word)">
              {{ word }}
            </div>
            <div v-if="currentWords.length === 0" class="no-words">
              未找到相关词语
            </div>
          </div>
        </div>
      </div>

      <!-- 导航按钮 -->
      <button class="nav-button next-button" @click="nextChar" :disabled="currentIndex === validCharacters.length - 1">
        下一个
      </button>
    </div>

    <!-- 进度指示器 -->
    <div class="progress-indicator">
      <span>{{ currentIndex + 1 }} / {{ validCharacters.length }}</span>
    </div>
  </div>
</template>

<style scoped>
.home-page h1 {
  color: #333;
  text-align: center;
}

.term-selector {
  display: flex;
  justify-content: center;
  margin: 2rem 0;
  padding: 1.5rem;
  background-color: #fff;
  border-radius: 0.8rem;
  box-shadow: 0 0.2rem 0.4rem rgba(0, 0, 0, 0.1);
}

.term-selector select {
  padding: 1.2rem 2rem;
  font-size: 1.8rem;
  border: 0.1rem solid #ddd;
  border-radius: 0.6rem;
  background-color: #fff;
  cursor: pointer;
  transition: all 0.3s;
  min-width: 20rem;
}

.term-selector select:hover {
  border-color: #44f;
}

.term-selector select:focus {
  outline: none;
  border-color: #44f;
  box-shadow: 0 0 0 2px rgba(68, 68, 255, 0.2);
}

#draw-area {
  margin-top: 2rem;
  padding: 2rem;
  background-color: #f5f5f5;
  border-radius: 0.8rem;
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 300px;
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

.current-character {
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

.char-display {
  text-align: center;
}

.char-display .char {
  font-size: 6rem;
  margin: 0;
  color: #333;
}

.char-display .pinyin {
  font-size: 2rem;
  color: #666;
  margin: 0.5rem 0 0 0;
}

.stroke-keyboard-container {
  display: flex;
  gap: 2rem;
  width: 100%;
  flex-wrap: wrap;
  justify-content: center;
}

.stroke-display,
.keyboard-display {
  flex: 1;
  min-width: 300px;
  text-align: center;
}

.word-groups {
  width: 100%;
  text-align: center;
}

.stroke-display h3,
.keyboard-display h3,
.word-groups h3 {
  font-size: 1.5rem;
  color: #333;
  margin: 0 0 1rem 0;
}

.keyboard-display {
  margin-top: 0;
}

.keyboard-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.toggle-keyboard-btn {
  padding: 0.5rem 1rem;
  font-size: 1rem;
  border: none;
  border-radius: 0.5rem;
  background-color: #44f;
  color: white;
  cursor: pointer;
  transition: background-color 0.3s;
}

.toggle-keyboard-btn:hover {
  background-color: #33d;
}

.words-container {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 1rem;
  margin-top: 1rem;
}

.word-item {
  padding: 0.5rem 1rem;
  background-color: #f0f0f0;
  border-radius: 0.5rem;
  font-size: 1.2rem;
  color: #333;
}

.no-words {
  color: #999;
  font-style: italic;
}

.progress-indicator {
  text-align: center;
  font-size: 1.2rem;
  color: #666;
  margin-top: 1rem;
}

/* 汉字列表样式 */
.characters-list {
  margin: 1.5rem 0;
  text-align: center;
}

.characters-list h3 {
  font-size: 1.3rem;
  color: #333;
  margin-bottom: 0.8rem;
}

.characters-container {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 0.3rem;
  max-width: 1200px;
  margin: 0 auto;
  padding: 0.8rem;
  background-color: #f5f5f5;
  border-radius: 0.8rem;
}

.character-item {
  padding: 0.5rem 0.8rem;
  font-size: 1.2rem;
  background-color: white;
  border: 1px solid #ddd;
  border-radius: 0.4rem;
  cursor: pointer;
  transition: all 0.3s;
  min-width: 35px;
  text-align: center;
}

.character-item:hover {
  border-color: #44f;
  background-color: #f0f0ff;
}

.character-item.active {
  border-color: #44f;
  background-color: #44f;
  color: white;
  font-weight: bold;
  box-shadow: 0 0 10px rgba(68, 68, 255, 0.5);
}
</style>