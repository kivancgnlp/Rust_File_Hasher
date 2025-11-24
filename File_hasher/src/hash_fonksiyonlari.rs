
use md4::{Md4, Digest};

use base16ct;


fn print_hex_bytes(bytes: &[u8]) {
    for b in bytes {
        print!("{:02x}", b);
    }
    println!("");
}

pub fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Md4::new();
    hasher.update(bytes);
    let hash = hasher.finalize();

    let mut ex_output_buffer = [0u8; 64];
    let hex_hash = base16ct::lower::encode_str(&hash, &mut ex_output_buffer);

    String::from(hex_hash.unwrap())
}