<script setup lang="ts">
import { SUB_VALUE } from '@/config/key';
import { reactive, watch, computed } from 'vue';

import type { KEY_PERMUTATION_VALUE } from '@/config/key';

type Props = {
  unit: number;
  backgroundColor: string;
  color: string;
  value: string;
  code: string;
  keysPressed: any;
  heightType: string;
  type: string;
  area: Array<Array<KEY_PERMUTATION_VALUE>> | null;
  currentSystem: string;
  isActiveKey: boolean;
};

const props = withDefaults(defineProps<Partial<Props>>(), {
  unit: 1,
  backgroundColor: '#4F5767',
  color: '#fff',
  value: '',
  code: '',
  keysPressed: {},
  heightType: 'normal',
  type: 'normal',
  area: null,
  currentSystem: 'win',
  isActiveKey: true
});

const state = reactive({
  isActive: false // 是否已经被点击过了
});

watch(
  () => props.keysPressed,
  (val) => {
    if (val) {
      const triggeredKeys = Object.keys(val);
      if (triggeredKeys.includes(props.code)) {
        state.isActive = true;
      }
    }
  },
  {
    deep: true
  }
);

const isKeyPressed = computed(() => {
  return props.keysPressed[props.code];
});

const subValue = computed(() => {
  return SUB_VALUE[props.value];
});
</script>
<template>
  <div class="y-single-key__inner" v-if="type === 'inner' && area">
    <div class="y-single-key__inner-item" v-for="(v, i) in area" :key="i">
      <single-key v-for="item in v" :key="item.code" height-type="half"
        :code="currentSystem === 'mac' && item.macCode ? item.macCode : item.code"
        :value="currentSystem === 'mac' && item.macValue ? item.macValue : item.value" :unit="item.unit"
        :keys-pressed="keysPressed"></single-key>
    </div>
  </div>
  <div v-else class="y-single-key" :class="[
    code ? '' : 'y-single-key--empty',
    'y-single-key--' + unit,
    subValue ? 'y-single-key__small-size' : '',
    value.length > 1 ? 'y-single-key__word' : 'y-single-key__letter',
    state.isActive ? 'y-single-key--active' : '',
    isKeyPressed ? 'y-single-key--pressed' : '',
    heightType === 'half' ? 'y-single-key--half' : '',
    !isActiveKey ? 'y-single-key--inactive' : ''
  ]" :style="{
    backgroundColor,
    color
  }">
    <span v-if="subValue" class="y-single-key__sub-value">{{ subValue }}</span>
    <span class="y-single-key__value">{{ value }}</span>
  </div>
</template>
<style lang="scss">
$unit: 2.2rem;

.y-single-key {
  display: flex;
  align-items: center;
  width: $unit;
  /* 键帽宽度 */
  height: $unit;
  /* 键帽高度 */
  font-size: 1.8rem;
  /* 字体大小 */
  font-weight: bold;
  border-radius: 0.5rem;
  box-shadow: 0.2rem 0.2rem 0.4rem rgba(0, 0, 0, 0.4);
  /* 键帽阴影效果 */
  padding: 0.6rem;
  margin: 0.4rem 0;
  position: relative;

  &:after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    border-radius: 0.5rem;
    background-color: rgba(255, 255, 255, 0);
    transition: background-color 0.1s;
  }
}

.y-single-key--half {
  height: calc($unit / 2);
  margin: 0.1rem 0;
}

.y-single-key--active {
  &:after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    border-radius: 0.5rem;
    background-color: rgba(255, 255, 255, 0.2);
  }
}

.y-single-key--pressed {
  &:after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    border-radius: 0.5rem;
    background-color: rgba(255, 255, 255, 0.6);
  }
}

.y-single-key--inactive {
  opacity: 0.3;
  filter: brightness(0.5);
  transition: opacity 0.3s, filter 0.3s;
}

.y-single-key--empty {
  background: rgba(0, 0, 0, 0) !important;
  box-shadow: none !important;

  &:after {
    content: none;
  }
}

.y-single-key__small-size {
  font-size: 1.2rem;
  flex-direction: column;
  align-items: flex-start;
  justify-content: space-between;
}

/* 这是单词类型的，比如 Enter Esc F1，字号需要小一点                                                                                                                           */
.y-single-key__word {
  font-size: 1.2rem;
}

.y-single-key__letter {
  align-items: start;
  line-height: 1rem;
}

.y-single-key--1 {
  width: $unit;
}

.y-single-key--125 {
  width: $unit * 1.25;
}

.y-single-key--15 {
  width: $unit * 1.5;
}

.y-single-key--175 {
  width: $unit * 1.75;
}

.y-single-key--2 {
  width: $unit * 2;
}

.y-single-key--225 {
  width: $unit * 2.25;
}

.y-single-key--25 {
  width: $unit * 2.5;
}

.y-single-key--275 {
  width: $unit * 2.75;
}

.y-single-key--3 {
  width: $unit * 3;
}

.y-single-key--6 {
  width: $unit * 6;
}

.y-single-key--7 {
  width: $unit * 7;
}

.y-single-key__inner {
  display: flex;
  flex-direction: column;
}

.y-single-key__inner-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
</style>
