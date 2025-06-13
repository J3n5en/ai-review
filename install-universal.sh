#!/bin/bash

# 寸止 通用安装脚本 - 支持 macOS、Linux

set -e

echo "🚀 开始安装 寸止..."

# 检测操作系统
OS="unknown"
case "$OSTYPE" in
    darwin*)  OS="macos" ;;
    linux*)   OS="linux" ;;
    msys*|cygwin*|mingw*) OS="windows" ;;
    *)        echo "❌ 不支持的操作系统: $OSTYPE"; exit 1 ;;
esac

echo "🔍 检测到操作系统: $OS"

# 检查必要的工具
check_command() {
    if ! command -v "$1" &> /dev/null; then
        echo "❌ 错误: 未找到 $1 命令"
        echo "请先安装 $1"
        exit 1
    fi
}

echo "🔧 检查必要工具..."
check_command "cargo"
check_command "pnpm"

# 构建前端
echo "📦 构建前端资源..."
pnpm build

# 根据操作系统选择构建方式
if [[ "$OS" == "macos" ]]; then
    echo "🍎 macOS 构建模式..."
    
    # 构建 Tauri 应用包
    echo "🔨 构建 Tauri 应用包..."
    cargo tauri build
    
    # 检查构建结果
    APP_BUNDLE="target/release/bundle/macos/寸止.app"
    if [[ ! -d "$APP_BUNDLE" ]]; then
        echo "❌ 应用包构建失败: $APP_BUNDLE"
        exit 1
    fi
    
    echo "✅ 应用包构建成功: $APP_BUNDLE"
    
    # 安装应用到 Applications 目录
    echo "📋 安装应用到 Applications 目录..."
    
    # 移除旧版本（如果存在）
    if [[ -d "/Applications/寸止.app" ]]; then
        echo "🗑️  移除旧版本..."
        sudo rm -rf "/Applications/寸止.app"
    fi

    # 复制新版本
    sudo cp -R "$APP_BUNDLE" "/Applications/"
    echo "✅ 应用已安装到 /Applications/寸止.app"
    
    # 运行 postinstall 脚本
    echo "⚙️  配置命令行工具..."
    if [[ -f "scripts/postinstall.sh" ]]; then
        bash scripts/postinstall.sh
    else
        echo "❌ 未找到 postinstall.sh 脚本"
        exit 1
    fi
    
elif [[ "$OS" == "linux" ]]; then
    echo "🐧 Linux 构建模式..."
    
    # 构建二进制文件
    echo "🔨 构建二进制文件..."
    cargo build --release
    
    # 检查构建结果
    if [[ ! -f "target/release/cunzhi" ]]; then
        echo "❌ 二进制文件构建失败"
        exit 1
    fi
    
    echo "✅ 二进制文件构建成功"
    
    # 创建用户本地目录
    LOCAL_DIR="$HOME/.local"
    BIN_DIR="$LOCAL_DIR/bin"
    APP_DIR="$LOCAL_DIR/share/applications"
    ICON_DIR="$LOCAL_DIR/share/icons/hicolor/128x128/apps"
    
    mkdir -p "$BIN_DIR" "$APP_DIR" "$ICON_DIR"
    
    # 复制二进制文件
    cp "target/release/cunzhi" "$BIN_DIR/cunzhi"
    chmod +x "$BIN_DIR/cunzhi"

    # 创建软链接
    ln -sf "$BIN_DIR/cunzhi" "$BIN_DIR/等一下"
    ln -sf "$BIN_DIR/cunzhi" "$BIN_DIR/寸止"
    
    echo "✅ 命令行工具已安装到 $BIN_DIR"
    
    # 复制图标（如果存在）
    if [[ -f "icons/icon-128.png" ]]; then
        cp "icons/icon-128.png" "$ICON_DIR/cunzhi.png"
    fi

    # 创建桌面文件
    cat > "$APP_DIR/cunzhi.desktop" << EOF
[Desktop Entry]
Name=寸止
Name[zh_CN]=寸止
Comment=告别AI提前终止烦恼，助力AI更加持久
Comment[zh_CN]=告别AI提前终止烦恼，助力AI更加持久
Exec=$BIN_DIR/cunzhi
Icon=cunzhi
Terminal=false
Type=Application
Categories=Development;
StartupNotify=true
EOF
    
    echo "✅ 桌面应用已创建"
    
    # 检查PATH
    if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
        echo ""
        echo "💡 请将以下内容添加到您的 shell 配置文件中 (~/.bashrc 或 ~/.zshrc):"
        echo "export PATH=\"\$PATH:$BIN_DIR\""
        echo ""
        echo "然后运行: source ~/.bashrc (或 source ~/.zshrc)"
    fi
    
else
    echo "❌ Windows 平台请使用 Windows 专用安装程序"
    exit 1
fi

echo ""
echo "🎉 寸止 安装完成！"
echo ""
echo "📋 使用方法："
if [[ "$OS" == "macos" ]]; then
    echo "  🖥️  GUI模式: 在 Applications 中打开 '寸止'"
fi
echo "  💻 命令行模式:"
echo "    等一下                          - 启动 UI 界面"
echo "    等一下 --mcp-request file       - MCP 弹窗模式"
echo "    寸止                            - 启动 MCP 服务器"
echo ""
echo "📝 配置 MCP 客户端："
echo "将以下内容添加到您的 MCP 客户端配置中："
echo ""
cat << 'EOF'
{
  "mcpServers": {
    "寸止": {
      "command": "寸止"
    }
  }
}
EOF
echo ""

if [[ "$OS" == "macos" ]]; then
    echo "🔗 命令行工具已链接到 /usr/local/bin/"
    echo "📁 应用位置: /Applications/寸止.app"
elif [[ "$OS" == "linux" ]]; then
    echo "🔗 命令行工具已安装到 $BIN_DIR"
    echo "📁 桌面应用: $APP_DIR/cunzhi.desktop"
fi
