use phf::{phf_map, Map};
use std::collections::HashMap;

static LETTER_FREQS: Map<char, f32> = phf_map! {
    'e' => 11.1607,
    'a' => 8.4966,
    'r' => 7.5809,
    'i' => 7.5448,
    'o' => 7.1635,
    't' => 6.9509,
    'n' => 6.6544,
    's' => 5.7351,
    'l' => 5.4893,
    'c' => 4.5388,
    'u' => 3.6308,
    'd' => 3.3844,
    'p' => 3.1617,
    'm' => 3.0129,
    'h' => 3.0034,
    'g' => 2.4705,
    'b' => 2.0720,
    'f' => 1.8121,
    'y' => 1.7779,
    'w' => 1.2899,
    'k' => 1.1016,
    'v' => 1.0074,
    'x' => 0.2902,
    'z' => 0.2722,
    'j' => 0.1965,
    'q' => 0.1962
};

pub fn score_freqs(text: &str) -> f32 {
    let text_freqs: HashMap<_, _> = LETTER_FREQS.keys()
        .map(|c| {
            let char_count = text.chars().filter(|x| x == c).count();
            let text_size = text.chars().count();
            let freq = 100.0 * char_count as f32 / text_size as f32;
            (c, freq)
        })
        .collect();

    LETTER_FREQS.keys()
        .map(|c| (text_freqs[c] - LETTER_FREQS[c]).abs())
        .sum()
}
