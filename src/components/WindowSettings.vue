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

defineEmits(['toggle-always-on-top', 'toggle-audio-notification', 'update-audio-url', 'test-audio'])
</script>

<template>
  <div class="bg-white dark:bg-dark-secondary rounded-xl p-6 shadow-lg border border-gray-200 dark:border-gray-700 transition-colors duration-300">
    <div class="flex items-center mb-6">
      <div class="w-12 h-12 bg-green-100 dark:bg-green-900 rounded-lg flex items-center justify-center mr-4 transition-colors duration-300">
        <span class="text-2xl">⚙️</span>
      </div>
      <div>
        <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 transition-colors duration-300">
          窗口设置
        </h3>
        <p class="text-sm text-gray-600 dark:text-gray-400 transition-colors duration-300">
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
            <div class="text-sm font-medium text-gray-900 dark:text-gray-100 transition-colors duration-300">
              总在最前
            </div>
            <div class="text-xs text-gray-500 dark:text-gray-400 transition-colors duration-300">
              启用后窗口将始终保持在其他应用程序之上
            </div>
          </div>
        </div>
        <button
          class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-white dark:focus:ring-offset-gray-800"
          :class="[
            alwaysOnTop ? 'bg-blue-600' : 'bg-gray-300 dark:bg-gray-600',
          ]"
          @click="$emit('toggle-always-on-top')"
        >
          <span
            class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform duration-200 ease-in-out"
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
            <div class="text-sm font-medium text-gray-900 dark:text-gray-100 transition-colors duration-300">
              音频通知
            </div>
            <div class="text-xs text-gray-500 dark:text-gray-400 transition-colors duration-300">
              启用后在MCP工具被触发时播放音频提示
            </div>
          </div>
        </div>
        <button
          class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 focus:ring-offset-white dark:focus:ring-offset-gray-800"
          :class="[
            audioNotificationEnabled ? 'bg-green-600' : 'bg-gray-300 dark:bg-gray-600',
          ]"
          @click="$emit('toggle-audio-notification')"
        >
          <span
            class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform duration-200 ease-in-out"
            :class="[
              audioNotificationEnabled ? 'translate-x-6' : 'translate-x-1',
            ]"
          />
        </button>
      </div>

      <!-- 音效URL设置 -->
      <div v-if="audioNotificationEnabled" class="pt-4 border-t border-gray-200 dark:border-gray-600">
        <div class="flex items-start">
          <span class="w-2 h-2 bg-orange-500 rounded-full mr-3 mt-2" />
          <div class="flex-1">
            <div class="text-sm font-medium text-gray-900 dark:text-gray-100 mb-2 transition-colors duration-300">
              自定义音效
            </div>
            <div class="text-xs text-gray-500 dark:text-gray-400 mb-3 transition-colors duration-300">
              留空使用默认音效，支持本地文件路径或网络URL（如：https://example.com/sound.mp3）
            </div>
            <div class="flex gap-2">
              <input
                type="text"
                :value="audioUrl"
                placeholder="音效文件路径或URL（可选）"
                class="flex-1 px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-orange-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 transition-colors duration-300"
                @input="$emit('update-audio-url', $event.target.value)"
              >
              <button
                class="px-3 py-2 text-sm bg-orange-500 hover:bg-orange-600 text-white rounded-md transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-orange-500 focus:ring-offset-2 focus:ring-offset-white dark:focus:ring-offset-gray-800 flex items-center gap-1"
                title="试听音效"
                @click="$emit('test-audio')"
              >
                <span class="text-xs">🔊</span>
                试听
              </button>
            </div>
            <div class="mt-2 text-xs text-gray-500 dark:text-gray-400">
              示例：/path/to/sound.mp3 或 https://example.com/notification.wav
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
