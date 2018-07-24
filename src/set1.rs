

pub const BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

//turn hex to bytes
pub fn hex_to_bytes(hex_string: &str) -> Vec<u8> {
    assert!(hex_string.len() % 2 == 0);
    let chars: Vec<_> = hex_string.chars().collect();
    chars.chunks(2).map ( |chunk| {
       let first = chunk[0].to_digit(16).unwrap();
       let second = chunk[1].to_digit(16).unwrap();
       (first << 4 | second) as u8
    }).collect()
}
//turn bytes to hex string
pub fn bytes_to_hex(x: &[u8]) -> String {
    (0..x.len()).map(|i| format!("{:02x}", x[i])).collect::<Vec<String>>().concat()
}

//turn bytes to base64 string
pub fn bytes_to_base64(bytes: &[u8]) -> String {
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
        .map (|(l,r)| l ^ r)
        .collect()
}

//XOR Vec<u8> with single byte
pub fn xor_with_one_bytes(source: &[u8], key: &u8) -> Vec<u8> {
    source.iter()
          .map (|b| b ^ key)
          .collect()
}

//XOR Vec<u8> with a cycle bytes
pub fn xor_with_cycle_bytes(source: &[u8], cycle: &[u8]) -> Vec<u8> {
    let mut cycle_iter = cycle.iter().cycle();
    source.iter()
          .map (|b| b ^ cycle_iter.next().unwrap())
          .collect()
}

//Hamming distance between two strings
pub fn hamming_distance(lhs: &str, rhs: &str) -> u32 {
    assert_eq!(lhs.len(), rhs.len());
    let (lhs_bytes, rhs_bytes) = (lhs.as_bytes(), rhs.as_bytes());
    (0..lhs.len())
        .map (|i| (lhs_bytes[i] ^ rhs_bytes[i]).count_ones() )
        .sum()
}

//decode base64 from base64string to vec u8
pub fn decode_base64(base64_str: &str)-> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::with_capacity(base64_str.len()*4/3);
    base64_str.chars()
              .filter (|c| (*c as u8) != b'=')
              .collect::<Vec<char>>()
              .chunks(4)
              .for_each (|chunk| {
                  println!("{:?}", chunk);
        let mut y = match chunk.len() {
            4 => transform_u32_to_vec_u8(
                index_of_base64_table(&chunk[0]) << 18 | 
                index_of_base64_table(&chunk[1]) << 12 | 
                index_of_base64_table(&chunk[2]) << 6 | 
                index_of_base64_table(&chunk[3]), 
                3),
            3 => transform_u32_to_vec_u8(
                index_of_base64_table(&chunk[0]) << 18 | 
                index_of_base64_table(&chunk[1]) << 12 | 
                index_of_base64_table(&chunk[2]) << 6 , 
                2),
            2 => transform_u32_to_vec_u8(
                index_of_base64_table(&chunk[0]) << 18 | 
                index_of_base64_table(&chunk[1]) << 12 , 
                1),
            _ => panic!("invalid base64 string"),
        };
        buffer.append(&mut y);
    });
    return buffer;
} 

fn transform_u32_to_vec_u8(x: u32, size: usize) -> Vec<u8> {
    let offset_array = [16, 8, 0];
    (0..size).map (|i| 
        ((x >> offset_array[i]) & 0xff) as u8
     ).collect()
}

fn index_of_base64_table(character: &char) -> u32 {
    match BASE64_TABLE.iter().position(|x| *x == (*character as u8)) {
        Some(i) => i as u32,
        None => panic!("invalid base64 character"),
    }
}