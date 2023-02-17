#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod db;
mod handler;

use clap::Parser;
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

static CONF_MAP: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

fn get_value_mutex_safe(key: &str) -> String {
    loop {
        match CONF_MAP.clone().try_lock() {
            Ok(unlocked) => break unlocked.get(key).unwrap().to_owned(),
            Err(_) => std::thread::sleep(std::time::Duration::from_millis(3)),
        }
    }
}

fn set_value_mutex_safe(key: &str, value: String) {
    loop {
        match CONF_MAP.clone().try_lock() {
            Ok(mut unlocked) => {unlocked.insert(key.to_string(), value).unwrap(); break},
            Err(_) => std::thread::sleep(std::time::Duration::from_millis(3)),
        }
    }
}

fn set_init_parameter(conf_location: &str) {
    let settings = config::Config::builder()
        .add_source(config::File::with_name(conf_location))
        .build()
        .unwrap();
    settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap()
        .into_iter()
        .for_each(|(key, value)| {
            CONF_MAP.lock().unwrap().insert(key, value);
        })
}

// fn set_init_parameter(conf: HashMap<String, String>) {
//     conf.into_iter().for_each(|(key, value)| {
//         CONF_MAP.lock().unwrap().insert(key, value);
//     })
// }

// Configuration Arguments Executable for Content Server Installer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Location of the configuration file. Can be full or relative path
    #[arg(short, long, default_value_t = String::from("./settings.toml"))]
    config_file: String,
}

// // Configuration Arguments Executable for Content Server Installer
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     /// Location of the Database. Can be full or relative path
//     #[arg(short, long, default_value_t = String::from("./kcs-installer.db"))]
//     database: String,

//     /// Default Username of new KOOMPI Content Server admin
//     #[arg(short, long, default_value_t = String::from("admin"))]
//     default_username: String,

//     /// Default Password of new KOOMPI Content Server admin
//     #[arg(short, long, default_value_t = String::from("123"))]
//     default_password: String,

//     /// Multicast ID of KOOMPI Content Server master device to search for
//     #[arg(short, long, default_value_t = String::from("koompi-content-master"))]
//     master_multicast_id: String,

//     /// Username of KOOMPI Content Server Master
//     #[arg(short, long, default_value_t = String::from("root"))]
//     master_username: String,

//     /// Password of KOOMPI Content Server Master
//     #[arg(short, long, default_value_t = String::from("123"))]
//     master_password: String,
// }

// impl Args {
//     pub fn create_hash_map(self) -> HashMap<String, String> {
//         let mut map: HashMap<String, String> = HashMap::new();
//         map.insert(String::from("DATABASE"), self.database);
//         map.insert(String::from("DEFAULT_USERNAME"), self.default_username);
//         map.insert(String::from("DEFAULT_PASSWORD"), self.default_password);
//         map.insert(
//             String::from("MASTER_MULTICAST_ID"),
//             self.master_multicast_id,
//         );
//         map.insert(String::from("MASTER_USERNAME"), self.master_username);
//         map.insert(String::from("MASTER_PASSWORD"), self.master_password);

//         map
//     }
// }

fn main() {
    let args = Args::parse();
    set_init_parameter(&args.config_file);
    // set_init_parameter(args.create_hash_map());
    db::create_tables();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            handler::get_install_status,
            handler::reboot,
            handler::start_installation,
            handler::get_disks,
            handler::set_disk_and_ip
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
