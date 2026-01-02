<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import cnchar from 'cnchar-all';
import HanziWriter from 'hanzi-writer';
// 导入键盘遮罩组件
import KeyboardMask from '@/components/keyboard-mask/KeyboardMask.vue';

// 汉字列表
const characters = ref<string[]>([]);
// 测验是否正确
const quiz_is_right = ref(false);
// 当前汉字
const currentChar = ref('');
// 拼音
const currentPinyin = ref('');
// 词语列表
const topWords = ref<string[]>([]);
// 已经听写过的汉字索引
const usedIndices = ref<number[]>([]);
// 当前在已用列表中的位置
const currentPosition = ref(-1);
// HanziWriter引用
const writerContainer = ref<HTMLElement | null>(null);
let writer: any = null;



// 通知相关变量
const notification = ref({ visible: false, message: '', type: 'info' as 'success' | 'error' | 'info' });

// 键盘遮罩状态
const showKeyboardMask = ref(false);
const clickX = ref(0);
const clickY = ref(0);

// 打开键盘遮罩
const openKeyboardMask = (event) => {
  if (!currentChar.value) return;
  toggleAnswer();
  // 记录点击位置
  clickX.value = event.clientX;
  clickY.value = event.clientY;
  showKeyboardMask.value = true;
};

// 关闭键盘遮罩
const closeKeyboardMask = () => {
  showKeyboardMask.value = false;
};

// 从txt文件加载汉字
const loadCharacters = async () => {
  try {
    const response = await fetch('/生字.txt');
    const text = await response.text();
    // 将文本按字符分割成数组，过滤掉空字符
    characters.value = text.split('').filter(char => char.trim());
    // 设置第一个随机汉字
    selectRandomChar();
  } catch (error) {
    console.error('加载汉字失败:', error);
    // 如果加载失败，使用默认的汉字列表
    characters.value = '天地人你我他一二三四五上下口耳目手足站坐日月水火山石田禾对云雨风花鸟虫六七八九十爸妈马土不画打棋鸡字词语句子桌纸文数学音乐妹奶小桥台雪儿白草家是车路灯走秋气了树叶片大飞会个的船两头在里看见闪星江南可采莲鱼东西北尖说春青蛙夏弯皮冬男女开关正反远有色近听无声去还来多少黄牛只猫边鸭苹果杏桃书包尺作业本笔刀课早校明力尘从众双木林森条心升国旗中红歌起么美丽立午晚昨今年影前后黑狗左右它好朋友比尾巴谁长短把伞兔最公写诗点要过给当串们以成彩半空问到方没更绿出睡那海真老师吗同什才亮时候觉得自己很穿衣服门快蓝又笑着向和贝娃挂活金哥姐弟叔爷群竹牙用几步为参加洞乌鸦处找办旁许法放进高住孩玩吧发芽爬呀久回全变工厂医院生'.split('');
    selectRandomChar();
  }
};

// 处理词语中的汉字，将与当前测验汉字相同的字替换为田字格
const processWord = (word: string): { type: 'char' | 'tianzige'; value: string }[] => {
  return word.split('').map(char => {
    if (char === currentChar.value) {
      return { type: 'tianzige', value: char };
    } else {
      return { type: 'char', value: char };
    }
  });
};

// 随机选择一个汉字
const selectRandomChar = () => {
  if (characters.value.length === 0) return;

  // 如果当前显示答案，先翻回正面
  if (quiz_is_right.value) {
    quiz_is_right.value = false;
    // 等待翻转动画完成后再更新内容（CSS transition时间为0.6s）
    setTimeout(() => {
      loadNextChar();
    }, 600);
    return;
  }

  loadNextChar();
};

// 加载下一个汉字的实际逻辑
const loadNextChar = () => {
  // 如果所有汉字都用过了，重置已用列表
  if (usedIndices.value.length >= characters.value.length) {
    usedIndices.value = [];
  }

  let randomIndex;
  // 随机选择一个未使用过的汉字
  do {
    randomIndex = Math.floor(Math.random() * characters.value.length);
  } while (usedIndices.value.includes(randomIndex));

  // 记录使用过的索引
  usedIndices.value.push(randomIndex);
  // 更新当前位置
  currentPosition.value = usedIndices.value.length - 1;

  // 设置当前汉字
  currentChar.value = characters.value[randomIndex];

  // 同时继续执行原来的词语朗读功能
  const words = cnchar.words(currentChar.value);

  if (words && words.length > 0) {
    // 先打乱词语数组，然后取前3个
    const shuffledWords = [...words];
    // 使用Fisher-Yates洗牌算法打乱数组
    for (let i = shuffledWords.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [shuffledWords[i], shuffledWords[j]] = [shuffledWords[j], shuffledWords[i]];
    }
    topWords.value = shuffledWords.slice(0, 3);

    currentPinyin.value = String(cnchar.spell(currentChar.value));
    quiz_is_right.value = false;
  };
};

// 下一个随机汉字
const nextRandomChar = () => {
  selectRandomChar();
};

// 上一个汉字
const prevChar = () => {
  if (currentPosition.value <= 0) return;

  // 如果当前显示答案，先翻回正面
  if (quiz_is_right.value) {
    quiz_is_right.value = false;
    // 等待翻转动画完成后再更新内容（CSS transition时间为0.6s）
    setTimeout(() => {
      loadPrevChar();
    }, 600);
    return;
  }

  loadPrevChar();
};

// 加载上一个汉字的实际逻辑
const loadPrevChar = () => {
  currentPosition.value--;
  const prevIndex = usedIndices.value[currentPosition.value];
  currentChar.value = characters.value[prevIndex];
  currentPinyin.value = String(cnchar.spell(currentChar.value));

  // 获取上一个汉字的词语
  const words = cnchar.words(currentChar.value);
  if (words && words.length > 0) {
    const shuffledWords = [...words];
    for (let i = shuffledWords.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [shuffledWords[i], shuffledWords[j]] = [shuffledWords[j], shuffledWords[i]];
    }
    topWords.value = shuffledWords.slice(0, 3);
  } else {
    topWords.value = [];
  }

  quiz_is_right.value = false;
};

// 发音
const speakChar = () => {
  console.log('currentChar.value', currentChar.value);
  if (currentChar.value) {
    cnchar.voice.speak(currentChar.value);
  }
};

// 发音指定的文本（单个字符或词语）
const speakText = (text: string) => {
  if (text) {
    cnchar.voice.speak(text);
  }
};

// 显示答案
const toggleAnswer = () => {
  quiz_is_right.value = !quiz_is_right.value;
};



// 显示通知
const showNotification = (message: string, type: 'success' | 'error' | 'info' = 'info') => {
  notification.value = { visible: true, message, type };
  // 1秒后自动隐藏通知
  setTimeout(() => {
    notification.value.visible = false;
  }, 1000);
};

// 初始化HanziWriter
const initHanziWriter = () => {
  if (!writerContainer.value || !currentChar.value) return;

  // 如果已有writer实例，先销毁
  if (writer) {
    try {
      // 检查destroy方法是否存在
      if (typeof writer.destroy === 'function') {
        writer.destroy();
      } else {
        // 如果没有destroy方法，尝试其他可能的清理方式
        console.log('destroy method not found, using alternative cleanup');
        // 移除事件监听器
        writer.off && writer.off('quiz-complete');
        // 清空容器内容
        if (writerContainer.value) {
          writerContainer.value.innerHTML = '';
        }
      }
    } catch (error) {
      console.error('Error destroying writer instance:', error);
    } finally {
      // 确保writer被重置为null
      writer = null;
    }
  }

  // 创建新的writer实例
  writer = HanziWriter.create(writerContainer.value, currentChar.value, {
    width: 250,
    height: 250,
    showCharacter: false,
    padding: 0,
    strokeAnimationSpeed: 2,
    delayBetweenStrokes: 50,
    showOutline: false,
    highlightOnComplete: false,
  });

  // 开始测验
  writer.quiz({
    onMistake: (strokeData) => {
      console.log('Oh no! you made a mistake on stroke ' + strokeData.strokeNum);
      console.log("You've made " + strokeData.mistakesOnStroke + " mistakes on this stroke so far");
      console.log("You've made " + strokeData.totalMistakes + " total mistakes on this quiz");
      console.log("There are " + strokeData.strokesRemaining + " strokes remaining in this character");
      showNotification(`笔画${strokeData.strokeNum}写错了！`, 'error');
    },
    onCorrectStroke: (strokeData) => {
      console.log('Yes!!! You got stroke ' + strokeData.strokeNum + ' correct!');
      console.log('You made ' + strokeData.mistakesOnStroke + ' mistakes on this stroke');
      console.log("You've made " + strokeData.totalMistakes + ' total mistakes on this quiz');
      console.log('There are ' + strokeData.strokesRemaining + ' strokes remaining in this character');
    },
    onComplete: (summaryData) => {
      console.log('You did it! You finished drawing ' + summaryData.character);
      console.log('You made ' + summaryData.totalMistakes + ' total mistakes on this quiz');
      showNotification(`恭喜你，完成了"${summaryData.character}"的书写！`, 'success');
      // 测验完成，翻转卡片显示答案
      quiz_is_right.value = true;
    }
  });
};

// 监听当前汉字变化，更新HanziWriter
watch(currentChar, (newChar) => {
  if (newChar && writerContainer.value) {
    quiz_is_right.value = false;
    // 延迟一下确保DOM已更新
    setTimeout(() => {
      initHanziWriter();
    }, 0);
  }
});

// 组件挂载时加载汉字
onMounted(() => {
  loadCharacters();
});

// 组件卸载时清理资源
onUnmounted(() => {
  if (writer) {
    try {
      if (typeof writer.destroy === 'function') {
        writer.destroy();
      } else {
        writer.off && writer.off('quiz-complete');
        if (writerContainer.value) {
          writerContainer.value.innerHTML = '';
        }
      }
    } catch (error) {
      console.error('Error destroying writer instance on unmount:', error);
    } finally {
      writer = null;
    }
  }
});
</script>

<template>
  <div class="dictation-page">
    <!-- 通知组件 -->
    <div v-if="notification.visible" class="notification" :class="notification.type">
      {{ notification.message }}
    </div>

    <div class="dictation-container">
      <!-- 上一个按钮 -->
      <button class="prev-btn" @click="prevChar" title="上一个">
        ⬅️ 上一个
      </button>

      <!-- 下一个按钮 -->
      <button class="next-btn" @click="nextRandomChar" title="下一个">
        ➡️ 下一个
      </button>
    </div>


    <!-- 主要功能按钮区域 -->
    <div class="primary-btn-group">
      <button class="speak-btn" @click="speakChar" title="播放发音">
        🔊 听发音
      </button>
      <button class="show-answer-btn" @click="toggleAnswer" title="查看答案">
        👁️ 看答案
      </button>
      <button class="show-keyboard-btn" @click="openKeyboardMask" title="查看键位">
        ⌨️ 看键位
      </button>
    </div>

    <div class="tip-group">
      <div class="tip-display" v-for="(word, index) in topWords" :key="index" @click="speakText(word)">
        <span v-for="(item, charIndex) in processWord(word)" :key="charIndex" class="char-with-pinyin">
          <span class="pinyin">{{ cnchar.spell(item.value, 'tone') }}</span>
          <span v-if="item.type === 'char'">{{ item.value }}</span>
          <div v-else class="tianzige">
            <span></span>
            <span></span>
          </div>
        </span>
      </div>
    </div>

    <!-- 中间内容区域 -->
    <div class="main-content">
      <!-- 汉字输入区域容器 -->
      <div class="input-container">
        <!-- 手写输入区域 -->
        <div class="char-display" :class="{ flipped: quiz_is_right }" title="点击查看答案">
          <div class="char-inner">
            <span></span>
            <span></span>
            <div class="char-front">
              <div ref="writerContainer" class="char-writer-container"></div>
            </div>
            <div class="char-back">
              <div class="char">{{ currentChar }}</div>
            </div>
          </div>
        </div>
      </div>





    </div>

  </div>

  <!-- 键盘遮罩组件 -->
  <KeyboardMask :show="showKeyboardMask" :char="currentChar" :click-x="clickX" :click-y="clickY"
    @close="closeKeyboardMask" />
</template>

<style scoped lang="scss">
.dictation-page {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: flex-start;
  padding: 20px 20px 0;
  background-color: #f8f9fa;
  font-family: 'KaiTi SC', 'KaiTi', 'STKaiti', 'SimKai', cursive, sans-serif;
}

.page-title {
  font-size: 2.5rem;
  color: #2c3e50;
  margin-bottom: 40px;
  text-align: center;
  font-weight: bold;
}

.dictation-container {
  display: flex;
  align-items: center;
  justify-content: space-between;
  max-width: 1200px;
  width: 100%;
  gap: 20px;
}

.main-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 15px;
  flex: 1;
  max-width: 600px;
  margin-top: 10px;
}

.char-display {
  width: 250px;
  height: 250px;
  background-color: white;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
  margin-bottom: 20px;
  border: 3px dashed rgba(52, 73, 94, 0.5);
  transition: all 0.3s ease;
  perspective: 1000px;
  /* 创建3D空间 */
  cursor: pointer;
  position: relative;
  overflow: hidden;
}

/* 输入容器样式 */
.input-container {
  display: flex;
  justify-content: center;
  align-items: center;
}

.char-inner {
  width: 100%;
  height: 100%;
  position: relative;
  transform-style: preserve-3d;
  /* 保持3D空间 */
  transition: transform 0.6s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 卡牌格子样式 - 横线 */
.char-inner::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  width: 100%;
  height: 2px;
  background: repeating-linear-gradient(90deg, rgba(52, 73, 94, 0.5) 0, rgba(52, 73, 94, 0.5) 2px, transparent 2px, transparent 5px);
  transform: translateY(-50%);
  z-index: 2;
  pointer-events: none;
}

/* 卡牌格子样式 - 竖线 */
.char-inner::after {
  content: '';
  position: absolute;
  left: 50%;
  top: 0;
  width: 2px;
  height: 100%;
  background: repeating-linear-gradient(180deg, rgba(52, 73, 94, 0.5) 0, rgba(52, 73, 94, 0.5) 2px, transparent 2px, transparent 5px);
  transform: translateX(-50%);
  z-index: 2;
  pointer-events: none;
}

/* 卡牌格子样式 - 45度斜线 */
.char-inner span:first-of-type {
  position: absolute;
  left: -50%;
  top: 50%;
  width: 200%;
  height: 1px;
  background: repeating-linear-gradient(90deg, rgba(52, 73, 94, 0.25) 0, rgba(52, 73, 94, 0.25) 1px, transparent 1px, transparent 3px);
  transform: translateY(-50%) rotate(45deg);
  z-index: 1;
  pointer-events: none;
}

/* 卡牌格子样式 - -45度斜线 */
.char-inner span:last-of-type {
  position: absolute;
  left: -50%;
  top: 50%;
  width: 200%;
  height: 1px;
  background: repeating-linear-gradient(90deg, rgba(52, 73, 94, 0.25) 0, rgba(52, 73, 94, 0.25) 1px, transparent 1px, transparent 3px);
  transform: translateY(-50%) rotate(-45deg);
  z-index: 1;
  pointer-events: none;
}

.char-front,
.char-back {
  position: absolute;
  width: 100%;
  height: 100%;
  backface-visibility: hidden;
  /* 隐藏背面 */
  display: flex;
  align-items: center;
  justify-content: center;
}

.char-back {
  transform: rotateY(180deg);
  /* 背面初始旋转180度 */
}

.char-display.flipped .char-inner {
  transform: rotateY(180deg);
  /* 翻转效果 */
}

.char {
  font-size: 12rem;
  color: #2c3e50;
  font-weight: bold;
  font-family: 'KaiTi SC', 'KaiTi', 'STKaiti', 'SimKai', cursive, sans-serif;
}

.char-writer-container {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
  box-sizing: border-box;
}

.primary-btn-group {
  display: flex;
  gap: 10px;
  justify-content: center;
  align-items: center;
}

.speak-btn,
.prev-btn,
.next-btn,
.show-answer-btn,
.show-keyboard-btn {
  padding: 15px 30px;
  border: none;
  border-radius: 10px;
  font-size: 1.2rem;
  cursor: pointer;
  transition: all 0.3s ease;
  font-weight: 600;
  min-width: 160px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.speak-btn,
.show-answer-btn,
.show-keyboard-btn {
  background-color: #e74c3c;
  color: white;

  &:hover {
    background-color: #c0392b;
    transform: translateY(-2px);
    box-shadow: 0 6px 16px rgba(231, 76, 60, 0.4);
  }

  &:active {
    transform: translateY(0);
  }
}

.show-answer-btn {
  background-color: #3498db;

  &:hover {
    background-color: #2980b9;
    box-shadow: 0 6px 16px rgba(52, 152, 219, 0.4);
  }
}

.show-keyboard-btn {
  background-color: #27ae60;

  &:hover {
    background-color: #229954;
    box-shadow: 0 6px 16px rgba(39, 174, 96, 0.4);
  }
}



.prev-btn {
  background-color: #95a5a6;
  color: white;

  &:hover {
    background-color: #7f8c8d;
    transform: translateY(-2px);
    box-shadow: 0 6px 16px rgba(149, 165, 166, 0.4);
  }

  &:active {
    transform: translateY(0);
  }
}

.next-btn {
  background-color: #3498db;
  color: white;

  &:hover {
    background-color: #2980b9;
    transform: translateY(-2px);
    box-shadow: 0 6px 16px rgba(52, 152, 219, 0.4);
  }

  &:active {
    transform: translateY(0);
  }
}

.tip-display {
  font-size: 1.8rem;
  color: #34495e;
  text-align: center;
  min-height: 3rem;
  font-weight: 500;
  padding: 10px 20px;
  border-radius: 8px;
  background-color: #ecf0f1;
  display: flex;
  align-items: center;
  gap: 4px;
}

.tip-display span {
  display: inline-flex;
  align-items: center;
  gap: 2px;
}

.char-with-pinyin {
  display: inline-flex;
  flex-direction: column;
  align-items: center;
  position: relative;
  justify-content: flex-start;
  min-height: calc(1.8rem + 1rem + 2px);
  /* 汉字高度 + 拼音高度 + 间距 */
}

.pinyin {
  font-size: 0.8rem;
  color: #666;
  margin-bottom: 2px;
  line-height: 1rem;
  white-space: nowrap;
  text-align: center;
  height: 1rem;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  font-family: 'Helvetica Neue', Arial, 'PingFang SC', 'Hiragino Sans GB', 'Microsoft YaHei', sans-serif;
}

/* 确保普通汉字和田字格有相同的垂直对齐 */
.char-with-pinyin>span:not(.pinyin) {
  display: inline-block;
  vertical-align: middle;
  line-height: 1.8rem;
  height: 1.8rem;
  width: 1.8rem;
  text-align: center;
  flex-shrink: 0;
}

.tianzige {
  width: 1.8rem;
  height: 1.8rem;
  border: 2px dashed rgba(52, 73, 94, 0.5);
  position: relative;
  display: inline-block;
  vertical-align: middle;
  margin: 0 2px;
  overflow: hidden;
}

/* 横线 - 只显示一条穿过中点 */
.tianzige::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  width: 100%;
  height: 2px;
  background: repeating-linear-gradient(90deg, rgba(52, 73, 94, 0.5) 0, rgba(52, 73, 94, 0.5) 2px, transparent 2px, transparent 5px);
  transform: translateY(-50%);
  z-index: 2;
}

/* 竖线 - 只显示一条穿过中点 */
.tianzige::after {
  content: '';
  position: absolute;
  left: 50%;
  top: 0;
  width: 2px;
  height: 100%;
  background: repeating-linear-gradient(180deg, rgba(52, 73, 94, 0.5) 0, rgba(52, 73, 94, 0.5) 2px, transparent 2px, transparent 5px);
  transform: translateX(-50%);
  z-index: 2;
}

/* 45度斜线 - 只显示一条穿过中点 */
.tianzige span:first-of-type {
  position: absolute;
  left: -50%;
  top: 50%;
  width: 200%;
  height: 1px;
  background: repeating-linear-gradient(90deg, rgba(52, 73, 94, 0.25) 0, rgba(52, 73, 94, 0.25) 1px, transparent 1px, transparent 3px);
  transform: translateY(-50%) rotate(45deg);
  z-index: 1;
}

/* -45度斜线 - 只显示一条穿过中点 */
.tianzige span:last-of-type {
  position: absolute;
  left: -50%;
  top: 50%;
  width: 200%;
  height: 1px;
  background: repeating-linear-gradient(90deg, rgba(52, 73, 94, 0.25) 0, rgba(52, 73, 94, 0.25) 1px, transparent 1px, transparent 3px);
  transform: translateY(-50%) rotate(-45deg);
  z-index: 1;
}

.tip-group {
  display: flex;
  flex-direction: row;
  gap: 15px;
  margin-top: 10px;
}

@media (max-width: 768px) {
  .page-title {
    font-size: 2rem;
  }

  .dictation-container {
    flex-direction: column;
    gap: 20px;
  }

  /* 响应式调整输入容器 */
  .input-container {
    flex-direction: column;
    gap: 15px;
  }

  .char-display {
    width: 220px;
    height: 220px;
  }

  .char {
    font-size: 9rem;
  }

  .speak-btn,
  .prev-btn,
  .next-btn {
    min-width: 200px;
    padding: 14px 35px;
    font-size: 1.1rem;
  }

  .tip-display {
    font-size: 1.5rem;
  }
}

@media (max-width: 480px) {
  .page-title {
    font-size: 1.8rem;
  }

  .char-display {
    width: 180px;
    height: 180px;
  }

  .char {
    font-size: 7rem;
  }

  .speak-btn,
  .prev-btn,
  .next-btn {
    min-width: 180px;
    padding: 12px 30px;
    font-size: 1rem;
  }

  .tip-display {
    font-size: 1.3rem;
  }
}

/* 通知样式 */
.notification {
  position: fixed;
  top: 20px;
  right: 20px;
  padding: 15px 20px;
  border-radius: 8px;
  font-size: 1.1rem;
  font-weight: 600;
  color: white;
  z-index: 10000;
  opacity: 0.95;
  pointer-events: none;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  transition: all 0.3s ease;
}

.notification.success {
  background-color: #2ecc71;
}

.notification.error {
  background-color: #e74c3c;
}

.notification.info {
  background-color: #3498db;
}
</style>