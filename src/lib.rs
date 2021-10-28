extern crate nom_bibtex;

use nom_bibtex::error::BibtexError;
use nom_bibtex::*;
use std::fs;

pub fn load_file(filename: &str) -> Result<Bibtex, BibtexError> {
    let input = fs::read_to_string(filename).unwrap();
    Bibtex::parse(&input)
}
