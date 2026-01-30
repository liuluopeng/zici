import json
import requests
import time

# 读取拼音数据
with open('/Volumes/six/FLUT/wasm_zici/www_vue/pinyin_key_value.json', 'r', encoding='utf-8') as f:
    pinyin_data = json.load(f)

# 准备存储结果
results = {}

# API 地址
api_url = 'http://192.168.31.58:23001/api/dict/spelltoword-notone'

# 遍历所有拼音
pinyin_count = 0
for item in pinyin_data:
    for initial, finals in item.items():
        for final, pinyin in finals.items():
            pinyin_count += 1
            print(f'Processing {pinyin_count}: {pinyin}')
            
            try:
                # 构建请求参数
                params = {'search': pinyin}
                
                # 发送请求
                response = requests.get(api_url, params=params, timeout=10)
                
                if response.status_code == 200:
                    # 保存结果
                    results[pinyin] = response.json()
                else:
                    # 保存错误信息
                    results[pinyin] = {
                        'error': f'HTTP {response.status_code}',
                        'message': response.text
                    }
                
                # 添加延迟，避免请求过快
                time.sleep(0.1)
                
            except Exception as e:
                # 保存异常信息
                results[pinyin] = {
                    'error': 'Exception',
                    'message': str(e)
                }

# 保存结果到 JSON 文件
output_file = 'pinyin_api_results.json'
with open(output_file, 'w', encoding='utf-8') as f:
    json.dump(results, f, ensure_ascii=False, indent=2)

print(f'\nProcessing complete!')
print(f'Total pinyin processed: {pinyin_count}')
print(f'Results saved to: {output_file}')