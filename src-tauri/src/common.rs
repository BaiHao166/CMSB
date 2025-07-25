use std::{env, time::SystemTime};
use std::path::PathBuf;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};


pub fn get_full_path(file_name: &str) -> Option<PathBuf> {
    let Some(install_dir) = get_install_dir() else {
        return None;
    };

    let full_path = install_dir.join(file_name);
    Some(full_path)
}

pub fn get_install_dir() -> Option<PathBuf> {
    if let Ok(path) = env::current_exe() {
        let dir = path.parent().map(|p| p.to_path_buf());
        dir
    } else {
        None
    }
}


static CURRENT_USER: OnceCell<User> = OnceCell::new();

// 记录登录用户信息
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    address: String,
    name: String,
    login_time: SystemTime,
}

impl User {
    pub fn get_or_new(address: String, name: String) -> &'static Self {
        CURRENT_USER.get_or_init(|| {
            User {
                address,
                name,
                login_time: SystemTime::now(),
            }
        })
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn login_time(&self) -> &SystemTime {
        &self.login_time
    }
}
