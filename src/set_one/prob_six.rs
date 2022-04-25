use std::fs;

use crate::prompt::prompt_user;
use crate::set_one::xor::{decode_repeating_xor, DetectXORSettings};

pub fn break_repeating_key_xor_exercise() {
    let filename = prompt_user("Enter ciphertext filename: ");
    let ct_base64: String = fs::read_to_string(filename)
        .expect("Can't read file!")
        .lines()
        .collect();
    let ct = base64::decode(ct_base64)
        .expect("Invalid base64 encoding!");

    let ds = DetectXORSettings {
        pairs: 2,
        maxlen: 40
    };

    if let Some(decode) = decode_repeating_xor(&ct, &ds) {
        println!("Result: {}", decode.pt);
    } else { 
        println!("No plaintext found!");
    }
}
