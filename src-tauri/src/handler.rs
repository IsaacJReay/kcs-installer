use super::{db, get_value_mutex_safe};

#[tauri::command]
pub async fn start_installation() {
    tokio::spawn(async move {
        loop {
            println!("Got here");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });
    
}

#[tauri::command]
pub fn reboot() {

}

#[tauri::command]
pub fn get_install_status() -> InstallStatus {
    let _a = get_value_mutex_safe("DEFAULT_USERNAME");
    println!("continued");
    db::query_status()
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
    id: u8,
    name: String,
    status: String,
}

impl InstallStatusListItem {
    pub fn new(id:u8, name: String, status: String) -> Self {
        Self { id, name, status }
    }
    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }
    pub fn get_id(&self) -> u8 {
        self.id
    }
}
