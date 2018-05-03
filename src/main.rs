extern crate first_task;
use first_task::tasks::*;
use std::env;
use std::io::{self, Write};

fn main() {
    manage_task(env::args().nth(1));
}

fn manage_task(task: Option<String>) {
    let (task_num, label) = match task {
        Some(task_num) => (task_num, "".to_string()),
        None => {
            let mut line = String::new();
            print!("Input task number [1~7]: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut line).unwrap();
            let task_num = line.trim_matches('\n');
            let label = format!("Task{}: ", task_num);
            (task_num.to_string(), label)
        }
    };
    match task_num.as_str() {
        "1" => task1::solve(&label),
        "2" => task2::solve(&label),
        "3" => task3::solve(&label),
        "4" => task4::solve(&label),
        "5" => task5::solve(&label),
        "6" => task6::solve(&label),
        _ => println!("Please input 1 to 7."),
    }
}
