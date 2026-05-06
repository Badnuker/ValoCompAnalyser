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

impl AppState {
    pub fn new() -> Self {
        // 1. 预设标签
        let tags = vec![
            Tag {
                id: "t_anti_rush".into(),
                name: "防 rush".into(),
                is_key: true,
            },
            Tag {
                id: "t_damage".into(),
                name: "伤害".into(),
                is_key: false,
            },
            Tag {
                id: "t_intel".into(),
                name: "信息".into(),
                is_key: true,
            },
            Tag {
                id: "t_mobility".into(),
                name: "位移".into(),
                is_key: true,
            },
            Tag {
                id: "t_smoke".into(),
                name: "烟雾".into(),
                is_key: true,
            },
        ];

        // 2. 预设角色
        let agents = vec![
            Agent {
                id: "a_jett".into(),
                name: "捷风 (Jett)".into(),
                avatar_url: "/avatars/jett.png".into(),
                tags: vec!["t_mobility".into(), "t_smoke".into()],
            },
            Agent {
                id: "a_omen".into(),
                name: "幽影 (Omen)".into(),
                avatar_url: "/avatars/omen.png".into(),
                tags: vec!["t_anti_rush".into(), "t_mobility".into(), "t_smoke".into()],
            },
            Agent {
                id: "a_sova".into(),
                name: "猎枭 (Sova)".into(),
                avatar_url: "/avatars/sova.png".into(),
                tags: vec!["t_damage".into(), "t_intel".into()],
            },
        ];

        Self {
            tags: std::sync::Mutex::new(tags),
            agents: std::sync::Mutex::new(agents),
        }
    }
}
