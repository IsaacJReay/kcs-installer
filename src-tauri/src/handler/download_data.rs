use super::{get_value_mutex_safe, manage_status, Command};

pub async fn download_data() {
    let mut rsync_child_process = Command::new("sshpass")
        .arg("-p")
        .arg(get_value_mutex_safe("MASTER_PASSWORD"))
        .arg("rsync")
        .arg("-e")
        .arg("ssh -o 'StrictHostKeyChecking no'")
        .arg("-avP")
        .arg(format!(
            "{}@{}:/kmp/",
            get_value_mutex_safe("MASTER_USERNAME"),
            get_value_mutex_safe("MASTER_MULTICAST_ID")
        ))
        .arg("/kmp")
        .spawn()
        .unwrap();

    manage_status(
        "Retrieving Data From Master",
        10000,
        &mut rsync_child_process,
        100,
        true,
    )
    .await;
}
