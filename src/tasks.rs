use std::{fs, io, path::PathBuf};

use crate::setup::get_os_scare_folder;

fn read_tasks_file() -> io::Result<Vec<String>> {
    if let Some(folder_path) = get_os_scare_folder() {
        let mut tasks_file_path = PathBuf::from(folder_path);
        tasks_file_path.push("tasks.txt");
        match fs::read_to_string(&tasks_file_path) {
            Ok(contents) => Ok(contents.lines().map(|s| s.to_string()).collect()),
            Err(err) => Err(err),
        }
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Scare folder not found"))
    }
}

fn write_tasks_to_file(tasks: &[String]) {
    if let Some(folder_path) = get_os_scare_folder() {
        let mut tasks_file_path = PathBuf::from(folder_path);
        tasks_file_path.push("tasks.txt");
        let _ = fs::write(tasks_file_path, tasks.join("\n"));
    }
}

pub fn load_tasks() -> Vec<String> {
    let mut tasks: Vec<String> = Vec::new();

    if let Ok(lines) = read_tasks_file() {
        for line in lines {
            tasks.push(line)
        }
    }

    tasks
}

pub fn toggle_done(index: i32) {
    if let Ok(mut tasks) = read_tasks_file() {
        if !tasks.is_empty() && index >= 0 && (index as usize) < tasks.len() {
            let line = &mut tasks[index as usize];
            if line.starts_with("[DONE] ") {
                *line = line.trim_start_matches("[DONE] ").to_string();
            } else {
                *line = format!("[DONE] {}", line);
            }
            write_tasks_to_file(&tasks);
        }
    }
}

pub fn remove_task(index: i32, force: bool) {
    if let Ok(mut tasks) = read_tasks_file() {
        if !tasks.is_empty() && index >= 0 && (index as usize) < tasks.len() {
            let line = &mut tasks[index as usize];
            if line.starts_with("[DONE] ") || force == true {
                tasks.remove(index as usize);
            }
            write_tasks_to_file(&tasks);
        }
    }
}

pub fn add_task(task: String) {
    if let Ok(mut tasks) = read_tasks_file() {
        tasks.push(task);
        write_tasks_to_file(&tasks);
    }
}

pub fn edit_task(index: i32, new_task: String) {
    if let Ok(mut tasks) = read_tasks_file() {
        if !tasks.is_empty() && index >= 0 && (index as usize) < tasks.len() {
            let line = &mut tasks[index as usize];
            if line.starts_with("[DONE] ") {
                tasks[index as usize] = format!("[DONE] {}", new_task);
            } else {
                tasks[index as usize] = new_task;
            }
            write_tasks_to_file(&tasks);
        }
    }
}