use hex;
use base64;

use crate::prompt::prompt_user;

pub fn hex_to_base64_exercise() {
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
