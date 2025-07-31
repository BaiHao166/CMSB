use crate::{
    common::{self, User},
    consts,
};
use log::{error};
use serde_json;
use std::collections::HashMap;
use std::fs;

pub fn read_user_data() -> Result<HashMap<String, User>, &'static str> {
    let Some(user_file_path) = common::get_full_path(consts::USERS_FILE) else {
        return Err("用户数据文件路径获取失败");
    };

    let content = fs::read_to_string(user_file_path).map_err(|e| {
        error!("读取用户数据文件失败: {}", e);
        "读取用户数据文件失败"
    })?;

    // 文件中没有内容（一般出现在用户刚安装且没有使用过），是正常情况，返回空的哈希表，
    if content.is_empty() {
        return Ok(HashMap::new());
    }

    // 将文件内容解析为Hashmap<String, User>
    let user_map: HashMap<String, User> = serde_json::from_str(&content).map_err(|e| {
        error!("反序列化用户map错误: {}", e);
        "反序列化用户map错误"
    })?;

    Ok(user_map)
}
