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
    match line.trim_matches('\n') {
        "1" => task1::solve(),
        "2" => task2::solve(),
        "3" => task3::solve(),
        "4" => task4::solve(),
        _ => println!("please input 1 to 5"),
    }
}
