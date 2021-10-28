use std::env;
use std::fs::File;
use std::io::Result;
use std::io::Write;

fn main() -> Result<()> {
    let mut args = env::args();
    let filename = args.nth(1).expect("Expect a bib file as argument");
    match zenodotus::load_file(&filename) {
        Err(err) => panic!("Parsing failed: {}", err),
        Ok(entries) => {
            println!("Parsed successfully! Found {} entries", entries.len());
            let mut file = File::create("output.bib")?;
            for entry in &entries {
                writeln!(file, "{}", entry)?;
            }
        }
    };
    Ok(())
}
