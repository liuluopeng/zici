<script setup>
import { ref, computed, watch } from 'vue';
import cnchar from 'cnchar-all';

// 声母列表
const initialSounds = [
  { char: 'b', pinyin: 'b', example: '波' },
  { char: 'p', pinyin: 'p', example: '坡' },
  { char: 'm', pinyin: 'm', example: '摸' },
  { char: 'f', pinyin: 'f', example: '佛' },
  { char: 'd', pinyin: 'd', example: '的' },
  { char: 't', pinyin: 't', example: '特' },
  { char: 'n', pinyin: 'n', example: '呢' },
  { char: 'l', pinyin: 'l', example: '乐' },
  { char: 'g', pinyin: 'g', example: '哥' },
  { char: 'k', pinyin: 'k', example: '科' },
  { char: 'h', pinyin: 'h', example: '喝' },
  { char: 'j', pinyin: 'j', example: '鸡' },
  { char: 'q', pinyin: 'q', example: '七' },
  { char: 'x', pinyin: 'x', example: '希' },
  { char: 'zh', pinyin: 'zh', example: '知' },
  { char: 'ch', pinyin: 'ch', example: '吃' },
  { char: 'sh', pinyin: 'sh', example: '狮' },
  { char: 'r', pinyin: 'r', example: '日' },
  { char: 'z', pinyin: 'z', example: '资' },
  { char: 'c', pinyin: 'c', example: '疵' },
  { char: 's', pinyin: 's', example: '撕' },
  { char: 'y', pinyin: 'y', example: '医' },
  { char: 'w', pinyin: 'w', example: '乌' }
];

// 完整的韵母列表
const allFinalSounds = [
  { char: 'a', pinyin: 'a', example: '啊' },
  { char: 'o', pinyin: 'o', example: '窝' },
  { char: 'e', pinyin: 'e', example: '鹅' },
  { char: 'i', pinyin: 'i', example: '衣' },
  { char: 'u', pinyin: 'u', example: '乌' },
  { char: 'ü', pinyin: 'ü', example: '迂' },
  { char: 'ai', pinyin: 'ai', example: '爱' },
  { char: 'ei', pinyin: 'ei', example: '飞' },
  { char: 'ui', pinyin: 'ui', example: '水' },
  { char: 'ao', pinyin: 'ao', example: '奥' },
  { char: 'ou', pinyin: 'ou', example: '欧' },
  { char: 'iu', pinyin: 'iu', example: '牛' },
  { char: 'ie', pinyin: 'ie', example: '姐' },
  { char: 'ue', pinyin: 'ue', example: '月' },
  { char: 'er', pinyin: 'er', example: '儿' },
  { char: 'an', pinyin: 'an', example: '安' },
  { char: 'en', pinyin: 'en', example: '恩' },
  { char: 'in', pinyin: 'in', example: '音' },
  { char: 'un', pinyin: 'un', example: '云' },
  { char: 'ang', pinyin: 'ang', example: '昂' },
  { char: 'eng', pinyin: 'eng', example: '风' },
  { char: 'ing', pinyin: 'ing', example: '英' },
  { char: 'ong', pinyin: 'ong', example: '东' }
];

// 拼音字典：声母 -> 韵母 -> 组合拼音
const pinyinDictionary = {
  'b': {
    'a': 'ba',
    'o': 'bo',
    'ai': 'bai',
    'ei': 'bei',
    'ao': 'bao',
    'an': 'ban',
    'en': 'ben',
    'ang': 'bang',
    'eng': 'beng',
    'i': 'bi',
    'ie': 'bie',
    'iao': 'biao',
    'ian': 'bian',
    'in': 'bin',
    'ing': 'bing',
    'u': 'bu'
  },
  'p': {
    'a': 'pa',
    'o': 'po',
    'ai': 'pai',
    'ei': 'pei',
    'ao': 'pao',
    'ou': 'pou',
    'an': 'pan',
    'en': 'pen',
    'ang': 'pang',
    'eng': 'peng',
    'i': 'pi',
    'ie': 'pie',
    'iao': 'piao',
    'ian': 'pian',
    'in': 'pin',
    'ing': 'ping',
    'u': 'pu'
  },
  'm': {
    'a': 'ma',
    'o': 'mo',
    'ai': 'mai',
    'ei': 'mei',
    'ao': 'mao',
    'ou': 'mou',
    'an': 'man',
    'en': 'men',
    'ang': 'mang',
    'eng': 'meng',
    'i': 'mi',
    'ie': 'mie',
    'iao': 'miao',
    'iou': 'miu',
    'ian': 'mian',
    'in': 'min',
    'ing': 'ming',
    'u': 'mu'
  },
  'f': {
    'a': 'fa',
    'o': 'fo',
    'e': 'me',
    'ei': 'fei',
    'ou': 'fou',
    'an': 'fan',
    'en': 'fen',
    'ang': 'fang',
    'eng': 'feng',
    'u': 'fu'
  },
  'd': {
    'a': 'da',
    'e': 'de',
    'ai': 'dai',
    'ei': 'dei',
    'ao': 'dao',
    'ou': 'dou',
    'an': 'dan',
    'en': 'den',
    'ang': 'dang',
    'eng': 'deng',
    'ong': 'dong',
    'i': 'di',
    'ia': 'dia',
    'ie': 'die',
    'iao': 'diao',
    'iou': 'diu',
    'ian': 'dian',
    'ing': 'ding',
    'u': 'du',
    'uo': 'duo',
    'uei': 'dui',
    'uan': 'duan',
    'uen': 'dun'
  },
  't': {
    'a': 'ta',
    'e': 'te',
    'ai': 'tai',
    'ei': 'tei',
    'ao': 'tao',
    'ou': 'tou',
    'an': 'tan',
    'ang': 'tang',
    'eng': 'teng',
    'ong': 'tong',
    'i': 'ti',
    'ie': 'tie',
    'iao': 'tiao',
    'ian': 'tian',
    'ing': 'ting',
    'u': 'tu',
    'uo': 'tuo',
    'uei': 'tui',
    'uan': 'tuan',
    'uen': 'tun'
  },
  'n': {
    'a': 'na',
    'e': 'ne',
    'ai': 'nai',
    'ei': 'nei',
    'ao': 'nao',
    'ou': 'nou',
    'an': 'nan',
    'en': 'nen',
    'ang': 'nang',
    'eng': 'neng',
    'ong': 'nong',
    'i': 'ni',
    'ie': 'nie',
    'iao': 'niao',
    'iou': 'niu',
    'ian': 'nian',
    'in': 'nin',
    'iang': 'niang',
    'ing': 'ning',
    'u': 'nu',
    'uo': 'nuo',
    'uan': 'nuan',
    'ü': 'nü',
    'üe': 'nüe'
  },
  'l': {
    'a': 'la',
    'e': 'le',
    'ai': 'lai',
    'ei': 'lei',
    'ao': 'lao',
    'ou': 'lou',
    'an': 'lan',
    'ang': 'lang',
    'eng': 'leng',
    'ong': 'long',
    'i': 'li',
    'ia': 'lia',
    'ie': 'lie',
    'iao': 'liao',
    'iou': 'liu',
    'ian': 'lian',
    'in': 'lin',
    'iang': 'liang',
    'ing': 'ling',
    'u': 'lu',
    'uo': 'luo',
    'uan': 'luan',
    'uen': 'lun',
    'ü': 'lü',
    'üe': 'lüe'
  },
  'g': {
    'a': 'ga',
    'e': 'ge',
    'ai': 'gai',
    'ei': 'gei',
    'ao': 'gao',
    'ou': 'gou',
    'an': 'gan',
    'en': 'gen',
    'ang': 'gang',
    'eng': 'geng',
    'ong': 'gong',
    'u': 'gu',
    'ua': 'gua',
    'uo': 'guo',
    'uai': 'guai',
    'uei': 'gui',
    'uan': 'guan',
    'uen': 'gun',
    'uang': 'guang'
  },
  'k': {
    'a': 'ka',
    'e': 'ke',
    'ai': 'kai',
    'ao': 'kao',
    'ou': 'kou',
    'an': 'kan',
    'en': 'ken',
    'ang': 'kang',
    'eng': 'keng',
    'ong': 'kong',
    'u': 'ku',
    'ua': 'kua',
    'uo': 'kuo',
    'uai': 'kuai',
    'uei': 'kui',
    'uan': 'kuan',
    'uen': 'kun',
    'uang': 'kuang'
  },
  'h': {
    'a': 'ha',
    'e': 'he',
    'ai': 'hai',
    'ei': 'hei',
    'ao': 'hao',
    'ou': 'hou',
    'an': 'han',
    'en': 'hen',
    'ang': 'hang',
    'eng': 'heng',
    'ong': 'hong',
    'u': 'hu',
    'ua': 'hua',
    'uo': 'huo',
    'uai': 'huai',
    'uei': 'hui',
    'uan': 'huan',
    'uen': 'hun',
    'uang': 'huang'
  },
  'j': {
    'i': 'ji',
    'ia': 'jia',
    'ie': 'jie',
    'iao': 'jiao',
    'iou': 'jiu',
    'ian': 'jian',
    'in': 'jin',
    'iang': 'jiang',
    'ing': 'jing',
    'iong': 'jiong',
    'ü': 'ju',
    'üe': 'jue',
    'üan': 'juan',
    'ün': 'jun'
  },
  'q': {
    'i': 'qi',
    'ia': 'qia',
    'ie': 'qie',
    'iao': 'qiao',
    'iou': 'qiu',
    'ian': 'qian',
    'in': 'qin',
    'iang': 'qiang',
    'ing': 'qing',
    'iong': 'qiong',
    'ü': 'qu',
    'üe': 'que',
    'üan': 'quan',
    'ün': 'qun'
  },
  'x': {
    'i': 'xi',
    'ia': 'xia',
    'ie': 'xie',
    'iao': 'xiao',
    'iou': 'xiu',
    'ian': 'xian',
    'in': 'xin',
    'iang': 'xiang',
    'ing': 'xing',
    'iong': 'xiong',
    'ü': 'xu',
    'üe': 'xue',
    'üan': 'xuan',
    'ün': 'xun'
  },
  'zh': {
    'a': 'zha',
    'e': 'zhe',
    'ai': 'zhai',
    'ei': 'zhei',
    'ao': 'zhao',
    'ou': 'zhou',
    'an': 'zhan',
    'en': 'zhen',
    'ang': 'zhang',
    'eng': 'zheng',
    'ong': 'zhong',
    'u': 'zhu',
    'ua': 'zhua',
    'uo': 'zhuo',
    'uai': 'zhuai',
    'uei': 'zhui',
    'uan': 'zhuan',
    'uen': 'zhun',
    'uang': 'zhuang'
  },
  'ch': {
    'a': 'cha',
    'e': 'che',
    'ai': 'chai',
    'ao': 'chao',
    'ou': 'chou',
    'an': 'chan',
    'en': 'chen',
    'ang': 'chang',
    'eng': 'cheng',
    'ong': 'chong',
    'u': 'chu',
    'uo': 'chuo',
    'uai': 'chuai',
    'uei': 'chui',
    'uan': 'chuan',
    'uen': 'chun',
    'uang': 'chuang'
  },
  'sh': {
    'a': 'sha',
    'e': 'she',
    'ai': 'shai',
    'ei': 'shei',
    'ao': 'shao',
    'ou': 'shou',
    'an': 'shan',
    'en': 'shen',
    'ang': 'shang',
    'eng': 'sheng',
    'u': 'shu',
    'ua': 'shua',
    'uo': 'shuo',
    'uai': 'shuai',
    'uei': 'shui',
    'uan': 'shuan',
    'uen': 'shun',
    'uang': 'shuang'
  },
  'r': {
    'e': 're',
    'ao': 'rao',
    'ou': 'rou',
    'an': 'ran',
    'en': 'ren',
    'ang': 'rang',
    'eng': 'reng',
    'ong': 'rong',
    'u': 'ru',
    'uo': 'ruo',
    'uei': 'rui',
    'uan': 'ruan',
    'uen': 'run'
  },
  'z': {
    'a': 'za',
    'e': 'ze',
    'ai': 'zai',
    'ei': 'zei',
    'ao': 'zao',
    'ou': 'zou',
    'an': 'zan',
    'en': 'zen',
    'ang': 'zang',
    'eng': 'zeng',
    'ong': 'zong',
    'u': 'zu',
    'uo': 'zuo',
    'uei': 'zui',
    'uan': 'zuan',
    'uen': 'zun'
  },
  'c': {
    'a': 'ca',
    'e': 'ce',
    'ai': 'cai',
    'ao': 'cao',
    'ou': 'cou',
    'an': 'can',
    'en': 'cen',
    'ang': 'cang',
    'eng': 'ceng',
    'ong': 'cong',
    'u': 'cu',
    'uo': 'cuo',
    'uei': 'cui',
    'uan': 'cuan',
    'uen': 'cun'
  },
  's': {
    'a': 'sa',
    'e': 'se',
    'ai': 'sai',
    'ao': 'sao',
    'ou': 'sou',
    'an': 'san',
    'en': 'sen',
    'ang': 'sang',
    'eng': 'seng',
    'ong': 'song',
    'u': 'su',
    'uo': 'suo',
    'uei': 'sui',
    'uan': 'suan',
    'uen': 'sun'
  }
};

// 选择状态
const selectedInitialIndex = ref(0);
const selectedFinalIndex = ref(0);
const initialScrollRef = ref(null);
const finalScrollRef = ref(null);
const wordsScrollRef = ref(null);

// 本地 API 结果数据
const pinyinApiResults = ref(null);
const isLoadingApiData = ref(false);

// 解释窗口状态
const showExplainModal = ref(false);
const explainContent = ref('');
const currentWord = ref('');

// 加载本地 API 结果数据
const loadLocalApiData = async () => {
  isLoadingApiData.value = true;
  try {
    const response = await fetch('/pinyin_api_results.json');
    if (!response.ok) {
      throw new Error(`加载本地数据失败: ${response.status}`);
    }
    const data = await response.json();
    pinyinApiResults.value = data;
    console.log('本地 API 结果数据加载成功:', Object.keys(data).length, '个拼音条目');
  } catch (error) {
    console.error('加载本地 API 结果数据失败:', error);
    pinyinApiResults.value = {};
  } finally {
    isLoadingApiData.value = false;
  }
};

// 页面加载时加载本地数据
loadLocalApiData();

const ITEM_HEIGHT = 60;

// 根据当前声母获取合法的韵母列表
const filteredFinalSounds = computed(() => {
  const currentInitial = initialSounds[selectedInitialIndex.value].pinyin;
  const finalsMap = pinyinDictionary[currentInitial];
  if (!finalsMap) return [];

  return Object.keys(finalsMap).map(final => {
    const finalItem = allFinalSounds.find(f => f.pinyin === final);
    return finalItem || { char: final, pinyin: final, example: '' };
  });
});

// 组合拼音
const combinedPinyin = computed(() => {
  const initial = initialSounds[selectedInitialIndex.value].pinyin;
  const validFinals = filteredFinalSounds.value;
  if (validFinals.length === 0) return initial;

  const final = validFinals[Math.min(selectedFinalIndex.value, validFinals.length - 1)];
  const finalsMap = pinyinDictionary[initial];

  if (finalsMap && finalsMap[final.pinyin]) {
    return finalsMap[final.pinyin];
  }

  return initial;
});

// 监听声母变化，重置韵母选择
watch(selectedInitialIndex, () => {
  selectedFinalIndex.value = 0;
  // 滚动到顶部
  if (finalScrollRef.value) {
    finalScrollRef.value.scrollTo({ top: 0, behavior: 'smooth' });
  }
});



// 发音并显示解释
const speakWithExplain = async (word) => {
  try {
    // 停止正在播放的语音
    stopExplanation();

    currentWord.value = word;

    // 从生成的词语列表中查找对应的解释
    const wordItem = generatedWords.value.find(item => item.word === word);

    if (wordItem && wordItem.explanation) {
      explainContent.value = wordItem.explanation;
      showExplainModal.value = true;

      // 解析 HTML 内容，提取文本并朗读
      const explanationText = extractExplanationText(wordItem.explanation, word);
      if (explanationText) {
        speakExplanation(explanationText);
      }
    } else {
      //  fallback: 如果没有找到解释，显示提示
      explainContent.value = `<div style="padding: 20px; text-align: center; color: #999;">暂无解释</div>`;
      showExplainModal.value = true;
    }
  } catch (error) {
    console.error('获取解释失败:', error);
    explainContent.value = `<div style="padding: 20px; text-align: center; color: #999;">获取解释失败，请稍后再试</div>`;
    showExplainModal.value = true;
  }
};

// 从 HTML 中提取解释文本
const extractExplanationText = (html, word) => {
  try {
    // 创建临时 DOM 元素来解析 HTML
    const parser = new DOMParser();
    const doc = parser.parseFromString(html, 'text/html');

    // 提取所有定义
    const defElements = doc.querySelectorAll('def');
    let definitions = [];

    defElements.forEach(defElement => {
      // 克隆节点以避免修改原始 DOM
      const defClone = defElement.cloneNode(true);

      // 移除词性标签 (ps, pt, pu, pv)
      const posTags = defClone.querySelectorAll('ps, pt, pu, pv');
      posTags.forEach(tag => tag.remove());

      // 移除序号标签 (num)
      const numTags = defClone.querySelectorAll('num');
      numTags.forEach(tag => tag.remove());

      // 获取纯文本内容
      let defText = defClone.textContent.trim();

      // 移除序号符号
      defText = defText.replace(/[①②③④⑤⑥⑦⑧⑨⑩]/g, '');
      defText = defText.replace(/[1-9][0-9]*[、.．]/g, '');
      defText = defText.replace(/^[、.．\s]+/, '');

      // 清理多余的空格和标点
      defText = defText.replace(/\s+/g, ' ');
      defText = defText.trim();

      if (defText) {
        definitions.push(defText);
      }
    });

    // 如果有解释文本，替换 ～ 符号为词语
    if (definitions.length > 0) {
      // 替换 ～ 符号为词语
      definitions = definitions.map(def => def.replace(/～/g, word));

      // 直接返回解释文本，不包含词语和拼音
      return definitions.join(' ');
    }

    return null;
  } catch (error) {
    console.error('解析解释文本失败:', error);
    return null;
  }
};

// 朗读解释
const speakExplanation = (text) => {
  try {
    cnchar.voice.speak(text);
  } catch (error) {
    console.error('朗读解释失败:', error);
  }
};

// 停止语音播放
const stopExplanation = () => {
  try {
    // 检查 cnchar.voice 是否有 stop 方法
    if (cnchar.voice && typeof cnchar.voice.stop === 'function') {
      cnchar.voice.stop();
    } else if (cnchar.voice && typeof cnchar.voice.pause === 'function') {
      cnchar.voice.pause();
    }
  } catch (error) {
    console.error('停止语音失败:', error);
  }
};

// 关闭解释窗口
const closeExplainModal = () => {
  showExplainModal.value = false;
  explainContent.value = '';
  currentWord.value = '';
};

// 生成词语
const generatedWords = ref([]);
const isLoadingWords = ref(false);



// 生成词语并检查解释
const generateWords = async () => {
  const pinyin = combinedPinyin.value;
  isLoadingWords.value = true;

  try {
    // 检查本地数据是否加载完成
    if (!pinyinApiResults.value) {
      await loadLocalApiData();
    }

    // 从本地数据中获取词语列表
    const wordsData = pinyinApiResults.value[pinyin];
    console.log('本地数据返回的词语数据:', wordsData);

    if (!wordsData || (Array.isArray(wordsData) && wordsData.length === 0)) {
      generatedWords.value = [];
      return;
    }

    // 处理词语数据，使用本地数据中的explanation
    const processedWords = Array.isArray(wordsData) ? wordsData.map(wordObj => {
      // 替换 CSS 引用，从 hycd.css 改为 cidian.css
      const updatedExplanation = wordObj.explanation ? wordObj.explanation.replace(/href="hycd\.css"/g, 'href="cidian.css"') : '';

      return {
        word: wordObj.word,
        pinyin: wordObj.pinyin_array ? wordObj.pinyin_array.join(' ') : '',
        hasExplanation: true, // 所有本地数据中的词语都有解释
        explanation: updatedExplanation,
        frequency: wordObj.frequency || 0
      };
    }) : [];

    // 按照frequency从小到大排序
    processedWords.sort((a, b) => a.frequency - b.frequency);

    generatedWords.value = processedWords;

    // 滚动到词语网格顶部
    if (wordsScrollRef.value) {
      wordsScrollRef.value.scrollTo({ top: 0, behavior: 'smooth' });
    }
  } catch (error) {
    console.error('词语生成失败:', error);
    generatedWords.value = [];
  } finally {
    isLoadingWords.value = false;
  }
};

// 监听拼音变化，生成词语
watch(combinedPinyin, () => {
  generateWords();
}, { immediate: true });

// 处理滚动事件
const handleScroll = (type, event) => {
  const scrollTop = event.target.scrollTop;
  const index = Math.round(scrollTop / ITEM_HEIGHT);

  if (type === 'initial') {
    selectedInitialIndex.value = Math.max(0, Math.min(index, initialSounds.length - 1));
  } else {
    const validFinals = filteredFinalSounds.value;
    selectedFinalIndex.value = Math.max(0, Math.min(index, validFinals.length - 1));
  }
};

// 滚动到指定索引
const scrollToIndex = (type, index) => {
  const targetScroll = index * ITEM_HEIGHT;
  if (type === 'initial' && initialScrollRef.value) {
    initialScrollRef.value.scrollTo({ top: targetScroll, behavior: 'smooth' });
  } else if (type === 'final' && finalScrollRef.value) {
    finalScrollRef.value.scrollTo({ top: targetScroll, behavior: 'smooth' });
  }
};

// 获取拼音的所有音调形式（基于cnchar源码实现）
const getAllTonePinyin = (pinyin) => {
  // 定义声调标记（与cnchar源码一致）
  const tones = {
    a: ['a', 'ā', 'á', 'ǎ', 'à'],
    e: ['e', 'ē', 'é', 'ě', 'è'],
    i: ['i', 'ī', 'í', 'ǐ', 'ì'],
    o: ['o', 'ō', 'ó', 'ǒ', 'ò'],
    u: ['u', 'ū', 'ú', 'ǔ', 'ù'],
    ü: ['ü', 'ǖ', 'ǘ', 'ǚ', 'ǜ']
  };

  // 处理特殊情况
  if (!pinyin || typeof pinyin !== 'string') {
    return [];
  }

  // 标准化拼音（转为小写）
  pinyin = pinyin.toLowerCase();

  // 检查拼音中包含的元音（优先级：a > e > i > o > u > ü）
  const vowelOrder = ['a', 'e', 'i', 'o', 'u', 'ü'];
  let targetVowel = '';
  let vowelIndex = -1;

  // 找到拼音中的第一个元音（按优先级顺序）
  for (const vowel of vowelOrder) {
    const index = pinyin.indexOf(vowel);
    if (index !== -1) {
      targetVowel = vowel;
      vowelIndex = index;
      break;
    }
  }

  // 如果没有元音，返回原拼音
  if (!targetVowel) {
    return [pinyin];
  }

  // 生成所有音调的拼音
  const result = [];
  const vowelTones = tones[targetVowel];

  // 生成轻声到四声的所有形式
  for (let i = 0; i < 5; i++) {
    const toneChar = vowelTones[i];
    const newPinyin = pinyin.substring(0, vowelIndex) + toneChar + pinyin.substring(vowelIndex + 1);
    result.push(newPinyin);
  }

  return result;
};
</script>

<template>
  <div class="picker-page">

    <div class="picker-container">
      <div class="picker-column">
        <div class="picker-header">声母</div>
        <div class="picker-scroll" ref="initialScrollRef" @scroll="handleScroll('initial', $event)">
          <div class="picker-items">
            <div v-for="(item, index) in initialSounds" :key="item.pinyin" class="picker-item"
              :class="{ 'picker-item--selected': index === selectedInitialIndex }"
              @click="scrollToIndex('initial', index)">
              <div class="pinyin">{{ item.pinyin }}</div>
              <div class="example">{{ item.example }}</div>
            </div>
          </div>
        </div>
      </div>

      <div class="picker-column">
        <div class="picker-header">韵母</div>
        <div class="picker-scroll" ref="finalScrollRef" @scroll="handleScroll('final', $event)">
          <div class="picker-items">
            <div v-for="(item, index) in filteredFinalSounds" :key="item.pinyin" class="picker-item"
              :class="{ 'picker-item--selected': index === selectedFinalIndex }" @click="scrollToIndex('final', index)">
              <div class="pinyin">{{ item.pinyin }}</div>
              <div class="example">{{ item.example }}</div>
            </div>
          </div>
        </div>
      </div>

      <div class="picker-column">
        <div class="picker-header">组合结果</div>
        <div class="result-container" style="user-select: none;">
          <div class="result-pinyin">
            {{ combinedPinyin }}
          </div>

          <div class="words-container">
            <div class="words-header" style="user-select: none;">词语</div>
            <div class="words-grid" ref="wordsScrollRef">
              <div v-for="(item, index) in generatedWords" :key="index" class="word-card"
                @click="speakWithExplain(item.word)">
                <div v-if="item.hasExplanation" class="explanation-dot"></div>
                <div class="word-pinyin">{{ item.pinyin }}</div>
                <div class="word-text">{{ item.word }}</div>
                <div class="word-frequency" v-if="item.frequency">{{ (1 - item.frequency / 56000).toFixed(6) }}</div>
              </div>
              <div v-if="generatedWords.length === 0 && !isLoadingWords" class="no-words">
                暂无相关词语
              </div>
              <div v-if="isLoadingWords" class="loading-words">
                正在生成词语...
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- 解释模态窗口 -->
  <div v-if="showExplainModal" class="explain-modal-overlay">
    <div class="explain-modal">
      <div class="explain-modal-header">
        <h3>{{ currentWord }} - 词语解释</h3>
        <button class="close-btn" @click="closeExplainModal">&times;</button>
      </div>
      <div class="explain-modal-content" v-html="explainContent"></div>
    </div>
  </div>
</template>

<style scoped>
.picker-page {
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

html,
body {
  overflow: hidden;
  height: 100%;
  margin: 0;
  padding: 0;
}

.picker-page h1 {
  text-align: center;
  color: #333;
  margin-bottom: 2rem;
  flex-shrink: 0;
}

.picker-container {
  display: flex;
  gap: 1.5rem;
  flex: 1;
  min-height: 0;
}

.picker-column {
  display: flex;
  flex-direction: column;
  background-color: #f8f9fa;
  border-radius: 1.5rem;
  box-shadow:
    0 0.5rem 1rem rgba(0, 0, 0, 0.1),
    inset 0 0 0 1px rgba(255, 255, 255, 0.8),
    inset 0 0 20px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  position: relative;
}



.picker-column:nth-child(1),
.picker-column:nth-child(2) {
  flex: 0.7;
}

.picker-column:nth-child(3) {
  flex: 2.6;
}

.picker-header {
  padding: 1rem;
  text-align: center;
  font-size: 1.2rem;
  font-weight: bold;
  color: white;
  background: linear-gradient(135deg, #4caf50 0%, #45a049 100%);
  flex-shrink: 0;
  user-select: none;
}

.picker-scroll {
  flex: 1;
  overflow-y: auto;
  position: relative;
  scroll-behavior: smooth;
  background: linear-gradient(to bottom, rgba(0, 0, 0, 0.1) 0%, rgba(0, 0, 0, 0) 20%, rgba(0, 0, 0, 0) 80%, rgba(0, 0, 0, 0.1) 100%);
}

.picker-scroll::-webkit-scrollbar {
  display: none;
}

.picker-scroll {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

.picker-items {
  padding: 50vh 0;
  display: flex;
  flex-direction: column;
}

.picker-item {
  height: 70px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
  opacity: 0.2;
  transform: scale(0.7) translateZ(0);
  z-index: 1;
  position: relative;
  overflow: hidden;
  user-select: none;
  border-radius: 8px;
  background: linear-gradient(to bottom, #f5f5f5 0%, #e0e0e0 100%);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.5), inset 0 -1px 0 rgba(0, 0, 0, 0.1);
  margin: 2px 0;
  padding: 0 10px;
}

.picker-item:hover {
  opacity: 0.6;
  transform: scale(1) translateZ(0);
  z-index: 2;
}

.picker-item--selected {
  opacity: 1;
  transform: scale(1.4) translateZ(0);
  background: linear-gradient(to bottom, #4caf50 0%, #45a049 100%);
  color: white;
  border-radius: 12px;
  box-shadow:
    0 0 0 3px rgba(255, 255, 255, 0.3),
    0 0.8rem 1.6rem rgba(76, 175, 80, 0.6);
  z-index: 3;
  padding: 0 1.5rem;
  animation: magnify 0.3s ease-out;
}

@keyframes magnify {
  0% {
    transform: scale(1) translateZ(0);
    box-shadow: 0 0 0 0 rgba(76, 175, 80, 0.4);
  }

  50% {
    transform: scale(1.5) translateZ(0);
    box-shadow: 0 0 0 4px rgba(255, 255, 255, 0.3);
  }

  100% {
    transform: scale(1.4) translateZ(0);
    box-shadow:
      0 0 0 3px rgba(255, 255, 255, 0.3),
      0 0.8rem 1.6rem rgba(76, 175, 80, 0.6);
  }
}

.picker-item .pinyin {
  font-size: 1.6rem;
  font-weight: bold;
  color: #333;
  font-family: 'Arial', sans-serif;
  text-shadow: 0 1px 0 rgba(255, 255, 255, 0.8);
}

.picker-item .example {
  font-size: 0.7rem;
  color: #666;
  margin-top: 0.2rem;
  font-family: 'Arial', sans-serif;
}

.picker-item--selected .pinyin {
  color: white;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

.picker-item--selected .example {
  color: rgba(255, 255, 255, 0.9);
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
}

.result-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  gap: 1rem;
}

.result-pinyin {
  font-size: 4rem;
  font-weight: bold;
  color: white;
  padding: 2rem 3rem;
  background: linear-gradient(135deg, #4caf50 0%, #45a049 100%);
  border-radius: 1rem;
  cursor: pointer;
  transition: all 0.3s;
  box-shadow: 0 0.5rem 1rem rgba(76, 175, 80, 0.4);
}

.result-pinyin:hover {
  transform: scale(1.05);
  box-shadow: 0 0.8rem 1.5rem rgba(76, 175, 80, 0.6);
}

.result-hint {
  font-size: 1rem;
  color: #999;
  text-align: center;
}

.words-container {
  margin-top: 2rem;
  width: 100%;
}

.words-header {
  font-size: 1.1rem;
  font-weight: bold;
  color: #333;
  margin-bottom: 1rem;
  text-align: center;
  user-select: none;
}

.words-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 1rem;
  max-height: 400px;
  overflow-y: auto;
  padding: 1rem;
  background-color: #f8f9fa;
  border-radius: 0.5rem;
}

.word-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 1rem;
  background-color: white;
  border-radius: 0.5rem;
  box-shadow: 0 0.2rem 0.4rem rgba(0, 0, 0, 0.1);
  transition: all 0.3s;
  text-align: center;
  cursor: pointer;
  position: relative;
  user-select: none;
}

.explanation-dot {
  position: absolute;
  top: 0.5rem;
  right: 0.5rem;
  width: 8px;
  height: 8px;
  background-color: #4caf50;
  border-radius: 50%;
  box-shadow: 0 0 4px rgba(76, 175, 80, 0.5);
  animation: dotPulse 2s infinite;
}

.word-frequency {
  position: absolute;
  bottom: 0.25rem;
  right: 0.25rem;
  font-size: 0.6rem;
  color: #999;
  font-family: 'Arial', sans-serif;
}

@keyframes dotPulse {

  0%,
  100% {
    transform: scale(1);
    opacity: 1;
  }

  50% {
    transform: scale(1.2);
    opacity: 0.8;
  }
}

.word-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 0.4rem 0.8rem rgba(0, 0, 0, 0.15);
  background-color: #f8f9ff;
}

.word-pinyin {
  font-size: 0.9rem;
  color: #666;
  margin-bottom: 0.5rem;
}

.word-text {
  font-size: 1.2rem;
  font-weight: bold;
  color: #333;
}

.no-words {
  text-align: center;
  color: #999;
  padding: 2rem;
  font-style: italic;
  grid-column: 1 / -1;
}

.loading-words {
  text-align: center;
  color: #666;
  padding: 2rem;
  grid-column: 1 / -1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
}

.loading-words::before {
  content: '';
  display: inline-block;
  width: 16px;
  height: 16px;
  border: 2px solid #4caf50;
  border-top-color: transparent;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.words-grid::-webkit-scrollbar {
  width: 4px;
}

.words-grid::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 2px;
}

.words-grid::-webkit-scrollbar-thumb {
  background: #888;
  border-radius: 2px;
}

.words-grid::-webkit-scrollbar-thumb:hover {
  background: #555;
}

/* 解释模态窗口样式 */
.explain-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.explain-modal {
  background-color: white;
  border-radius: 10px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  animation: modalFadeIn 0.3s ease-out;
}

@keyframes modalFadeIn {
  from {
    opacity: 0;
    transform: scale(0.9) translateY(-20px);
  }

  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.explain-modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  background: linear-gradient(135deg, #4caf50 0%, #45a049 100%);
  color: white;
  border-bottom: 1px solid #eee;
}

.explain-modal-header h3 {
  margin: 0;
  font-size: 1.2rem;
  font-weight: bold;
}

.close-btn {
  background: none;
  border: none;
  color: white;
  font-size: 1.5rem;
  cursor: pointer;
  padding: 0;
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: all 0.2s;
}

.close-btn:hover {
  background-color: rgba(255, 255, 255, 0.2);
  transform: scale(1.1);
}

.explain-modal-content {
  flex: 1;
  padding: 1.5rem;
  overflow-y: auto;
  font-size: 1rem;
  line-height: 1.6;
}

.explain-modal-content::-webkit-scrollbar {
  width: 6px;
}

.explain-modal-content::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 3px;
}

.explain-modal-content::-webkit-scrollbar-thumb {
  background: #888;
  border-radius: 3px;
}

.explain-modal-content::-webkit-scrollbar-thumb:hover {
  background: #555;
}

/* 修复 API 返回的 HTML 样式 */
.explain-modal-content entry {
  display: block;
}

.explain-modal-content hw {
  font-size: 1.5rem;
  font-weight: bold;
  color: #333;
  display: block;
  margin-bottom: 0.5rem;
}

.explain-modal-content py {
  font-size: 1rem;
  color: #666;
  display: block;
  margin-bottom: 1rem;
}

.explain-modal-content def {
  display: block;
  margin-bottom: 1rem;
  padding-left: 20px;
  border-left: 3px solid #4caf50;
}

.explain-modal-content ps {
  font-weight: bold;
  color: #4caf50;
  margin-right: 0.5rem;
}

.explain-modal-content ci {
  display: block;
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid #eee;
}

.explain-modal-content .title {
  font-weight: bold;
  margin-bottom: 0.5rem;
  color: #333;
}

.explain-modal-content .cont {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.explain-modal-content .cont a {
  color: #4caf50;
  text-decoration: none;
  padding: 0.2rem 0.5rem;
  background-color: #f0fff0;
  border-radius: 4px;
  transition: all 0.2s;
}

.explain-modal-content .cont a:hover {
  background-color: #e0ffe0;
  transform: translateY(-1px);
}

.explain-modal-content a {
  color: #4caf50;
  text-decoration: none;
}

.explain-modal-content a:hover {
  text-decoration: underline;
}
</style>