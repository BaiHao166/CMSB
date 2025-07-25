mod storage;
mod common;
mod login;
mod consts;
use log::{debug};
use env_logger::{Builder};
use storage::init;
use std::result::Result;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), String> {
    // 进行一些初始化操作
    init()?;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("程序启动错误");

    Ok(())
}

// 进行一些初始化配置
fn init() -> Result<(), String> {
    init_log_config();
    init::init_file()?;

    Ok(())
}

fn init_log_config() {
       Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();
    
    debug!("日志系统已初始化");
}
