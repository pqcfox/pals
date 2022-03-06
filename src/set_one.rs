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

    if let Ok(base64_output) = hex_to_base64(hex_input.trim()) {
        println!("Result: {}", base64_output);
    } else {
        println!("Invalid input.");
    }
}

fn fixed_xor_exercise() {
    
}

fn hex_to_base64(data: &str) -> Result<String, hex::FromHexError> {
    let decoded_data = hex::decode(data)?;
    Ok(base64::encode(decoded_data))
}
