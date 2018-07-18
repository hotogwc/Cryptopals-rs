pub mod set1;

#[cfg(test)]
mod set1_test {
    use super::*;
    #[test]
    fn encode_base64() {
        let expected = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
        let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        assert_eq!(set1::bytes_to_base64(&set1::hex_to_bytes(hex_string)), expected);
    }

    #[test]
    fn fixed_xor() {
        let expected = String::from("746865206b696420646f6e277420706c6179");
        let left_bytes = set1::hex_to_bytes("1c0111001f010100061a024b53535009181c");
        let right_bytes = set1::hex_to_bytes("686974207468652062756c6c277320657965");
        assert_eq!(
            set1::bytes_to_hex(&set1::xor_bytes(&left_bytes, &right_bytes)), 
            expected
        );
    }

    #[test]
    fn single_byte_xor() {
        let target_key = b'X';
        let target = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let result = set1::xor_with_one_bytes(&set1::hex_to_bytes(target), &target_key);
        assert_eq!(String::from("Cooking MC's like a pound of bacon"), String::from_utf8(result).unwrap());
    }
    
}
