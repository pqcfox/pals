use crate::prompt::prompt_user;
use crate::set_one::xor::xor;

pub fn fixed_xor_exercise() {
    let hex_a = prompt_user("Enter first string: ");
    let a = hex::decode(&hex_a).expect("Invalid hex!");

    let hex_b = prompt_user("Enter second string: ");
    let b = hex::decode(&hex_b).expect("Invalid hex!");

    let hex_xor = hex::encode(xor(a, b));
    println!("Result: {}", hex_xor);
}
