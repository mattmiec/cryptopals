use crate::primitives::hex::hex_bytes_to_raw_bytes;
use crate::primitives::hex::raw_bytes_to_hex_bytes;

use super::primitives::hex;
use super::primitives::base64;

pub fn convert_hex_string_to_b64_string(hex_string: &String) -> String {

    let raw_bytes = hex::hex_bytes_to_raw_bytes(hex_string.as_bytes());
    let b64_bytes = base64::raw_bytes_to_b64_bytes(&raw_bytes[..]);
    let b64_string = String::from_utf8(b64_bytes).expect("failed to convert b64_bytes to string!");
    return b64_string;
}

pub fn xor_hex_strings(string_1: &str, string_2: &str) -> String {
    let raw_bytes_1 = hex_bytes_to_raw_bytes(string_1.as_bytes());
    let raw_bytes_2 = hex_bytes_to_raw_bytes(string_2.as_bytes());
    let mut raw_bytes_xor = Vec::with_capacity(raw_bytes_1.len());
    if raw_bytes_1.len() != raw_bytes_2.len() {
        panic!("bytes to xor must have equal length")
    }
    let mut i = 0;
    while i < raw_bytes_1.len() {
        raw_bytes_xor.push(raw_bytes_1[i] ^ raw_bytes_2[i]);
        i += 1;
    }

    return String::from_utf8(raw_bytes_to_hex_bytes(&raw_bytes_xor[..])).expect("failed to convert hex_bytes to string!");
}