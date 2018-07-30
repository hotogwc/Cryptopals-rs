extern crate cryptopals_rs;

use cryptopals_rs::set1::*;
use cryptopals_rs::util::*;

fn main() {
    let input = "YELLOW SUBMARINE".as_bytes();
    let output = pkcs7_padding(input, 20);
    println!("{:?}", output);
    println!("{:?}", "YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes());
    // println!("{}", String::from_utf8(output).unwrap());
}
