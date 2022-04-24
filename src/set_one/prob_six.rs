use std::fs;

use crate::prompt::prompt_user;
use crate::set_one::xor::{detect_xor_keysizes, decode_single_byte_xor};

pub fn break_repeating_key_xor_exercise() {
    let filename = prompt_user("Enter ciphertext filename: ");
    let ct_base64: String = fs::read_to_string(filename)
        .expect("Can't read file!")
        .lines()
        .collect();
    let ct = base64::decode(ct_base64)
        .expect("Invalid base64 encoding!");
    let sizes = detect_xor_keysizes(&ct);

    for &size in sizes[0..2].iter() {
        let chunks: Vec<&[u8]> = ct.chunks(size).collect();
        let maybe_key: Vec<u8> = (0..size)
            .map(|i| {
                let ct: Vec<u8> = chunks
                    .iter()
                    .filter_map(move |chunk| chunk.get(i))
                    .map(|&x| x)
                    .collect();
                decode_single_byte_xor(&ct).map(|d| d.key)
            }).flatten().collect();

    }
    println!("Key size: {:?}", sizes);
}
