use std::convert::From;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};

#[derive(Debug)]
pub struct Gff {
    pub comment: String,
    pub records: Vec<GffRecord>,
}

#[derive(Debug)]
pub struct GffRecord {
    pub seqname: String,
    pub source: String,
    pub feature: String,
    pub start: String,
    pub end: String,
    pub score: String,
    pub strand: String,
    pub frame: String,
    pub attribute: String,
}

impl From<Vec<String>> for GffRecord {
    fn from(data: Vec<String>) -> Self {
        GffRecord {
            seqname: data[0].clone(),
            source: data[1].clone(),
            feature: data[2].clone(),
            start: data[3].clone(),
            end: data[4].clone(),
            score: data[5].clone(),
            strand: data[6].clone(),
            frame: data[7].clone(),
            attribute: data[8].clone(),
        }
    }
}

impl Gff {
    pub fn new() -> Gff {
        Gff {
            comment: "".to_string(),
            records: Vec::new(),
        }
    }
    pub fn from_file(path: &str) -> Result<Gff, Error> {
        let mut line = String::new();
        let mut buf = BufReader::new(File::open(path)?);
        let mut comment: Vec<String> = Vec::new();
        let mut records: Vec<_> = Vec::new();
        while buf.read_line(&mut line).unwrap() > 0 {
            if !line.starts_with("#") {
                records.push(
                    line.split('\t')
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .into(),
                );
                line.clear();
                break;
            }
            comment.push(line.to_string());
            line.clear();
        }
        while buf.read_line(&mut line).unwrap() > 0 {
            if line.starts_with("#") {
                continue;
            }
            records.push(
                line.split('\t')
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .into(),
            );
            line.clear();
        }
        Ok(Gff {
            comment: comment.join("\n"),
            records: records,
        })
    }
}

#[derive(Debug)]
pub struct Fasta {
    pub num: u64,
    pub records: Vec<FastaRecord>,
}

#[derive(Debug)]
pub struct FastaRecord {
    pub id: String,
    pub seq: String,
}

impl Fasta {
    pub fn from_file(path: &str) -> Result<Fasta, Error> {
        let mut line = String::new();
        let mut buf = BufReader::new(File::open(path)?);

        let mut records: Vec<_> = Vec::new();
        let mut id = String::new();
        let mut seq: Vec<String> = Vec::new();
        let mut count: u64 = 0;
        while buf.read_line(&mut line)? > 0 {
            if line.starts_with(">") {
                count += 1;
                id = line.clone();
                line.clear();
                break;
            }
        }
        while buf.read_line(&mut line)? > 0 {
            if line.starts_with(">") {
                count += 1;
                records.push(FastaRecord {
                    id: id,
                    seq: seq.join(""),
                });
                id = line.clone();
                seq.clear();
            } else {
                seq.push(line.clone());
            }
            line.clear();
        }
        records.push(FastaRecord {
            id: id,
            seq: seq.join(""),
        });
        Ok(Fasta {
            num: count,
            records: records,
        })
    }
}
