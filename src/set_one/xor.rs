use std::str::from_utf8;

use crate::set_one::freq::score_freqs;
use crate::set_one::dist::hamming_dist;

pub struct SingleByteXORDecode {
    pub pt: String,
    pub key: u8,
    pub score: f32
}

pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}


// single byte XOR

pub fn decode_single_byte_xor(ct: &[u8]) -> Option<SingleByteXORDecode> {
    let pt_key_pairs = (0..=255)
        .filter_map(|key| {
            let pt_bytes: Vec<u8> = ct.iter()
                .map(|x| key ^ x)
                .collect();

            let pt = from_utf8(&pt_bytes)
                .ok()
                .map(|text| text.to_owned());

            pt.map(|pt| (pt, key))
        });

    let decodes = pt_key_pairs
        .map(|(pt, key)| {
            let score = score_freqs(&pt);
            SingleByteXORDecode {pt, key, score}
        });

    decodes.min_by(|d1, d2| {
        d1.score.partial_cmp(&d2.score).unwrap()
    })
}

pub fn find_single_byte_xor(cts: &[Vec<u8>]) -> Option<(Vec<u8>, SingleByteXORDecode)>{
    let decode_pairs = cts
        .iter()
        .filter_map(|ct| decode_single_byte_xor(ct).map(|d| (ct, d)));

    decode_pairs.min_by(|(_, d1), (_, d2)| {
        d1.score.partial_cmp(&d2.score).unwrap()
    }).map(|(ct, d)| (ct.to_owned(), d))
}


// repeating XOR

pub fn encrypt_repeating_xor(pt: &[u8], key: &[u8]) -> Vec<u8> {
    pt.iter()
        .zip(key.iter().cycle())
        .map(|(x, y)| x ^ y)
        .collect()
}

pub fn decrypt_repeating_xor(pt: &[u8], key: &[u8]) -> Vec<u8> {
    encrypt_repeating_xor(pt, key)
}


pub fn detect_xor_keysizes(ct: &[u8]) -> Vec<usize>{
    let mut dist_pairs: Vec<(usize, u32)> = (2..40).map(|size| {
        let dist = hamming_dist(&ct[0..size], &ct[size..2*size]);
        (size, dist)
    }).collect();

    dist_pairs.sort_by(|&(s1, d1), &(s2, d2)| {
        let r1 = d1 as f32 / s1 as f32;
        let r2 = d2 as f32 / s2 as f32;
        r1.partial_cmp(&r2).unwrap()
    });

    dist_pairs.into_iter().map(|(size, _)| size).collect()
}
