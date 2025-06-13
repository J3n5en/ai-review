<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { useMagicKeys } from '@vueuse/core'
import hljs from 'highlight.js'
import MarkdownIt from 'markdown-it'
import { computed, nextTick, onMounted, ref, watch } from 'vue'
import { message } from '../utils/message'
import ThemeIcon from './common/ThemeIcon.vue'
import 'highlight.js/styles/github-dark.css'

interface McpRequest {
  id: string
  message: string
  predefined_options?: string[]
  is_markdown?: boolean
}

// Props
const props = defineProps<{
  request: McpRequest
  currentTheme?: string
}>()

// Emits
const emit = defineEmits<{
  response: [response: any[]]
  cancel: []
  themeChange: [theme: string]
}>()

// 响应式数据
const userInput = ref('')
const selectedOptions = ref<string[]>([])
const submitting = ref(false)
const textareaRef = ref<HTMLTextAreaElement | null>(null)
const loading = ref(false)
const draggedImages = ref<string[]>([])
const currentTheme = ref(props.currentTheme || 'dark') // 当前主题，从props同步

// 跨平台快捷键支持
const keys = useMagicKeys()
const ctrlEnter = keys['Ctrl+Enter']
const metaEnter = keys['Meta+Enter']

// 检测平台并显示正确的快捷键提示
const isMac = computed(() => {
  return navigator.platform.toUpperCase().includes('MAC')
})

const shortcutText = computed(() => {
  return isMac.value ? '⌘+回车 快速发送' : 'Ctrl+回车 快速发送'
})

// 计算属性 - 优化版本，减少不必要的计算
const canSubmit = computed(() => {
  const hasOptions = selectedOptions.value.length > 0
  const hasInput = userInput.value.trim().length > 0
  const hasImages = draggedImages.value.length > 0

  if (props.request?.predefined_options?.length) {
    return hasOptions || hasInput
  }
  return hasInput || hasImages
})

// 主题切换函数
function toggleTheme() {
  const themes = ['light', 'dark']
  const currentIndex = themes.indexOf(currentTheme.value)
  const nextIndex = (currentIndex + 1) % themes.length
  const newTheme = themes[nextIndex]
  currentTheme.value = newTheme

  // 立即应用主题到DOM
  document.documentElement.classList.remove('light', 'dark')
  document.documentElement.classList.add(newTheme)

  emit('themeChange', newTheme)
}

// 静态值，避免重复计算
const connectionStatus = '已连接'

// 创建 Markdown 实例
const md = new MarkdownIt({
  html: true, // 允许HTML标签
  xhtmlOut: false,
  breaks: true, // 换行符转换为<br>
  langPrefix: 'language-',
  linkify: true, // 自动转换链接
  typographer: true, // 启用智能引号等
  quotes: '""\'\'',
  // 启用代码高亮
  highlight(str: string, lang: string) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return hljs.highlight(str, { language: lang }).value
      }
      catch {
        // 忽略错误
      }
    }
    return '' // 使用默认的转义
  },
})

// Markdown 渲染函数
function renderMarkdown(content: string) {
  try {
    return md.render(content)
  }
  catch (error) {
    console.error('Markdown 渲染失败:', error)
    return content // 如果渲染失败，返回原始内容
  }
}

// 提交输入
function submitInput() {
  if (canSubmit.value && !submitting.value) {
    handleSubmit()
  }
}

function buildTextContent() {
  const textParts: string[] = []
  if (selectedOptions.value.length > 0) {
    textParts.push(`选择的选项: ${selectedOptions.value.join(', ')}`)
  }
  if (userInput.value.trim()) {
    textParts.push(userInput.value.trim())
  }
  return textParts.length > 0 ? textParts.join('\n\n') : null
}

// 构建图片内容
function buildImageContent() {
  const imageContent: any[] = []
  for (const imageDataUrl of draggedImages.value) {
    const matches = imageDataUrl.match(/^data:(image\/[^;]+);base64,(.+)$/)
    if (matches) {
      const [, mediaType, base64Data] = matches
      imageContent.push({
        type: 'image',
        source: { type: 'base64', media_type: mediaType, data: base64Data },
      })
    }
  }
  return imageContent
}

// 处理提交
async function handleSubmit() {
  if (!canSubmit.value || submitting.value)
    return

  submitting.value = true
  try {
    const responseContent: any[] = []

    const textContent = buildTextContent()
    if (textContent) {
      responseContent.push({ type: 'text', text: textContent })
    }

    responseContent.push(...buildImageContent())

    if (responseContent.length === 0) {
      responseContent.push({ type: 'text', text: '用户确认继续' })
    }

    emit('response', responseContent)
  }
  catch (error) {
    console.error('提交失败:', error)
    message.error('提交失败，请重试')
  }
  finally {
    submitting.value = false
  }
}

// 处理继续
async function handleContinue() {
  if (submitting.value)
    return

  submitting.value = true
  try {
    // 获取继续回复配置
    const replyConfig = await invoke('get_reply_config')
    const continuePrompt = replyConfig.continue_prompt || '请按照最佳实践继续'

    const continueResponse = [{
      type: 'text',
      text: continuePrompt,
    }]
    emit('response', continueResponse)
  }
  catch (error) {
    console.error('获取继续回复配置失败:', error)
    // 使用默认提示词
    const continueResponse = [{
      type: 'text',
      text: '请按照最佳实践继续',
    }]
    emit('response', continueResponse)
  }
  finally {
    submitting.value = false
  }
}

// 处理图片拖拽
function handleImageDrop(event: DragEvent) {
  event.preventDefault()
  const files = event.dataTransfer?.files
  if (files) {
    handleImageFiles(files)
  }
}

function handleImagePaste(event: ClipboardEvent) {
  const items = event.clipboardData?.items
  let hasImage = false

  if (items) {
    for (const item of items) {
      if (item.type.includes('image')) {
        hasImage = true
        const file = item.getAsFile()
        if (file) {
          handleImageFiles([file])
        }
      }
    }
  }

  // 如果检测到图片，阻止默认的粘贴行为（防止文件名被粘贴到输入框）
  if (hasImage) {
    event.preventDefault()
  }
}

async function handleImageFiles(files: FileList | File[]) {
  for (const file of files) {
    if (file.type.startsWith('image/')) {
      try {
        const base64 = await fileToBase64(file)
        draggedImages.value.push(base64)
        message.success(`图片 ${file.name} 已添加`)
      }
      catch {
        message.error(`图片 ${file.name} 处理失败`)
      }
    }
  }
}

function fileToBase64(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = () => resolve(reader.result as string)
    reader.onerror = reject
    reader.readAsDataURL(file)
  })
}

function removeImage(index: number) {
  draggedImages.value.splice(index, 1)
}

// 创建复制按钮
function createCopyButton(preEl: Element) {
  if (preEl.querySelector('.copy-button'))
    return

  const copyButton = document.createElement('button')
  copyButton.className = 'copy-button absolute top-2 right-2 px-2 py-1 text-xs rounded transition-colors'
  copyButton.innerHTML = '📋 复制'
  copyButton.classList.add('bg-gray-200', 'text-gray-700', 'hover:bg-gray-300', 'dark:bg-gray-700', 'dark:text-gray-300', 'dark:hover:bg-gray-600')

  copyButton.addEventListener('click', async (e) => {
    e.stopPropagation()
    try {
      const codeEl = preEl.querySelector('code')
      const text = codeEl?.textContent || preEl.textContent || ''
      await navigator.clipboard.writeText(text)
      copyButton.innerHTML = '✅ 已复制'
      setTimeout(() => {
        copyButton.innerHTML = '📋 复制'
      }, 2000)
      message.success('代码已复制到剪贴板')
    }
    catch {
      message.error('复制失败')
    }
  })

  ; (preEl as HTMLElement).style.position = 'relative'
  preEl.appendChild(copyButton)
}

// 设置内联代码复制
function setupInlineCodeCopy() {
  const inlineCodeElements = document.querySelectorAll('.markdown-content p code, .markdown-content li code')
  inlineCodeElements.forEach((codeEl) => {
    codeEl.addEventListener('click', async () => {
      try {
        await navigator.clipboard.writeText(codeEl.textContent || '')
        message.success('代码已复制到剪贴板')
      }
      catch {
        message.error('复制失败')
      }
    })
  })
}

// 设置代码复制功能 - 优化版本，使用防抖避免重复执行
let setupCodeCopyTimer: number | null = null
function setupCodeCopy() {
  // 清除之前的定时器，避免重复执行
  if (setupCodeCopyTimer) {
    clearTimeout(setupCodeCopyTimer)
  }

  setupCodeCopyTimer = window.setTimeout(() => {
    nextTick(() => {
      const preElements = document.querySelectorAll('.markdown-content pre')
      preElements.forEach(createCopyButton)
      setupInlineCodeCopy()
    })
  }, 50) // 短暂延迟，让DOM稳定后再执行
}

// 生命周期 - 优化版本，确保丝滑体验
onMounted(() => {
  // 使用 requestIdleCallback 在浏览器空闲时设置代码复制功能
  // 如果不支持则回退到 requestAnimationFrame
  if (window.requestIdleCallback) {
    window.requestIdleCallback(() => {
      setupCodeCopy()
    })
  }
  else {
    requestAnimationFrame(() => {
      setupCodeCopy()
    })
  }
})

// 监听request变化，重新设置代码复制
watch(() => props.request, () => {
  setupCodeCopy()
  // 移除自动焦点设置，让用户自然交互
}, { deep: true })

// 监听父组件主题变化
watch(() => props.currentTheme, (newTheme) => {
  if (newTheme) {
    currentTheme.value = newTheme
  }
})

// 监听跨平台快捷键
watch([ctrlEnter, metaEnter], ([ctrlPressed, metaPressed]) => {
  if (ctrlPressed || metaPressed) {
    submitInput()
  }
})

// 自动调整textarea高度
function autoResizeTextarea() {
  const textarea = textareaRef.value
  if (!textarea)
    return

  // 重置高度以获取正确的scrollHeight
  textarea.style.height = 'auto'

  // 计算新高度，限制最大高度
  const maxHeight = 200 // 最大高度200px
  const newHeight = Math.min(textarea.scrollHeight, maxHeight)

  textarea.style.height = `${newHeight}px`
  textarea.style.overflowY = textarea.scrollHeight > maxHeight ? 'auto' : 'hidden'
}

// 监听用户输入变化，自动调整高度
watch(userInput, () => {
  nextTick(() => {
    autoResizeTextarea()
  })
})

// 组件挂载后初始化textarea高度
onMounted(() => {
  nextTick(() => {
    autoResizeTextarea()
  })
})
</script>

<template>
  <div
    class="fixed inset-0 flex flex-col z-50 popup-container transition-all duration-200"
  >
    <div
      class="relative w-full h-full flex flex-col shadow-xl popup-content transition-all duration-200"
    >
      <!-- 头部区域 -->
      <div
        class="flex items-center justify-between px-4 py-2 border-b card-border card-bg transition-all duration-200"
      >
        <div class="flex items-center gap-2">
          <div class="w-2 h-2 rounded-full bg-green-500" />
          <span class="text-sm card-text-secondary">寸止 - 告别AI提前终止烦恼</span>
        </div>
        <div class="flex items-center gap-2">
          <!-- 主题切换按钮 -->
          <button
            class="p-2 rounded-lg hover-bg transition-colors duration-200 flex items-center justify-center"
            :title="`切换到${currentTheme === 'light' ? '深色' : '浅色'}主题`"
            @click="toggleTheme"
          >
            <ThemeIcon :theme="currentTheme" class="card-text-secondary w-4 h-4" />
          </button>
        </div>
      </div>

      <!-- 内容区域 -->
      <div
        class="flex-1 overflow-y-auto p-2 space-y-1.5 card-bg-secondary transition-all duration-200"
        @drop="handleImageDrop"
        @dragover.prevent
        @dragenter.prevent
      >
        <!-- 简化的加载状态 -->
        <div
          v-if="loading"
          class="flex items-center justify-center py-8"
        >
          <div class="text-center">
            <div class="w-6 h-6 border-2 border-primary-500 border-t-transparent rounded-full animate-spin mx-auto mb-3" />
            <p class="card-text-secondary text-sm">
              加载中...
            </p>
          </div>
        </div>

        <!-- 消息显示区域 -->
        <div
          v-else-if="request?.message"
          class="mb-1.5"
        >
          <div
            class="card rounded-lg p-2.5 shadow-sm transition-all duration-200"
          >
            <div
              class="leading-relaxed text-sm markdown-content popup-markdown-text text-left transition-all duration-200"
            >
              <div
                v-if="request.is_markdown"
                class="markdown-rendered"
                v-html="renderMarkdown(request.message)"
              />
              <div
                v-else
                class="whitespace-pre-wrap text-left"
              >
                {{ request.message }}
              </div>
            </div>
          </div>
        </div>

        <!-- 错误状态 -->
        <div
          v-else
          class="rounded-lg p-4 border bg-red-50 border-red-200"
        >
          <div class="text-red-800">
            <h4 class="font-medium mb-1 text-sm">
              ❌ 数据加载错误
            </h4>
            <p class="text-xs">
              Request对象: {{ JSON.stringify(request) }}
            </p>
          </div>
        </div>

        <!-- 预定义选项 -->
        <div
          v-if="!loading && request.predefined_options && request.predefined_options.length > 0"
          class="mb-1.5"
        >
          <h4 class="text-sm font-medium mb-1.5 card-text">
            请选择选项
          </h4>

          <div class="w-full">
            <div class="grid gap-1">
              <label
                v-for="(option, index) in request.predefined_options"
                :key="`option-${index}`"
                class="checkbox flex items-center p-2 rounded-lg transition-all duration-200 group card hover:bg-primary-50 cursor-pointer smooth-option"
              >
                <input
                  v-model="selectedOptions"
                  type="checkbox"
                  :value="option"
                  class="absolute w-px h-px p-0 -m-px overflow-hidden clip-rect-0 whitespace-nowrap border-0"
                >
                <div class="checkbox-box" />
                <span
                  class="ml-3 text-sm card-text group-hover:text-primary-700"
                >{{ option }}</span>
              </label>
            </div>
          </div>
        </div>

        <!-- 图片预览区域 -->
        <div
          v-if="!loading && draggedImages.length > 0"
          class="mb-3"
        >
          <h4 class="text-sm font-medium mb-2 card-text">
            已添加的图片
          </h4>
          <div class="grid grid-cols-2 gap-2">
            <div
              v-for="(image, index) in draggedImages"
              :key="index"
              class="relative rounded-md overflow-hidden border-2 border-gray-300"
            >
              <img
                :src="image"
                class="w-full h-20 object-cover"
              >
              <button
                class="absolute top-1 right-1 w-5 h-5 rounded-full flex items-center justify-center text-xs bg-red-500 text-white"
                @click.exact="removeImage(index)"
              >
                ×
              </button>
            </div>
          </div>
        </div>

        <!-- 通用回复输入 -->
        <div
          v-if="!loading"
          class="mb-1.5"
        >
          <h4 class="text-sm font-medium mb-1.5 card-text">
            {{ request.predefined_options ? '补充说明 (可选)' : '请输入您的回复' }}
          </h4>

          <div
            class="relative rounded-md border-2 border-dashed p-1.5 mb-1.5 card-border card-bg-accent"
          >
            <p class="text-xs text-center card-text-secondary">
              拖拽图片到此处或在输入框中粘贴图片 (⌘+V)
            </p>
          </div>

          <textarea
            ref="textareaRef"
            v-model="userInput"
            :placeholder="request.predefined_options ? '您可以在这里添加补充说明...' : '请输入您的回复...'"
            class="textarea smooth-textarea auto-resize-textarea"
            :disabled="submitting"
            @paste="handleImagePaste"
            @input="autoResizeTextarea"
          />
        </div>
      </div>

      <!-- 底部操作栏 -->
      <div class="border-t px-3 py-2.5 card-border card-bg-accent">
        <div
          v-if="!loading"
          class="flex justify-between items-center"
        >
          <div class="flex items-center gap-3">
            <!-- 连接状态 -->
            <div class="flex items-center gap-2 text-xs">
              <div class="w-2 h-2 rounded-full bg-green-500" />
              <span class="card-text">{{ connectionStatus }}</span>
              <span class="card-text-secondary">|</span>
              <span class="card-text-secondary">
                {{ request.predefined_options ? '选择选项或输入文本' : shortcutText }}
              </span>
            </div>
          </div>

          <div class="flex items-center gap-2">
            <!-- 继续按钮 - 弱化样式 -->
            <button
              :disabled="submitting"
              class="px-3 py-2 text-sm btn-secondary rounded-md transition-colors duration-200 flex items-center gap-1"
              :class="submitting ? 'opacity-50' : ''"
              @click="handleContinue"
            >
              <span class="text-xs">▶</span>
              <span>继续</span>
            </button>

            <!-- 发送按钮 - 强化样式 -->
            <button
              :disabled="!canSubmit || submitting"
              class="px-4 py-2 text-sm font-medium rounded-md transition-all duration-200 flex items-center gap-2 shadow-md"
              :class="[
                canSubmit && !submitting
                  ? 'bg-primary-500 text-white hover:bg-primary-600 hover:shadow-lg transform hover:scale-105'
                  : 'bg-gray-300 text-gray-500',
              ]"
              @click="handleSubmit"
            >
              <!-- 加载动画 -->
              <div
                v-if="submitting"
                class="w-3 h-3 border-2 border-white border-t-transparent rounded-full animate-spin"
              />
              <!-- 发送图标 -->
              <span
                v-else
                class="text-sm"
              >↗</span>
              <span>{{ submitting ? '发送中...' : '发送' }}</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
