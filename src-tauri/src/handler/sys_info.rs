use super::{partitions_mgmt, sys_info, Command, DisksInfo};
use byte_unit::{Byte, ByteUnit};
use std::str::FromStr;
use sysinfo::{System, SystemExt};

pub fn findram(conversion_unit: Option<ByteUnit>) -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    let byte_u128 = sys.total_memory() as u128;

    let byte_obj = Byte::from_bytes(byte_u128);
    let byte_obj_converted = byte_obj.get_adjusted_unit(match conversion_unit {
        Some(t) => t,
        None => ByteUnit::MB,
    });
    let byte_obj_converted_str = byte_obj_converted.to_string();
    if byte_obj_converted_str.contains(".") {
        let splited_vec = byte_obj_converted_str
            .split(&['.', ' '])
            .collect::<Vec<&str>>();
        format!("{}M", splited_vec[0])
    } else {
        let splited_vec = byte_obj_converted_str
            .split_whitespace()
            .collect::<Vec<&str>>();
        format!("{}M", splited_vec[0])
    }
}

pub fn get_disk_info(disk_name: &str) -> String {
    // let mut output = String::from_utf8(
    //     Command::new("blockdev")
    //         .arg("--getsize64")
    //         .arg(disk_name)
    //         .output()
    //         .unwrap()
    //         .stdout,
    // )
    // .unwrap();
    let mut output = String::from_utf8(
        Command::new("sudo")
            .arg("blockdev")
            .arg("--getsize64")
            .arg(disk_name)
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    output.pop();

    let byte_u128 = u128::from_str(&output).unwrap();
    let byte_obj = Byte::from_bytes(byte_u128);
    let byte_obj_converted = byte_obj.get_adjusted_unit(ByteUnit::GB);
    byte_obj_converted.to_string()
}

#[tauri::command]
pub fn get_disks() -> Vec<DisksInfo> {
    let all_disks = partitions_mgmt::print_all_disks();

    all_disks
        .iter()
        .map(|each| {
            let name = each.to_owned();
            let info = format!("{} {}", &name, &sys_info::get_disk_info(&each));
            DisksInfo::new(name, info)
        })
        .collect::<Vec<DisksInfo>>()
}
