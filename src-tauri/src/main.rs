// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
use models::{Agent, AppState, Tag};
use tauri::State;

#[tauri::command]
fn get_tags(state: State<AppState>) -> Vec<Tag> {
    state.tags.lock().unwrap().clone()
}

#[tauri::command]
fn get_agents(state: State<AppState>) -> Vec<Agent> {
    state.agents.lock().unwrap().clone()
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::new()) // 注册全局状态 (预设数据)
        .invoke_handler(tauri::generate_handler![get_tags, get_agents]) // 注册提供给前端的指令
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
