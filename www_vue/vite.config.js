import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

import wasm from 'vite-plugin-wasm'
import { copyFileSync, mkdirSync, readdirSync, existsSync } from 'fs'
import { resolve } from 'path'

// 自定义插件：将cnchar-data的子目录直接复制到dist根目录
const copyCncharData = {
  name: 'copy-cnchar-data',
  writeBundle() {
    const srcDir = resolve(__dirname, 'cnchar-data')
    const distDir = resolve(__dirname, 'dist')

    if (!existsSync(srcDir)) {
      console.warn('cnchar-data directory not found, skipping copy')
      return
    }

    // 读取cnchar-data目录下的所有子目录
    const entries = readdirSync(srcDir, { withFileTypes: true })

    // 复制所有子目录到dist根目录
    const copyRecursive = (src, dest) => {
      const entries = readdirSync(src, { withFileTypes: true })

      for (const entry of entries) {
        const srcPath = resolve(src, entry.name)
        const destPath = resolve(dest, entry.name)

        if (entry.isDirectory()) {
          mkdirSync(destPath, { recursive: true })
          copyRecursive(srcPath, destPath)
        } else {
          copyFileSync(srcPath, destPath)
        }
      }
    }

    for (const entry of entries) {
      if (entry.isDirectory()) {
        const srcSubDir = resolve(srcDir, entry.name)
        const destSubDir = resolve(distDir, entry.name)

        mkdirSync(destSubDir, { recursive: true })
        copyRecursive(srcSubDir, destSubDir)
        console.log(`cnchar-data/${entry.name} copied to dist/${entry.name} successfully`)
      }
    }
  }
}

// https://vite.dev/config/
export default defineConfig({
  base: "./",
  plugins: [
    vue(),
    wasm(),
    copyCncharData // 添加自定义插件
  ],
  server: {
    host: '0.0.0.0', // 允许局域网访问
    fs: {
      allow: ['../pkg', './']
      // allow: ['/Volumes/six/FLUT/wasm_zici/pkg', '/Volumes/six/FLUT/wasm_zici/www_vue'] // 使用绝对路径
    },
    proxy: {
      '/draw': {
        target: 'http://192.168.31.58:3002',
        changeOrigin: true,
        secure: false
      }
    }
  },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
})
