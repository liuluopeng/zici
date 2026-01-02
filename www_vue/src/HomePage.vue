<script setup>
import cnchar from 'cnchar-all';
import { onMounted, ref } from 'vue';
// 导入键盘遮罩组件
import KeyboardMask from '@/components/keyboard-mask/KeyboardMask.vue';





// 键盘遮罩状态
const showKeyboardMask = ref(false);
const currentChar = ref('');
// 点击位置
const clickX = ref(0);
const clickY = ref(0);

// 打开键盘遮罩
const openKeyboardMask = (char, event) => {
  currentChar.value = char;
  // 记录点击位置
  clickX.value = event.clientX;
  clickY.value = event.clientY;
  showKeyboardMask.value = true;
};

// 关闭键盘遮罩
const closeKeyboardMask = () => {
  showKeyboardMask.value = false;
};

onMounted(() => {
  var drawArea = document.getElementById('draw-area');

  // 从外部文件加载汉字
  fetch('/生字.txt')
    .then(response => {
      console.log('文件加载成功，状态码:', response.status);
      return response.text();
    })
    .then(content => {
      console.log('原始文件内容:', content);
      // 只去除空白字符和换行符，保留所有实际字符
      const characters = content.replace(/[\n\r\t\s]+/g, '');
      console.log('处理后的汉字:', characters);
      console.log('汉字长度:', characters.length);

      // 验证所有字符都是中文
      const isValidChinese = (char) => {
        return /^[\u4e00-\u9fa5]$/.test(char);
      };

      // 绘制函数
      const drawCharacters = () => {
        // 清空绘制区域
        drawArea.innerHTML = '';

        // 过滤出有效的中文字符
        const validCharacters = characters.split('').filter(isValidChinese);
        console.log('有效中文字符:', validCharacters);

        // 存储所有绘制任务
        const drawTasks = [];

        // 1. 先创建所有DOM容器
        validCharacters.forEach((char, index) => {
          // 创建新的绘制容器
          const charContainer = document.createElement('div');
          charContainer.style.textAlign = 'center'; // 居中显示
          charContainer.style.padding = '10px'; // 添加内边距

          // 添加<h1>汉字</h1>标题
          const heading = document.createElement('h1');
          heading.textContent = char;
          heading.style.cursor = 'pointer'; // 添加鼠标指针样式，表示可点击

          // 添加点击事件，点击汉字时显示键盘遮罩并模拟拼音按键
          heading.addEventListener('click', (event) => {


            // 同时继续执行原来的词语朗读功能
            const words = cnchar.words(char);

            if (words && words.length > 0) {
              // 先打乱词语数组，然后取前3个
              const shuffledWords = [...words];
              // 使用Fisher-Yates洗牌算法打乱数组
              for (let i = shuffledWords.length - 1; i > 0; i--) {
                const j = Math.floor(Math.random() * (i + 1));
                [shuffledWords[i], shuffledWords[j]] = [shuffledWords[j], shuffledWords[i]];
              }
              const topWords = shuffledWords.slice(0, 3);

              // 创建临时遮罩元素
              const mask = document.createElement('div');
              mask.style.position = 'absolute';
              mask.style.backgroundColor = 'rgba(0, 0, 0, 0.8)';
              mask.style.color = 'white';
              mask.style.padding = '20px';
              mask.style.borderRadius = '8px';
              mask.style.fontSize = '24px';
              mask.style.fontWeight = 'bold';
              mask.style.zIndex = '1000';
              mask.style.textAlign = 'center';
              mask.style.boxShadow = '0 4px 12px rgba(0, 0, 0, 0.3)';

              // 获取当前汉字元素的位置和尺寸
              const rect = heading.getBoundingClientRect();
              mask.style.left = `${rect.left + window.scrollX + rect.width / 2}px`;
              mask.style.top = `${rect.top + window.scrollY + rect.height / 2}px`;
              mask.style.transform = 'translate(-50%, -50%)';

              // 添加到页面
              document.body.appendChild(mask);

              // 逐个朗读词语，每个词之间停顿0.5秒
              topWords.forEach((word, index) => {
                setTimeout(() => {
                  // 更新遮罩显示的词语
                  mask.textContent = word;
                  // 朗读当前词语
                  cnchar.voice.speak(word);

                  // 如果是最后一个词语，朗读完后移除遮罩
                  if (index === topWords.length - 1) {
                    // 假设每个词语朗读时间约为1秒，所以在朗读后1秒移除遮罩
                    setTimeout(() => {
                      document.body.removeChild(mask);
                    }, 1000);
                  }
                }, index * 1500); // 每个词之间间隔1.5秒（包含朗读时间）
              });
            } else {
              // 创建临时遮罩显示提示信息
              const mask = document.createElement('div');
              mask.style.position = 'absolute';
              mask.style.backgroundColor = 'rgba(0, 0, 0, 0.8)';
              mask.style.color = 'white';
              mask.style.padding = '20px';
              mask.style.borderRadius = '8px';
              mask.style.fontSize = '24px';
              mask.style.fontWeight = 'bold';
              mask.style.zIndex = '1000';
              mask.style.textAlign = 'center';
              mask.style.boxShadow = '0 4px 12px rgba(0, 0, 0, 0.3)';

              const rect = heading.getBoundingClientRect();
              mask.style.left = `${rect.left + window.scrollX + rect.width / 2}px`;
              mask.style.top = `${rect.top + window.scrollY + rect.height / 2}px`;
              mask.style.transform = 'translate(-50%, -50%)';
              mask.textContent = char + '的词语未找到';

              document.body.appendChild(mask);
              cnchar.voice.speak(char + '的词语未找到');

              // 2秒后移除遮罩
              setTimeout(() => {
                document.body.removeChild(mask);
              }, 2000);
            }
          });

          charContainer.appendChild(heading);

          const pinyin = document.createElement('h1');
          pinyin.textContent = cnchar.spell(char, 'tone');
          pinyin.style.cursor = 'pointer'; // 添加鼠标指针样式，表示可点击

          // 添加点击事件，点击拼音时发音
          pinyin.addEventListener('click', (event) => {

            // 打开键盘遮罩并播放拼音动画
            openKeyboardMask(char, event);

            cnchar.voice.speak(char);
          });

          cnchar.voice(char);
          charContainer.appendChild(pinyin);

          drawArea.appendChild(charContainer);

          var option = {
            clear: false,
            el: charContainer,
            style: {
              radicalColor: '#44f',
              backgroundColor: '#eee',
              length: 80
            },
            type: cnchar.draw.TYPE.ANIMATION,
            animation: {
              strokeAnimationSpeed: 2.5,
              delayBetweenStrokes: 1,
              // delayBetweenLoops: 200, 
              loopAnimate: true,
            }
          };

          // 将绘制任务存储起来
          drawTasks.push({ char, option });
        });

        // 2. 在同一时间点启动所有字符的绘制
        drawTasks.forEach(task => {
          try {
            cnchar.draw(task.char, task.option);
          } catch (error) {
            console.error('绘制字符失败:', task.char, error);
          }
        });
      };

      // 初始绘制
      drawCharacters();

      // 计算循环间隔时间（所有汉字绘制时间 + 额外等待时间）
      const cycleTime = characters.length * 1000 + 2000; // 每个汉字1秒，最后额外等待2秒

      // 设置循环播放
      setInterval(drawCharacters, cycleTime);
    })
    .catch(error => {
      console.error('加载汉字文件失败:', error);
      // 失败时绘制默认汉字
      var option = {
        clear: false,
        el: drawArea,
        style: {
          radicalColor: '#44f',
          backgroundColor: '#eee',
          length: 60
        },
        type: cnchar.draw.TYPE.ANIMATION,
        animation: {
          strokeAnimationSpeed: 2.5,
          delayBetweenStrokes: 10
        }
      };
      cnchar.draw('加载失败', option);
    });
});
</script>

<template>
  <div class="home-page">
    <!-- <div id="character-target-div"></div> -->
    <h1>一年级 生字</h1>
    <div id="draw-area"></div>

    <!-- 键盘遮罩组件 -->
    <KeyboardMask :show="showKeyboardMask" :char="currentChar" :click-x="clickX" :click-y="clickY"
      @close="closeKeyboardMask" />
  </div>
</template>

<style scoped>
.home-page h1 {
  color: #333;
}

#draw-area {
  margin-top: 20px;
  padding: 20px;
  background-color: #f5f5f5;
  border-radius: 8px;
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 20px;
  justify-items: center;
}
</style>