// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
use models::{Agent, AppState, Tag};
use tauri::{LogicalSize, Manager, State};

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
        .setup(|app| {
            // 获取主窗口
            let window = app.get_webview_window("main").unwrap();

            // 获取主显示器
            if let Ok(Some(monitor)) = window.primary_monitor() {
                let size = monitor.size();
                // Tauri 默认返回的是物理像素 (PhysicalSize)，
                // 我们需要考虑系统的缩放比例 (scale_factor) 转为逻辑像素 (LogicalSize)
                // 但为了简单，我们可以直接用物理像素除以2，然后设为窗口大小

                let scale_factor = monitor.scale_factor();

                // 计算宽度和高度的一半
                let half_width = (size.width as f64 / scale_factor / 2.0) as f64;
                let half_height = (size.height as f64 / scale_factor / 2.0) as f64;

                // 设置窗口大小
                let _ = window.set_size(LogicalSize::new(half_width, half_height));

                // 居中显示窗口
                let _ = window.center();
            }

            Ok(())
        })
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![get_tags, get_agents])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
