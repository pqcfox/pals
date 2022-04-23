use crate::set_one::freq::score_freqs;
use std::str::from_utf8;

pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

pub fn decode_single_byte_xor(ct: &[u8]) -> Option<(String, f32)> {
    let pts = (0..=255)
        .filter_map(|x| {
            let pt_bytes: Vec<u8> = ct.iter()
                .map(|y| x ^ y)
                .collect();

            from_utf8(&pt_bytes)
                .ok()
                .map(|text| text.to_owned())
        });

    let score_pairs = pts
        .map(|pt| {
            let score = score_freqs(&pt);
            (pt, score) 
        });

    score_pairs.min_by(|(_, x), (_, y)| {
        x.partial_cmp(y).unwrap()
    })
}

pub fn find_single_byte_xor(cts: &[Vec<u8>]) -> Option<String>{
    let score_pairs = cts
        .iter()
        .filter_map(|ct| decode_single_byte_xor(ct));

    score_pairs.min_by(|(_, x), (_, y)| {
        x.partial_cmp(y).unwrap()
    }).map(|(ct, _)| ct)
}

pub fn encrypt_repeating_xor(pt: &[u8], key: &[u8]) -> Vec<u8> {
    pt.iter()
        .zip(key.iter().cycle())
        .map(|(x, y)| x ^ y)
        .collect()
}
