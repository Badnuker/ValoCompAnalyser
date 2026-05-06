// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
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

#[tauri::command]
async fn export_data(state: State<'_, AppState>) -> Result<(), String> {
    // 打开系统的“另存为”弹窗
    if let Some(path) = rfd::FileDialog::new()
        .set_file_name("valo_comp_config.json")
        .add_filter("JSON 文件", &["json"])
        .save_file()
    {
        // 获取当前数据
        let data = AppData {
            tags: state.tags.lock().unwrap().clone(),
            agents: state.agents.lock().unwrap().clone(),
        };
        // 转为 JSON 并写入用户指定的路径
        let json_str = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
        std::fs::write(path, json_str).map_err(|e| format!("写入文件失败: {}", e))?;
        Ok(())
    } else {
        Err("用户取消了导出".into())
    }
}

// 导入配置：从用户选择的 JSON 文件加载数据，并与当前内置默认数据合并，覆盖内存中的数据
#[tauri::command]
async fn import_data(state: State<'_, AppState>) -> Result<(), String> {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("JSON 文件", &["json"])
        .pick_file()
    {
        let json_str = std::fs::read_to_string(path).map_err(|e| format!("读取文件失败: {}", e))?;
        let mut parsed: AppData =
            serde_json::from_str(&json_str).map_err(|e| format!("文件格式不正确: {}", e))?;

        // 获取当前程序的最新默认数据作为模板
        let default_data = AppState::get_default_data();

        // 合并缺失的标签
        for def_tag in default_data.tags {
            if !parsed.tags.iter().any(|t| t.id == def_tag.id) {
                parsed.tags.push(def_tag);
            }
        }

        // 合并缺失的角色
        for def_agent in default_data.agents {
            if !parsed.agents.iter().any(|a| a.id == def_agent.id) {
                parsed.agents.push(def_agent);
            }
        }

        // 覆盖当前内存数据
        *state.tags.lock().unwrap() = parsed.tags;
        *state.agents.lock().unwrap() = parsed.agents;

        state.save(); // 保存合并后的最终结果
        Ok(())
    } else {
        Err("用户取消了导入".into())
    }
}

// 重置配置：恢复为代码中写死的默认列表
#[tauri::command]
fn reset_data(state: State<AppState>) -> Result<(), String> {
    let default_data = AppState::get_default_data();

    *state.tags.lock().unwrap() = default_data.tags;
    *state.agents.lock().unwrap() = default_data.agents;

    state.save();
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let _ = window.set_title("ValoCompAnalyser");
            if let Ok(Some(monitor)) = window.primary_monitor() {
                let size = monitor.size();
                let scale_factor = monitor.scale_factor();
                let half_width = (size.width as f64 / scale_factor / 2.0) as f64;
                let half_height = (size.height as f64 / scale_factor / 2.0) as f64;
                let _ = window.set_size(LogicalSize::new(half_width, half_height));
                let _ = window.center();
            }

            // 获取系统的 AppData 目录，将 JSON 文件存在这里
            let app_data_dir = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| std::env::current_dir().unwrap());
            let _ = fs::create_dir_all(&app_data_dir);
            let data_path = app_data_dir.join("valo_data.json");

            // 使用 load_or_default 传入路径
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
            reset_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
