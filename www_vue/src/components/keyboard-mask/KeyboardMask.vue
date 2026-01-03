<script setup lang="ts">
import { ref, onMounted, watch, onUnmounted, reactive } from 'vue';
import { KEY_PERMUTATION_ALPHABET, CAN_PRINT_KEY } from '@/config/key';
import cnchar from 'cnchar-all';

// 导入键盘组件
import SingleKey from '@/components/key/SingleKey.vue';
import KeyWrap from '@/components/key/KeyWrap.vue';

// 导入store
import { useConfigStore } from '@/store/config';
import { storeToRefs } from 'pinia';

const configStore = useConfigStore();
const { printContent, currentCode } = storeToRefs(configStore);

// 定义组件props
const props = defineProps({
  show: {
    type: Boolean,
    default: false
  },
  char: {
    type: String,
    default: ''
  },
  clickX: {
    type: Number,
    default: 0
  },
  clickY: {
    type: Number,
    default: 0
  }
});

// 定义组件events
const emit = defineEmits(['close']);

// 键盘按键状态
const keysPressed = ref({});

// 字符到键码的映射
const charToKeyCode = ref({});

// 存储打下的字母序列
const typedSequence = ref('');

// 定时器数组，用于管理所有定时器
const timers = ref<number[]>([]);

// 键盘位置
const keyboardPosition = reactive({
  top: 0,
  left: 0
});

// 监听点击位置和显示状态的变化，更新键盘位置
watch(() => [props.show, props.clickX, props.clickY], ([isShowing, x, y]) => {
  if (isShowing && x && y) {
    // 计算键盘位置，使其在点击位置附近显示
    // 键盘默认居中显示，如果点击位置在屏幕边缘，会自动调整
    keyboardPosition.top = y + 20; // 点击位置下方20px (注意：这里使用px是因为是相对于点击位置的像素偏移，不需要响应式)
    keyboardPosition.left = x;     // 点击位置的水平位置
  } else if (!isShowing) {
    // 当遮罩隐藏时，清除字符序列
    typedSequence.value = '';
  }
});

// 清除所有定时器
const clearAllTimers = () => {
  timers.value.forEach(timer => clearTimeout(timer));
  timers.value = [];
};

// 初始化字符到键码的映射
onMounted(() => {
  // 从CAN_PRINT_KEY反向映射
  for (const [keyCode, char] of Object.entries(CAN_PRINT_KEY)) {
    charToKeyCode.value[char.toLowerCase()] = keyCode;
  }

  // 添加更多拼音字符映射（包括声调字符）
  const additionalPinyinMapping = {
    'a': 'KeyA',
    'o': 'KeyO',
    'e': 'KeyE',
    'i': 'KeyI',
    'u': 'KeyU',
    'v': 'KeyV',
    'ü': 'KeyV', // 通常用v代替ü
    'b': 'KeyB',
    'p': 'KeyP',
    'm': 'KeyM',
    'f': 'KeyF',
    'd': 'KeyD',
    't': 'KeyT',
    'n': 'KeyN',
    'l': 'KeyL',
    'g': 'KeyG',
    'k': 'KeyK',
    'h': 'KeyH',
    'j': 'KeyJ',
    'q': 'KeyQ',
    'x': 'KeyX',
    'z': 'KeyZ',
    'c': 'KeyC',
    's': 'KeyS',
    'r': 'KeyR',
    'y': 'KeyY',
    'w': 'KeyW'
  };

  // 合并映射
  Object.assign(charToKeyCode.value, additionalPinyinMapping);
});

// 组件卸载时清除所有定时器
onUnmounted(() => {
  clearAllTimers();
});

// 当字符变化时，如果遮罩是显示状态，播放拼音动画
watch(() => [props.char, props.show], ([newChar, isShowing]) => {
  if (newChar && isShowing) {
    // 延迟执行按键动画
    setTimeout(() => {
      playPinyinAnimation(newChar);
    }, 500);
  }
});

// 关闭键盘遮罩
const closeKeyboardMask = () => {
  // 清空按键状态，解决阴影残留问题
  keysPressed.value = {};
  // 清除所有定时器
  clearAllTimers();
  // 清除字符序列
  typedSequence.value = '';
  emit('close');
};

// 播放拼音按键动画 - 循环播放
const playPinyinAnimation = (char: string) => {
  // 先清除所有现有的定时器
  clearAllTimers();

  // 清除字符序列
  typedSequence.value = '';

  // 获取拼音
  const pinyin = cnchar.spell(char);
  console.log('拼音:', pinyin);

  // 分解拼音为字符序列
  let pinyinSequence = pinyin.toLowerCase().split('');
  console.log('拼音序列:', pinyinSequence);

  // 过滤掉声调符号和其他不需要的字符
  const validPinyinChars = 'abcdefghijklmnopqrstuvwxyzü';
  pinyinSequence = pinyinSequence.filter(char => validPinyinChars.includes(char));
  console.log('过滤后的拼音序列:', pinyinSequence);

  // 如果没有有效的拼音字符，直接返回
  if (pinyinSequence.length === 0) return;

  // 循环播放按键
  const playNextKey = (index: number) => {
    const char = pinyinSequence[index % pinyinSequence.length];

    pressAndReleaseKey(char);

    // 设置下一个按键的定时器
    // 当轮播结束后，先清空字符序列，然后再开始新一轮循环
    if ((index + 1) % pinyinSequence.length === 0) {
      // 轮播结束时，先清空字符序列
      const clearTimer = window.setTimeout(() => {
        typedSequence.value = '';
        // 然后再开始新一轮循环
        const nextRoundTimer = window.setTimeout(() => {
          playNextKey(index + 1);
        }, 500); // 新一轮开始前的延迟
        timers.value.push(nextRoundTimer);
      }, 500); // 轮播结束后显示空状态的时间
      timers.value.push(clearTimer);
    } else {
      // 普通按键延迟
      const timer = window.setTimeout(() => {
        playNextKey(index + 1);
      }, 500);
      timers.value.push(timer);
    }
  };

  // 开始播放
  playNextKey(0);
};

// 按下并释放键
const pressAndReleaseKey = (char: string) => {
  const keyCode = charToKeyCode.value[char];
  if (!keyCode) return;

  // 按下键
  keysPressed.value[keyCode] = true;
  configStore.setCurrentCode(keyCode);

  // 设置打印内容
  configStore.setPrintContent(char);

  // 将字符添加到序列中
  typedSequence.value += char;

  // 100ms后释放键
  const timer = window.setTimeout(() => {
    keysPressed.value[keyCode] = false;
  }, 100);

  // 保存定时器到数组
  timers.value.push(timer);
};
</script>

<template>
  <div v-if="show" class="keyboard-mask" @click="closeKeyboardMask">
    <key-wrap title="" className="y-key-wrap__standard y-key-wrap__alphabet keyboard-wrap-mask"
      :style="{ top: `${keyboardPosition.top}px`, left: `${keyboardPosition.left}px` }" @click.stop>
      <!-- 显示打下的字符序列 -->
      <div class="typed-sequence-display">{{ typedSequence }}</div>

      <template v-for="(value, key) in KEY_PERMUTATION_ALPHABET" :key="key">
        <div class="y-keyboard__wrap y-keyboard__wrap--alphabet" :class="['y-keyboard__' + key]">
          <div class="y-keyboard__line y-keyboard__line--alphabet" v-for="(v, index) in value" :key="index + 'line'">
            <single-key v-for="item in v" :key="item.code" :code="item.code" :value="item.value" :unit="item.unit"
              :keys-pressed="keysPressed"></single-key>
          </div>
        </div>
      </template>
    </key-wrap>
  </div>
</template>

<style scoped lang="scss">
/* 键盘遮罩样式 */
.keyboard-mask {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: transparent;
  z-index: 1000;
}

/* 直接使用key-wrap作为遮罩的核心内容 */
.keyboard-wrap-mask {
  position: absolute;
  width: fit-content;
  margin: 0;
  padding: 0.8rem 1.2rem;
  background: #f0f2eb;
  border-radius: 0.8rem;
  box-shadow: 0.2rem 0.2rem 0.4rem rgba(0, 0, 0, 0.1);
  transform: translateX(-50%);
  display: flex;
  flex-direction: column;
}

/* 字母键盘特定样式 */
.keyboard-wrap-mask.y-key-wrap__alphabet {
  width: fit-content;
  margin: 0;
  padding: 0.8rem 1.2rem;
  background: #f0f2eb;
  border-radius: 0.8rem;
  box-shadow: 0.2rem 0.2rem 0.4rem rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}

/* 键盘行样式 */
.keyboard-wrap-mask .y-keyboard__line--alphabet {
  display: flex;
  justify-content: flex-start;
  margin-bottom: 0.6rem;
  flex-wrap: nowrap;
  width: 100%;
  align-items: center;
}

/* 为字母键盘按键添加水平间距 */
.keyboard-wrap-mask .y-keyboard__line--alphabet .y-single-key {
  margin: 0.3rem 0.2rem;
}

/* 确保键盘区域与字符序列垂直对齐 */
.keyboard-wrap-mask .y-keyboard__wrap--alphabet {
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
}

/* 第一行字母（QWERTYUIOP）左对齐 */
.keyboard-wrap-mask .y-keyboard__line--alphabet:nth-child(1) {
  justify-content: flex-start;
}

/* 第二行字母（ASDFGHJKL）相对第一行居中 */
.keyboard-wrap-mask .y-keyboard__line--alphabet:nth-child(2) {
  margin-left: 3.2rem;
}

/* 第三行字母（ZXCVBNM）相对第一行居中 */
.keyboard-wrap-mask .y-keyboard__line--alphabet:nth-child(3) {
  margin-left: 7.2rem;
}

/* 确保键盘在屏幕边缘点击时不会超出视口 */
@media (max-width: 768px) {
  .keyboard-wrap-mask {
    max-width: 90vw;
    left: 50% !important;
    transform: translateX(-50%);
  }
}

/* 字符序列显示样式 */
.typed-sequence-display {
  text-align: left;
  font-size: 3.6rem;
  margin-bottom: 0.8rem;
  padding: 0.4rem 0 0.4rem 1.4rem;
  color: #333;
  min-height: 4.8rem;
  width: 100%;
  position: relative;
  z-index: 1001;
  font-weight: bold;
  display: flex;
  align-items: center;
  justify-content: flex-start;
}
</style>