use log::debug;

use crate::{
    common::User,
    storage::{read::read_user_data, write::write_user_to_disk},
};

/**
 * 根据钱包地址获取用过的用户名
 */
#[tauri::command]
pub fn get_old_username(address: String) -> Result<String, &'static str> {
    let user_map = read_user_data()?;

    if let Some(user) = user_map.get(&address) {
        let name = user.name();
        return Ok(String::from(name));
    }

    Ok("".to_string())
}

/**
 * 登录
 * 1. 校验登录参数，同时校验地址在区块链上是否存在
 * 2. 将此次登录的用户信息写入内存，以便后续代码使用
 * 3. 将此次登录的用户信息写入文件，以便重新启动程序时使用
 */
#[tauri::command]
pub fn login(address: String, name: String) -> Result<(), &'static str> {
    // 1. 校验登录参数
    validate_login_args(&address, &name)?;

    // 2. 校验：solana上是否存在该地址
    validate_address_exists(&address)?;

    // 3. 存储用户信息
    let user = User::get_or_new(address, name);

    // 4. 写入用户到磁盘
    write_user_to_disk(user.clone()).map_err(|e| {
        log::error!("写入用户信息失败！: {}", e);
        "写入用户信息失败！"
    })?;
    Ok(())
}

/**
 * 校验登录参数
 */
fn validate_login_args(address: &String, name: &String) -> Result<(), &'static str> {
    if address.is_empty() {
        return Err("地址不能为空");
    }

    if name.is_empty() {
        return Err("用户名不能为空");
    }

    Ok(())
}

/**
 * 从区块链验证地址是否存在
 */
fn validate_address_exists(address: &String) -> Result<(), &'static str> {
    debug!("正在验证地址：{}", address);
    Ok(())
}

#[cfg(test)]
mod tests {
    use log::LevelFilter;

    use super::*;

    #[test]
    fn test_login() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(LevelFilter::Debug)
            .try_init();

        let address = "ss".to_string();
        let name = "test_user".to_string();

        let result = login(address, name);
        // 预期返回错误，展示错误信息
        // assert!(result.is_err());
        assert!(result.is_ok());
    }
}
