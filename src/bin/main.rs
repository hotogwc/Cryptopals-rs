extern crate cryptopals_rs;
extern crate openssl;

use cryptopals_rs::util::*;
use openssl::symm::{decrypt, Cipher};
use cryptopals_rs::set1::*;

fn main() {
    let cipher = Cipher::aes_128_ecb();
    let data_content = read_file_to_vec_string("7.txt").concat();
    let data = decode_base64(&data_content);

    // let mut f = File::open("7.txt").expect("Unable to open '7.txt'");
	// let mut contents = String::new();
	// f.read_to_string(&mut contents).expect("Trouble reading '7.txt'");


	// let contents = contents.lines().collect::<String>();
	// let ciphertext = base64::decode(&contents);
    let key = "YELLOW SUBMARINE".as_bytes();

    let cipher_text = decrypt(cipher, key, None, &data).unwrap();
    println!("{}", String::from_utf8(cipher_text).unwrap());
}
