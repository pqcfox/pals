use crate::set_one::freq::score_freqs;
use std::str::from_utf8;

pub fn xor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
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

pub fn find_single_byte_xor(cts: Vec<Vec<u8>>) -> Option<String>{
    let score_pairs: Vec<(String, f32)> = cts
        .iter()
        .filter_map(|ct| decode_single_byte_xor(ct))
        .collect();

    score_pairs.into_iter().min_by(|(_, x), (_, y)| {
        x.partial_cmp(y).unwrap()
    }).map(|(ct, _)| ct)
}
