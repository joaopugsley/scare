use std::{env, fs, path::PathBuf};
use crate::{
    utils::{clear, folder_exists},
    ui::print_tag
};

pub fn get_os_scare_folder() -> Option<String> {
    match env::var("APPDATA").ok() {
        Some(appdata) => Some(format!("{}/.scare", appdata)),
        None => env::var("HOME").ok().map(|home| format!("{}/.scare", home)),
    }
}

pub fn ensure_scare_folder_exists() {
    if let Some(folder_path) = get_os_scare_folder() {
        if !folder_exists(&folder_path) {
            setup();
        }
    }
}

pub fn setup() {
    clear();
    print_tag();
    if let Some(folder_path) = get_os_scare_folder() {
        if let Err(err) = fs::create_dir_all(&folder_path) {
            eprintln!("Error creating scare folder: {}", err);
            return;
        }

        let mut tasks_file_path = PathBuf::from(folder_path);
        tasks_file_path.push("tasks.txt");

        if let Err(err) = fs::write(&tasks_file_path, "") {
            eprintln!("Error creating tasks file: {}", err);
            return;
        }

        println!("Setup complete.");
        println!("Welcome to Scare v{}", env!("CARGO_PKG_VERSION"));
        println!("Run 'scare' again to start.");
    }
}
