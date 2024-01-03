use std::io::{self, Write};
use crate::utils::clear;

pub fn update(tasks: &Vec<String>, index: i32) {
    clear();
    print_tag();
    println!("SCARE v{} | Your current tasks:", env!("CARGO_PKG_VERSION"));
    println!(" ");
    if tasks.len() > 0 {
        for (current, task) in tasks.iter().enumerate() {
            let selected = current as i32 == index;
            if selected {
                println!(" > {task}");
            } else {
                println!(" {task}");
            }
        }
    } else {
        println!("You don't have any task yet :/");
    }   
    println!(" ");
    print!("A - EXIT | W - UP | S - DOWN | D - EDIT | C - CREATE NEW TASK | SPACE - TOGGLE DONE | R - REMOVE");
    let _ = io::stdout().flush();
}

pub fn print_tag() {
    println!("
    ▄▀▀▀▀▄  ▄▀▄▄▄▄   ▄▀▀█▄   ▄▀▀▄▀▀▀▄  ▄▀▀█▄▄▄▄ 
    █ █   ▐ █ █    ▌ ▐ ▄▀ ▀▄ █   █   █ ▐  ▄▀   ▐ 
       ▀▄   ▐ █        █▄▄▄█ ▐  █▀▀█▀    █▄▄▄▄▄  
    ▀▄   █    █       ▄▀   █  ▄▀    █    █    ▌  
     █▀▀▀    ▄▀▄▄▄▄▀ █   ▄▀  █     █    ▄▀▄▄▄▄   
     ▐      █     ▐  ▐   ▐   ▐     ▐    █    ▐   
            ▐                           ▐     
    ");
}