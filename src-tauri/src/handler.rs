mod id_system;
mod partitions_mgmt;
mod post_install;
mod sys_info;

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
    master_ip: String,
}

impl DiskandIPArgs {
    pub fn get_selected_disk(&self) -> String {
        self.selected_disk.to_owned()
    }
    pub fn get_selected_content_disk(&self) -> String {
        self.selected_content_disk.to_owned()
    }
    pub fn get_master_ip(&self) -> String {
        self.master_ip.to_owned()
    }
}

#[tauri::command]
pub async fn start_installation() {
    tokio::spawn(async move {
        let findram = self::sys_info::findram(None);
        let system = self::id_system::id_system();
        // let all_disks = self::partitions_mgmt::print_all_disks();
        let selected_disk: String = get_value_mutex_safe("SELECTED_DISK");
        let selected_content_disk: String = get_value_mutex_safe("SELECTED_CONTENT_DISK");

        println!(
            "ram: {}\nsystem: {}\ncontent_disk: {}\nselected_disk: {}",
            findram, system, selected_content_disk, selected_disk
        );

        let (format_label, boot_label) = if system == "UEFI" {
            ("gpt", "fat32")
        } else {
            ("msdos", "ext4")
        };

        self::partitions_mgmt::parted_partitioning(
            &selected_disk,
            format_label,
            &findram,
            boot_label,
            &selected_content_disk,
        )
        .await;

        let all_parts = self::partitions_mgmt::print_all_parts(&selected_disk);

        let selected_boot = &all_parts[0];
        let selected_swap = &all_parts[1];
        let selected_root = &all_parts[2];

        self::partitions_mgmt::mkfs_formating(
            selected_boot,
            selected_swap,
            selected_root,
            format_label,
            &selected_content_disk,
        )
        .await;

        Command::new("mount")
            .arg(selected_root)
            .arg("/mnt")
            .output()
            .unwrap();

        let mut install_system_process = Command::new("cp")
            .arg("-ax")
            .arg("/run/archiso/airootfs/.")
            .arg("/mnt/")
            .spawn()
            .unwrap();

        manage_status(
            "Installing System",
            2500,
            &mut install_system_process,
            "35",
            89,
            true,
        )
        .await;

        self::partitions_mgmt::mount_boot_swap_contentdisk(
            &system,
            &selected_swap,
            &selected_content_disk,
            Some(&selected_disk),
            &selected_boot,
        );

        self::post_install::prepare_source(&system, &selected_disk);

        self::post_install::post_installation().await;

        db::update_tbl_status("Progress", "100");
    });
}

#[tauri::command]
pub fn reboot() {
    Command::new("systemctl").arg("reboot").output().unwrap();
}

#[tauri::command]
pub fn get_disks() -> Vec<DisksInfo> {
    let all_disks = self::partitions_mgmt::print_all_disks();

    all_disks
        .iter()
        .map(|each| {
            let name = each.to_owned();
            let info = format!("{} {}", &name, &self::sys_info::get_disk_info(&each));
            DisksInfo::new(name, info)
        })
        .collect::<Vec<DisksInfo>>()
}

#[tauri::command]
pub fn set_disk_and_ip(args: DiskandIPArgs) {
    set_value_mutex_safe("SELECTED_DISK", args.get_selected_disk());
    set_value_mutex_safe("SELECTED_CONTENT_DISK", args.get_selected_content_disk());
    set_value_mutex_safe("MASTER_IP", args.get_master_ip());

    println!(
        "{}\n{}\n{}",
        get_value_mutex_safe("SELECTED_DISK"),
        get_value_mutex_safe("SELECTED_CONTENT_DISK"),
        get_value_mutex_safe("MASTER_IP")
    )
}

// #[tauri::command]
// pub fn set_disk_and_ip(selected_disk: String, selected_content_disk: String, master_ip: String) {
//     set_value_mutex_safe("SELECTED_DISK", selected_disk);
//     set_value_mutex_safe("SELECTED_CONTENT_DISK", selected_content_disk);
//     set_value_mutex_safe("MASTER_IP", master_ip);

//     println!(
//         "{}\n{}\n{}",
//         get_value_mutex_safe("SELECTED_DISK"),
//         get_value_mutex_safe("SELECTED_CONTENT_DISK"),
//         get_value_mutex_safe("MASTER_IP")
//     )
// }

#[tauri::command]
pub fn get_install_status() -> InstallStatus {
    db::query_status()
}

async fn manage_status(
    var: &str,
    sleep_millisec: u64,
    process: &mut Child,
    progress_jump: &str,
    progress_limit: u8,
    change_state: bool,
) {
    db::update_tbl_status(var, "working");
    db::update_tbl_status("Progress", progress_jump);
    loop {
        match process.try_wait() {
            Ok(t) => match t {
                Some(_) => {
                    if change_state {
                        db::update_tbl_status(var, "done");
                    }
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
