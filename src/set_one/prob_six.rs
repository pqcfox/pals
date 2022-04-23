use std::fs;

use crate::prompt::prompt_user;
use crate::set_one::xor::detect_xor_keysizes;

pub fn break_repeating_key_xor_exercise() {
    let filename = prompt_user("Enter ciphertext filename: ");
    let ct_base64: String = fs::read_to_string(filename)
        .expect("Can't read file!")
        .lines()
        .collect();
    let ct = base64::decode(ct_base64)
        .expect("Invalid base64 encoding!");
    let sizes = detect_xor_keysizes(&ct);
    println!("Key size: {:?}", sizes);
}
