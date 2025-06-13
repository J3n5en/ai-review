<script setup>
defineProps({
  currentTheme: {
    type: String,
    required: true,
  },
})

defineEmits(['theme-change'])
</script>

<template>
  <div class="bg-white dark:bg-dark-secondary rounded-xl p-6 shadow-lg border border-gray-200 dark:border-gray-700 transition-colors duration-300">
    <div class="flex items-center mb-6">
      <div class="w-12 h-12 bg-purple-100 dark:bg-purple-900 rounded-lg flex items-center justify-center mr-4 transition-colors duration-300">
        <span class="text-2xl">🎨</span>
      </div>
      <div>
        <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 transition-colors duration-300">
          主题设置
        </h3>
        <p class="text-sm text-gray-600 dark:text-gray-400 transition-colors duration-300">
          选择您喜欢的界面主题
        </p>
      </div>
    </div>

    <div class="flex items-center justify-between">
      <span class="text-sm font-medium text-gray-900 dark:text-gray-100 transition-colors duration-300">界面主题</span>
      <div class="flex items-center space-x-2">
        <!-- 浅色主题 -->
        <button
          class="flex items-center px-3 py-2 rounded-lg text-xs font-medium transition-all duration-200"
          :class="[
            currentTheme === 'light'
              ? 'bg-blue-500 text-white'
              : 'bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600',
          ]"
          @click="$emit('theme-change', 'light')"
        >
          <div class="w-3 h-3 bg-white rounded-full border border-gray-300 mr-2" />
          浅色
        </button>

        <!-- 深色主题 -->
        <button
          class="flex items-center px-3 py-2 rounded-lg text-xs font-medium transition-all duration-200"
          :class="[
            currentTheme === 'dark'
              ? 'bg-blue-500 text-white'
              : 'bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600',
          ]"
          @click="$emit('theme-change', 'dark')"
        >
          <div class="w-3 h-3 bg-gray-800 rounded-full border border-gray-600 mr-2" />
          深色
        </button>

        <!-- 系统跟随 -->
        <button
          class="flex items-center px-3 py-2 rounded-lg text-xs font-medium transition-all duration-200"
          :class="[
            currentTheme === 'system'
              ? 'bg-blue-500 text-white'
              : 'bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600',
          ]"
          @click="$emit('theme-change', 'system')"
        >
          <div class="w-3 h-3 bg-gradient-to-r from-white to-gray-800 rounded-full border border-gray-500 mr-2" />
          跟随
        </button>
      </div>
    </div>
  </div>
</template>
