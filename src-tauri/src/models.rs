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

#[derive(Serialize, Deserialize)]
pub struct AppData {
    pub tags: Vec<Tag>,
    pub agents: Vec<Agent>,
}

pub struct AppState {
    pub tags: std::sync::Mutex<Vec<Tag>>,
    pub agents: std::sync::Mutex<Vec<Agent>>,
    pub save_path: PathBuf,
}

impl AppState {
    pub fn get_default_data() -> AppData {
        let tags = vec![
            // 职业标签
            Tag {
                id: "t_duelist".into(),
                name: "决斗".into(),
                is_key: true,
            },
            Tag {
                id: "t_controller".into(),
                name: "控场".into(),
                is_key: true,
            },
            Tag {
                id: "t_initiator".into(),
                name: "先锋".into(),
                is_key: true,
            },
            Tag {
                id: "t_sentinel".into(),
                name: "哨卫".into(),
                is_key: true,
            },
            // 功能标签
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

        let agents = vec![
            // ================= 决斗 (Duelists) =================
            Agent {
                id: "a_iso".into(),
                name: "壹决 (Iso)".into(),
                avatar_url: "/avatars/Iso_icon.png".into(),
                tags: vec!["t_duelist".into()],
            },
            Agent {
                id: "a_jett".into(),
                name: "捷风 (Jett)".into(),
                avatar_url: "/avatars/Jett_icon.png".into(),
                tags: vec!["t_duelist".into(), "t_dash".into(), "t_short_smoke".into()],
            },
            Agent {
                id: "a_neon".into(),
                name: "霓虹 (Neon)".into(),
                avatar_url: "/avatars/Neon_icon.png".into(),
                tags: vec!["t_duelist".into(), "t_dash".into()],
            },
            Agent {
                id: "a_phoenix".into(),
                name: "不死鸟 (Phoenix)".into(),
                avatar_url: "/avatars/Phoenix_icon.png".into(),
                tags: vec!["t_duelist".into(), "t_short_smoke".into()],
            },
            Agent {
                id: "a_raze".into(),
                name: "雷兹 (Raze)".into(),
                avatar_url: "/avatars/Raze_icon.png".into(),
                tags: vec!["t_duelist".into(), "t_dash".into(), "t_damage".into()],
            },
            Agent {
                id: "a_reyna".into(),
                name: "芮娜 (Reyna)".into(),
                avatar_url: "/avatars/Reyna_icon.png".into(),
                tags: vec!["t_duelist".into(), "t_damage".into()],
            },
            Agent {
                id: "a_waylay".into(),
                name: "幻棱 (Waylay)".into(),
                avatar_url: "/avatars/Waylay_icon.png".into(),
                tags: vec!["t_duelist".into(), "t_dash".into()],
            },
            Agent {
                id: "a_yoru".into(),
                name: "夜露 (Yoru)".into(),
                avatar_url: "/avatars/Yoru_icon.png".into(),
                tags: vec!["t_duelist".into(), "t_tp".into()],
            },
            // ================= 控场 (Controllers) =================
            Agent {
                id: "a_astra".into(),
                name: "星礈 (Astra)".into(),
                avatar_url: "/avatars/Astra_icon.png".into(),
                tags: vec!["t_controller".into(), "t_long_smoke".into()],
            },
            Agent {
                id: "a_brimstone".into(),
                name: "炼狱 (Brimstone)".into(),
                avatar_url: "/avatars/Brimstone_icon.png".into(),
                tags: vec!["t_controller".into(), "t_long_smoke".into()],
            },
            Agent {
                id: "a_clove".into(),
                name: "暮蝶 (Clove)".into(),
                avatar_url: "/avatars/Clove_icon.png".into(),
                tags: vec!["t_controller".into(), "t_long_smoke".into()],
            },
            Agent {
                id: "a_harbor".into(),
                name: "海神 (Harbor)".into(),
                avatar_url: "/avatars/Harbor_icon.png".into(),
                tags: vec!["t_controller".into(), "t_long_smoke".into()],
            },
            Agent {
                id: "a_miks".into(),
                name: "迷核 (Miks)".into(),
                avatar_url: "/avatars/Miks_icon.png".into(),
                tags: vec!["t_controller".into(), "t_long_smoke".into()],
            },
            Agent {
                id: "a_omen".into(),
                name: "幽影 (Omen)".into(),
                avatar_url: "/avatars/Omen_icon.png".into(),
                tags: vec!["t_controller".into(), "t_long_smoke".into(), "t_tp".into()],
            },
            Agent {
                id: "a_viper".into(),
                name: "蝰蛇 (Viper)".into(),
                avatar_url: "/avatars/Viper_icon.png".into(),
                tags: vec![
                    "t_controller".into(),
                    "t_long_smoke".into(),
                    "t_anti_rush".into(),
                ],
            },
            // ================= 先锋 (Initiators) =================
            Agent {
                id: "a_breach".into(),
                name: "铁臂 (Breach)".into(),
                avatar_url: "/avatars/Breach_icon.png".into(),
                tags: vec!["t_initiator".into(), "t_anti_rush".into()],
            },
            Agent {
                id: "a_fade".into(),
                name: "黑梦 (Fade)".into(),
                avatar_url: "/avatars/Fade_icon.png".into(),
                tags: vec!["t_initiator".into(), "t_info".into()],
            },
            Agent {
                id: "a_gekko".into(),
                name: "盖可 (Gekko)".into(),
                avatar_url: "/avatars/Gekko_icon.png".into(),
                tags: vec!["t_initiator".into(), "t_info".into()],
            },
            Agent {
                id: "a_kayo".into(),
                name: "K/O (KAY/O)".into(),
                avatar_url: "/avatars/KAYO_icon.png".into(),
                tags: vec!["t_initiator".into(), "t_info".into(), "t_anti_rush".into()],
            },
            Agent {
                id: "a_skye".into(),
                name: "斯凯 (Skye)".into(),
                avatar_url: "/avatars/Skye_icon.png".into(),
                tags: vec!["t_initiator".into(), "t_info".into()],
            },
            Agent {
                id: "a_sova".into(),
                name: "猎枭 (Sova)".into(),
                avatar_url: "/avatars/Sova_icon.png".into(),
                tags: vec!["t_initiator".into(), "t_info".into(), "t_damage".into()],
            },
            Agent {
                id: "a_tejo".into(),
                name: "钛狐 (Tejo)".into(),
                avatar_url: "/avatars/Tejo_icon.png".into(),
                tags: vec!["t_initiator".into(), "t_info".into()],
            },
            // ================= 哨卫 (Sentinels) =================
            Agent {
                id: "a_chamber".into(),
                name: "尚勃勒 (Chamber)".into(),
                avatar_url: "/avatars/Chamber_icon.png".into(),
                tags: vec!["t_sentinel".into(), "t_tp".into()],
            },
            Agent {
                id: "a_cypher".into(),
                name: "零 (Cypher)".into(),
                avatar_url: "/avatars/Cypher_icon.png".into(),
                tags: vec!["t_sentinel".into(), "t_anti_rush".into(), "t_info".into()],
            },
            Agent {
                id: "a_deadlock".into(),
                name: "钢索 (Deadlock)".into(),
                avatar_url: "/avatars/Deadlock_icon.png".into(),
                tags: vec!["t_sentinel".into(), "t_anti_rush".into()],
            },
            Agent {
                id: "a_killjoy".into(),
                name: "奇乐 (Killjoy)".into(),
                avatar_url: "/avatars/Killjoy_icon.png".into(),
                tags: vec!["t_sentinel".into(), "t_anti_rush".into(), "t_info".into()],
            },
            Agent {
                id: "a_sage".into(),
                name: "贤者 (Sage)".into(),
                avatar_url: "/avatars/Sage_icon.png".into(),
                tags: vec!["t_sentinel".into(), "t_anti_rush".into()],
            },
            Agent {
                id: "a_veto".into(),
                name: "禁灭 (Veto)".into(),
                avatar_url: "/avatars/Veto_icon.png".into(),
                tags: vec!["t_sentinel".into(), "t_anti_rush".into()],
            },
            Agent {
                id: "a_vyse".into(),
                name: "维斯 (Vyse)".into(),
                avatar_url: "/avatars/Vyse_icon.png".into(),
                tags: vec!["t_sentinel".into(), "t_anti_rush".into()],
            },
        ];

        AppData { tags, agents }
    }

    pub fn load_or_default(path: PathBuf) -> Self {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(data) = serde_json::from_str::<AppData>(&content) {
                return Self {
                    tags: std::sync::Mutex::new(data.tags),
                    agents: std::sync::Mutex::new(data.agents),
                    save_path: path,
                };
            }
        }

        let default_data = Self::get_default_data();
        let state = Self {
            tags: std::sync::Mutex::new(default_data.tags),
            agents: std::sync::Mutex::new(default_data.agents),
            save_path: path,
        };

        state.save();
        state
    }

    pub fn save(&self) {
        let data = AppData {
            tags: self.tags.lock().unwrap().clone(),
            agents: self.agents.lock().unwrap().clone(),
        };
        if let Ok(json) = serde_json::to_string_pretty(&data) {
            let _ = fs::write(&self.save_path, json);
        }
    }
}
