extern crate cryptopals_rs;

use cryptopals_rs::set1::*;
fn main() {

    let bytes = decode_base64("aGV5YWQqTWE=");
    println!("{:?}", bytes);
    // println!("{:?}", bytes);
    println!("{}", String::from_utf8(bytes).unwrap());
}