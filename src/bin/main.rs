extern crate cryptopals_rs;

use cryptopals_rs::set1::*;


fn main() {
    let input = "0e3647e8592d35514a081243582536ed3de6734059001e3f535ce6271032";
 
    println!("{:?}", String::from_utf8_lossy(&hex_to_bytes(input)));
}