//? 課題4: enzyme.txtは、KEGG DBに登録されているEnzymeのリスト（をランダムにシャッフルしたもの）である。このリストを、EC番号順にソートせよ。
//? EC番号は４つのレベルからなる。これをソートするとは、以下のような並びにして出力すること。
//? ec:1.1.1.1
//? ec:1.1.1.2
//? （中略）
//? ec:1.99.2.6
//? ec:2.1.1.1
//? ec:2.1.1.2
//? （中略）
//? ec:6.6.1.2
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn solve() {
    let ec_list = read_ec_list("data/enzyme.txt");
    print!("Task4: {}", sort_ec_list(ec_list).join("Task4: "));
}

fn read_ec_list(path: &str) -> Vec<String> {
    let mut buf = BufReader::new(File::open(path).unwrap());
    let mut line = String::new();

    let mut ec_list: Vec<String> = Vec::new();
    while buf.read_line(&mut line).unwrap() > 0 {
        ec_list.push(line.clone());
        line.clear();
    }
    ec_list
}

fn sort_ec_list(ec_list: Vec<String>) -> Vec<String> {
    let pat: &[_] = &['.', ':', '\t'];
    let ecids: Vec<Vec<u32>> = ec_list
        .clone()
        .into_iter()
        .map(|v| {
            v.split(pat).collect::<Vec<_>>()[1..5]
                .into_iter()
                .map(|v| v.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    let mut sorted = ecids.into_iter().zip(ec_list).collect::<Vec<_>>();
    sorted.sort_by_key(|v| v.clone().0);
    sorted.into_iter().map(|v| v.1).collect()
}
