mod storage;
mod common;
mod login;
mod consts;
use log::{debug};
use env_logger::{Builder};
use storage::init;
use std::result::Result;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), String> {
    // 进行一些初始化操作
    init()?;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(list_export_methods())
        .run(tauri::generate_context!())
        .expect("程序启动错误");

    Ok(())
}

fn list_export_methods() -> impl Fn(tauri::ipc::Invoke) -> bool {
    tauri::generate_handler![
        login::login, 
        login::get_old_username,
        login::is_login
    ]
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
