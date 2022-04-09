use crate::set_one::freq::score_freqs;
use std::str::from_utf8;

pub fn xor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    a.iter().zip(b.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

pub fn decode_single_byte_xor(ct: Vec<u8>) -> Option<String>{
    (0..=255)
        .filter_map(|x| {
            let pt: Vec<u8> = ct.iter()
                .map(|y| x ^ y)
                .collect();

            from_utf8(&pt).ok().map(|text| text.to_owned())
        })
        .min_by(|x, y| {
            score_freqs(&x).partial_cmp(&score_freqs(&y)).unwrap()
        })
}
