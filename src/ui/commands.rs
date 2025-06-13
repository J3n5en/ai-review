use tauri::{AppHandle, State, Manager};
use crate::config::{AppState, WindowConfig, ReplyConfig, save_config};

#[tauri::command]
pub async fn get_app_info() -> Result<String, String> {
    Ok("寸止 v0.1.0".to_string())
}

#[tauri::command]
pub async fn get_always_on_top(state: State<'_, AppState>) -> Result<bool, String> {
    let config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
    Ok(config.always_on_top)
}

#[tauri::command]
pub async fn set_always_on_top(enabled: bool, state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    {
        let mut config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        config.always_on_top = enabled;
    }

    // 保存配置到文件
    save_config(&state, &app).await.map_err(|e| format!("保存配置失败: {}", e))?;

    // 应用到当前窗口
    if let Some(window) = app.get_webview_window("main") {
        window.set_always_on_top(enabled).map_err(|e| format!("设置窗口置顶失败: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn sync_window_state(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    // 根据配置同步窗口状态
    let always_on_top = {
        let config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        config.always_on_top
    };

    // 应用到当前窗口
    if let Some(window) = app.get_webview_window("main") {
        window.set_always_on_top(always_on_top).map_err(|e| format!("同步窗口状态失败: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_theme(state: State<'_, AppState>) -> Result<String, String> {
    let config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
    Ok(config.theme.clone())
}

#[tauri::command]
pub async fn set_theme(theme: String, state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    // 验证主题值
    if !["light", "dark", "system"].contains(&theme.as_str()) {
        return Err("无效的主题值，只支持 light、dark、system".to_string());
    }

    {
        let mut config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        config.theme = theme;
    }

    // 保存配置到文件
    save_config(&state, &app).await.map_err(|e| format!("保存配置失败: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_window_config(state: State<'_, AppState>) -> Result<WindowConfig, String> {
    let config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
    Ok(config.window_config.clone())
}

#[tauri::command]
pub async fn set_window_config(window_config: WindowConfig, state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    {
        let mut config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        config.window_config = window_config;
    }

    // 保存配置到文件
    save_config(&state, &app).await.map_err(|e| format!("保存配置失败: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_reply_config(state: State<'_, AppState>) -> Result<ReplyConfig, String> {
    let config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
    Ok(config.reply_config.clone())
}

#[tauri::command]
pub async fn set_reply_config(reply_config: ReplyConfig, state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    {
        let mut config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        config.reply_config = reply_config;
    }

    // 保存配置到文件
    save_config(&state, &app).await.map_err(|e| format!("保存配置失败: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn send_mcp_response(response: serde_json::Value, state: State<'_, AppState>) -> Result<(), String> {
    // 将响应序列化为JSON字符串
    let response_str = serde_json::to_string(&response)
        .map_err(|e| format!("序列化响应失败: {}", e))?;

    if response_str.trim().is_empty() {
        return Err("响应内容不能为空".to_string());
    }

    // 检查是否为MCP模式
    let args: Vec<String> = std::env::args().collect();
    let is_mcp_mode = args.len() >= 3 && args[1] == "--mcp-request";

    if is_mcp_mode {
        // MCP模式：直接输出到stdout
        println!("{}", response_str);
        std::io::Write::flush(&mut std::io::stdout()).map_err(|e| format!("刷新stdout失败: {}", e))?;
    } else {
        // 通过channel发送响应（如果有的话）
        let sender = {
            let mut channel = state.response_channel.lock().map_err(|e| format!("获取响应通道失败: {}", e))?;
            channel.take()
        };

        if let Some(sender) = sender {
            let _ = sender.send(response_str);
        }
    }

    Ok(())
}

#[tauri::command]
pub fn get_cli_args() -> Result<serde_json::Value, String> {
    let args: Vec<String> = std::env::args().collect();
    let mut result = serde_json::Map::new();

    // 检查是否有 --mcp-request 参数
    if args.len() >= 3 && args[1] == "--mcp-request" {
        result.insert("mcp_request".to_string(), serde_json::Value::String(args[2].clone()));
    }

    Ok(serde_json::Value::Object(result))
}

#[tauri::command]
pub fn read_mcp_request(file_path: String) -> Result<serde_json::Value, String> {
    if !std::path::Path::new(&file_path).exists() {
        return Err(format!("文件不存在: {}", file_path));
    }

    match std::fs::read_to_string(&file_path) {
        Ok(content) => {
            if content.trim().is_empty() {
                return Err("文件内容为空".to_string());
            }
            match serde_json::from_str(&content) {
                Ok(json) => Ok(json),
                Err(e) => Err(format!("解析JSON失败: {}", e))
            }
        }
        Err(e) => Err(format!("读取文件失败: {}", e))
    }
}

#[tauri::command]
pub async fn exit_app(app: AppHandle) -> Result<(), String> {
    // 关闭所有窗口
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.close();
    }

    // 强制退出应用
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    app.exit(0);
    Ok(())
}
