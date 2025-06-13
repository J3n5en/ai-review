// 自定义消息提示系统
let messageContainer: HTMLDivElement | null = null

// 创建消息容器
function createMessageContainer() {
  if (messageContainer)
    return messageContainer

  messageContainer = document.createElement('div')
  messageContainer.className = 'message-container'
  document.body.appendChild(messageContainer)
  return messageContainer
}

// 创建单个消息
function createMessage(content: string, type: string = 'info', duration: number = 3000) {
  const container = createMessageContainer()

  const messageEl = document.createElement('div')
  messageEl.className = `message-item message-${type}`

  // 添加图标和内容
  const icon = getIcon(type)
  messageEl.innerHTML = `
    <span class="flex-shrink-0">${icon}</span>
    <span class="flex-1">${content}</span>
  `

  container.appendChild(messageEl)

  // 自动移除
  setTimeout(() => {
    messageEl.style.opacity = '0'
    messageEl.style.transform = 'translateY(-20px)'
    setTimeout(() => {
      if (messageEl.parentNode) {
        messageEl.parentNode.removeChild(messageEl)
      }
    }, 300)
  }, duration)
}

function getIcon(type: string) {
  switch (type) {
    case 'success':
      return '✅'
    case 'error':
      return '❌'
    case 'warning':
      return '⚠️'
    default:
      return 'ℹ️'
  }
}

// 导出消息API
export const message = {
  success: (content: string, duration?: number) => createMessage(content, 'success', duration),
  error: (content: string, duration?: number) => createMessage(content, 'error', duration),
  warning: (content: string, duration?: number) => createMessage(content, 'warning', duration),
  info: (content: string, duration?: number) => createMessage(content, 'info', duration),
}
