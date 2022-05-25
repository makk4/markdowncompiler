mod scanner;

use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let f = File::open("test.md")?;
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        scanner::scan_token(l);
    }
    Ok(())
}