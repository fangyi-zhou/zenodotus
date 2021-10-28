use std::env;

fn main() {
    let mut args = env::args();
    let _ = args.next().unwrap();
    let filename = args.next().unwrap();
    match zenodotus::load_file(&filename) {
        Ok(_) => println!("Parsed successfully!"),
        Err(err) => println!("Parsing failed: {}", err),
    }
}
