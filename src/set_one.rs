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
    let hex_b = prompt_user("Enter second string: ");

    // if let (Ok(a), Ok(b)) = (hex::decode(hex_a), &hex_b) {
    // }
}

fn hex_to_base64(data: &str) -> Result<String, hex::FromHexError> {
    let decoded_data = hex::decode(data)?;
    Ok(base64::encode(decoded_data))
}


