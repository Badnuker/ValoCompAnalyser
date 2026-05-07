mod models;

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use flate2::{Compression, read::DeflateDecoder, write::DeflateEncoder};
use std::io::{Read, Write};

// ---- 精简版序列化结构（用于 packed 导出，减少字段冗余） ----
#[derive(serde::Serialize, serde::Deserialize)]
struct CompactTag {
    #[serde(rename = "i")]
    id: String,
    #[serde(rename = "k")]
    is_key: bool,
    #[serde(rename = "n", skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    name: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
struct CompactAgent {
    #[serde(rename = "i")]
    id: String,
    #[serde(rename = "t", skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    tags: Vec<String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
struct CompactData {
    #[serde(rename = "g")]
    tags: Vec<CompactTag>,
    #[serde(rename = "a")]
    agents: Vec<CompactAgent>,
}
use models::{Agent, AppData, AppState, Tag};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{LogicalSize, Manager, State};

#[tauri::command]
fn get_tags(state: State<AppState>) -> Vec<Tag> {
    state.tags.lock().unwrap().clone()
}

#[tauri::command]
fn get_agents(state: State<AppState>) -> Vec<Agent> {
    state.agents.lock().unwrap().clone()
}

#[tauri::command]
fn toggle_tag_key(id: String, state: State<AppState>) -> Result<(), String> {
    {
        let mut tags = state.tags.lock().unwrap();
        if let Some(tag) = tags.iter_mut().find(|t| t.id == id) {
            tag.is_key = !tag.is_key;
        } else {
            return Err("未找到该标签".into());
        }
    }
    state.save();
    Ok(())
}

#[tauri::command]
fn add_tag(name: String, is_key: bool, state: State<AppState>) -> Result<Tag, String> {
    let new_tag = {
        let mut tags = state.tags.lock().unwrap();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let id = format!("t_custom_{}", timestamp);
        let tag = Tag { id, name, is_key };
        tags.push(tag.clone());
        tag
    };
    state.save();
    Ok(new_tag)
}

#[tauri::command]
fn delete_tag(id: String, state: State<AppState>) -> Result<(), String> {
    {
        let mut tags = state.tags.lock().unwrap();
        tags.retain(|t| t.id != id);
    }
    {
        let mut agents = state.agents.lock().unwrap();
        for agent in agents.iter_mut() {
            agent.tags.retain(|tag_id| tag_id != &id);
        }
    }
    state.save();
    Ok(())
}

#[tauri::command]
fn update_agent_tags(
    agent_id: String,
    new_tags: Vec<String>,
    state: State<AppState>,
) -> Result<(), String> {
    {
        let mut agents = state.agents.lock().unwrap();
        if let Some(agent) = agents.iter_mut().find(|a| a.id == agent_id) {
            agent.tags = new_tags;
        } else {
            return Err("未找到该角色".into());
        }
    }
    state.save();
    Ok(())
}

fn get_download_dir() -> std::path::PathBuf {
    let candidates: [Option<std::path::PathBuf>; 3] = [
        std::env::var("EXTERNAL_STORAGE")
            .ok()
            .map(|s| std::path::PathBuf::from(s).join("Download")),
        Some(std::path::PathBuf::from("/storage/emulated/0/Download")),
        Some(std::path::PathBuf::from("/sdcard/Download")),
    ];
    candidates
        .into_iter()
        .flatten()
        .find(|p| p.exists())
        .unwrap_or_else(|| std::path::PathBuf::from("/storage/emulated/0/Download"))
}

#[tauri::command]
fn get_data_json(state: State<AppState>) -> Result<String, String> {
    let data = AppData {
        tags: state.tags.lock().unwrap().clone(),
        agents: state.agents.lock().unwrap().clone(),
    };
    serde_json::to_string(&data).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_data_packed(state: State<AppState>) -> Result<String, String> {
    let tags = state.tags.lock().unwrap().clone();
    let agents = state.agents.lock().unwrap().clone();
    let compact = CompactData {
        tags: tags.into_iter().map(|t| CompactTag {
            id: t.id.clone(),
            is_key: t.is_key,
            name: if t.id.starts_with("t_custom_") { Some(t.name) } else { None },
        }).collect(),
        agents: agents.into_iter().map(|a| CompactAgent {
            id: a.id,
            tags: a.tags,
        }).collect(),
    };
    let packed = rmp_serde::to_vec(&compact).map_err(|e| e.to_string())?;
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&packed).map_err(|e| e.to_string())?;
    let compressed = encoder.finish().map_err(|e| e.to_string())?;
    Ok(BASE64.encode(&compressed))
}

#[tauri::command]
fn import_data_string(json_str: String, state: State<AppState>) -> Result<(), String> {
    let mut parsed: AppData =
        serde_json::from_str(&json_str).map_err(|e| format!("格式错误: {}", e))?;
    let default_data = AppState::get_default_data();
    for def_tag in default_data.tags {
        if !parsed.tags.iter().any(|t| t.id == def_tag.id) {
            parsed.tags.push(def_tag);
        }
    }
    for def_agent in default_data.agents {
        if !parsed.agents.iter().any(|a| a.id == def_agent.id) {
            parsed.agents.push(def_agent);
        }
    }
    *state.tags.lock().unwrap() = parsed.tags;
    *state.agents.lock().unwrap() = parsed.agents;
    state.save();
    Ok(())
}

#[tauri::command]
fn import_data_packed(encoded: String, state: State<AppState>) -> Result<(), String> {
    let bytes = BASE64.decode(encoded.trim()).map_err(|e| format!("解码失败: {}", e))?;
    let mut decoder = DeflateDecoder::new(&bytes[..]);
    let mut packed = Vec::new();
    decoder.read_to_end(&mut packed).map_err(|e| format!("解压失败: {}", e))?;
    let compact: CompactData = rmp_serde::from_slice(&packed).map_err(|e| format!("格式错误: {}", e))?;

    let default_data = AppState::get_default_data();

    let mut parsed_tags: Vec<Tag> = compact.tags.into_iter().map(|ct| {
        let name = ct.name.unwrap_or_else(|| {
            default_data.tags.iter().find(|dt| dt.id == ct.id).map(|dt| dt.name.clone()).unwrap_or_default()
        });
        Tag { id: ct.id, name, is_key: ct.is_key }
    }).collect();
    for def_tag in default_data.tags {
        if !parsed_tags.iter().any(|t| t.id == def_tag.id) { parsed_tags.push(def_tag); }
    }

    let mut parsed_agents: Vec<Agent> = compact.agents.into_iter().map(|ca| {
        let name = default_data.agents.iter().find(|da| da.id == ca.id).map(|da| da.name.clone()).unwrap_or_default();
        let avatar_url = default_data.agents.iter().find(|da| da.id == ca.id).map(|da| da.avatar_url.clone()).unwrap_or_default();
        Agent { id: ca.id, name, avatar_url, tags: ca.tags }
    }).collect();
    for def_agent in default_data.agents {
        if !parsed_agents.iter().any(|a| a.id == def_agent.id) { parsed_agents.push(def_agent); }
    }

    *state.tags.lock().unwrap() = parsed_tags;
    *state.agents.lock().unwrap() = parsed_agents;
    state.save();
    Ok(())
}

#[tauri::command]
async fn export_data(state: State<'_, AppState>) -> Result<String, String> {
    let download_dir = get_download_dir();
    let _ = std::fs::create_dir_all(&download_dir);
    let export_path = download_dir.join("valo_comp_export.json");
    let data = AppData {
        tags: state.tags.lock().unwrap().clone(),
        agents: state.agents.lock().unwrap().clone(),
    };
    let json_str = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
    std::fs::write(&export_path, json_str).map_err(|e| format!("写入文件失败: {}", e))?;
    Ok(export_path.to_string_lossy().to_string())
}

#[tauri::command]
async fn import_data(file_path: String, state: State<'_, AppState>) -> Result<(), String> {
    let path = std::path::PathBuf::from(&file_path);
    let json_str = if path.exists() {
        std::fs::read_to_string(&path)
    } else {
        std::fs::read_to_string(get_download_dir().join("valo_comp_export.json"))
    }
    .map_err(|e| format!("读取文件失败: {}", e))?;
    let mut parsed: AppData =
        serde_json::from_str(&json_str).map_err(|e| format!("文件格式不正确: {}", e))?;
    let default_data = AppState::get_default_data();
    for def_tag in default_data.tags {
        if !parsed.tags.iter().any(|t| t.id == def_tag.id) {
            parsed.tags.push(def_tag);
        }
    }
    for def_agent in default_data.agents {
        if !parsed.agents.iter().any(|a| a.id == def_agent.id) {
            parsed.agents.push(def_agent);
        }
    }
    *state.tags.lock().unwrap() = parsed.tags;
    *state.agents.lock().unwrap() = parsed.agents;
    state.save();
    Ok(())
}

#[tauri::command]
fn reset_data(state: State<AppState>) -> Result<(), String> {
    let default_data = AppState::get_default_data();
    *state.tags.lock().unwrap() = default_data.tags;
    *state.agents.lock().unwrap() = default_data.agents;
    state.save();
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let _ = window.set_title("无畏契约阵容分析器");
            if let Ok(Some(monitor)) = window.primary_monitor() {
                let size = monitor.size();
                let scale_factor = monitor.scale_factor();
                let half_width = (size.width as f64 / scale_factor / 2.0) as f64;
                let half_height = (size.height as f64 / scale_factor / 2.0) as f64;
                let _ = window.set_size(LogicalSize::new(half_width, half_height));
            }
            let app_data_dir = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| std::env::current_dir().unwrap());
            let _ = fs::create_dir_all(&app_data_dir);
            let data_path = app_data_dir.join("valo_data.json");
            app.manage(AppState::load_or_default(data_path));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_tags,
            get_agents,
            toggle_tag_key,
            add_tag,
            delete_tag,
            update_agent_tags,
            export_data,
            import_data,
            reset_data,
            get_data_json,
            import_data_string,
            get_data_packed,
            import_data_packed
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}




