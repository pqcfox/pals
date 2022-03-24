use hex;
use std::str::from_utf8;

use crate::prompt::prompt_user;
use crate::freq::score_freqs;

pub fn single_byte_xor_cipher_exercise() {
    let hex_ciphertext = prompt_user("Enter ciphertext: ");
    let ciphertext = match hex::decode(&hex_ciphertext) {
        Ok(text) => text,
        Err(_) => {
            println!("Invalid ciphertext!");
            std::process::exit(1)
        }
    };

    let plaintext: Option<String> = (0..=255)
        .filter_map(|x| {
            let plaintext: Vec<u8> = ciphertext.iter()
                .map(|y| x ^ y)
                .collect();

            from_utf8(&plaintext).ok().map(|text| text.to_owned())
        })
        .min_by(|x, y| {
            let x_score = score_freqs(&x);
            let y_score = score_freqs(&y);
            x_score.partial_cmp(&y_score).unwrap()
        });

    match plaintext {
        Some(text) => println!("Result: {}", text),
        None => println!("No plaintext found!")
    }
}
