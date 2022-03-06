use std::io;
use std::io::Write;
use base64;
use hex;

pub fn get_problem_fns() -> Vec<fn()> {
    vec![
        hex_to_base64_exercise
    ]
}

fn hex_to_base64_exercise() {
    print!("Enter data: ");
    io::stdout().flush().unwrap();

    let mut hex_input = String::new();
    io::stdin()
        .read_line(&mut hex_input)
        .expect("Failed to read line");

    if let Ok(base64_output) = hex_to_base64(hex_input.trim()) {
        println!("Result: {}", base64_output);
    } else {
        println!("Invalid input.");
    }
}

fn hex_to_base64(data: &str) -> Result<String, hex::FromHexError> {
    let decoded_data = hex::decode(data)?;
    Ok(base64::encode(decoded_data))
}
