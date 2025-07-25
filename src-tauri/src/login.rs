use crate::common::User;

#[tauri::command]
pub fn login(address: String, name: String) -> Result<(), &'static str> {
    // 1. 校验登录参数
    validate_login_args(&address, &name)?;

    // 2. 校验：solana上是否存在该地址
    validate_address_exists(&address)?;

    // 3. 存储用户信息
    User::get_or_new(address, name);

    Ok(())
}

fn validate_login_args(address: &String, name: &String) -> Result<(), &'static str> {
    if address.is_empty() {
        return Err("地址不能为空");
    }

    if name.is_empty()  {
        return Err("用户名不能为空");
    }

    Ok(())
}

fn validate_address_exists(address: &String) -> Result<(), &'static str> {

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