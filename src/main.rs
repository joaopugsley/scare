mod utils;
mod setup;
mod tasks;
mod ui;

use std::io::{self, Write};

use console::Term;

use crate::{
    ui::{update, print_tag},
    utils::clear,
    setup::ensure_scare_folder_exists,
    tasks::{load_tasks, toggle_done, remove_task, add_task, edit_task},
};

pub fn main() {
    ensure_scare_folder_exists();

    let mut tasks = load_tasks();
    let mut total_tasks = tasks.len() as i32;

    let mut mode = "view";
    let mut current_index: i32;
    current_index = 0;

    update(&tasks, current_index);
    let stdout = Term::buffered_stdout();
    loop {
        match mode {
            "view" => {
                if let Ok(character) = stdout.read_char() {
                    match character {
                        'c' => mode = "create",
                        'w' => current_index -= 1,
                        'W' => current_index = 0,
                        'a' => {
                            clear();
                            println!("Goodbye!");
                            break;
                        }
                        's' => current_index += 1,
                        'S' => current_index = total_tasks - 1,
                        'd' => {
                            if total_tasks > 0 {
                                mode = "edit"
                            }
                        },
                        'r' => {
                            let _ = remove_task(current_index, false);
                            tasks = load_tasks();
                            total_tasks = tasks.len() as i32;
                            current_index -= 1;
                        }
                        'R' => {
                            let _ = remove_task(current_index, true);
                            tasks = load_tasks();
                            total_tasks = tasks.len() as i32;
                            current_index -= 1;
                        }
                        ' ' => {
                            let _ = toggle_done(current_index);
                            tasks = load_tasks();
                        }
                        _ => {}
                    }

                    if current_index < 0 {
                        current_index = 0;
                    }

                    if current_index >= total_tasks - 1 {
                        current_index = total_tasks - 1;
                    }

                    update(&tasks, current_index);
                }
            }
            "create" => {
                clear();
                print_tag();
                println!("SCARE v{} | Type your new task:", env!("CARGO_PKG_VERSION"));
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");

                add_task(input.trim().to_owned());
                tasks = load_tasks();
                total_tasks = tasks.len() as i32;
                current_index = 0;

                mode = "view";

                update(&tasks, current_index);
            }
            "edit" => {
                clear();
                print_tag();

                let current_task = &tasks[current_index as usize];

                println!("SCARE v{} | Editing task '{}':", env!("CARGO_PKG_VERSION"), current_task);
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");

                edit_task(current_index, input.trim().to_owned());
                tasks = load_tasks();
                total_tasks = tasks.len() as i32;
                current_index = 0;

                mode = "view";

                update(&tasks, current_index);
            }
            _ => {}
        }
    }
}
