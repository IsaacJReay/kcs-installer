use super::{Command, Stdio};

pub fn id_system() -> String {
    let output = Command::new("cat")
        .arg("/sys/class/dmi/id/sys_vendor")
        .output()
        .expect("failed to execute process");

    let output_cow = String::from_utf8_lossy(&output.stdout);
    let output_str = output_cow.as_ref();

    if (output_str == "Apple Inc.") || (output_str == "Apple Computer, Inc.") {
        Command::new("modprobe")
            .arg("-r")
            .arg("-q")
            .arg("efivars")
            .output()
            .expect("failed to execute process");
    } else {
        Command::new("modprobe")
            .arg("-q")
            .arg("efivars")
            .output()
            .expect("failed to execute process");
    }

    match std::path::Path::new("/sys/firmware/efi").exists() {
        true => {
            let mount_command_output = Command::new("mount")
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            let grep_command_output = Command::new("grep")
                .arg("/sys/firmware/efi/efivars")
                .stdin(mount_command_output.stdout.unwrap())
                .output()
                .unwrap();
            if grep_command_output.stdout.is_empty() {
                Command::new("mount")
                    .arg("-t")
                    .arg("efivarfs")
                    .arg("efivarfs")
                    .arg("/sys/firmware/efi/efivars")
                    .output()
                    .expect("failed to execute process");
            }
            String::from("UEFI")
        }
        false => String::from("BIOS"),
    }
}
