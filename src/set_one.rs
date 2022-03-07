use base64;
use hex;
use std::str::from_utf8;

use crate::prompt::prompt_user;
use crate::freq::score_freqs;

pub fn get_problem_fns() -> Vec<fn()> {
    vec![
        hex_to_base64_exercise,
        fixed_xor_exercise,
        single_byte_xor_cipher_exercise
    ]
}

fn hex_to_base64_exercise() {
    let hex_input = prompt_user("Enter data: ");
    let input = match hex::decode(&hex_input) {
        Ok(input) => input,
        Err(_) => {
            println!("Invalid hex string!");
            std::process::exit(1)
        }
    };

    let base64_output = base64::encode(input);
    println!("Result: {}", base64_output);
}

fn fixed_xor_exercise() {
    let hex_a = prompt_user("Enter first string: ");
    let a = match hex::decode(&hex_a) {
        Ok(a) => a,
        Err(_) => {
            println!("Invalid first string!");
            std::process::exit(1)
        }
    };

    let hex_b = prompt_user("Enter second string: ");
    let b = match hex::decode(&hex_b) {
        Ok(b) => b,
        Err(_) => {
            println!("Invalid second string!");
            std::process::exit(1)
        }
    };

    let xor: Vec<u8> = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| a ^ b)
        .collect();

    let hex_xor = hex::encode(xor);
    println!("Result: {}", hex_xor);
}


fn single_byte_xor_cipher_exercise() {
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
