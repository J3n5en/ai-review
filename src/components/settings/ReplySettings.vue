<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { onMounted, ref } from 'vue'

interface ReplyConfig {
  enable_continue_reply: boolean
  auto_continue_threshold: number
  continue_prompt: string
}

const localConfig = ref<ReplyConfig>({
  enable_continue_reply: true,
  auto_continue_threshold: 1000,
  continue_prompt: '请继续',
})

const presetPrompts = [
  '请继续',
  '继续完成',
  '请继续详细说明',
  '继续实现剩余部分',
  '请继续完善代码',
  '继续下一步',
]

// 加载配置
async function loadConfig() {
  try {
    const config = await invoke('get_reply_config')
    localConfig.value = config as ReplyConfig
  }
  catch (error) {
    console.error('加载继续回复配置失败:', error)
  }
}

// 更新配置
async function updateConfig() {
  try {
    await invoke('set_reply_config', { replyConfig: localConfig.value })
  }
  catch (error) {
    console.error('保存继续回复配置失败:', error)
  }
}

// 设置预设提示词
function setPrompt(prompt: string) {
  localConfig.value.continue_prompt = prompt
  updateConfig()
}

onMounted(() => {
  loadConfig()
})
</script>

<template>
  <div class="space-y-4">
    <h3 class="text-lg font-medium text-gray-900 dark:text-gray-100">
      继续回复设置
    </h3>

    <!-- 启用继续回复 -->
    <div class="flex items-center justify-between">
      <div>
        <label class="text-sm font-medium text-gray-700 dark:text-gray-300">启用继续回复</label>
        <p class="text-xs text-gray-500 dark:text-gray-400">
          当回复内容较长时自动显示继续按钮
        </p>
      </div>
      <label class="relative inline-flex items-center cursor-pointer">
        <input
          v-model="localConfig.enable_continue_reply"
          type="checkbox"
          class="sr-only"
          @change="updateConfig"
        >
        <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-teal-300 dark:peer-focus:ring-teal-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-teal-600" />
      </label>
    </div>

    <!-- 自动继续阈值 -->
    <div v-if="localConfig.enable_continue_reply" class="space-y-2">
      <label class="text-sm font-medium text-gray-700 dark:text-gray-300">自动继续阈值</label>
      <p class="text-xs text-gray-500 dark:text-gray-400">
        当回复字符数超过此阈值时显示继续按钮
      </p>
      <div class="flex items-center space-x-3">
        <input
          v-model.number="localConfig.auto_continue_threshold"
          type="range"
          min="500"
          max="3000"
          step="100"
          class="flex-1 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700"
          @change="updateConfig"
        >
        <span class="text-sm text-gray-600 dark:text-gray-400 min-w-[60px]">
          {{ localConfig.auto_continue_threshold }} 字符
        </span>
      </div>
    </div>

    <!-- 继续提示词 -->
    <div v-if="localConfig.enable_continue_reply" class="space-y-2">
      <label class="text-sm font-medium text-gray-700 dark:text-gray-300">继续提示词</label>
      <p class="text-xs text-gray-500 dark:text-gray-400">
        点击继续按钮时发送的提示词
      </p>
      <input
        v-model="localConfig.continue_prompt"
        type="text"
        class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-teal-500 focus:border-teal-500 dark:bg-gray-700 dark:border-gray-600 dark:text-gray-100"
        placeholder="请继续"
        @input="updateConfig"
      >
    </div>

    <!-- 预设提示词 -->
    <div v-if="localConfig.enable_continue_reply" class="space-y-2">
      <label class="text-sm font-medium text-gray-700 dark:text-gray-300">预设提示词</label>
      <div class="grid grid-cols-2 gap-2">
        <button
          v-for="preset in presetPrompts"
          :key="preset"
          class="px-3 py-2 text-sm bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600 rounded-md transition-colors"
          @click="setPrompt(preset)"
        >
          {{ preset }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 自定义滑块样式 */
input[type="range"]::-webkit-slider-thumb {
  appearance: none;
  height: 20px;
  width: 20px;
  border-radius: 50%;
  background: #14b8a6;
  cursor: pointer;
  box-shadow: 0 0 2px 0 #555;
  transition: background .15s ease-in-out;
}

input[type="range"]::-webkit-slider-thumb:hover {
  background: #0d9488;
}

input[type="range"]::-moz-range-thumb {
  height: 20px;
  width: 20px;
  border-radius: 50%;
  background: #14b8a6;
  cursor: pointer;
  border: none;
  box-shadow: 0 0 2px 0 #555;
  transition: background .15s ease-in-out;
}

input[type="range"]::-moz-range-thumb:hover {
  background: #0d9488;
}
</style>
