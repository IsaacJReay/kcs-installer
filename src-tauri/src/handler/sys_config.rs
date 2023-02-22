use super::{
    download_data, get_value_mutex_safe, id_system, manage_status, partitions_mgmt, post_install,
    sys_info, Command,
};

#[tauri::command]
pub async fn start_installation() {
    tokio::spawn(async move {
        disable_sleep(true);
        let findram = sys_info::findram(None);
        let system = id_system::id_system();
        let selected_disk: String = get_value_mutex_safe("SELECTED_DISK");
        let selected_content_disk: String = get_value_mutex_safe("SELECTED_CONTENT_DISK");

        let (format_label, boot_label) = match system == "UEFI" {
            true => ("gpt", "fat32"),
            false => ("msdos", "ext4"),
        };

        partitions_mgmt::parted_partitioning(
            &selected_disk,
            format_label,
            &findram,
            boot_label,
            &selected_content_disk,
        )
        .await;

        let all_parts = partitions_mgmt::print_all_parts(&selected_disk);

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

        install_system().await;

        self::partitions_mgmt::mount_boot_swap_contentdisk(
            &system,
            &selected_swap,
            &selected_content_disk,
            Some(&selected_disk),
            &selected_boot,
        );

        self::partitions_mgmt::prepare_boot();
        self::post_install::prepare_source(&system, &selected_disk);
        self::post_install::post_installation().await;
        self::download_data::download_data().await;
    });
}

async fn install_system() {
    let mut install_system_process = Command::new("cp")
        .arg("-avx")
        .arg("/run/archiso/airootfs/.")
        .arg("/mnt/")
        .spawn()
        .unwrap();

    manage_status(
        "Installing System",
        2500,
        &mut install_system_process,
        45,
        true,
    )
    .await;
}

pub fn disable_sleep(state: bool) {
    Command::new("systemctl")
        .arg(match state {
            true => "mask",
            false => "unmask"
        })
        .args(&[
            "sleep.target",
            "suspend.target",
            "suspend-then-hibernate.target",
            "hibernate.target",
            "hybrid-sleep.target",
        ])
        .output()
        .unwrap();
}