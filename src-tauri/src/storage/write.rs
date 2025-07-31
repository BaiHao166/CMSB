use crate::common::{self, User};
use crate::consts;
use crate::storage::read::read_user_data;
use serde::{Deserialize, Serialize};
use std::fs::{File};
use std::io::{Write};
use std::path::PathBuf;

type IoResult<T> = std::io::Result<T>;

/**
 * 将用户信息写入硬盘
 * 1. 读取硬盘上的用户数据
 * 2. 将用户信息追加到原数据中，然后重新存到磁盘上
 */
pub fn write_user_to_disk(user: User) -> IoResult<()> {
    let mut user_map = read_user_data().expect("从硬盘读取用户数据失败！");
    
    let address: String = user.address().into();
    user_map.insert(address, user);

    let path = common::get_full_path(consts::USERS_FILE).unwrap();
    write_content_to_disk(path, user_map)?;

    Ok(())
}

/**
 * 将数据写入到磁盘
 */
pub fn write_content_to_disk<'a, T>(path: PathBuf, content: T) -> IoResult<()>
where
  T: Serialize + Deserialize<'a>
{
    let serialized_content = serde_json::to_string(&content)?;
    let mut file = File::create(path)?;
    file.write_all(serialized_content.as_bytes())?;
    file.sync_all()?;

    Ok(())
}
