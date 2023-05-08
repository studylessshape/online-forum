import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import topLevelAwait from 'vite-plugin-top-level-await'

// https://vitejs.dev/config/
export default defineConfig({
  base: "/",
  build: {
    assetsDir: "assets",
  },
  plugins: [vue(), topLevelAwait({
    promiseExportName: '__tla',
    promiseImportName: i => `__tla_${i}`
  })],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    }
  },
})
