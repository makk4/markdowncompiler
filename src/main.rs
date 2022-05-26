mod scanner;

use std::io;
use std::fs::File;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut f = File::open("test.md")?;
    let mut buffer = Vec::new();

    // read up to 10 bytes
    let _n = f.read_to_end(&mut buffer)?;
    scanner::scan_token(&buffer);
    println!("The bytes: {:?}", &buffer);
    Ok(())
}