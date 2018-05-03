extern crate first_task;
use first_task::tasks::*;
use std::io::{self, Write};

fn main() {
    manage_task();
}

fn manage_task() {
    let mut line = String::new();
    print!("Input task number [1~7]: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line).unwrap();
    let task_num = line.trim_matches('\n');
    let label = format!("Task{}: ", task_num);
    match task_num {
        "1" => task1::solve(&label),
        "2" => task2::solve(&label),
        "3" => task3::solve(&label),
        "4" => task4::solve(&label),
        _ => println!("Please input 1 to 7."),
    }
}
