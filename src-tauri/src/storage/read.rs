use std::{collections::HashMap};
use std::fs;
use crate::{common::{self, User}, consts};
use log::error;

pub fn read_user_data() -> Result<HashMap<String, User>, &'static str> {
    let Some(user_file_path) = common::get_full_path(consts::USERS_FILE) else {
        return Err("用户数据文件路径获取失败");
    };

    let content = fs::read_to_string(user_file_path)
        .map_err(|e| {
            error!("读取用户数据文件失败: {}", e);
            "读取用户数据文件失败"
        })?;

    
    Ok(HashMap::new())
}