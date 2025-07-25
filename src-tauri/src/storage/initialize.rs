use crate::common;
use std::fs::File;
use std::path::PathBuf;
use std::{result::Result, fs};
use log::{error};

const DB_CORE_DIR: &str = "db-core";
const USERS_FILE: &str = "db-core/users.json";

pub fn init_file() -> Result<(), String> {
    let Some(install_dir) = common::get_install_dir() else {
        error!("获取安装目录失败");
        return Err("获取安装目录失败".to_string());
    };
    create(&install_dir, DB_CORE_DIR, true)?;
    create(&install_dir, USERS_FILE, false)
}

fn create(install_dir: &PathBuf, file_name: &str, is_dir: bool) -> Result<(), String>  {
    let full_path = install_dir.join(file_name);

    if full_path.exists() {
        return Ok(());
    }

    if is_dir {
        return fs::create_dir(full_path)
                .map_err(|e| e.to_string());
    }

    File::create(full_path)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

