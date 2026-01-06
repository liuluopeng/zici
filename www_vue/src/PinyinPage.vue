<script setup>
import { ref, onMounted } from 'vue';
import cnchar from 'cnchar-all';

// 定义声母、韵母和完整拼音列表
const initialConsonants = ref(['b', 'p', 'm', 'f', 'd', 't', 'n', 'l', 'g', 'k', 'h', 'j', 'q', 'x', 'zh', 'ch', 'sh', 'r', 'z', 'c', 's', 'y', 'w']);

// 声母对应的完整拼音映射表
const consonantFullPinyin = {
  'b': 'bo',
  'p': 'po',
  'm': 'mo',
  'f': 'fo',
  'd': 'de',
  't': 'te',
  'n': 'ne',
  'l': 'le',
  'g': 'ge',
  'k': 'ke',
  'h': 'he',
  'j': 'ji',
  'q': 'qi',
  'x': 'xi',
  'zh': 'zhi',
  'ch': 'chi',
  'sh': 'shi',
  'r': 'ri',
  'z': 'zi',
  'c': 'ci',
  's': 'si',
  'y': 'yi',
  'w': 'wu'
};
const finalVowels = ref(['a', 'o', 'e', 'i', 'u', 'ü', 'ai', 'ei', 'ui', 'ao', 'ou', 'iu', 'ie', 'üe', 'er', 'an', 'en', 'in', 'un', 'ün', 'ang', 'eng', 'ing', 'ong', 'ia', 'iao', 'iang', 'ua', 'uan', 'uang', 'uo']);

// 完整拼音列表（包含所有可能的组合和声调）
const allPinyin = ref([]);

// 拼音表格数据，二维数组：[声母][韵母] = 拼音组合
const pinyinTableData = ref([]);

// 拼音字典：key是声母，value是可搭配的韵母数组
const pinyinDictionary = ref({
  'b': ['a', 'o', 'i', 'u', 'ai', 'ei', 'ao', 'an', 'ie', 'en', 'in', 'ang', 'eng', 'ing', 'ian', 'iao',],
  'p': ['a', 'o', 'i', 'u', 'ai', 'ei', 'ao', 'ou', 'ie', 'an', 'en', 'in', 'ang', 'eng', 'ing', 'iao', 'ian'],
  'm': ['a', 'o', 'e', 'i', 'u', 'ai', 'ei', 'ao', 'ou', 'ie', 'an', 'en', 'in', 'ang', 'eng', 'ing', 'iao', 'ian'],
  'f': ['a', 'o', 'u', 'ei', 'ou', 'an', 'en', 'ang', 'eng',],
  'd': ['a', 'e', 'i', 'u', 'ai', 'ei', 'ui', 'ao', 'ou', 'iu', 'ie', 'an', 'en', 'un', 'ang', 'eng', 'ing', 'ong', 'iao', 'ian', 'uan', 'uo'],
  't': ['a', 'e', 'i', 'u', 'ai', 'ei', 'ao', 'ou', 'ie', 'an', 'un', 'ang', 'eng', 'ing', 'ong', 'iao', 'ian', 'uan', 'uo'],
  'n': ['a', 'e', 'i', 'u', 'ü', 'ai', 'ei', 'ao', 'iu', 'ie', 'üe', 'an', 'en', 'in', 'un', 'ang', 'eng', 'ing', 'ong', 'iao', 'ian', 'iang', 'uan', 'uo'],
  'l': ['a', 'e', 'i', 'u', 'ü', 'ai', 'ei', 'ao', 'ou', 'iu', 'ie', 'üe', 'an', 'in', 'un', 'ang', 'eng', 'ing', 'ong', 'iao', 'ian', 'iang', 'uan', 'uo'],
  'g': ['a', 'e', 'u', 'ai', 'ei', 'ui', 'ao', 'ou', 'an', 'en', 'un', 'ang', 'eng', 'ong', 'ua', 'uan', 'uang', 'uo'],
  'k': ['a', 'e', 'u', 'ai', 'ui', 'ao', 'ou', 'an', 'en', 'un', 'ang', 'eng', 'ong', 'ua', 'uan', 'uang', 'uo'],
  'h': ['a', 'e', 'u', 'ai', 'ui', 'ao', 'ou', 'an', 'en', 'un', 'ang', 'eng', 'ong', 'ua', 'uan', 'uang', 'uo'],
  'j': ['i', 'u', 'iu', 'ie', 'ue', 'ia', 'ie', 'iao', 'ian', 'in', 'iang', 'iong',],
  'q': ['i', 'u', 'iu', 'ie', 'ue', 'ia', 'ie', 'iao', 'ian', 'in', 'iang', 'iong',],
  'x': ['i', 'u', 'iu', 'ie', 'ue', 'ia', 'ie', 'iao', 'ian', 'in', 'iang', 'iong',],
  'zh': ['a', 'e', 'u', 'ai', 'ao', 'ou', 'an', 'en', 'un', 'ang', 'eng', 'ong'],
  'ch': ['a', 'e', 'u', 'ai', 'ao', 'ou', 'an', 'en', 'un', 'ang', 'eng', 'ong'],
  'sh': ['a', 'e', 'u', 'ai', 'ao', 'ou', 'an', 'en', 'un', 'ang', 'eng',],
  'r': ['e', 'u', 'ao', 'ou', 'an', 'en', 'un', 'ang', 'eng', 'ong'],
  'z': ['a', 'e', 'u', 'ai', 'ei', 'ao', 'ou', 'an', 'en', 'un', 'ang', 'eng', 'ong'],
  'c': ['a', 'e', 'u', 'ai', 'ao', 'ou', 'an', 'en', 'un', 'ang', 'eng', 'ong'],
  's': ['a', 'e', 'u', 'ai', 'ao', 'ou', 'an', 'en', 'un', 'ang', 'eng', 'ong'],
  'y': ['i', 'u', 'a', 'o', 'e', 'ao', 'ou', 'an', 'in', 'un', 'ang', 'ing', 'ong', 'uan',],
  'w': ['a', 'o', 'u', 'ai', 'ei', 'an', 'en', 'ang', 'eng',]
});



// 生成带声调的拼音和对应的数字声调标记
const generateTonedPinyin = (pinyin) => {
  // 定义声调符号
  const toneMarks = {
    'a': ['a', 'ā', 'á', 'ǎ', 'à'],
    'o': ['o', 'ō', 'ó', 'ǒ', 'ò'],
    'e': ['e', 'ē', 'é', 'ě', 'è'],
    'i': ['i', 'ī', 'í', 'ǐ', 'ì'],
    'u': ['u', 'ū', 'ú', 'ǔ', 'ù'],
    'ü': ['ü', 'ǖ', 'ǘ', 'ǚ', 'ǜ']
  };

  // 定义拼音中需要添加声调的优先级：a > o > e > i > u > ü
  const priorityOrder = ['a', 'o', 'e', 'i', 'u', 'ü'];

  // 为每个拼音生成四个声调的版本
  const tonedPinyin = [];

  // 找出拼音中需要添加声调的元音
  let targetVowel = '';
  for (const vowel of priorityOrder) {
    if (pinyin.includes(vowel)) {
      targetVowel = vowel;
      break;
    }
  }

  // 处理ü的情况，将其转换为v用于数字声调标记
  const pinyinForNumberTone = pinyin.replace('ü', 'v');

  if (targetVowel) {
    // 为找到的元音添加四个声调
    for (let tone = 1; tone <= 4; tone++) {
      const tonedVowel = toneMarks[targetVowel][tone];
      const newPinyin = pinyin.replace(targetVowel, tonedVowel);
      // 创建数字声调标记版本（如shang4）
      const numberTonePinyin = pinyinForNumberTone + tone;
      tonedPinyin.push({
        withToneMark: newPinyin,
        withNumberTone: numberTonePinyin
      });
    }
  } else {
    // 如果没有找到元音（这种情况很少见），使用数字声调作为后备
    for (let tone = 1; tone <= 4; tone++) {
      const numberTonePinyin = pinyinForNumberTone + tone;
      tonedPinyin.push({
        withToneMark: pinyin + tone,
        withNumberTone: numberTonePinyin
      });
    }
  }

  return tonedPinyin;
};

// 获取拼音对应的示例汉字
const getExampleChar = (pinyinWithNumberTone) => {
  try {
    // 使用cnchar.spellToWord()获取拼音对应的汉字
    const chars = cnchar.spellToWord(pinyinWithNumberTone, 'simple');
    // 返回第一个找到的汉字，如果没有找到则返回空字符串
    return chars && chars.length > 0 ? chars[0] : '';
  } catch (error) {
    console.error('获取示例汉字失败:', error);
    return '';
  }
};

// 点击发音函数
const speakText = (pinyin) => {
  if (!pinyin) return;

  // 检查是否是韵母
  if (finalVowels.value.includes(pinyin)) {
    // 韵母直接发音
    cnchar.voice.speak(pinyin);
    return;
  }

  // 检查是否是声母完整拼音
  const isConsonantFullPinyin = Object.values(consonantFullPinyin).includes(pinyin);
  if (isConsonantFullPinyin) {
    // 声母完整拼音直接发音
    cnchar.voice.speak(pinyin);
    return;
  }

  // 数字声调标记（如 a1, bo2）
  const exampleChar = getExampleChar(pinyin);
  if (exampleChar) {
    cnchar.voice.speak(exampleChar);
  }
};

// 生成拼音表格数据
const generatePinyinTableData = () => {
  const tableData = [];

  // 遍历所有声母
  initialConsonants.value.forEach(consonant => {
    const row = [];

    // 遍历所有韵母
    finalVowels.value.forEach(vowel => {
      // 检查该声母是否可以搭配该韵母
      if (pinyinDictionary.value[consonant] && pinyinDictionary.value[consonant].includes(vowel)) {
        // 如果组合合法，生成带声调的拼音
        const fullPinyin = consonant + vowel;
        row.push(generateTonedPinyin(fullPinyin));
      } else {
        // 如果组合不合法，添加null
        row.push(null);
      }
    });

    tableData.push(row);
  });

  return tableData;
};

onMounted(() => {
  // 生成拼音表格数据
  pinyinTableData.value = generatePinyinTableData();
});
</script>

<template>
  <div class="pinyin-page">
    <header class="page-header">
      <h1>拼音列表</h1>
    </header>


    <section class="pinyin-section">
      <h2>韵母（点击发音）</h2>
      <div class="vowel-grid">
        <div v-for="(vowel, index) in finalVowels" :key="index" class="vowel-item" @click="speakText(vowel)">
          <div class="vowel-text">{{ vowel }}</div>
        </div>
      </div>
    </section>

    <main class="page-content">
      <section class="pinyin-section">
        <h2>声母（点击发音）</h2>
        <div class="vowel-grid">
          <div v-for="(consonant, index) in initialConsonants" :key="index" class="vowel-item"
            @click="speakText(consonantFullPinyin[consonant])">
            <div class="vowel-text">{{ consonant }}</div>
          </div>
        </div>
      </section>


      <section class="pinyin-section">
        <h2>所有拼音表格</h2>
        <div class="pinyin-table-container">
          <table class="pinyin-table">
            <!-- 表头：韵母 -->
            <thead>
              <tr>
                <th></th> <!-- 空白单元格，用于显示声母 -->
                <th v-for="(vowel, index) in finalVowels" :key="index" class="vowel-header">
                  {{ vowel }}
                </th>
              </tr>
            </thead>
            <tbody>
              <!-- 表格内容：行是声母，列是韵母 -->
              <tr v-for="(row, rowIndex) in pinyinTableData" :key="rowIndex">
                <td class="consonant-label">{{ initialConsonants[rowIndex] }}</td>
                <td v-for="(cell, colIndex) in row" :key="colIndex" class="pinyin-cell">
                  <div v-if="cell" class="pinyin-simple" @click="speakText(cell[0].withNumberTone)">
                    <div class="pinyin-text">{{cell[0].withToneMark.replace(/[āáǎàōóǒòēéěèīíǐìūúǔùǖǘǚǜ]/g, (match) => {
                      const toneMap =
                      {
                        'ā': 'a', 'á': 'a', 'ǎ': 'a', 'à': 'a', 'ō': 'o', 'ó': 'o', 'ǒ': 'o', 'ò': 'o', 'ē': 'e', 'é':
                          'e', 'ě': 'e', 'è': 'e', 'ī': 'i', 'í': 'i', 'ǐ': 'i', 'ì': 'i', 'ū': 'u', 'ú': 'u', 'ǔ': 'u',
                        'ù': 'u', 'ǖ': 'ü', 'ǘ': 'ü', 'ǚ': 'ü', 'ǜ': 'ü'
                      };
                      return toneMap[match] || match;
                    })}}</div>
                    <div v-if="getExampleChar(cell[0].withNumberTone)" class="example-char">
                      {{ getExampleChar(cell[0].withNumberTone) }}
                    </div>
                  </div>
                  <div v-else class="empty-cell"></div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>
    </main>
  </div>
</template>

<style scoped lang="scss">
.pinyin-page {
  min-height: 100vh;
  background-color: #ffffff;
  padding: 0;
}

.page-header {
  text-align: center;
  margin-bottom: 2rem;

  h1 {
    font-size: 2.5rem;
    color: #333;
    margin: 0;
  }
}

.page-content {
  width: 100%;
  margin: 0;
}

.pinyin-section {
  background-color: white;
  border-radius: 8px;
  padding: 1rem;
  margin-bottom: 2rem;
  width: 100%;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);

  h2 {
    font-size: 1.8rem;
    color: #555;
    margin-bottom: 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 2px solid #e0e0e0;
  }
}

.pinyin-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(80px, 1fr));
  gap: 1rem;
}

.vowel-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(80px, 1fr));
  gap: 1rem;
}

.vowel-item {
  background-color: #f0f8ff;
  border: 1px solid #b3e5fc;
  border-radius: 8px;
  padding: 1.5rem;
  text-align: center;
  font-size: 1.5rem;
  color: #1976d2;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;

  &:hover {
    background-color: #b3e5fc;
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }
}

.vowel-text {
  font-size: 1.5rem;
  color: #1976d2;
  font-weight: bold;
}

.vowel-group {
  display: flex;
  flex-direction: column;
  align-items: center;
  background-color: #f0f8ff;
  border: 1px solid #b3e5fc;
  border-radius: 8px;
  padding: 1rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.vowel-header {
  font-size: 1.4rem;
  font-weight: bold;
  color: #1976d2;
  margin-bottom: 0.8rem;
}

.vowel-tones {
  display: flex;
  flex-direction: row;
  gap: 0.5rem;
  justify-content: center;
}

.pinyin-item {
  background-color: #e8f5e9;
  border: 1px solid #c8e6c9;
  border-radius: 4px;
  padding: 0.5rem;
  text-align: center;
  font-size: 1.2rem;
  color: #2e7d32;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;

  &:hover {
    background-color: #c8e6c9;
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }

  .pinyin-text {
    margin-bottom: 0.2rem;
  }

  .example-char {
    font-size: 1.5rem;
    color: #1976d2;
    font-weight: bold;
  }
}

// 拼音表格样式
.pinyin-table-container {
  overflow-x: auto;
  margin-top: 1rem;
  width: 100%;
  max-width: none;
}

.pinyin-table {
  width: 3000px;
  border-collapse: collapse;
  table-layout: auto;
}

.pinyin-table th,
.pinyin-table td {
  border: 1px solid #e0e0e0;
  padding: 0.5rem;
  text-align: center;
}

.pinyin-table th {
  background-color: #f0f8ff;
  font-weight: bold;
  color: #1976d2;
  font-size: 1rem;
  position: sticky;
  top: 0;
  z-index: 10;
}

.vowel-header {
  width: 100px;
}

.consonant-label {
  background-color: #e8f5e9;
  font-weight: bold;
  color: #2e7d32;
  position: sticky;
  left: 0;
  z-index: 5;
}

.pinyin-cell {
  min-width: 100px;
  background-color: #fafafa;
  height: 60px;
  vertical-align: middle;
}

.pinyin-tone-group {
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
  align-items: center;
  justify-content: center;
  height: 100%;
}

.pinyin-simple {
  background-color: #e8f5e9;
  border: 1px solid #c8e6c9;
  border-radius: 4px;
  padding: 0.5rem;
  text-align: center;
  font-size: 1.2rem;
  color: #2e7d32;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;

  &:hover {
    background-color: #c8e6c9;
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }
}

.empty-cell {
  background-color: #f5f5f5;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .pinyin-page {
    padding: 1rem;
    background-color: #ffffff;
  }

  .vowel-grid {
    grid-template-columns: repeat(auto-fill, minmax(80px, 1fr));
    gap: 0.8rem;
  }

  .vowel-item {
    padding: 1rem;
    font-size: 1.2rem;
  }

  .vowel-text {
    font-size: 1.2rem;
  }

  .pinyin-cell {
    min-width: 80px;
    max-width: 80px;
    height: 50px;
  }

  .pinyin-text {
    font-size: 1rem;
  }

  .example-char {
    font-size: 1.2rem;
  }
}
</style>