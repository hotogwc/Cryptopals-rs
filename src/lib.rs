pub mod set1;
pub mod util;

#[cfg(test)]
mod set1_test {
    use super::set1::*;
    use super::util::*;
    #[test]
    fn encode_base64() {
        let expected = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
        let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        assert_eq!(bytes_to_base64(&hex_to_bytes(hex_string)), expected);
    }

    #[test]
    fn fixed_xor() {
        let expected = String::from("746865206b696420646f6e277420706c6179");
        let left_bytes = hex_to_bytes("1c0111001f010100061a024b53535009181c");
        let right_bytes = hex_to_bytes("686974207468652062756c6c277320657965");
        assert_eq!(
            bytes_to_hex(&xor_bytes(&left_bytes, &right_bytes)), 
            expected
        );
    }

    #[test]
    fn single_byte_xor() {
        let target_key = b'X';
        let target = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let result = xor_with_one_bytes(&hex_to_bytes(target), &target_key);
        assert_eq!(String::from("Cooking MC's like a pound of bacon"), String::from_utf8(result).unwrap());
    }

    #[test]
    fn single_char_file() {
        let target_key = b'5';  
        let lines = read_file_to_vec_string("4.txt"); 
        let result = lines
                     .iter()
                     .filter_map (|line| { 
                        String::from_utf8(
                            xor_with_one_bytes(&hex_to_bytes(line), &target_key)
                        ).ok() 
                      })
                     .collect::<Vec<String>>();
        assert!(result.contains(&String::from("Now that the party is jumping\n")));
    }

    #[test]
    fn repeat_key_xor() {
        let source = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = b"ICE";
        let expected = String::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
        assert_eq!(
            bytes_to_hex(&xor_with_cycle_bytes(&source.as_bytes(), key)), 
            expected
        );
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance("this is a test", "wokka wokka!!!"), 37);
    }
}
