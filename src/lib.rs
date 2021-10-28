extern crate nom_bibtex;

use nom_bibtex::error::BibtexError;
use nom_bibtex::*;
use std::fmt;
use std::fs;

pub struct BibEntry {
    entry_type: String,
    citation_key: String,
    tags: Vec<(String, String)>,
}

impl fmt::Display for BibEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "@{}{{{},", self.entry_type, self.citation_key)?;
        for tag in &self.tags {
            writeln!(f, "  {} = {{{}}},", tag.0, tag.1)?;
        }
        write!(f, "}}")
    }
}

impl From<&Bibliography> for BibEntry {
    fn from(b: &Bibliography) -> Self {
        BibEntry {
            entry_type: b.entry_type().to_string().to_lowercase(),
            citation_key: b.citation_key().to_string(),
            tags: b
                .tags()
                .iter()
                .map(|(k, v)| (k.to_lowercase(), v.clone()))
                .collect(),
        }
    }
}

pub fn load_file(filename: &str) -> Result<Vec<BibEntry>, BibtexError> {
    let input = fs::read_to_string(filename).unwrap();
    let bibtex = Bibtex::parse(&input)?;
    Ok(bibtex
        .bibliographies()
        .iter()
        .map(|b| BibEntry::from(b))
        .collect())
}
