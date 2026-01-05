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
const finalVowels = ref(['a', 'o', 'e', 'i', 'u', 'ü', 'ai', 'ei', 'ui', 'ao', 'ou', 'iu', 'ie', 'üe', 'er', 'an', 'en', 'in', 'un', 'ün', 'ang', 'eng', 'ing', 'ong']);

// 完整拼音列表（包含所有可能的组合和声调）
const allPinyin = ref([]);

// 拼音表格数据，二维数组：[韵母][声母] = 拼音组合
const pinyinTableData = ref([]);

// 检查拼音组合是否可能存在
const isPossiblePinyin = (consonant, vowel) => {
  // 过滤掉不可能的组合
  return !(consonant === 'j' && (vowel === 'u' || vowel === 'uo' || vowel === 'ong')) &&
    !(consonant === 'q' && (vowel === 'u' || vowel === 'uo' || vowel === 'ong')) &&
    !(consonant === 'x' && (vowel === 'u' || vowel === 'uo' || vowel === 'ong')) &&
    !(consonant === 'zh' && vowel === 'i') &&
    !(consonant === 'ch' && vowel === 'i') &&
    !(consonant === 'sh' && vowel === 'i') &&
    !(consonant === 'r' && vowel === 'i');
};

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
const speakText = (pinyinWithNumberTone) => {
  if (pinyinWithNumberTone) {
    // 首先获取拼音对应的示例汉字
    const exampleChar = getExampleChar(pinyinWithNumberTone);
    if (exampleChar) {
      // 使用示例汉字进行发音
      cnchar.voice.speak(exampleChar);
    }
  }
};

// 生成所有可能的拼音组合
const generateAllPinyin = () => {
  const pinyinList = [];

  // 添加单韵母
  finalVowels.value.forEach(vowel => {
    generateTonedPinyin(vowel).forEach(toned => {
      pinyinList.push(toned);
    });
  });

  // 添加声母+韵母组合
  initialConsonants.value.forEach(consonant => {
    finalVowels.value.forEach(vowel => {
      if (isPossiblePinyin(consonant, vowel)) {
        generateTonedPinyin(consonant + vowel).forEach(toned => {
          pinyinList.push(toned);
        });
      }
    });
  });

  // 去重并排序
  return [...new Set(pinyinList)].sort();
};

// 生成拼音表格数据
const generatePinyinTableData = () => {
  const tableData = [];

  // 遍历所有韵母
  finalVowels.value.forEach(vowel => {
    const row = [];

    // 遍历所有声母
    initialConsonants.value.forEach(consonant => {
      if (isPossiblePinyin(consonant, vowel)) {
        // 如果组合可能存在，生成带声调的拼音
        const fullPinyin = consonant + vowel;
        row.push(generateTonedPinyin(fullPinyin));
      } else {
        // 如果组合不可能存在，添加null
        row.push(null);
      }
    });

    tableData.push(row);
  });

  return tableData;
};

onMounted(() => {
  // 生成所有拼音
  allPinyin.value = generateAllPinyin();
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
        <div v-for="(vowel, index) in finalVowels" :key="index" class="vowel-group">
          <div class="vowel-header">{{ vowel }}</div>
          <div class="vowel-tones">
            <div v-for="(tonedVowel, toneIndex) in generateTonedPinyin(vowel)" :key="toneIndex" class="pinyin-item"
              @click="speakText(tonedVowel.withNumberTone)">
              <div class="pinyin-text">{{ tonedVowel.withToneMark }}</div>
              <div v-if="getExampleChar(tonedVowel.withNumberTone)" class="example-char">{{
                getExampleChar(tonedVowel.withNumberTone) }}</div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <main class="page-content">
      <section class="pinyin-section">
        <h2>声母（点击发音）</h2>
        <div class="vowel-grid">
          <div v-for="(consonant, index) in initialConsonants" :key="index" class="vowel-group">
            <div class="vowel-header">{{ consonant }}</div>
            <div class="vowel-tones">
              <div v-for="(tonedPinyin, toneIndex) in generateTonedPinyin(consonantFullPinyin[consonant])"
                :key="toneIndex" class="pinyin-item" @click="speakText(tonedPinyin.withNumberTone)">
                <div class="pinyin-text">{{ tonedPinyin.withToneMark }}</div>
                <div v-if="getExampleChar(tonedPinyin.withNumberTone)" class="example-char">{{
                  getExampleChar(tonedPinyin.withNumberTone) }}</div>
              </div>
            </div>
          </div>
        </div>
      </section>


      <section class="pinyin-section">
        <h2>所有拼音表格</h2>
        <div class="pinyin-table-container">
          <table class="pinyin-table">
            <!-- 表头：声母 -->
            <thead>
              <tr>
                <th></th> <!-- 空白单元格，用于显示韵母 -->
                <th v-for="(consonant, index) in initialConsonants" :key="index" class="consonant-header">
                  {{ consonant }}
                </th>
              </tr>
            </thead>
            <tbody>
              <!-- 表格内容：行是韵母，列是声母 -->
              <tr v-for="(row, rowIndex) in pinyinTableData" :key="rowIndex">
                <td class="vowel-label">{{ finalVowels[rowIndex] }}</td>
                <td v-for="(cell, colIndex) in row" :key="colIndex" class="pinyin-cell">
                  <div v-if="cell" class="pinyin-tone-group">
                    <div v-for="(tonedPinyin, toneIndex) in cell" :key="toneIndex" class="pinyin-item"
                      @click="speakText(tonedPinyin.withNumberTone)">
                      <div class="pinyin-text">{{ tonedPinyin.withToneMark }}</div>
                      <div v-if="getExampleChar(tonedPinyin.withNumberTone)" class="example-char">
                        {{ getExampleChar(tonedPinyin.withNumberTone) }}
                      </div>
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
  background-color: #f5f5f5;
  padding: 2rem;
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
  max-width: 1200px;
  margin: 0 auto;
}

.pinyin-section {
  background-color: white;
  border-radius: 8px;
  padding: 1.5rem;
  margin-bottom: 2rem;
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
  grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  gap: 1.5rem;
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
}

.pinyin-table {
  width: 100%;
  border-collapse: collapse;
  table-layout: fixed;
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

.consonant-header {
  width: 60px;
}

.vowel-label {
  background-color: #e8f5e9;
  font-weight: bold;
  color: #2e7d32;
  position: sticky;
  left: 0;
  z-index: 5;
}

.pinyin-cell {
  min-width: 150px;
  max-width: 150px;
  background-color: #fafafa;
  height: 100px;
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

.empty-cell {
  background-color: #f5f5f5;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .pinyin-page {
    padding: 1rem;
  }

  .vowel-grid {
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    gap: 1rem;
  }

  .pinyin-cell {
    min-width: 120px;
    max-width: 120px;
    height: 80px;
  }

  .pinyin-text {
    font-size: 1rem;
  }

  .example-char {
    font-size: 1.2rem;
  }
}
</style>