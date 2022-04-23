use crate::set_one::dist::hamming_dist;

pub fn break_repeating_key_xor_exercise() {
    let dist: u32 = hamming_dist("this is a test".as_bytes(), "wokka wokka!!!".as_bytes());
    println!("{}", dist);
}
