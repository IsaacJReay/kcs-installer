use super::{
    get_value_mutex_safe,
    handler::{InstallStatus, InstallStatusListItem},
};
use rusqlite::{params, Connection};
use std::str::FromStr;

fn open_database() -> Connection {
    Connection::open(get_value_mutex_safe("DATABASE")).unwrap()
}

pub fn create_tables() {
    let database = get_value_mutex_safe("DATABASE");
    match std::path::Path::new(&database).exists() {
        true => {
            std::fs::remove_file(database).unwrap();
            create_tables();
        }
        false => {
            open_database()
            .execute_batch("BEGIN;
        CREATE TABLE tblStatusVar(
            ID INTEGER PRIMARY KEY AUTOINCREMENT, 
            Variable VARCHAR(100), 
            Value VARCHAR(255)
        ); 
        INSERT OR REPLACE INTO tblStatusVar(Variable, Value) VALUES('Progress', '0'); 
        INSERT OR REPLACE INTO tblStatusVar(Variable, Value) VALUES('Partitioning Drives', 'pending'); 
        INSERT OR REPLACE INTO tblStatusVar(Variable, Value) VALUES('Formating Partitions', 'pending'); 
        INSERT OR REPLACE INTO tblStatusVar(Variable, Value) VALUES('Installing System', 'pending');
        INSERT OR REPLACE INTO tblStatusVar(Variable, Value) VALUES('Setting Up User Profiles', 'pending');
        INSERT OR REPLACE INTO tblStatusVar(Variable, Value) VALUES('Performing Post-Installation', 'pending'); 
        INSERT OR REPLACE INTO tblStatusVar(Variable, Value) VALUES('Retrieving Data From Master', 'pending'); 
        COMMIT;")
                .unwrap();
        }
    }
}

pub fn _insert_replace_tbl_status(var: &str, key: &str) {
    let db = open_database();
    db.execute(
        "INSERT INTO tblStatusVar(Variable, Value) VALUES(?1, ?2);",
        params![var, key],
    )
    .expect("Failed");
}

pub fn update_tbl_status(var: &str, key: &str) {
    let db = open_database();
    db.execute(
        "UPDATE tblStatusVar SET VALUE = ?2 WHERE VARIABLE = ?1",
        params![var, key],
    )
    .expect("Failed");
}

pub fn increment_progress(progress_limit: u8) {
    let connection = open_database();
    let mut stmt = connection
        .prepare("SELECT Value FROM tblStatusVar WHERE Variable='Progress';")
        .unwrap();

    let current_progress = u8::from_str(
        stmt.query_row([], |line| Ok(line.get::<usize, String>(0).unwrap()))
            .unwrap()
            .as_str(),
    )
    .unwrap();

    if current_progress < progress_limit {
        update_tbl_status("Progress", &(current_progress + 1).to_string())
    }
}

pub fn query_status() -> InstallStatus {
    let connection = open_database();

    let mut stmt = connection.prepare("SELECT * FROM tblStatusVar;").unwrap();

    let mut progress: u8 = 0;

    let mut status_list = stmt
        .query_map([], |row| {
            let id = row.get::<usize, usize>(0).unwrap();
            let name = row.get::<usize, String>(1).unwrap();
            let status = row.get::<usize, String>(2).unwrap();
            if name == "Progress" {
                progress = u8::from_str(&status).unwrap();
            }
            Ok(InstallStatusListItem::new(id as u8, name, status))
        })
        .unwrap()
        .filter(|each| each.as_ref().is_ok() && each.as_ref().unwrap().get_name() != "progress")
        .map(|each| each.unwrap())
        .collect::<Vec<InstallStatusListItem>>();

    status_list.sort_by(|item_a, item_b| item_a.get_id().cmp(&item_b.get_id()));

    InstallStatus::new(progress, status_list)
}
