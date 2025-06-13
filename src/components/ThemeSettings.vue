<script setup>
defineProps({
  currentTheme: {
    type: String,
    required: true,
  },
})

defineEmits(['themeChange'])
</script>

<template>
  <div class="card">
    <div class="card-header">
      <div class="card-icon bg-purple-100">
        <span class="text-2xl">🎨</span>
      </div>
      <div>
        <h3 class="card-title">
          主题设置
        </h3>
        <p class="card-subtitle">
          选择您喜欢的界面主题
        </p>
      </div>
    </div>

    <div class="flex items-center justify-between">
      <div class="flex items-center">
        <span class="w-2 h-2 bg-primary-500 rounded-full mr-3" />
        <div>
          <div class="text-sm font-medium card-text">
            界面主题
          </div>
          <div class="text-xs card-text-secondary">
            选择浅色、深色或跟随系统
          </div>
        </div>
      </div>
      <div class="flex items-center space-x-2">
        <!-- 浅色主题 -->
        <button
          class="flex items-center px-3 py-2 rounded-lg text-xs font-medium transition-all duration-200"
          :class="[
            currentTheme === 'light'
              ? 'btn-primary'
              : 'btn-secondary',
          ]"
          @click="$emit('themeChange', 'light')"
        >
          <div class="w-3 h-3 bg-white rounded-full border border-gray-300 mr-2" />
          浅色
        </button>

        <!-- 深色主题 -->
        <button
          class="flex items-center px-3 py-2 rounded-lg text-xs font-medium transition-all duration-200"
          :class="[
            currentTheme === 'dark'
              ? 'btn-primary'
              : 'btn-secondary',
          ]"
          @click="$emit('themeChange', 'dark')"
        >
          <div class="w-3 h-3 bg-gray-800 rounded-full border border-gray-600 mr-2" />
          深色
        </button>

        <!-- 系统跟随 -->
        <button
          class="flex items-center px-3 py-2 rounded-lg text-xs font-medium transition-all duration-200"
          :class="[
            currentTheme === 'system'
              ? 'btn-primary'
              : 'btn-secondary',
          ]"
          @click="$emit('themeChange', 'system')"
        >
          <div class="w-3 h-3 bg-gradient-to-r from-white to-gray-800 rounded-full border border-gray-500 mr-2" />
          跟随
        </button>
      </div>
    </div>
  </div>
</template>
