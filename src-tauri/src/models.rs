use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub is_key: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub avatar_url: String,
    pub tags: Vec<String>,
}

// 全局状态，用于在程序运行时保存在内存中
pub struct AppState {
    pub tags: std::sync::Mutex<Vec<Tag>>,
    pub agents: std::sync::Mutex<Vec<Agent>>,
}
