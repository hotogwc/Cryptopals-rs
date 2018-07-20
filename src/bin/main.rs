extern crate cryptopals_rs;

use cryptopals_rs::set1::*;
fn main() {
    let source = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = b"ICE";
    
    let bytes = xor_with_cycle_bytes(&source.as_bytes(), key);
    println!("{}", bytes_to_hex(&bytes));

}