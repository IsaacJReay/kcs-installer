use super::{get_value_mutex_safe, manage_status, Command};

pub async fn copy_data() {
    let selected_data_disk = get_value_mutex_safe("SELECTED_DATA_DISK");
    if !selected_data_disk.is_empty() {
        Command::new("mount")
            .arg(selected_data_disk)
            .arg("/mnt/mnt")
            .output()
            .unwrap();

        let mut rsync_child_process = Command::new("rsync")
            .arg("rsync")
            .arg("-e")
            .arg("-avPW")
            .arg("/mnt/mnt/")
            .arg("/mnt/kmp")
            .spawn()
            .unwrap();

        Command::new("rsync")
            .arg("-avP")
            .arg("/mnt/mnt/services/")
            .arg("/mnt/etc/systemd/system/")
            .output()
            .unwrap();

        Command::new("rsync")
            .arg("-avP")
            .arg("/mnt/mnt/bind/conf/")
            .arg("/mnt/etc")
            .output()
            .unwrap();

        Command::new("rsync")
            .arg("-avP")
            .arg("/mnt/mnt/bind/zones/")
            .arg("/mnt/var/named")
            .output()
            .unwrap();

        Command::new("chown")
            .arg("root:named")
            .arg("/var/named/")
            .arg("-R")
            .output()
            .unwrap();  

        manage_status(
            "Copy Data From External",
            30000,
            &mut rsync_child_process,
            100,
            true,
        )
        .await;

    }
}
