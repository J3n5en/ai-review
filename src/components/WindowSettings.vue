<script setup>
defineProps({
  alwaysOnTop: {
    type: Boolean,
    required: true,
  },
  audioNotificationEnabled: {
    type: Boolean,
    required: true,
  },
  audioUrl: {
    type: String,
    default: '',
  },
})

defineEmits(['toggleAlwaysOnTop', 'toggleAudioNotification', 'updateAudioUrl', 'testAudio'])
</script>

<template>
  <div class="card">
    <div class="card-header">
      <div class="card-icon bg-green-100">
        <span class="text-2xl">⚙️</span>
      </div>
      <div>
        <h3 class="card-title">
          窗口设置
        </h3>
        <p class="card-subtitle">
          配置窗口显示行为
        </p>
      </div>
    </div>

    <div class="space-y-6">
      <!-- 置顶显示设置 -->
      <div class="flex items-center justify-between">
        <div class="flex items-center">
          <span class="w-2 h-2 bg-blue-500 rounded-full mr-3" />
          <div>
            <div class="text-sm font-medium card-text">
              总在最前
            </div>
            <div class="text-xs card-text-secondary">
              启用后窗口将始终保持在其他应用程序之上
            </div>
          </div>
        </div>
        <button
          class="relative inline-flex h-6 w-11 items-center rounded-full focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-white"
          :class="[
            alwaysOnTop ? 'bg-blue-600' : 'bg-gray-300',
          ]"
          @click="$emit('toggleAlwaysOnTop')"
        >
          <span
            class="inline-block h-4 w-4 transform rounded-full bg-white"
            :class="[
              alwaysOnTop ? 'translate-x-6' : 'translate-x-1',
            ]"
          />
        </button>
      </div>

      <!-- 音频通知设置 -->
      <div class="flex items-center justify-between">
        <div class="flex items-center">
          <span class="w-2 h-2 bg-green-500 rounded-full mr-3" />
          <div>
            <div class="text-sm font-medium card-text">
              音频通知
            </div>
            <div class="text-xs card-text-secondary">
              启用后在MCP工具被触发时播放音频提示
            </div>
          </div>
        </div>
        <button
          class="relative inline-flex h-6 w-11 items-center rounded-full focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 focus:ring-offset-white"
          :class="[
            audioNotificationEnabled ? 'bg-green-600' : 'bg-gray-300',
          ]"
          @click="$emit('toggleAudioNotification')"
        >
          <span
            class="inline-block h-4 w-4 transform rounded-full bg-white"
            :class="[
              audioNotificationEnabled ? 'translate-x-6' : 'translate-x-1',
            ]"
          />
        </button>
      </div>

      <!-- 音效URL设置 -->
      <div v-if="audioNotificationEnabled" class="pt-4 border-t card-border">
        <div class="flex items-start">
          <span class="w-2 h-2 bg-primary-500 rounded-full mr-3 mt-2" />
          <div class="flex-1">
            <div class="text-sm font-medium card-text mb-2">
              自定义音效
            </div>
            <div class="text-xs card-text-secondary mb-3">
              留空使用默认音效，支持本地文件路径或网络URL（如：https://example.com/sound.mp3）
            </div>
            <div class="flex gap-2">
              <input
                type="text"
                :value="audioUrl"
                placeholder="音效文件路径或URL（可选）"
                class="input flex-1"
                @input="$emit('updateAudioUrl', $event.target.value)"
              >
              <button
                class="btn btn-primary"
                title="试听音效"
                @click="$emit('testAudio')"
              >
                <span class="text-xs">🔊</span>
                试听
              </button>
            </div>
            <div class="mt-2 text-xs card-text-secondary">
              示例：/path/to/sound.mp3 或 https://example.com/notification.wav
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
