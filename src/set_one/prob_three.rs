use hex;

use crate::prompt::prompt_user;
use crate::set_one::xor::decode_single_byte_xor;

pub fn single_byte_xor_cipher_exercise() {
    let hex_ct = prompt_user("Enter ciphertext: ");
    let ct = hex::decode(&hex_ct).expect("Invalid hex!");

    match decode_single_byte_xor(ct) {
        Some(text) => println!("Result: {}", text),
        None => println!("No plaintext found!")
    }
}
