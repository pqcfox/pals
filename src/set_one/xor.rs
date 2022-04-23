use std::str::from_utf8;

use crate::set_one::freq::score_freqs;
use crate::set_one::dist::hamming_dist;

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

pub fn detect_xor_keysizes(ct: &[u8]) -> Vec<usize>{
    let mut dist_pairs: Vec<(usize, u32)> = (2..40).map(|size| {
        let dist = hamming_dist(&ct[0..size], &ct[size..2*size]);
        (size, dist)
    }).collect();

    dist_pairs.sort_by(|&(s1, d1), &(s2, d2)| {
        let r1 = d1 as f32 / s1 as f32;
        let r2 = d2 as f32/ s2 as f32;
        r1.partial_cmp(&r2).unwrap()
    });

    dist_pairs.into_iter().map(|(size, _)| size).collect()
}
