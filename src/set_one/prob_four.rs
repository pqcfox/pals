use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::prompt::prompt_user;

pub fn detect_single_character_xor_exercise() {
    let filename = prompt_user("Enter ciphertext filename: ");
    let file = File::open(filename).expect("Can't open file!");
    let reader = BufReader::new(file);
    
    /*
    let cts: Vec<u8> = reader.lines()
        .filter_map(|line| hex::decode(&line).ok())
        .collect();
    */
}
