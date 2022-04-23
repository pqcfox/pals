pub fn hamming_dist(a: &[u8], b: &[u8]) -> u32{
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (0..7)
            .map(|s| ((x ^ y) >> s) & 0x1)
            .sum::<u8>() as u32)
        .sum()
}
