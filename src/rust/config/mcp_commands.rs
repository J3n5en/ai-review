use std::collections::HashMap;
use tauri::{AppHandle, State};
// use serde::{Deserialize, Serialize};

use crate::config::{AppState, save_config};

/// MCP工具配置
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct MCPToolConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub can_disable: bool,
    pub icon: String,
    pub icon_bg: String,
    pub dark_icon_bg: String,
}

/// 获取MCP工具配置列表
#[tauri::command]
pub async fn get_mcp_tools_config(state: State<'_, AppState>) -> Result<Vec<MCPToolConfig>, String> {
    let config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
    
    let mut tools = vec![
        MCPToolConfig {
            id: "zhi".to_string(),
            name: "寸止".to_string(),
            description: "智能代码审查交互工具，支持预定义选项、自由文本输入和图片上传".to_string(),
            enabled: config.mcp_config.tools.get("zhi").copied().unwrap_or(true),
            can_disable: false, // 寸止工具是必需的
            icon: "i-carbon-chat".to_string(),
            icon_bg: "bg-blue-100 dark:bg-blue-900".to_string(),
            dark_icon_bg: "dark:bg-blue-800".to_string(),
        },
        MCPToolConfig {
            id: "ji".to_string(),
            name: "记忆管理".to_string(),
            description: "全局记忆管理工具，用于存储和管理重要的开发规范、用户偏好和最佳实践".to_string(),
            enabled: config.mcp_config.tools.get("ji").copied().unwrap_or(true),
            can_disable: true,
            icon: "i-carbon-data-base".to_string(),
            icon_bg: "bg-green-100 dark:bg-green-900".to_string(),
            dark_icon_bg: "dark:bg-green-800".to_string(),
        },
    ];
    
    // 按启用状态排序，启用的在前
    tools.sort_by(|a, b| b.enabled.cmp(&a.enabled));
    
    Ok(tools)
}

/// 设置MCP工具启用状态
#[tauri::command]
pub async fn set_mcp_tool_enabled(
    tool_id: String,
    enabled: bool,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    {
        let mut config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        
        // 检查工具是否可以禁用
        if tool_id == "zhi" && !enabled {
            return Err("寸止工具是必需的，无法禁用".to_string());
        }
        
        // 更新工具状态
        config.mcp_config.tools.insert(tool_id.clone(), enabled);
    }
    
    // 保存配置
    save_config(&state, &app).await
        .map_err(|e| format!("保存配置失败: {}", e))?;
    
    println!("✅ MCP工具 {} 状态已更新为: {}", tool_id, enabled);
    
    Ok(())
}

/// 获取所有MCP工具状态
#[tauri::command]
pub async fn get_mcp_tools_status(state: State<'_, AppState>) -> Result<HashMap<String, bool>, String> {
    let config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
    Ok(config.mcp_config.tools.clone())
}

/// 重置MCP工具配置为默认值
#[tauri::command]
pub async fn reset_mcp_tools_config(
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    {
        let mut config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        config.mcp_config.tools = crate::config::default_mcp_tools();
    }
    
    // 保存配置
    save_config(&state, &app).await
        .map_err(|e| format!("保存配置失败: {}", e))?;
    
    println!("✅ MCP工具配置已重置为默认值");
    Ok(())
}
