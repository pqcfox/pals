use crate::set_one::freq::score_freqs;
use std::str::from_utf8;

pub fn xor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    a.iter().zip(b.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

pub fn decode_single_byte_xor(ciphertext: Vec<u8>) -> Option<String>{
    (0..=255)
        .filter_map(|x| {
            let plaintext: Vec<u8> = ciphertext.iter()
                .map(|y| x ^ y)
                .collect();
            from_utf8(&plaintext).ok().map(|text| text.to_owned())
        })
        .min_by(|x, y| {
            let x_score = score_freqs(&x);
            let y_score = score_freqs(&y);
            x_score.partial_cmp(&y_score).unwrap()
        })
}
