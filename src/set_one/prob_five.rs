use std::fs;

use crate::prompt::prompt_user;
use crate::set_one::xor::encrypt_repeating_xor;

pub fn implement_repeating_key_xor_exercise() {
    let filename = prompt_user("Enter plaintext filename: ");
    let pt_str = fs::read_to_string(filename)
        .expect("Can't read file!");

    let pt = pt_str.trim().as_bytes();
    let key = "ICE".as_bytes();
    
    let ct = encrypt_repeating_xor(pt, key);
    println!("Result: {}", hex::encode(ct));
}
