// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
use models::{Agent, AppState, Tag};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{LogicalSize, Manager, State};

// --- 读取 API ---

#[tauri::command]
fn get_tags(state: State<AppState>) -> Vec<Tag> {
    state.tags.lock().unwrap().clone()
}

#[tauri::command]
fn get_agents(state: State<AppState>) -> Vec<Agent> {
    state.agents.lock().unwrap().clone()
}

// --- 写入 API ---

// 切换标签的 Key 属性
#[tauri::command]
fn toggle_tag_key(id: String, state: State<AppState>) -> Result<(), String> {
    let mut tags = state.tags.lock().unwrap(); // 获取写锁
    if let Some(tag) = tags.iter_mut().find(|t| t.id == id) {
        tag.is_key = !tag.is_key; // 取反
        Ok(())
    } else {
        Err("未找到该标签".into())
    }
}

// 添加自定义标签
#[tauri::command]
fn add_tag(name: String, is_key: bool, state: State<AppState>) -> Result<Tag, String> {
    let mut tags = state.tags.lock().unwrap();

    // 生成一个唯一 ID：用当前时间的时间戳
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let id = format!("t_custom_{}", timestamp);

    let new_tag = Tag { id, name, is_key };

    tags.push(new_tag.clone()); // 存入后端内存
    Ok(new_tag) // 把新创建的标签返回给前端
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            if let Ok(Some(monitor)) = window.primary_monitor() {
                let size = monitor.size();
                let scale_factor = monitor.scale_factor();
                let half_width = (size.width as f64 / scale_factor / 2.0) as f64;
                let half_height = (size.height as f64 / scale_factor / 2.0) as f64;
                let _ = window.set_size(LogicalSize::new(half_width, half_height));
                let _ = window.center();
            }
            Ok(())
        })
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            get_tags,
            get_agents,
            toggle_tag_key,
            add_tag
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
