use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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

// 用于序列化到文件的结构体
#[derive(Serialize, Deserialize)]
struct AppData {
    tags: Vec<Tag>,
    agents: Vec<Agent>,
}

// 全局状态，用于在程序运行时保存在内存中
pub struct AppState {
    pub tags: std::sync::Mutex<Vec<Tag>>,
    pub agents: std::sync::Mutex<Vec<Agent>>,
    pub save_path: PathBuf, // 记录保存文件的路径
}

impl AppState {
    // 启动时读取本地文件，如果不存在则使用默认数据
    pub fn load_or_default(path: PathBuf) -> Self {
        // 尝试读取文件
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(data) = serde_json::from_str::<AppData>(&content) {
                return Self {
                    tags: std::sync::Mutex::new(data.tags),
                    agents: std::sync::Mutex::new(data.agents),
                    save_path: path,
                };
            }
        }

        // 如果文件不存在或解析失败，使用你设置的默认数据
        let tags = vec![
            Tag {
                id: "t_anti_rush".into(),
                name: "抗点".into(),
                is_key: true,
            },
            Tag {
                id: "t_damage".into(),
                name: "伤害".into(),
                is_key: false,
            },
            Tag {
                id: "t_dash".into(),
                name: "进点".into(),
                is_key: true,
            },
            Tag {
                id: "t_short_smoke".into(),
                name: "短烟".into(),
                is_key: false,
            },
            Tag {
                id: "t_info".into(),
                name: "信息".into(),
                is_key: true,
            },
            Tag {
                id: "t_long_smoke".into(),
                name: "长烟".into(),
                is_key: true,
            },
            Tag {
                id: "t_tp".into(),
                name: "位移".into(),
                is_key: false,
            },
        ];

        // 2. 预设角色
        let agents = vec![
            Agent {
                id: "a_jett".into(),
                name: "捷风 (Jett)".into(),
                avatar_url: "/avatars/Jett_icon.png".into(),
                tags: vec!["t_dash".into(), "t_short_smoke".into()],
            },
            Agent {
                id: "a_omen".into(),
                name: "幽影 (Omen)".into(),
                avatar_url: "/avatars/Omen_icon.png".into(),
                tags: vec!["t_anti_rush".into(), "t_long_smoke".into(), "t_tp".into()],
            },
            Agent {
                id: "a_sova".into(),
                name: "猎枭 (Sova)".into(),
                avatar_url: "/avatars/Sova_icon.png".into(),
                tags: vec!["t_info".into(), "t_damage".into()],
            },
        ];

        let state = Self {
            tags: std::sync::Mutex::new(tags),
            agents: std::sync::Mutex::new(agents),
            save_path: path,
        };

        // 第一次生成默认数据后，顺手保存一份到本地
        state.save();
        state
    }

    // 将当前内存中的数据写入本地文件
    pub fn save(&self) {
        let data = AppData {
            tags: self.tags.lock().unwrap().clone(),
            agents: self.agents.lock().unwrap().clone(),
        };
        // 格式化为 JSON 字符串并写入
        if let Ok(json) = serde_json::to_string_pretty(&data) {
            let _ = fs::write(&self.save_path, json);
        }
    }
}
