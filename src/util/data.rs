use std::io::{Error, ErrorKind};
#[derive(Debug)]
pub enum Amino {
    A,
    R,
    N,
    D,
    C,
    E,
    Q,
    G,
    H,
    I,
    L,
    K,
    M,
    F,
    P,
    S,
    T,
    W,
    Y,
    V,
}

impl<'a> Amino {
    pub fn new(data: &str) -> Result<Self, Error> {
        match data {
            "A" | "Ala" => Ok(Amino::A),
            "R" | "Arg" => Ok(Amino::R),
            "N" | "Asn" => Ok(Amino::N),
            "D" | "Asp" => Ok(Amino::D),
            "C" | "Cys" => Ok(Amino::C),
            "E" | "Glu" => Ok(Amino::E),
            "Q" | "Gln" => Ok(Amino::Q),
            "G" | "Gly" => Ok(Amino::G),
            "H" | "His" => Ok(Amino::H),
            "I" | "Ile" => Ok(Amino::I),
            "L" | "Leu" => Ok(Amino::L),
            "K" | "Lys" => Ok(Amino::K),
            "M" | "Met" => Ok(Amino::M),
            "F" | "Phe" => Ok(Amino::F),
            "P" | "Pro" => Ok(Amino::P),
            "S" | "Ser" => Ok(Amino::S),
            "T" | "Thr" => Ok(Amino::T),
            "W" | "Trp" => Ok(Amino::W),
            "Y" | "Tyr" => Ok(Amino::Y),
            "V" | "Val" => Ok(Amino::V),
            input @ _ => Err(Error::new(ErrorKind::InvalidInput, input)),
        }
    }

    pub fn to_codon(self: &Amino) -> &'a [&'a str] {
        match self {
            &Amino::A => &["GCU", "GCC", "GCA", "GCG"],
            &Amino::R => &["CGU", "CGC", "CGA", "CGG", "AGA", "AGG"],
            &Amino::N => &["AAU", "AAC"],
            &Amino::D => &["GAU", "GAC"],
            &Amino::C => &["UGU", "UGC"],
            &Amino::E => &["GAA", "GAG"],
            &Amino::Q => &["CAA", "CAG"],
            &Amino::G => &["GGU", "GGC", "GGA", "GGG"],
            &Amino::H => &["CAU", "CAC"],
            &Amino::I => &["AUU", "AUC", "AUA"],
            &Amino::L => &["UUA", "UUG", "CUU", "CUC", "CUA", "CUG"],
            &Amino::K => &["AAA", "AAG"],
            &Amino::M => &["AUG"],
            &Amino::F => &["UUU", "UUC"],
            &Amino::P => &["CCU", "CCC", "CCA", "CCG"],
            &Amino::S => &["UCU", "UCC", "UCA", "UCG", "AGU", "AGC"],
            &Amino::T => &["ACU", "ACC", "ACA", "ACG"],
            &Amino::W => &["UGG"],
            &Amino::Y => &["UAU", "UAC"],
            &Amino::V => &["GUU", "GUC", "GUA", "GUG"],
        }
    }
}
