# 读取生字.txt文件
with open('public/生字.txt', 'r', encoding='utf-8') as f:
    content = f.read()

# 移除所有换行符，将所有汉字合并成一行
merged_content = content.replace('\n', '')

# 将合并后的内容写回文件
with open('public/生字.txt', 'w', encoding='utf-8') as f:
    f.write(merged_content)

print(f'成功将{len(merged_content)}个汉字合并成一行')
print('已更新public/生字.txt文件')
