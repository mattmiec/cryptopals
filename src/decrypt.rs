use super::primitives::xor;

pub fn crack_key(bytes: &[u8], keylen: usize) -> Vec<u8> {
    let mut key = Vec::with_capacity(keylen);
    for i in 0..keylen {
        let mut sub = Vec::new();
        let mut ii = i;
        while ii < bytes.len() {
            sub.push(bytes[ii]);
            ii += keylen;
        }
        let (_, _, k) = most_likely_char_xor(&sub);
        key.push(k);
    }
    return key;
}

pub fn most_likely_key_len(bytes: &[u8]) -> usize {
    let mut min_hamming = 8.0;
    let mut best_key_len = 1;
    for k in 2..50 {
        let mut hamming_distances = Vec::new();
        for i in 0..bytes.len() / (2 * k) {
            if i + (2 * k) <= bytes.len() {
                hamming_distances.push(hamming_distance(&bytes[i..i+k], &bytes[i+k..i+(2*k)]) as f64 / k as f64);
            }
        }
        let mut sum = 0.0;
        for dist in &hamming_distances {
            sum += dist;
        }
        let dist = sum / hamming_distances.len() as f64;
        if dist < min_hamming {
            min_hamming = dist;
            best_key_len = k;
        }
    }
    return best_key_len;
}

pub fn most_likely_char_xor(bytes: &[u8]) -> (String, i32, u8) {
    let mut best_score = -1;
    let mut best_string = String::new();
    let mut best_key = 0;
    for key in 0..=255 {
        let flipped_bytes = xor::xor_byte_slice_fixed(bytes, key);
        let s = String::from_utf8(flipped_bytes);
        let s = match s {
            Ok(val) => val,
            Err(_) => continue,
        };
        let score = score_string(&s);
        if score > best_score {
            best_score = score;
            best_string = s;
            best_key = key;
        }
    }
    return (best_string, best_score, best_key);
}

pub fn score_string(s: &str) -> i32 {
    let mut score = 0;
    for c in s.chars() {
        if c.is_alphabetic() || c == ' ' {
            score += 20;
        }
        match c {
            ' ' => score += 10,
            'a' | 'A' => score += 8,
            'b' | 'B' => score += 2,
            'c' | 'C' => score += 3,
            'd' | 'D' => score += 4,
            'e' | 'E' => score += 13,
            'f' | 'F' => score += 2,
            'g' | 'G' => score += 2,
            'h' | 'H' => score += 6,
            'i' | 'I' => score += 7,
            'j' | 'J' => score += 0,
            'k' | 'K' => score += 1,
            'l' | 'L' => score += 4,
            'm' | 'M' => score += 2,
            'n' | 'N' => score += 7,
            'o' | 'O' => score += 8,
            'p' | 'P' => score += 2,
            'q' | 'Q' => score += 0,
            'r' | 'R' => score += 6,
            's' | 'S' => score += 6,
            't' | 'T' => score += 9,
            'u' | 'U' => score += 3,
            'v' | 'V' => score += 1,
            'w' | 'W' => score += 2,
            'x' | 'X' => score += 0,
            'y' | 'Y' => score += 2,
            'z' | 'Z' => score += 0,
            _ => (),
        }
    }
    return score;
}

pub fn hamming_distance(bytes1: &[u8], bytes2: &[u8]) -> i32 {
    if bytes1.len() != bytes2.len() {
        panic!("Input byte strings must be equal length!");
    }

    let mut distance = 0;

    for i in 0..bytes1.len() {
        let diff = bytes1[i] ^ bytes2[i];
        for j in 0..8 {
            if ((diff >> j) & 0b0000_0001) == 1 {
                distance += 1;
            }
        }
    }

    return distance;
}