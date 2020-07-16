extern crate hex;
extern crate pest;
extern crate radix_fmt;
#[macro_use]
extern crate pest_derive;

use hex::FromHex;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct BinaryParser;

type Mp3Parse<'a> = pest::iterators::Pair<'a, Rule>;

fn main() -> std::io::Result<()> {
    let fname = "res/ghohor017_15_owlcreekbridge_bierce_lems_64kb.mp3";
    let buffer = std::fs::read(fname)?;
    let hexbuffer = hex::encode(&buffer);

    // maybe something like this if you want all base 2 data to parse...?
    let _base2buffer = buffer
        .into_iter()
        .map(|n| format!("{}", radix_fmt::radix(n, 2)))
        .collect::<Vec<String>>();

    let mut m: usize = 0;
    let mut n: usize = 3;
    let end: usize = hexbuffer.len() - 1;

    let header: Mp3Parse = BinaryParser::parse(Rule::mp3_header, &hexbuffer[m..n])
        .unwrap()
        .next()
        .unwrap();

    println!("header: {:?}", header);

    m = 4;
    n = 8;
    while n < end {
        let mut step = 4;

        let parsed_data = BinaryParser::parse(Rule::mp3_hex_content, &hexbuffer[m..n])
            .unwrap()
            .next();

        if let Some(value) = parsed_data {
            match value.as_rule() {
                Rule::important_hex_shape_a => {
                    if let Ok(v) = Vec::from_hex(value.as_str()) {
                        if let Some(last) = v.last() {
                            step = *last as usize
                        }
                    } else {
                        step = 8;
                    }
                    println! {"got importnat shape a: {:?}", value.as_str()};
                }

                Rule::important_hex_shape_b => {
                    if let Ok(v) = Vec::from_hex(value.as_str()) {
                        if let Some(first) = v.first() {
                            step = *first as usize
                        }
                    } else {
                        step = 10;
                    }
                    println! {"got importnat shape b: {:?}", value.as_str()};
                }

                Rule::hex_byte => {
                    // look for something else...?
                }

                _ => {
                    // handle the other things
                    break;
                }
            }
        }

        m = n;
        n = n + step;
    }

    Ok(())
}
