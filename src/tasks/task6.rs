//? 課題6: genus1.list、genus2.list、genus3.listを読み込んで、微生物ごとの数を集計し、2次元マトリックスを作成しなさい。
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};

pub fn solve(label: &str) {
    let path_list = ["data/genus1.list", "data/genus2.list", "data/genus3.list"].to_vec();
    let table = combine_list(path_list.clone());
    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut buf = BufWriter::new(lock);
    writeln!(buf, "{}\t{}", label, path_list.join("\t")).unwrap();
    let mut table = table.iter().collect::<Vec<_>>();
    table.sort_by_key(|v| v.0);
    for row in table {
        writeln!(
            buf,
            "{}{}\t{}",
            label,
            row.0,
            row.1
                .into_iter()
                .map(|v| format!("{}", v))
                .collect::<Vec<String>>()
                .join("\t")
        ).unwrap();
    }
}

fn combine_list(path_list: Vec<&str>) -> HashMap<String, Vec<u32>> {
    let map_list: Vec<_> = path_list.into_iter().map(read_list).collect();
    let microbe_list = map_list.iter().fold(HashSet::new(), |list, v| {
        list.union(&v.keys().map(|v| v.to_string()).collect())
            .map(|v| v.to_string())
            .collect()
    });

    let mut combined: HashMap<String, Vec<u32>> = HashMap::new();
    for microbe in microbe_list {
        let nums = map_list
            .iter()
            .map(|v| match v.get(&microbe) {
                Some(num) => *num,
                None => 0,
            })
            .collect();
        combined.insert(microbe, nums);
    }
    combined
}

fn read_list(path: &str) -> HashMap<String, u32> {
    let mut microbes: HashMap<String, u32> = HashMap::new();
    let pat: &[_] = &['\n', '"'];

    let mut line = String::new();
    let mut buf = BufReader::new(File::open(path).unwrap());
    while buf.read_line(&mut line).unwrap() > 0 {
        let microbe = line.trim_matches(pat).to_string();
        if &microbe == "" {
            continue;
        }
        if microbes.contains_key(&microbe) {
            *microbes.get_mut(&microbe).unwrap() += 1;
        } else {
            microbes.insert(microbe, 1);
        }
        line.clear();
    }
    microbes
}
