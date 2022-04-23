use std::fs;

use crate::prompt::prompt_user;
use crate::set_one::xor::find_single_byte_xor;

pub fn detect_single_character_xor_exercise() {
    let filename = prompt_user("Enter ciphertext filename: ");
    let cts: Vec<Vec<u8>> = fs::read_to_string(filename)
        .expect("Can't read file!")
        .lines()
        .filter_map(|line| hex::decode(&line).ok())
        .collect();

    if let Some(pt) = find_single_byte_xor(cts) {
        println!("Result: {}", pt);
    } else {
        println!("No plaintext found!");
    }
}
