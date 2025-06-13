use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
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

fn default_is_markdown() -> bool {
    true
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JiyiRequest {
    #[schemars(description = "操作类型：记忆(添加记忆), 回忆(获取项目信息)")]
    pub action: String,
    #[schemars(description = "项目路径（必需）")]
    pub project_path: String,
    #[schemars(description = "记忆内容（记忆操作时必需）")]
    #[serde(default)]
    pub content: String,
    #[schemars(description = "记忆分类：rule(规范规则), preference(用户偏好), pattern(最佳实践), context(项目上下文)")]
    #[serde(default = "default_category")]
    pub category: String,
}

fn default_category() -> String {
    "context".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PopupRequest {
    pub id: String,
    pub message: String,
    pub predefined_options: Option<Vec<String>>,
    pub is_markdown: bool,
}

#[derive(Debug, Deserialize)]
pub struct McpResponseContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
    pub source: Option<ImageSource>,
}

#[derive(Debug, Deserialize)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub media_type: String,
    pub data: String,
}
