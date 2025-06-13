use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, State, Manager};

use super::settings::{AppConfig, AppState};

pub fn get_config_path(app: &AppHandle) -> Result<PathBuf> {
    let app_dir = app.path()
        .app_config_dir()
        .map_err(|e| anyhow::anyhow!("无法获取应用配置目录: {}", e))?;

    // 确保目录存在
    fs::create_dir_all(&app_dir)?;

    Ok(app_dir.join("config.json"))
}

pub async fn save_config(state: &State<'_, AppState>, app: &AppHandle) -> Result<()> {
    let config_path = get_config_path(app)?;
    
    // 确保目录存在
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let config = state.config.lock().map_err(|e| anyhow::anyhow!("获取配置失败: {}", e))?;
    let config_json = serde_json::to_string_pretty(&*config)?;
    
    fs::write(config_path, config_json)?;
    
    Ok(())
}

pub async fn load_config(state: &State<'_, AppState>, app: &AppHandle) -> Result<()> {
    let config_path = get_config_path(app)?;

    if config_path.exists() {
        let config_json = fs::read_to_string(config_path)?;
        let config: AppConfig = serde_json::from_str(&config_json)?;

        let mut config_guard = state.config.lock()
            .map_err(|e| anyhow::anyhow!("获取配置锁失败: {}", e))?;
        *config_guard = config;
    }

    Ok(())
}

pub async fn load_config_and_apply_window_settings(state: &State<'_, AppState>, app: &AppHandle) -> Result<()> {
    // 先加载配置
    load_config(state, app).await?;

    // 然后应用窗口设置
    let always_on_top = {
        let config = state.config.lock()
            .map_err(|e| anyhow::anyhow!("获取配置失败: {}", e))?;
        config.always_on_top
    };

    // 应用到窗口
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_always_on_top(always_on_top);
    }

    Ok(())
}
