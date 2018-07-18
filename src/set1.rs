pub const BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

//turn hex to bytes
pub fn hex_to_bytes(hex_string: &str) -> Vec<u8> {
    assert!(hex_string.len() % 2 == 0);
    let chars: Vec<_> = hex_string.chars().collect();
    chars.chunks(2).map ( |chunk| {
       let first = chunk[0].to_digit(16).unwrap();
       let second = chunk[1].to_digit(16).unwrap();
       (first << 4 | second) as u8
    }).collect::<Vec<u8>>()
}
//turn bytes to hex string
pub fn bytes_to_hex(x: &[u8]) -> String {
    (0..x.len()).map(|i| format!("{:02x}", x[i])).collect::<Vec<String>>().concat()
}

//turn bytes to base64 string
pub fn bytes_to_base64(bytes: &Vec<u8>) -> String {
    let mut buffer = Vec::with_capacity(bytes.len()/3*4);
    bytes.chunks(3).for_each ( |chunk| {
        let y = match chunk.len() {
            3 => ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32),
            2 => ((chunk[0] as u32) << 10) | ((chunk[1] as u32) << 2),
            _ => (chunk[0] as u32) << 4,
        };
        for i in 0..chunk.len()+1 {
            buffer.push(BASE64_TABLE[((y >> 6*(chunk.len() - i)) & 0x3F) as usize]);
        }

        for _ in 0..3-chunk.len() {
            buffer.push(b'=');
        }
    });
    String::from_utf8(buffer).unwrap()
}


//XOR two Vec<u8>, produce a result Vec<u8>
pub fn xor_bytes(left: &[u8], right: &[u8]) -> Vec<u8> {
    assert_eq!(left.len(), right.len());
    left.iter()
        .zip(right.iter())
        .map (|(l,r)| l ^ r).collect::<Vec<u8>>()
}

//XOR Vec<u8> with single byte
pub fn xor_with_one_bytes(source: &[u8], key: &u8) -> Vec<u8> {
    source.iter()
          .map (|b| b ^ key).collect::<Vec<u8>>()
}
