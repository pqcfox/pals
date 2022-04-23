use std::fs;

use crate::prompt::prompt_user;

pub fn implement_repeating_key_xor_exercise() {
    let filename = prompt_user("Enter plaintext filename: ");
    let pt: Vec<u8> = fs::read_to_string().expect("Can't read file!");
    let ct = encrypt_repeating_xor(pt, base64::encode("ICE"));
    println!("Result: {}", hex::encode(ct));
}
