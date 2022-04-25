mod prob_one;
mod prob_two;
mod prob_three;
mod prob_four;
mod prob_five;
mod prob_six;
mod prob_seven;

mod xor;
mod freq;
mod dist;

pub fn get_problem_fns() -> Vec<fn()> {
    vec![
        prob_one::hex_to_base64_exercise,
        prob_two::fixed_xor_exercise,
        prob_three::single_byte_xor_cipher_exercise,
        prob_four::detect_single_character_xor_exercise,
        prob_five::implement_repeating_key_xor_exercise,
        prob_six::break_repeating_key_xor_exercise,
        prob_seven::aes_in_ecb_mode_exercise
    ]
}
