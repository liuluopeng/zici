import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

// 获取当前文件的目录路径
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// 读取draft.txt文件
const draftPath = path.join(__dirname, 'public', 'draft.txt');
const draftContent = fs.readFileSync(draftPath, 'utf8');

// 提取所有中文字符
const chineseChars = draftContent.match(/[\u4e00-\u9fa5]/g) || [];

// 去重
const uniqueChars = [...new Set(chineseChars)];

// 按照一行一个字的格式写入生字.txt
const outputPath = path.join(__dirname, 'public', '生字.txt');
const outputContent = uniqueChars.join('\n');

fs.writeFileSync(outputPath, outputContent);

console.log(`成功提取了${uniqueChars.length}个汉字`);
console.log(`已写入到${outputPath}`);
