use std::env;
use std::path::PathBuf;
use std::time::Instant;
use once_cell::sync::OnceCell;

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
#[derive(Debug)]
pub struct User {
    address: String,
    name: String,
    login_time: Instant,
}

impl User {
    pub fn get_or_new(address: String, name: String) -> &'static Self {
        CURRENT_USER.get_or_init(|| {
            User {
                address,
                name,
                login_time: Instant::now(),
            }
        })
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn login_time(&self) -> &Instant {
        &self.login_time
    }
}
