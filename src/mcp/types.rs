use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ZhiRequest {
    #[schemars(description = "要显示给用户的消息")]
    pub message: String,
    #[schemars(description = "预定义的选项列表（可选）")]
    #[serde(default)]
    pub predefined_options: Vec<String>,
    #[schemars(description = "消息是否为Markdown格式，默认为true")]
    #[serde(default = "default_is_markdown")]
    pub is_markdown: bool,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct JiyiRequest {
    #[schemars(description = "操作类型：记忆(添加记忆), 回忆(获取项目信息)")]
    pub action: String,
    #[schemars(description = "项目路径（必需）")]
    pub project_path: String,
    #[schemars(description = "记忆分类：rule(规范规则), preference(用户偏好), pattern(最佳实践), context(项目上下文)")]
    #[serde(default = "default_category")]
    pub category: String,
    #[schemars(description = "记忆内容（记忆操作时必需）")]
    #[serde(default)]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpPopupRequest {
    pub id: String,
    pub message: String,
    pub predefined_options: Option<Vec<String>>,
    pub is_markdown: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpPopupResponse {
    pub id: String,
    pub selected_options: Vec<String>,
    pub text_input: String,
    pub image_data: Option<String>,
}

pub fn default_is_markdown() -> bool {
    true
}

pub fn default_category() -> String {
    "context".to_string()
}
