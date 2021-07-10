use super::primitives::hex;
use super::primitives::base64;

use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn convert_hex_string_to_b64_string(hex_string: &String) -> String {

    let raw_bytes = hex::hex_bytes_to_raw_bytes(hex_string.as_bytes());
    let b64_bytes = base64::raw_bytes_to_b64_bytes(&raw_bytes[..]);
    let b64_string = String::from_utf8(b64_bytes).expect("failed to convert b64_bytes to string!");
    return b64_string;
}

pub fn xor_hex_strings(string_1: &str, string_2: &str) -> String {
    let raw_bytes_1 = hex::hex_bytes_to_raw_bytes(string_1.as_bytes());
    let raw_bytes_2 = hex::hex_bytes_to_raw_bytes(string_2.as_bytes());
    let raw_bytes_xor = xor_byte_slices(raw_bytes_1.as_slice(), raw_bytes_2.as_slice());

    return String::from_utf8(hex::raw_bytes_to_hex_bytes(&raw_bytes_xor[..])).expect("failed to convert hex_bytes to string!");
}

pub fn most_likely_char_xor(bytes: &[u8]) -> (String, i32) {
    let mut best_score = -1;
    let mut best_string = String::new();
    for key in 0..=255 {
        let flipped_bytes = xor_byte_slice_fixed(bytes, key);
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

pub fn most_likely_string_from_file(filename: &str) -> (String, String, i32, i32) {
    let file = File::open(filename).expect("unable to open file!");
    let reader = BufReader::new(file);

    let mut best_score = -1;
    let mut original_string = String::new();
    let mut decrypted_string = String::new();
    let mut l = 0;
    for line in reader.lines() {
        let s = line.expect("problem reading line");
        let xored_bytes = hex::hex_bytes_to_raw_bytes(s.as_bytes());
        let (most_likely_string, score) = most_likely_char_xor(xored_bytes.as_slice());
        if score > best_score {
            original_string = s;
            best_score = score;
            decrypted_string = most_likely_string;
        }
        l += 1;
    }

    return (original_string, decrypted_string, best_score, l);
}

fn xor_byte_slices(raw_bytes_1: &[u8], raw_bytes_2: &[u8]) -> Vec<u8> {
    let mut raw_bytes_xor = Vec::with_capacity(raw_bytes_1.len());
    if raw_bytes_1.len() != raw_bytes_2.len() {
        panic!("bytes to xor must have equal length")
    }
    let mut i = 0;
    while i < raw_bytes_1.len() {
        raw_bytes_xor.push(raw_bytes_1[i] ^ raw_bytes_2[i]);
        i += 1;
    }
    return raw_bytes_xor;
}

fn xor_byte_slice_fixed(raw_bytes: &[u8], key: u8) -> Vec<u8> {
    let mut raw_bytes_xor = Vec::with_capacity(raw_bytes.len());
    let mut i = 0;
    while i < raw_bytes.len() {
        raw_bytes_xor.push(raw_bytes[i] ^ key);
        i += 1;
    }
    return raw_bytes_xor;
}

fn score_string(s: &str) -> i32 {
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