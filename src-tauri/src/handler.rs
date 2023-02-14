// use super::db;

#[tauri::command]
pub fn get_install_status() -> InstallStatus {
    InstallStatus::new(
        10,
        vec![
            InstallStatusListItem::new(String::from("Partitioning Data"), String::from("done")),
            InstallStatusListItem::new(String::from("Installing System"), String::from("working")),
            InstallStatusListItem::new(String::from("Finalising Jobs"), String::from("pending")),
        ],
    )
}

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
    name: String,
    status: String,
}

impl InstallStatusListItem {
    pub fn new(name: String, status: String) -> Self {
        Self { name, status }
    }
}
