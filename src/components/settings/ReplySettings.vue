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
  continue_prompt: '请按照最佳实践继续',
})

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

onMounted(() => {
  loadConfig()
})
</script>

<template>
  <div class="space-y-6">
    <div class="card-header">
      <div class="card-icon bg-blue-100">
        <span class="text-2xl">🔁</span>
      </div>
      <div>
        <h3 class="card-title">
          继续回复设置
        </h3>
        <p class="card-subtitle">
          配置 AI 回复继续行为
        </p>
      </div>
    </div>

    <!-- 启用继续回复 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center">
        <span class="w-2 h-2 bg-primary-500 rounded-full mr-3" />
        <div>
          <div class="text-sm font-medium card-text">启用继续回复</div>
          <div class="text-xs card-text-secondary">
            启用后将显示继续按钮
          </div>
        </div>
      </div>
      <button
        class="relative inline-flex h-6 w-11 items-center rounded-full focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-white"
        :class="[
          localConfig.enable_continue_reply ? 'bg-blue-600' : 'bg-gray-300',
        ]"
        @click="localConfig.enable_continue_reply = !localConfig.enable_continue_reply; updateConfig()"
      >
        <span
          class="inline-block h-4 w-4 transform rounded-full bg-white"
          :class="[
            localConfig.enable_continue_reply ? 'translate-x-6' : 'translate-x-1',
          ]"
        />
      </button>
    </div>

    <!-- 继续提示词 -->
    <div v-if="localConfig.enable_continue_reply" class="space-y-2">
      <div class="flex items-center">
        <span class="w-2 h-2 bg-green-500 rounded-full mr-3" />
        <div>
          <div class="text-sm font-medium card-text">继续提示词</div>
          <div class="text-xs card-text-secondary">
            点击继续按钮时发送的提示词
          </div>
        </div>
      </div>
      <input
        v-model="localConfig.continue_prompt"
        type="text"
        class="input"
        placeholder="请按照最佳实践继续"
        @input="updateConfig"
      >
    </div>
  </div>
</template>
