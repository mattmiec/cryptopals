use super::primitives::xor;

pub fn most_likely_char_xor(bytes: &[u8]) -> (String, i32) {
    let mut best_score = -1;
    let mut best_string = String::new();
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
        }
    }
    return (best_string, best_score);
}

pub fn score_string(s: &str) -> i32 {
    let mut score = 0;
    for c in s.chars() {
        if c.is_alphabetic() {
            score += 20;
        }
        match c {
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