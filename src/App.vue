<script setup>
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { onMounted, ref } from 'vue'
import McpPopup from './components/McpPopup.vue'
import ThemeSettings from './components/ThemeSettings.vue'
import WindowSettings from './components/WindowSettings.vue'
import { REFERENCE_PROMPT } from './constants/prompts'
import { message } from './utils/message'

// 响应式数据
const mcpRequest = ref(null)
const showMcpPopup = ref(false)
const currentTheme = ref('dark')
const alwaysOnTop = ref(true)
const audioNotificationEnabled = ref(true)
const audioUrl = ref('')
const activeTab = ref('intro') // 'intro' 或 'settings'
const windowConfig = ref({
  auto_resize: true,
  max_width: 800,
  max_height: 1000,
  min_width: 600,
  min_height: 800,
})
const copyButtonText = ref('复制')

// 参考提示词内容
const promptContent = ref(REFERENCE_PROMPT)

// 应用主题
function applyTheme(theme) {
  // 移除所有主题类
  document.documentElement.classList.remove('light', 'dark')

  let effectiveTheme = theme

  // 如果是系统跟随模式，检测系统主题
  if (theme === 'system') {
    effectiveTheme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
  }

  // 应用主题类
  document.documentElement.classList.add(effectiveTheme)
  currentTheme.value = theme
}

// 切换主题
async function setTheme(theme) {
  try {
    await invoke('set_theme', { theme })
    applyTheme(theme)
  }
  catch (error) {
    console.error('切换主题失败:', error)
  }
}

// 加载主题设置
async function loadTheme() {
  try {
    const theme = await invoke('get_theme')
    applyTheme(theme)
  }
  catch (error) {
    // 默认使用深色主题
    applyTheme('dark')
    console.error('加载主题失败:', error)
  }
}

// 统一的MCP响应处理
async function handleMcpResponse(response) {
  try {
    // 通过Tauri命令发送响应并退出应用
    await invoke('send_mcp_response', { response })
    await invoke('exit_app')
  }
  catch (error) {
    console.error('处理MCP响应失败:', error)
  }
}

// 统一的MCP取消处理
async function handleMcpCancel() {
  try {
    // 发送取消信息并退出应用
    await invoke('send_mcp_response', { response: 'CANCELLED' })
    await invoke('exit_app')
  }
  catch (error) {
    console.error('处理MCP取消失败:', error)
  }
}

// 加载窗口设置
async function loadWindowSettings() {
  try {
    const enabled = await invoke('get_always_on_top')
    alwaysOnTop.value = enabled

    // 加载音频通知设置
    const audioEnabled = await invoke('get_audio_notification_enabled')
    audioNotificationEnabled.value = audioEnabled

    // 加载音效URL设置
    const audioUrlValue = await invoke('get_audio_url')
    audioUrl.value = audioUrlValue

    // 同步窗口状态
    await invoke('sync_window_state')
  }
  catch (error) {
    console.error('加载窗口设置失败:', error)
  }
}

// 切换置顶设置
async function toggleAlwaysOnTop() {
  try {
    const newValue = !alwaysOnTop.value
    await invoke('set_always_on_top', { enabled: newValue })
    alwaysOnTop.value = newValue
  }
  catch (error) {
    console.error('切换置顶设置失败:', error)
  }
}

// 切换音频通知设置
async function toggleAudioNotification() {
  try {
    const newValue = !audioNotificationEnabled.value
    await invoke('set_audio_notification_enabled', { enabled: newValue })
    audioNotificationEnabled.value = newValue
  }
  catch (error) {
    console.error('切换音频通知设置失败:', error)
  }
}

// 更新音效URL
async function updateAudioUrl(url) {
  try {
    await invoke('set_audio_url', { url })
    audioUrl.value = url
  }
  catch (error) {
    console.error('更新音效URL失败:', error)
  }
}

// 测试音效
async function testAudioSound() {
  try {
    await invoke('test_audio_sound')
    // 显示成功提示
    message.success('音效测试播放成功')
  }
  catch (error) {
    // 显示错误提示给用户
    message.error(`音效测试失败: ${error}`)
  }
}

// 加载窗口配置
async function loadWindowConfig() {
  try {
    const config = await invoke('get_window_config')
    windowConfig.value = config
  }
  catch (error) {
    console.error('加载窗口配置失败:', error)
  }
}

// 显示MCP弹窗
async function showMcpDialog(request) {
  mcpRequest.value = request
  showMcpPopup.value = true

  // 播放音频通知
  try {
    await invoke('play_notification_sound')
  }
  catch (error) {
    console.error('显示MCP弹窗失败:', error)
  }
}

// 检查MCP模式
async function checkMcpMode() {
  try {
    const args = await invoke('get_cli_args')

    if (args && args.mcp_request) {
      // 读取MCP请求文件
      const content = await invoke('read_mcp_request', { filePath: args.mcp_request })

      if (content) {
        showMcpDialog(content)
      }
      return true // 表示是MCP模式
    }
  }
  catch (error) {
    console.error('检查MCP模式失败:', error)
  }
  return false // 表示不是MCP模式
}

// 设置MCP事件监听器
async function setupMcpEventListener() {
  try {
    await listen('mcp-request', (event) => {
      showMcpDialog(event.payload)
    })
  }
  catch (error) {
    console.error('设置MCP事件监听器失败:', error)
  }
}

// 复制参考提示词内容
async function copyPromptContent() {
  try {
    await navigator.clipboard.writeText(promptContent.value)
    copyButtonText.value = '已复制'
    setTimeout(() => {
      copyButtonText.value = '复制'
    }, 2000)
  }
  catch (error) {
    copyButtonText.value = '复制失败'
    setTimeout(() => {
      copyButtonText.value = '复制'
    }, 2000)
    console.error('复制失败:', error)
  }
}

// 初始化
onMounted(async () => {
  // 首先加载主题设置
  await loadTheme()

  // 检查是否为MCP模式
  const isMcp = await checkMcpMode()

  // 无论是否为MCP模式，都加载窗口设置
  await loadWindowSettings()
  await loadWindowConfig()

  // 如果不是MCP模式，设置事件监听器
  if (!isMcp) {
    await setupMcpEventListener()
  }

  // 监听系统主题变化（仅在系统跟随模式下）
  if (currentTheme.value === 'system') {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    mediaQuery.addEventListener('change', () => {
      if (currentTheme.value === 'system') {
        applyTheme('system')
      }
    })
  }
})
</script>

<template>
  <div id="app"
    class="min-h-screen bg-light-primary dark:bg-dark-primary transition-colors duration-300">
    <!-- MCP弹窗 -->
    <McpPopup v-if="showMcpPopup && mcpRequest"
      :request="mcpRequest"
      @response="handleMcpResponse"
      @cancel="handleMcpCancel"
      @theme-change="setTheme" />

    <!-- 主界面 -->
    <div v-else
      class="flex items-center justify-center min-h-screen p-6">
      <div class="max-w-6xl w-full">
        <!-- 主标题 -->
        <div class="text-center mb-8">
          <div class="w-20 h-20 rounded-2xl flex items-center justify-center mx-auto mb-4 shadow-lg overflow-hidden">
            <img src="/icons/icon-128.png"
              alt="Zhi Logo"
              class="w-full h-full object-contain">
          </div>
          <h1 class="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-2 transition-colors duration-300">
            寸止
          </h1>
          <p class="text-lg text-gray-600 dark:text-gray-400 transition-colors duration-300">
            智能代码审查与交互工具
          </p>
        </div>

        <!-- Tab导航 -->
        <div class="flex justify-center mb-8">
          <div
            class="tab-container bg-gray-100 dark:bg-dark-secondary rounded-lg p-1 border border-gray-300 dark:border-gray-700 flex transition-colors duration-300">
            <button
              class="tab-button px-6 py-2 rounded-md text-sm font-medium transition-all duration-200 relative z-10"
              :class="[
                activeTab === 'intro'
                  ? 'tab-active'
                  : 'tab-inactive',
              ]"
              @click="activeTab = 'intro'">
              介绍
            </button>
            <button
              class="tab-button px-6 py-2 rounded-md text-sm font-medium transition-all duration-200 relative z-10"
              :class="[
                activeTab === 'settings'
                  ? 'tab-active'
                  : 'tab-inactive',
              ]"
              @click="activeTab = 'settings'">
              设置
            </button>
            <button
              class="tab-button px-6 py-2 rounded-md text-sm font-medium transition-all duration-200 relative z-10"
              :class="[
                activeTab === 'prompts'
                  ? 'tab-active'
                  : 'tab-inactive',
              ]"
              @click="activeTab = 'prompts'">
              参考提示词
            </button>
          </div>
        </div>

        <!-- Tab内容区域 -->
        <!-- 介绍Tab -->
        <div v-if="activeTab === 'intro'"
          class="tab-content">
          <!-- 功能卡片 -->
          <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
            <!-- MCP服务器功能 -->
            <div
              class="bg-white dark:bg-dark-secondary rounded-xl p-6 shadow-lg border border-gray-200 dark:border-gray-700 transition-colors duration-300">
              <div class="flex items-center mb-4">
                <div
                  class="w-12 h-12 bg-blue-100 dark:bg-blue-900 rounded-lg flex items-center justify-center mr-4 transition-colors duration-300">
                  <span class="text-2xl">🔧</span>
                </div>
                <div>
                  <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 transition-colors duration-300">
                    MCP 服务器
                  </h3>
                  <p class="text-sm text-gray-600 dark:text-gray-400 transition-colors duration-300">
                    Model Context Protocol
                  </p>
                </div>
              </div>
              <ul class="space-y-2 text-sm text-gray-600 dark:text-gray-400 transition-colors duration-300">
                <li class="flex items-center">
                  <span class="w-2 h-2 bg-green-500 rounded-full mr-2" />
                  智能代码审查交互
                </li>
                <li class="flex items-center">
                  <span class="w-2 h-2 bg-green-500 rounded-full mr-2" />
                  支持文本和图片输入
                </li>
                <li class="flex items-center">
                  <span class="w-2 h-2 bg-green-500 rounded-full mr-2" />
                  预定义选项支持
                </li>
                <li class="flex items-center">
                  <span class="w-2 h-2 bg-green-500 rounded-full mr-2" />
                  Markdown 渲染
                </li>
              </ul>
            </div>

            <!-- 记忆管理功能 -->
            <div
              class="bg-white dark:bg-dark-secondary rounded-xl p-6 shadow-lg border border-gray-200 dark:border-gray-700 transition-colors duration-300">
              <div class="flex items-center mb-4">
                <div
                  class="w-12 h-12 bg-purple-100 dark:bg-purple-900 rounded-lg flex items-center justify-center mr-4 transition-colors duration-300">
                  <span class="text-2xl">🧠</span>
                </div>
                <div>
                  <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 transition-colors duration-300">
                    记忆管理
                  </h3>
                  <p class="text-sm text-gray-600 dark:text-gray-400 transition-colors duration-300">
                    智能记忆系统
                  </p>
                </div>
              </div>
              <ul class="space-y-2 text-sm text-gray-600 dark:text-gray-400 transition-colors duration-300">
                <li class="flex items-center">
                  <span class="w-2 h-2 bg-green-500 rounded-full mr-2" />
                  开发规范存储
                </li>
                <li class="flex items-center">
                  <span class="w-2 h-2 bg-green-500 rounded-full mr-2" />
                  用户偏好记录
                </li>
                <li class="flex items-center">
                  <span class="w-2 h-2 bg-green-500 rounded-full mr-2" />
                  项目信息管理
                </li>
                <li class="flex items-center">
                  <span class="w-2 h-2 bg-green-500 rounded-full mr-2" />
                  最佳实践收集
                </li>
              </ul>
            </div>
          </div>

          <!-- 状态指示 -->
          <div class="mt-6 text-center">
            <div
              class="inline-flex items-center px-4 py-2 bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-300 rounded-full text-sm transition-colors duration-300">
              <div class="w-2 h-2 bg-green-500 rounded-full mr-2 animate-pulse" />
              MCP 服务器就绪
            </div>
          </div>
        </div>

        <!-- 设置Tab -->
        <div v-else-if="activeTab === 'settings'"
          class="max-w-3xl mx-auto space-y-6 tab-content">
          <!-- 主题设置组件 -->
          <ThemeSettings :current-theme="currentTheme"
            @theme-change="setTheme" />

          <!-- 窗口设置组件 -->
          <WindowSettings :always-on-top="alwaysOnTop"
            :audio-notification-enabled="audioNotificationEnabled"
            :audio-url="audioUrl"
            @toggle-always-on-top="toggleAlwaysOnTop"
            @toggle-audio-notification="toggleAudioNotification"
            @update-audio-url="updateAudioUrl"
            @test-audio="testAudioSound" />
        </div>

        <!-- 参考提示词Tab -->
        <div v-else-if="activeTab === 'prompts'"
          class="max-w-4xl mx-auto tab-content">
          <div
            class="bg-white dark:bg-dark-secondary rounded-xl p-6 shadow-lg border border-gray-200 dark:border-gray-700 transition-colors duration-300">
            <div class="flex items-center justify-between mb-6">
              <div class="flex items-center">
                <div
                  class="w-12 h-12 bg-orange-100 dark:bg-orange-900 rounded-lg flex items-center justify-center mr-4 transition-colors duration-300">
                  <span class="text-2xl">📝</span>
                </div>
                <div>
                  <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 transition-colors duration-300">
                    参考提示词
                  </h3>
                  <p class="text-sm text-gray-600 dark:text-gray-400 transition-colors duration-300">
                    AI助手交互规范和智能记忆管理指南
                  </p>
                </div>
              </div>
              <button class="btn btn-primary"
                @click="copyPromptContent">
                <span class="text-sm mr-2">📋</span>
                {{ copyButtonText }}
              </button>
            </div>

            <div
              class="bg-gray-100 dark:bg-gray-900 rounded-lg p-4 text-sm text-gray-800 dark:text-gray-300 font-mono leading-relaxed max-h-96 overflow-y-auto transition-colors duration-300">
              <pre class="whitespace-pre-wrap">{{ promptContent }}</pre>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 确保平滑的主题切换动画 */
#app {
  transition:
    background-color 0.3s ease,
    color 0.3s ease;
}

/* 加载动画 */
@keyframes pulse {

  0%,
  100% {
    opacity: 1;
  }

  50% {
    opacity: 0.5;
  }
}

.animate-pulse {
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}
</style>
