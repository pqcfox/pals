use base64;
use hex;

use crate::prompt::prompt_user;

pub fn get_problem_fns() -> Vec<fn()> {
    vec![
        hex_to_base64_exercise,
        fixed_xor_exercise
    ]
}

fn hex_to_base64_exercise() {
    let hex_input = prompt_user("Enter data: ");

    if let Ok(base64_output) = hex_to_base64(&hex_input) {
        println!("Result: {}", base64_output);
    } else {
        println!("Invalid input.");
    }
}

fn fixed_xor_exercise() {
    let hex_a = prompt_user("Enter first string: ");
    let a = match hex::decode(&hex_a) {
        Ok(a) => a,
        Err(_) => {
            println!("Invalid first string!");
            std::process::exit(1);
        }
    };

    let hex_b = prompt_user("Enter second string: ");
    let b = match hex::decode(&hex_b) {
        Ok(b) => b,
        Err(_) => {
            println!("Invalid second string!");
            std::process::exit(1);
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

fn hex_to_base64(data: &str) -> Result<String, hex::FromHexError> {
    let decoded_data = hex::decode(data)?;
    Ok(base64::encode(decoded_data))
}


