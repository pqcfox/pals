use std::fs;
use std::str;
use openssl::symm;

use crate::prompt::prompt_user;

pub fn aes_in_ecb_mode_exercise() {
    let filename = prompt_user("Enter ciphertext filename: ");
    let ct_base64: String = fs::read_to_string(filename)
        .expect("Can't read file!")
        .lines()
        .collect();
    let ct = base64::decode(ct_base64)
        .expect("Invalid base64 encoding!");

    let key = b"YELLOW SUBMARINE";
    let maybe_pt = symm::decrypt(symm::Cipher::aes_128_ecb(), key, None, &ct);

    if let Ok(pt_bytes) = maybe_pt {
        let pt = str::from_utf8(&pt_bytes).expect("Invalid UTF8 in plaintext!");
        println!("Result: {}", pt);
    } else {
        println!("Invalid ciphertext!");
    }
}
