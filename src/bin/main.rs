extern crate cryptopals_rs;

use cryptopals_rs::set1::*;
use cryptopals_rs::util;
use cryptopals_rs::set1::BASE64_TABLE;

fn main() {
    util::read_file_to_vec_string("4.txt").iter().for_each ( |line| {
        BASE64_TABLE.iter()
                    .for_each (|b| {
                        let r = xor_with_one_bytes(&hex_to_bytes(line), b);
                        let s = String::from_utf8(r);
                        match s {
                            Ok(string) => {
                                println!("{}:{}", string, *b as char);
                            }
                            _ => (),
                        };
                    });
    });
}