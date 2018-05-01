//? 課題2: 任意のアミノ酸配列に対応する塩基配列を列挙するプログラムを作成せよ。
use std::io::{self, Write};
use util::data::Amino;

pub fn solve() {
    let mut line = String::new();
    print!("Task2: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line).unwrap();
    println!("Task2: {:?}", aa_to_na(&line.trim_matches('\n')));
}

fn aa_to_na(amino: &str) -> Vec<String> {
    if amino.contains("-") {
        amino_convert_recursive(amino.split("-").collect(), vec!["".to_string()])
    } else {
        let amino: Vec<&str> = amino.split("").collect();
        amino_convert_recursive(amino[1..amino.len() - 1].to_vec(), vec!["".to_string()])
    }
}

fn amino_convert_recursive(amino: Vec<&str>, nucleic: Vec<String>) -> Vec<String> {
    if amino.len() == 0 {
        return nucleic;
    }
    let mut passed: Vec<String> = Vec::new();
    for codon in Amino::new(amino[0]).unwrap().to_codon() {
        for i in nucleic.iter() {
            passed.push(format!("{}{}", i.clone(), codon));
        }
    }
    amino_convert_recursive(amino[1..].to_vec(), passed)
}
