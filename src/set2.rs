use ::set1::*;



pub fn pkcs7_padding(bytes: &[u8], block_size: usize) -> Vec<u8> {
    let padding = (block_size - (bytes.len() % block_size)) as u8;
    let mut with_padding = bytes.to_vec().clone();
    if padding == 0 {
        (0..block_size).for_each ( |_| { with_padding.push(block_size as u8) } );
    } else {
        (0..padding).for_each ( |_| { with_padding.push(padding) } );
    }

    return with_padding;
}





