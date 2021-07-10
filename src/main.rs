mod primitives;
mod decrypt;

use std::fs::File;
use std::io::{BufReader, prelude::*};

fn main() {
    // 1-1
    println!("====1-1====");
    let hex_string = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let raw_bytes = primitives::hex::hex_bytes_to_raw_bytes(hex_string.as_bytes());
    let b64_bytes = primitives::base64::raw_bytes_to_b64_bytes(&raw_bytes[..]);
    let b64_string = String::from_utf8(b64_bytes).expect("failed to convert b64_bytes to string!");
    println!("{}", b64_string);
    println!();

    // 1-2
    println!("====1-2====");
    let hex_string_1 = String::from("1c0111001f010100061a024b53535009181c");
    let hex_string_2 = String::from("686974207468652062756c6c277320657965");
    let raw_bytes_1 = primitives::hex::hex_bytes_to_raw_bytes(hex_string_1.as_bytes());
    let raw_bytes_2 = primitives::hex::hex_bytes_to_raw_bytes(hex_string_2.as_bytes());
    let raw_bytes_xor = primitives::xor::xor_byte_slices(raw_bytes_1.as_slice(), raw_bytes_2.as_slice());

    let hex_string_xor = String::from_utf8(primitives::hex::raw_bytes_to_hex_bytes(&raw_bytes_xor[..])).expect("failed to convert hex_bytes to string!");
    println!("{}", hex_string_xor);
    println!();

    // 1-3
    println!("====1-3====");
    let orig_string = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let orig_bytes = primitives::hex::hex_bytes_to_raw_bytes(orig_string.as_bytes());
    let (most_likely_string, _) = decrypt::most_likely_char_xor(orig_bytes.as_slice());
    println!("{}", most_likely_string);
    println!();

    // 1-4
    println!("====1-4====");
    let filename = "material/4.txt";
    let file = File::open(filename).expect("unable to open file!");
    let reader = BufReader::new(file);

    let mut best_score = -1;
    let mut original_string = String::new();
    let mut decrypted_string = String::new();
    let mut l = 0;
    for line in reader.lines() {
        let s = line.expect("problem reading line");
        let xored_bytes = primitives::hex::hex_bytes_to_raw_bytes(s.as_bytes());
        let (most_likely_string, score) = decrypt::most_likely_char_xor(xored_bytes.as_slice());
        if score > best_score {
            original_string = s;
            best_score = score;
            decrypted_string = most_likely_string;
        }
        l += 1;
    }
    println!("Found decrypted string with score {} on line {}.", best_score, l);
    println!("Decrypted string:");
    println!("{}", decrypted_string);
    println!("Original string:");
    println!("{}", original_string);
    println!();

    // 1-5
    println!("====1-5====");
    let orig_string = "Burning 'em, if you ain't quick and nimble\n\
                            I go crazy when I hear a cymbal";
    let key = "ICE";
    let ciphertext_bytes = primitives::xor::encrypt_bytes_repeating_key_xor(orig_string.as_bytes(), key.as_bytes());
    let ciphertext_hex_bytes = primitives::hex::raw_bytes_to_hex_bytes(&ciphertext_bytes);
    let ciphertext_string = String::from_utf8(ciphertext_hex_bytes).expect("Failed to encode ciphertext as hex string!");
    println!("Encrypted string: {}", ciphertext_string);
}