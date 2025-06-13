use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub always_on_top: bool,
    #[serde(default = "default_audio_notification_enabled")]
    pub audio_notification_enabled: bool,
    #[serde(default = "default_theme")]
    pub theme: String, // "light", "dark", "system"
    #[serde(default = "default_window_config")]
    pub window_config: WindowConfig,
    #[serde(default = "default_audio_url")]
    pub audio_url: String, // 自定义音效URL
    #[serde(default = "default_reply_config")]
    pub reply_config: ReplyConfig, // 新增：继续回复配置
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowConfig {
    #[serde(default = "default_auto_resize")]
    pub auto_resize: bool,
    #[serde(default = "default_max_width")]
    pub max_width: f64,
    #[serde(default = "default_max_height")]
    pub max_height: f64,
    #[serde(default = "default_min_width")]
    pub min_width: f64,
    #[serde(default = "default_min_height")]
    pub min_height: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReplyConfig {
    #[serde(default = "default_enable_continue_reply")]
    pub enable_continue_reply: bool,
    #[serde(default = "default_auto_continue_threshold")]
    pub auto_continue_threshold: u32, // 字符数阈值
    #[serde(default = "default_continue_prompt")]
    pub continue_prompt: String, // 继续回复的提示词
}

#[derive(Debug)]
pub struct AppState {
    pub config: Mutex<AppConfig>,
    pub response_channel: Mutex<Option<tokio::sync::oneshot::Sender<String>>>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            always_on_top: true, // 默认启用置顶
            audio_notification_enabled: false, // 默认关闭音频通知
            theme: "dark".to_string(), // 默认深色主题
            window_config: default_window_config(),
            audio_url: default_audio_url(), // 默认音效URL
            reply_config: default_reply_config(), // 默认回复配置
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            config: Mutex::new(AppConfig::default()),
            response_channel: Mutex::new(None),
        }
    }
}

// 默认值函数
pub fn default_audio_notification_enabled() -> bool {
    false // 默认关闭音频通知
}

pub fn default_theme() -> String {
    "dark".to_string() // 默认深色主题
}

pub fn default_audio_url() -> String {
    "".to_string() // 默认为空，使用内置音效
}

pub fn default_window_config() -> WindowConfig {
    WindowConfig {
        auto_resize: true,
        max_width: 600.0,
        max_height: 1200.0,
        min_width: 600.0,
        min_height: 400.0,
    }
}

pub fn default_reply_config() -> ReplyConfig {
    ReplyConfig {
        enable_continue_reply: true,
        auto_continue_threshold: 1000,
        continue_prompt: "请继续".to_string(),
    }
}

pub fn default_auto_resize() -> bool {
    true
}

pub fn default_max_width() -> f64 {
    600.0
}

pub fn default_max_height() -> f64 {
    1200.0
}

pub fn default_min_width() -> f64 {
    600.0
}

pub fn default_min_height() -> f64 {
    400.0
}

pub fn default_enable_continue_reply() -> bool {
    true
}

pub fn default_auto_continue_threshold() -> u32 {
    1000
}

pub fn default_continue_prompt() -> String {
    "请继续".to_string()
}
