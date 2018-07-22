extern crate cryptopals_rs;

use cryptopals_rs::set1::*;
fn main() {
    let bytes1 = "this is a test".as_bytes();
    let bytes2 = "wokka wokka!!!".as_bytes();
    let mut acc: u32 = 0;
    (0..bytes1.len()).for_each (|i|{
        acc += (bytes1[i] ^ bytes2[i]).count_ones();
    });
    
    println!("acc is {}", acc);
}