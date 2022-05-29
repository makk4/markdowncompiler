pub mod scanner;
pub mod htmlout;

use std::io;
use std::fs::File;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut f = File::open("tests/test.md")?;
    let mut buffer = Vec::new();

    // read up to 10 bytes
    let _n = f.read_to_end(&mut buffer)?;
    let token_list = scanner::scan_token(&buffer);
    let output = htmlout::html_out(&token_list);
    print!("{}", output);
    println!("The bytes: {:?}", &buffer);
    Ok(())
}