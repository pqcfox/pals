mod prob_one;
mod prob_two;
mod prob_three;

pub fn get_problem_fns() -> Vec<fn()> {
    vec![
        prob_one::hex_to_base64_exercise,
        prob_two::fixed_xor_exercise,
        prob_three::single_byte_xor_cipher_exercise
    ]
}
