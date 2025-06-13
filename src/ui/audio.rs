use anyhow::Result;
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use tauri::{AppHandle, State, Manager};
use rodio::{Decoder, OutputStream, Sink};

use crate::config::{AppState, save_config};

#[tauri::command]
pub async fn get_audio_notification_enabled(state: State<'_, AppState>) -> Result<bool, String> {
    let config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
    Ok(config.audio_notification_enabled)
}

#[tauri::command]
pub async fn set_audio_notification_enabled(enabled: bool, state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    // 如果是首次启用音频通知，先复制音频文件
    if enabled {
        if let Err(e) = ensure_audio_file_exists(&app).await {
            return Err(format!("准备音频文件失败: {}", e));
        }
    }

    {
        let mut config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        config.audio_notification_enabled = enabled;
    }

    // 保存配置到文件
    save_config(&state, &app).await.map_err(|e| format!("保存配置失败: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn get_audio_url(state: State<'_, AppState>) -> Result<String, String> {
    let config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
    Ok(config.audio_url.clone())
}

#[tauri::command]
pub async fn set_audio_url(url: String, state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    {
        let mut config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        config.audio_url = url;
    }

    // 保存配置到文件
    save_config(&state, &app).await.map_err(|e| format!("保存配置失败: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn play_notification_sound(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    // 检查是否启用音频通知
    let (enabled, audio_url) = {
        let config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        (config.audio_notification_enabled, config.audio_url.clone())
    };

    if !enabled {
        return Ok(());
    }

    // 异步播放音频，避免阻塞主线程
    tokio::spawn(async move {
        if let Err(e) = play_audio_file(&app, &audio_url).await {
            eprintln!("播放音频失败: {}", e);
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn test_audio_sound(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    // 获取当前配置的音效URL
    let audio_url = {
        let config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        config.audio_url.clone()
    };

    // 异步播放音频，避免阻塞主线程
    tokio::spawn(async move {
        if let Err(e) = play_audio_file(&app, &audio_url).await {
            eprintln!("测试音频播放失败: {}", e);
        }
    });

    Ok(())
}

pub async fn play_audio_file(app: &AppHandle, audio_url: &str) -> Result<()> {
    if !audio_url.is_empty() {
        // 使用自定义URL播放音频
        if audio_url.starts_with("http://") || audio_url.starts_with("https://") {
            // 网络音频文件
            return play_audio_from_url(audio_url).await;
        } else {
            // 本地文件路径
            let audio_path = std::path::PathBuf::from(audio_url);
            if audio_path.exists() {
                let audio_path_clone = audio_path.clone();
                std::thread::spawn(move || {
                    if let Err(e) = play_audio_sync(&audio_path_clone) {
                        eprintln!("音频播放失败: {}", e);
                    }
                });
                return Ok(());
            } else {
                return Err(anyhow::anyhow!("自定义音频文件不存在: {:?}", audio_path));
            }
        }
    }

    // 使用默认音频文件
    let audio_path = get_audio_file_path(app)?;

    if !audio_path.exists() {
        return Err(anyhow::anyhow!("音频文件不存在: {:?}", audio_path));
    }

    // 在新线程中播放音频，避免阻塞
    let audio_path_clone = audio_path.clone();
    std::thread::spawn(move || {
        if let Err(e) = play_audio_sync(&audio_path_clone) {
            eprintln!("音频播放失败: {}", e);
        }
    });

    Ok(())
}

async fn play_audio_from_url(url: &str) -> Result<()> {
    // 下载音频文件到临时目录
    let response = reqwest::get(url).await
        .map_err(|e| anyhow::anyhow!("下载音频文件失败: {}", e))?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("下载音频文件失败，状态码: {}", response.status()));
    }

    let bytes = response.bytes().await
        .map_err(|e| anyhow::anyhow!("读取音频数据失败: {}", e))?;

    // 将bytes转换为Vec<u8>以便移动到线程中
    let bytes_vec = bytes.to_vec();

    // 在新线程中播放音频
    std::thread::spawn(move || {
        if let Err(e) = play_audio_from_bytes(bytes_vec) {
            eprintln!("播放网络音频失败: {}", e);
        }
    });

    Ok(())
}

fn play_audio_from_bytes(bytes: Vec<u8>) -> Result<()> {
    // 创建音频输出流
    let (_stream, stream_handle) = OutputStream::try_default()
        .map_err(|e| anyhow::anyhow!("创建音频输出流失败: {}", e))?;

    // 创建音频播放器
    let sink = Sink::try_new(&stream_handle)
        .map_err(|e| anyhow::anyhow!("创建音频播放器失败: {}", e))?;

    // 从字节数据解码音频
    let cursor = std::io::Cursor::new(bytes);
    let source = Decoder::new(cursor)
        .map_err(|e| anyhow::anyhow!("解码音频数据失败: {}", e))?;

    // 播放音频
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}

fn play_audio_sync(audio_path: &PathBuf) -> Result<()> {
    // 创建音频输出流
    let (_stream, stream_handle) = OutputStream::try_default()
        .map_err(|e| anyhow::anyhow!("创建音频输出流失败: {}", e))?;

    // 创建音频播放器
    let sink = Sink::try_new(&stream_handle)
        .map_err(|e| anyhow::anyhow!("创建音频播放器失败: {}", e))?;

    // 读取音频文件
    let file = std::fs::File::open(audio_path)
        .map_err(|e| anyhow::anyhow!("打开音频文件失败: {}", e))?;
    let buf_reader = BufReader::new(file);

    // 解码音频
    let source = Decoder::new(buf_reader)
        .map_err(|e| anyhow::anyhow!("解码音频文件失败: {}", e))?;

    // 播放音频
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}

fn get_audio_file_path(app: &AppHandle) -> Result<PathBuf> {
    // 获取应用配置目录中的音频文件路径
    let config_dir = app.path().app_config_dir()
        .map_err(|e| anyhow::anyhow!("无法获取应用配置目录: {}", e))?;

    let sounds_dir = config_dir.join("sounds");
    let audio_path = sounds_dir.join("notification.mp3");

    if audio_path.exists() {
        Ok(audio_path)
    } else {
        Err(anyhow::anyhow!("音频文件不存在，请先在设置中启用音频通知"))
    }
}

/// 确保音频文件存在，如果不存在则从Tauri资源目录复制
pub async fn ensure_audio_file_exists(app: &AppHandle) -> Result<()> {
    let config_dir = app.path().app_config_dir()
        .map_err(|e| anyhow::anyhow!("无法获取应用配置目录: {}", e))?;

    let sounds_dir = config_dir.join("sounds");
    let target_audio_path = sounds_dir.join("notification.mp3");

    // 如果音频文件已存在，直接返回
    if target_audio_path.exists() {
        return Ok(());
    }

    // 创建sounds目录
    fs::create_dir_all(&sounds_dir)
        .map_err(|e| anyhow::anyhow!("创建sounds目录失败: {}", e))?;

    // 从Tauri资源目录复制音频文件
    let resource_dir = app.path().resource_dir()
        .map_err(|e| anyhow::anyhow!("无法获取Tauri资源目录: {}", e))?;

    let source_audio_path = resource_dir.join("sounds/notification.mp3");

    if source_audio_path.exists() {
        fs::copy(&source_audio_path, &target_audio_path)
            .map_err(|e| anyhow::anyhow!("复制音频文件失败: {}", e))?;

        println!("✅ 音频文件已复制到: {:?}", target_audio_path);
        Ok(())
    } else {
        Err(anyhow::anyhow!("无法找到音频源文件: {:?}", source_audio_path))
    }
}
