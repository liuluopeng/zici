<script setup lang="ts">
import { reactive, onMounted, onBeforeUnmount } from 'vue';
import { KEY_PERMUTATION_STANDARD } from '@/config/key';

// components
import SingleKey from '@/components/key/SingleKey.vue';
import KeyWrap from '@/components/key/KeyWrap.vue';

// stores
import { storeToRefs } from 'pinia';
import { useConfigStore } from '@/store/config';

// types
import type { KeyBoardType, SystemType } from '@/types';

const configStore = useConfigStore();
const { printContent, currentCode } = storeToRefs(configStore);

const state = reactive({
  currentKeyBoard: 'standard' as KeyBoardType,
  currentSystem: 'win' as SystemType,
  keysPressed: {} as Record<string, boolean>,
  isPlaying: true
});

// 字符到键码的映射
const charToKeyCode: Record<string, string> = {
  h: 'KeyH',
  e: 'KeyE',
  l: 'KeyL',
  o: 'KeyO'
};

// 定义hello的字符序列
const helloSequence = ['h', 'e', 'l', 'l', 'o'];
let currentIndex = 0;
let animationInterval: number | null = null;

// 按下并释放键的函数
const pressAndReleaseKey = (char: string) => {
  const keyCode = charToKeyCode[char];
  if (!keyCode) return;

  // 按下键
  state.keysPressed[keyCode] = true;
  configStore.setCurrentCode(keyCode);

  // 设置打印内容
  configStore.setPrintContent(char);

  // 100ms后释放键
  setTimeout(() => {
    state.keysPressed[keyCode] = false;
  }, 100);
};

// 播放hello动画的函数
const playHelloAnimation = () => {
  if (!state.isPlaying) return;

  // 按下当前字符
  pressAndReleaseKey(helloSequence[currentIndex]);

  // 移动到下一个字符
  currentIndex++;
  if (currentIndex >= helloSequence.length) {
    currentIndex = 0;
    // 完成一轮后清空打印内容
    setTimeout(() => {
      configStore.clearPrintContent();
    }, 500);
  }
};

// 控制动画播放
const toggleAnimation = () => {
  state.isPlaying = !state.isPlaying;
  if (state.isPlaying) {
    animationInterval = window.setInterval(playHelloAnimation, 500);
  } else {
    if (animationInterval) {
      clearInterval(animationInterval);
      animationInterval = null;
    }
  }
};

onMounted(() => {
  // 启动动画
  animationInterval = window.setInterval(playHelloAnimation, 500);
});

onBeforeUnmount(() => {
  // 清理动画
  if (animationInterval) {
    clearInterval(animationInterval);
    animationInterval = null;
  }
});
</script>
<template>
  <main>
    <div class="y-keyboard__setting-wrap">
      <div class="y-keyboard__setting-item" @click="toggleAnimation">
        {{ state.isPlaying ? '暂停动画' : '开始动画' }}
      </div>
    </div>
    <div class="y-main__screen-wrap">
      <div class="y-main__screen" :class="[printContent ? 'y-main__screen--word' : '']">
        <div class="y-main__screen-main" v-html="printContent"></div>
      </div>
      <div class="y-main__sub-screen" :class="[currentCode.length ? 'y-main__sub-screen--word' : '']">
        <div class="y-main__sub-screen-text" v-html="currentCode"></div>
      </div>
    </div>
    <key-wrap v-if="state.currentKeyBoard === 'standard'" title="Hello 动画键盘" className="y-key-wrap__standard">
      <template v-for="(value, key) in KEY_PERMUTATION_STANDARD" :key="key">
        <div class="y-keyboard__wrap" :class="['y-keyboard__' + key]">
          <div class="y-keyboard__line" v-for="(v, index) in value" :key="index + 'line'">
            <single-key v-for="item in v" :key="item.code"
              :code="state.currentSystem === 'mac' && item.macCode ? item.macCode : item.code"
              :value="state.currentSystem === 'mac' && item.macValue ? item.macValue : item.value" :unit="item.unit"
              :keys-pressed="state.keysPressed"></single-key>
          </div>
        </div>
      </template>
    </key-wrap>
  </main>
</template>
<style lang="scss">
$gray-06: #999999;
$main-color: #15c5ce;
$box-shadow-color: rgba(255, 255, 255, 0.2);
$key-box-shadow: 0.2rem 0.2rem 0.4rem rgba(255, 255, 255, 0.2);

.y-keyboard__setting-wrap {
  margin-bottom: 3rem;
  display: flex;
  justify-content: center;
  align-items: center;
  height: 2.4rem;
  font-size: 1.6rem;
  color: $gray-06;
}

.y-keyboard__setting-item {
  cursor: pointer;
  padding: 0.8rem 1.6rem;
  background: $main-color;
  color: white;
  border-radius: 0.4rem;
  transition: background-color 0.3s;

  &:hover {
    opacity: 0.8;
  }
}

.y-main__screen-wrap {
  display: flex;
  margin: 2rem auto;
  justify-content: center;
  align-items: flex-end;
  position: relative;
}

.y-main__screen {
  max-width: 45vw;
  width: 60rem;
  height: 5rem;
  overflow: auto;
  font-size: 2.4rem;
  line-height: 3rem;
  font-weight: 600;
  color: $gray-06;
  word-break: break-all;
  border-radius: 0.8rem;
  transition: all 0.3s cubic-bezier(0.55, 0, 0.1, 1);

  &.y-main__screen--word {
    box-shadow: 0.2rem 0.2rem 0.5rem 0.2rem $box-shadow-color;
    transform: translate3d(0, -0.6rem, 0);
  }

  &::-webkit-scrollbar {
    display: none;
  }
}

.y-main__screen-main {
  width: 100%;
  padding: 2rem 3rem 0;
}

.y-main__sub-screen {
  margin-left: 3rem;
  width: 19rem;
  height: 8rem;
  overflow: hidden;
  font-size: 1.4rem;
  line-height: 2.4rem;
  font-weight: 600;
  color: $gray-06;
  word-break: break-all;
  border-radius: 0.8rem;
  transition: all 0.3s cubic-bezier(0.55, 0, 0.1, 1);

  &.y-main__sub-screen--word {
    box-shadow: 0.2rem 0.2rem 0.5rem 0.2rem $box-shadow-color;
    transform: translate3d(0, -0.6rem, 0);
  }

  &::-webkit-scrollbar {
    display: none;
  }
}

.y-main__sub-screen-text {
  margin-top: -1.6rem;
  padding-left: 1rem;
}

.y-key-wrap {
  box-sizing: content-box;
  padding: 0.8rem 1.2rem;
  background: #f0f2eb;
  border-radius: 0.8rem;
  box-shadow: $key-box-shadow;
  margin: 0 auto;
}

.y-key-wrap__standard {
  width: 100rem;

  .y-keyboard__main-area {
    width: 65rem;
  }

  .y-keyboard__sub-area {
    width: 13rem;
  }

  .y-keyboard__number-area {
    width: 18rem;
  }
}

.y-keyboard__wrap {
  flex-shrink: 0;
}

.y-keyboard__line {
  display: flex;
  justify-content: space-between;
  width: 100%;
}
</style>
