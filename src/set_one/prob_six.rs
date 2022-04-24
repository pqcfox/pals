use std::fs;
use std::str;

use crate::prompt::prompt_user;
use crate::set_one::dist::hamming_dist;
use crate::set_one::xor::{detect_xor_keysizes, decode_single_byte_xor,
    decrypt_repeating_xor, DetectXORSettings};

pub fn break_repeating_key_xor_exercise() {
    let filename = prompt_user("Enter ciphertext filename: ");
    let ct_base64: String = fs::read_to_string(filename)
        .expect("Can't read file!")
        .lines()
        .collect();
    let ct = base64::decode(ct_base64)
        .expect("Invalid base64 encoding!");

    let ds = DetectXORSettings {
        pairs: 10,
        maxlen: 40
    };

    let sizes = detect_xor_keysizes(&ct, 40, 10);


    // for &size in sizes[0..5].iter() {
    for &size in sizes.iter() {
        let chunks: Vec<&[u8]> = ct.chunks(size).collect();
        let maybe_key: Option<Vec<u8>> = (0..size)
            .map(|i| {
                let ct: Vec<u8> = chunks
                    .iter()
                    .filter_map(move |chunk| chunk.get(i))
                    .copied()
                    .collect();
                decode_single_byte_xor(&ct).map(|d| d.key)
            }).collect();

        if let Some(key) = maybe_key {
            let pt = decrypt_repeating_xor(&ct, &key);
            println!("{}", size);
            println!("{:?}", str::from_utf8(&key).unwrap());
            println!("{:?}", str::from_utf8(&pt));
            println!("{}", hamming_dist(&pt[0..size], &pt[size..2*size]));
            println!("\n\n\n\n");
        }
    }

    println!("Key size: {:?}", sizes);
}
