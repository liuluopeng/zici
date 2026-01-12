<script setup lang="ts">
import { ref, onMounted, watch, onUnmounted } from 'vue';
import { KEY_PERMUTATION_ALPHABET } from '@/config/key';

// 导入键盘组件
import SingleKey from '@/components/key/SingleKey.vue';
import KeyWrap from '@/components/key/KeyWrap.vue';

// 定义组件props
const props = defineProps({
  input: {
    type: String,
    default: ''
  }
});

// 键盘按键状态 - 使用本地状态
const keysPressed = ref({});

// 需要用到的键码集合
const activeKeys = ref(new Set<string>());

// 字符到键码的映射
const charToKeyCode = ref({});

// 存储打下的字母序列
const typedSequence = ref('');

// 定时器数组，用于管理所有定时器
const timers = ref<number[]>([]);

// 判断某个键是否需要用到
const isKeyActive = (keyCode: string) => {
  return activeKeys.value.has(keyCode);
};

// 清除所有定时器
const clearAllTimers = () => {
  timers.value.forEach(timer => clearTimeout(timer));
  timers.value = [];
};

// 初始化字符到键码的映射
onMounted(() => {
  const additionalPinyinMapping = {
    'a': 'KeyA',
    'o': 'KeyO',
    'e': 'KeyE',
    'i': 'KeyI',
    'u': 'KeyU',
    'v': 'KeyV',
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

  Object.assign(charToKeyCode.value, additionalPinyinMapping);
});

// 组件卸载时清除所有定时器
onUnmounted(() => {
  clearAllTimers();
});

// 监听输入变化，播放动画
watch(() => props.input, (newInput) => {
  if (newInput) {
    setTimeout(() => {
      playAnimation(newInput);
    }, 500);
  }
}, { immediate: true });

// 播放按键动画 - 循环播放
const playAnimation = (input: string) => {
  clearAllTimers();
  typedSequence.value = '';

  let pinyin = input;
  console.log('输入:', pinyin);

  let pinyinSequence = pinyin.toLowerCase().split('');
  console.log('输入序列:', pinyinSequence);

  const validPinyinChars = 'abcdefghijklmnopqrstuvwxyzv';
  pinyinSequence = pinyinSequence.filter(char => validPinyinChars.includes(char));
  console.log('过滤后的序列:', pinyinSequence);

  if (pinyinSequence.length === 0) return;

  // 统计所有需要用到的键码
  activeKeys.value.clear();
  pinyinSequence.forEach(char => {
    const keyCode = charToKeyCode.value[char];
    if (keyCode) {
      activeKeys.value.add(keyCode);
    }
  });

  const playNextKey = (index: number) => {
    const char = pinyinSequence[index % pinyinSequence.length];

    pressAndReleaseKey(char);

    if ((index + 1) % pinyinSequence.length === 0) {
      const clearTimer = window.setTimeout(() => {
        typedSequence.value = '';
        // 一轮结束后，清空活动键码集合，恢复所有按键
        activeKeys.value.clear();
        const nextRoundTimer = window.setTimeout(() => {
          // 新一轮开始前，重新统计需要用到的键码
          activeKeys.value.clear();
          pinyinSequence.forEach(char => {
            const keyCode = charToKeyCode.value[char];
            if (keyCode) {
              activeKeys.value.add(keyCode);
            }
          });
          playNextKey(index + 1);
        }, 500);
        timers.value.push(nextRoundTimer);
      }, 500);
      timers.value.push(clearTimer);
    } else {
      const timer = window.setTimeout(() => {
        playNextKey(index + 1);
      }, 500);
      timers.value.push(timer);
    }
  };

  playNextKey(0);
};

const pressAndReleaseKey = (char: string) => {
  const keyCode = charToKeyCode.value[char];
  if (!keyCode) return;

  keysPressed.value[keyCode] = true;
  typedSequence.value += char;

  const timer = window.setTimeout(() => {
    keysPressed.value[keyCode] = false;
  }, 100);

  timers.value.push(timer);
};
</script>

<template>
  <div class="keyboard-animation">
    <key-wrap title="" className="y-key-wrap__standard y-key-wrap__alphabet keyboard-wrap-animation">
      <div class="typed-sequence-display">{{ typedSequence }}</div>

      <template v-for="(value, key) in KEY_PERMUTATION_ALPHABET" :key="key">
        <div class="y-keyboard__wrap y-keyboard__wrap--alphabet" :class="['y-keyboard__' + key]">
          <div class="y-keyboard__line y-keyboard__line--alphabet" v-for="(v, index) in value" :key="index + 'line'">
            <single-key v-for="item in v" :key="item.code" :code="item.code" :value="item.value" :unit="item.unit"
              :keys-pressed="keysPressed" :is-active-key="isKeyActive(item.code)"></single-key>
          </div>
        </div>
      </template>
    </key-wrap>
  </div>
</template>

<style scoped lang="scss">
.keyboard-animation {
  width: 100%;
  background-color: transparent;
}

.keyboard-wrap-animation {
  position: relative;
  width: 100%;
  margin: 0;
  padding: 0.6rem 0.9rem;
  background: #f0f2eb;
  border-radius: 0.6rem;
  box-shadow: 0.2rem 0.2rem 0.4rem rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  transform: none;
}

.keyboard-wrap-animation.y-key-wrap__alphabet {
  width: 100%;
  margin: 0;
  padding: 0.8rem 1.2rem;
  background: #f0f2eb;
  border-radius: 0.8rem;
  box-shadow: 0.2rem 0.2rem 0.4rem rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  align-items: center;
}

.keyboard-wrap-animation .y-keyboard__line--alphabet {
  display: flex;
  justify-content: flex-start;
  margin-bottom: 0.4rem;
  flex-wrap: nowrap;
  width: 100%;
  align-items: center;
}

.keyboard-wrap-animation .y-keyboard__line--alphabet .y-single-key {
  margin: 0.2rem 0.15rem;
}

.keyboard-wrap-animation .y-keyboard__wrap--alphabet {
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.keyboard-wrap-animation .y-keyboard__line--alphabet:nth-child(1) {
  justify-content: flex-start;
}

.keyboard-wrap-animation .y-keyboard__line--alphabet:nth-child(2) {
  margin-left: 1.8rem;
}

.keyboard-wrap-animation .y-keyboard__line--alphabet:nth-child(3) {
  margin-left: 4.1rem;
}

@media (max-width: 768px) {
  .keyboard-wrap-animation {
    max-width: 90vw;
  }
}

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
