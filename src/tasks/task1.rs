//? 課題1: 以下のURLからgenbank形式のファイルをダウンロードして、含まれる遺伝子数とをカウントせよ。
//? ftp://ftp.ncbi.nlm.nih.gov/genomes/Bacteria/Escherichia_coli_K_12_substr__DH10B_uid58979/NC_010473.gff
use std::path::Path;
use util::io::Gff;

pub fn solve(label: &str) {
    let path = Path::new("data/NC_010473.gff");
    let gff = Gff::from_file(path.to_str().unwrap()).unwrap();
    let mut count = 0;
    for record in gff.records {
        if &record.feature == "gene" {
            count += 1;
        }
    }
    println!("{}{}", label, count);
}
