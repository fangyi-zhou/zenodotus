use std::env;

fn main() {
    let mut args = env::args();
    let _ = args.next().unwrap();
    let filename = args.next().unwrap();
    match zenodotus::load_file(&filename) {
        Ok(entries) => {
            println!("Parsed successfully! Found {} entries", entries.len());
            for entry in &entries {
                println!("{}", entry)
            }
        }
        Err(err) => println!("Parsing failed: {}", err),
    }
}
