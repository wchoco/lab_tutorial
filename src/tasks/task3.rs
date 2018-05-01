//? 課題3: λファージゲノムをEcoRI,HindIII,BamHI,NotIでそれぞれ切った時の断片の長さを列挙せよ。また、全部で切ったときの断片の長さも列挙せよ。ラムダファージのゲノム配列は以下のURLからダウンロードせよ。
//? ftp://ftp.ncbi.nlm.nih.gov/genomes/Viruses/Enterobacteria_phage_lambda_uid14204/NC_001416.fna
use std::collections::hash_set::HashSet;
use util::data::RestrictionEnzyme;
use util::io::Fasta;

pub fn solve() {
    let ecor1 = RestrictionEnzyme::ecor1();
    let hind3 = RestrictionEnzyme::hind3();
    let bamh1 = RestrictionEnzyme::bamh1();
    let not1 = RestrictionEnzyme::not1();

    let target = &Fasta::from_file("data/NC_001416.fna").unwrap().records[0].seq;

    println!(
        "Task3: EcoRI   : {:?}\nTask3: HindIII : {:?}\nTask3: BamHI   : {:?}\nTask3: NotI    : {:?}\nTask3: ALL     : {:?}",
        cut_length(&[&ecor1], target),
        cut_length(&[&hind3], target),
        cut_length(&[&bamh1], target),
        cut_length(&[&not1], target),
        cut_length(&[&ecor1, &hind3, &bamh1, &not1],target),
        );
}

fn cut_length(enzymes: &[&RestrictionEnzyme], target: &str) -> Vec<usize> {
    calc_length(cut_pos(enzymes, target), target.len())
}

fn cut_pos(enzymes: &[&RestrictionEnzyme], target: &str) -> Vec<usize> {
    let mut pos_set: HashSet<usize> = HashSet::new();
    for enzyme in enzymes {
        pos_set.extend(enzyme.cut(target));
    }
    let mut pos_vec = pos_set.into_iter().collect::<Vec<usize>>();
    pos_vec.sort();
    pos_vec
}

fn calc_length(pos: Vec<usize>, max: usize) -> Vec<usize> {
    let mut length: Vec<usize> = Vec::new();
    let mut prev: usize = 0;
    for i in pos.into_iter() {
        length.push(i - prev);
        prev = i;
    }
    length.push(max - prev);
    length
}
