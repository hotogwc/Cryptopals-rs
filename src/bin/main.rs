extern crate cryptopals_rs;
extern crate openssl;

use cryptopals_rs::util::*;
use openssl::symm::{decrypt, Cipher};

fn main() {
    let cipher = Cipher::aes_128_ecb();
    let data_content = read_file_to_vec_string("7.txt").concat();
    let data = data_content.as_bytes();
    let key = "YELLOW SUBMARINE".as_bytes();

    let cipher_text = decrypt(cipher, key, None, data).unwrap();
    println!("{}", String::from_utf8(cipher_text).unwrap());
}
