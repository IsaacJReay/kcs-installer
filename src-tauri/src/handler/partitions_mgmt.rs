use super::{manage_status, Command};

pub fn print_all_parts(selected_disk: &str) -> Vec<String> {
    block_utils::get_block_partitions()
        .unwrap()
        .into_iter()
        .filter(|each| each.to_str().unwrap().contains(selected_disk))
        .map(|each| each.to_str().unwrap().to_owned())
        .collect()
}

pub fn print_all_disks() -> Vec<String> {
    block_utils::get_block_devices()
        .unwrap()
        .into_iter()
        .map(|each| each.to_str().unwrap().to_owned())
        .collect()
}

pub async fn parted_partitioning(
    selected_disk: &str,
    format_label: &str,
    findram: &str,
    boot_label: &str,
    selected_content_disk: &str,
) {
    let mut part_label = Command::new("parted")
        .arg(selected_disk)
        .arg("mklabel")
        .arg(format_label)
        .arg("--script")
        .spawn()
        .expect("failed to execute process");

    manage_status("Partitioning Drives", 500, &mut part_label, 2, false).await;

    Command::new("parted")
        .arg(selected_content_disk)
        .arg("mklabel")
        .arg(format_label)
        .arg("--script")
        .output()
        .expect("failed to execute process");

    Command::new("parted")
        .arg(selected_content_disk)
        .arg("mkpart")
        .arg("primary")
        .arg("ext4")
        .arg("0%")
        .arg("100%")
        .arg("--script")
        .spawn()
        .expect("failed to execute process");

    let mut part_boot = Command::new("parted")
        .arg(selected_disk)
        .arg("mkpart")
        .arg("primary")
        .arg(boot_label)
        .arg("0%")
        .arg("100M")
        .arg("--script")
        .spawn()
        .expect("failed to execute process");

    manage_status("Partitioning Drives", 500, &mut part_boot, 4, false).await;

    let mut part_swap = Command::new("parted")
        .arg(selected_disk)
        .arg("mkpart")
        .arg("primary")
        .arg("linux-swap")
        .arg("538M")
        .arg(findram)
        .arg("--script")
        .spawn()
        .expect("failed to execute process");

    manage_status("Partitioning Drives", 500, &mut part_swap, 6, false).await;

    let mut part_root = Command::new("parted")
        .arg(selected_disk)
        .arg("mkpart")
        .arg("primary")
        .arg("ext4")
        .arg(findram)
        .arg("100%")
        .arg("--script")
        .spawn()
        .expect("failed to execute process");

    manage_status("Partitioning Drives", 500, &mut part_root, 8, true).await;
}

pub async fn mkfs_formating(
    selected_boot: &str,
    selected_swap: &str,
    selected_root: &str,
    format_label: &str,
    selected_content_disk: &str,
) {
    let mut make_boot = if format_label == "msdos" {
        Command::new("mkfs.ext4")
            .arg("-F")
            .arg(selected_boot)
            .spawn()
            .expect("failed to execute process")
    } else {
        Command::new("mkfs.vfat")
            .arg("-F32")
            .arg(selected_boot)
            .spawn()
            .expect("failed to execute process")
    };

    manage_status("Formating Partitions", 500, &mut make_boot, 10, false).await;

    Command::new("mkfs.ext4")
        .arg("-F")
        .arg(format!("{}1", selected_content_disk))
        .output()
        .expect("failed to execute process");

    let mut make_swap = Command::new("mkswap")
        .arg(selected_swap)
        .spawn()
        .expect("failed to execute process");

    manage_status("Formating Partitions", 500, &mut make_swap, 12, false).await;

    let mut make_root = Command::new("mkfs.ext4")
        .arg("-F")
        .arg(selected_root)
        .spawn()
        .expect("failed to execute process");

    manage_status("Formating Partitions", 500, &mut make_root, 14, true).await;
}

pub fn mount_boot_swap_contentdisk(
    system: &str,
    selected_swap: &str,
    selected_content_disk: &str,
    selected_disk: Option<&str>,
    selected_boot: &str,
) {
    if system == "UEFI" {
        Command::new("parted")
            .arg(selected_disk.unwrap())
            .arg("set")
            .arg("1")
            .arg("esp")
            .arg("on")
            .output()
            .unwrap();

        std::fs::create_dir("/mnt/boot/efi").unwrap_or(());

        Command::new("mount")
            .arg(selected_boot)
            .arg("/mnt/boot/efi")
            .output()
            .unwrap();
    } else {
        Command::new("mount")
            .arg(selected_boot)
            .arg("/mnt/boot")
            .output()
            .unwrap();
    }

    std::fs::create_dir("/mnt/kmp").unwrap_or(());

    Command::new("mount")
        .arg(format!("{}1", selected_content_disk))
        .arg("/mnt/kmp")
        .output()
        .unwrap();

    Command::new("swapon").arg(selected_swap).output().unwrap();
}

pub fn prepare_boot() {
    let paths = std::fs::read_dir("/root/packages").unwrap();

    let file_list = paths
        .into_iter()
        .map(|each| each.as_ref().unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<String>>();

    Command::new("pacstrap")
        .arg("-U")
        .arg("/mnt")
        .args(&file_list)
        .output()
        .unwrap();
}
