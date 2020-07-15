extern crate hex;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs;
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct BinaryParser;

fn main() -> std::io::Result<()> {
    let fname = "res/ghohor017_15_owlcreekbridge_bierce_lems_64kb.mp3";
    // an occurance at owl creek bridge
    // let mut file = File::open("res/ghohor017_15_owlcreekbridge_bierce_lems_64kb.mp3")?;

    // ```
    // $ wc -c res/ghohor017_15_owlcreekbridge_bierce_lems_64kb.mp3
    // 12289024
    // ```
    //let mut buf = [0u8; 12289024];

    //let bytes_read = file.read(&mut buf)?;

    // let mut buffer = String::new();
    // let mut f = File::open(fname)?;
    // f.read_to_string(&mut buffer)?;

    let buffer = std::fs::read(fname)?;
    let hexbuffer = hex::encode(&buffer);

    //println!("{:?}", buffer);

    let ret = BinaryParser::parse(Rule::mp3_header, &hexbuffer[0..3]).unwrap();

    println!("{:?}", ret);
    Ok(())
}
