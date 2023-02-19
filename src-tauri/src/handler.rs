mod id_system;
mod partitions_mgmt;
mod post_install;
mod download_data;
pub mod sys_info;
pub mod sys_config;

use super::{db, get_value_mutex_safe, set_value_mutex_safe};
use std::process::{Child, Command, Stdio};

#[derive(Clone, serde::Serialize)]
pub struct InstallStatus {
    progress: u8,
    status_list: Vec<InstallStatusListItem>,
}

impl InstallStatus {
    pub fn new(progress: u8, status_list: Vec<InstallStatusListItem>) -> Self {
        Self {
            progress,
            status_list,
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct InstallStatusListItem {
    id: u8,
    name: String,
    status: String,
}

impl InstallStatusListItem {
    pub fn new(id: u8, name: String, status: String) -> Self {
        Self { id, name, status }
    }
    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }
    pub fn get_id(&self) -> u8 {
        self.id
    }
}

#[derive(Clone, serde::Serialize)]
pub struct DisksInfo {
    name: String,
    info: String,
}

impl DisksInfo {
    pub fn new(name: String, info: String) -> Self {
        Self { name, info }
    }
}

#[derive(Clone, serde::Deserialize)]
pub struct DiskandIPArgs {
    selected_disk: String,
    selected_content_disk: String,
}

impl DiskandIPArgs {
    pub fn get_selected_disk(&self) -> String {
        self.selected_disk.to_owned()
    }
    pub fn get_selected_content_disk(&self) -> String {
        self.selected_content_disk.to_owned()
    }
}

#[tauri::command]
pub fn reboot() {
    Command::new("reboot").output().unwrap();
}

#[tauri::command]
pub fn set_disk_and_ip(args: DiskandIPArgs) {
    set_value_mutex_safe("SELECTED_DISK", args.get_selected_disk());
    set_value_mutex_safe("SELECTED_CONTENT_DISK", args.get_selected_content_disk());
}

#[tauri::command]
pub fn get_install_status() -> InstallStatus {
    db::query_status()
}

async fn manage_status(
    var: &str,
    sleep_millisec: u64,
    process: &mut Child,
    progress_limit: u8,
    change_state: bool,
) {
    db::update_tbl_status(var, "working");
    loop {
        match process.try_wait() {
            Ok(t) => match t {
                Some(_) => {
                    if change_state {
                        db::update_tbl_status(var, "done");
                    }
                    db::update_tbl_status("Progress", progress_limit.to_string().as_str());
                    break;
                }
                None => {
                    tokio::time::sleep(tokio::time::Duration::from_millis(sleep_millisec)).await;
                    db::increment_progress(progress_limit);
                    continue;
                }
            },
            Err(_) => panic!("error running process"),
        }
    }
}
