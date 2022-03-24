use hex;

use crate::prompt::prompt_user;

pub fn fixed_xor_exercise() {
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
