use cunzhi::config::{AppState, load_config_and_apply_window_settings};
use anyhow::Result;
use tauri::Manager;

// 重新导出所有命令函数
pub use cunzhi::ui::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            get_always_on_top,
            set_always_on_top,
            sync_window_state,
            get_audio_notification_enabled,
            set_audio_notification_enabled,
            get_audio_url,
            set_audio_url,
            play_notification_sound,
            test_audio_sound,
            get_theme,
            set_theme,
            get_window_config,
            set_window_config,
            get_reply_config,
            set_reply_config,
            apply_window_constraints,
            send_mcp_response,
            get_cli_args,
            read_mcp_request,
            exit_app
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            // 在setup中直接调用，不使用spawn
            tauri::async_runtime::block_on(async {
                let state = app.state::<AppState>();
                if let Err(e) = load_config_and_apply_window_settings(&state, &app_handle).await {
                    eprintln!("加载配置失败: {}", e);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() -> Result<()> {
    // 检查程序是如何被调用的
    let args: Vec<String> = std::env::args().collect();

    // 如果有 --mcp-request 参数，启动 GUI 模式处理MCP请求
    if args.len() >= 3 && args[1] == "--mcp-request" {
        // 这里可以添加MCP请求处理逻辑
        // 目前直接启动GUI
        run();
    } else {
        // 正常启动 GUI 应用
        run();
    }

    Ok(())
}
