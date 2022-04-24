use std::str::from_utf8;

use crate::set_one::freq::score_freqs;
use crate::set_one::dist::hamming_dist;


pub struct SingleByteXORDecode {
    pub pt: String,
    pub key: u8,
    pub score: f32
}


pub struct RepeatingXORDecode {
    pub pt: String,
    pub key: Vec<u8>,
    pub score: f32
}


pub struct DetectXORSettings {
    pairs: usize,
    maxlen: usize
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

pub fn decrypt_repeating_xor(ct: &[u8], key: &[u8]) -> Vec<u8> {
    encrypt_repeating_xor(ct, key)
}


pub fn detect_xor_keysizes(ct: &[u8], ds: &DetectXORSettings) -> Vec<usize>{
    let mut dist_pairs: Vec<(usize, f32)> = (2..ds.maxlen).map(|size| {
        let dist: u32 = (0..ds.pairs).map(|i| hamming_dist(
                &ct[i*size..(i + 1)*size],
                &ct[(i + 1)*size..(i + 2)*size]
            )).sum();
        (size, dist as f32 / (3 * size) as f32)
    }).collect();

    dist_pairs.sort_by(|&(_ , s1), &(_, s2)| {
        s1.partial_cmp(&s2).unwrap()
    });

    dist_pairs.into_iter().map(|(size, _)| size).collect()
}

pub fn decode_repeating_xor(ct: &[u8], ds: &DetectXORSettings) -> Option<RepeatingXORDecode> {
    let sizes = detect_xor_keysizes(ct, ds);

    let pt_key_pairs = sizes.into_iter().filter_map(|size| {
        let chunks: Vec<&[u8]> = ct.chunks(size).collect();

        let maybe_key: Option<Vec<u8>> = (0..size)
            .map(|i| {
                let ct: Vec<u8> = chunks
                    .iter()
                    .filter_map(move |chunk| chunk.get(i))
                    .copied()
                    .collect();
                decode_single_byte_xor(&ct).map(|d| d.key)
            }).collect();

        let maybe_pair = maybe_key.and_then(|key| {
            let pt_bytes: Vec<u8> = decrypt_repeating_xor(&ct, &key);
            
            let maybe_pt: Option<String> = from_utf8(&pt_bytes)
                .ok()
                .map(|text| text.to_owned());

            maybe_pt.map(|pt| (pt, key))
        });

        maybe_pair
    });

    let decodes = pt_key_pairs
        .map(|(pt, key)| {
            let score = score_freqs(&pt);
            RepeatingXORDecode {pt, key, score}
        });

    decodes.min_by(|d1, d2| {
        d1.score.partial_cmp(&d2.score).unwrap()
    })
}
