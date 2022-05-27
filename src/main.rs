pub mod scanner;
pub mod parser;
pub mod htmlout;

use std::io;
use std::fs::File;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut f = File::open("test.md")?;
    let mut buffer = Vec::new();

    // read up to 10 bytes
    let _n = f.read_to_end(&mut buffer)?;
    let mut token_list = scanner::scan_token(&buffer);
    token_list = parser::parse(&token_list);
    let output = htmlout::html_out(&token_list);
    print!("{}", output);
    println!("The bytes: {:?}", &buffer);
    Ok(())
}