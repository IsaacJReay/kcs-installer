use super::{get_value_mutex_safe, manage_status, Command};
use std::{
    fs::{read_to_string, File, OpenOptions},
    io::Write,
};

pub fn prepare_source(system: &str, selected_disk: &str) {
    let mut file = File::create("/mnt/system").unwrap();
    file.write_all(system.as_bytes())
        .expect("Error while writing to file");

    let mut file = File::create("/mnt/selected_disk").unwrap();
    file.write_all(selected_disk.as_bytes())
        .expect("Error while writing to file");

    let contents = read_to_string("/root/installerpart2.sh").unwrap();
    let new = contents
        .replace(
            "DEFAULT_HOSTNAME",
            &get_value_mutex_safe("DEFAULT_HOSTNAME"),
        )
        .replace(
            "DEFAULT_USERNAME",
            &get_value_mutex_safe("DEFAULT_USERNAME"),
        )
        .replace(
            "DEFAULT_PASSWORD",
            &get_value_mutex_safe("DEFAULT_PASSWORD"),
        );

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("/root/installerpart2.sh")
        .unwrap();
    file.write(new.as_bytes()).unwrap();
}

pub async fn post_installation() {
    Command::new("cp")
        .arg("/root/installerpart2.sh")
        .arg("/mnt")
        .output()
        .unwrap();

    let mut setting_up_profile = Command::new("tar")
        .arg("-xzpf")
        .arg("/root/kcs-conf-amd64.tar.gz")
        .arg("-C")
        .arg("/mnt")
        .spawn()
        .unwrap();

    manage_status(
        "Setting Up User Profiles",
        1500,
        &mut setting_up_profile,
        47,
        true,
    )
    .await;

    let mut post_install_process = Command::new("arch-chroot")
        .arg("/mnt")
        .arg("/installerpart2.sh")
        .spawn()
        .unwrap();

    manage_status(
        "Performing Post-Installation",
        1000,
        &mut post_install_process,
        49,
        true,
    )
    .await;

    // Cleanup install trash
    Command::new("rm")
        .arg("-f")
        .arg("/mnt/selected_disk")
        .arg("/mnt/system")
        .arg("/mnt/installerpart2.sh")
        .arg("/mnt/etc/systemd/network/20-ethernet.network")
        .arg("/mnt/etc/systemd/network/20-wlan.network")
        .arg("/mnt/etc/systemd/network/20-wwan.network")
        .output()
        .unwrap();
}
