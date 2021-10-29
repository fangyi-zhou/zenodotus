extern crate nom_bibtex;

use lazy_static::lazy_static;
use nom_bibtex::error::BibtexError;
use nom_bibtex::*;
use regex::Regex;
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::io;

pub struct BibEntry {
    entry_type: String,
    citation_key: String,
    tags: BTreeMap<String, String>,
}

fn is_stylish_citation_key(key: &str, entry_type: &str) -> bool {
    lazy_static! {
        // CONFNAME + 2 digit year + Some words
        static ref CONFERENCE_CITATION_KEY_REGEX: Regex =
            Regex::new("[[:alpha:]]+[[:digit:]]{2}[[:alnum:]]+").unwrap();
        // Any Combination of words
        static ref OTHER_CITATION_KEY_REGEX: Regex = Regex::new("[[:alnum:]]+").unwrap();
    }
    match entry_type {
        "article" | "conference" | "inproceedings" => CONFERENCE_CITATION_KEY_REGEX.is_match(key),
        _ => OTHER_CITATION_KEY_REGEX.is_match(key),
    }
}

fn prompt_new_citation_key(b: &mut BibEntry) -> io::Result<String> {
    println!("{}", b);
    println!("Please entry a new citation key for the following bib entry:");
    let mut key = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut key)?;
    Ok(key)
}

fn prompt_change_title(old_title: &str, new_title: &str) -> io::Result<bool> {
    println!("Change title\n{}\nto\n{}\n? [y/N]", old_title, new_title);
    let mut resp = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut resp)?;
    Ok(resp == "y")
}

fn fix_title(title: &str) -> Option<String> {
    if title.starts_with("{") && title.ends_with("}") {
        return None;
    }
    panic!("TODO")
}

impl BibEntry {
    pub fn stylise(&mut self) {
        while !is_stylish_citation_key(&self.citation_key, &self.entry_type) {
            if let Ok(new_key) = prompt_new_citation_key(self) {
                if !new_key.is_empty() {
                    self.citation_key = new_key
                }
            }
        }
        if let Some(title) = self.tags.get_mut(&String::from("title")) {
            if let Some(new_title) = fix_title(title) {
                if let Ok(true) = prompt_change_title(title, &new_title) {
                    *title = new_title
                }
            }
        }
    }
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
    Ok(bibtex.bibliographies().iter().map(BibEntry::from).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_title() {
        assert_eq!(fix_title("foo"), Some(String::from("Foo")));
        assert_eq!(fix_title("Foo"), None);
        assert_eq!(fix_title("FooBar"), Some(String::from("{FooBar}")));
        assert_eq!(fix_title("{FooBar}"), None);
        assert_eq!(
            fix_title("Foobar FooBar"),
            Some(String::from("Foobar {FooBar}"))
        );
        assert_eq!(
            fix_title("FooBar FooBar"),
            Some(String::from("{FooBar} {FooBar}"))
        );
        assert_eq!(fix_title("Foobar {FooBar}"), None);
        assert_eq!(fix_title("{FooBar FooBar}"), None);
    }
}
