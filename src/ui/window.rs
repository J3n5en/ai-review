use tauri::{State, Manager};
use crate::config::AppState;

#[tauri::command]
pub async fn apply_window_constraints(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    let window_config = {
        let config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        config.window_config.clone()
    };

    if let Some(window) = app.get_webview_window("main") {
        // 设置窗口约束
        if let Err(e) = window.set_min_size(Some(tauri::LogicalSize::new(
            window_config.min_width,
            window_config.min_height,
        ))) {
            return Err(format!("设置最小窗口大小失败: {}", e));
        }

        if let Err(e) = window.set_max_size(Some(tauri::LogicalSize::new(
            window_config.max_width,
            window_config.max_height,
        ))) {
            return Err(format!("设置最大窗口大小失败: {}", e));
        }

        // 如果启用了自动调整大小，设置为合适的初始大小
        if window_config.auto_resize {
            let initial_width = window_config.min_width;
            let initial_height = (window_config.min_height + window_config.max_height) / 2.0;
            
            if let Err(e) = window.set_size(tauri::LogicalSize::new(initial_width, initial_height)) {
                return Err(format!("设置窗口大小失败: {}", e));
            }
        }
    }

    Ok(())
}
