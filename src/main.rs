mod primitives;
mod decrypt;
mod util;

use std::fs::File;
use std::io::{BufReader, prelude::*};

use openssl::symm::{decrypt, Cipher};


fn main() {
    // 1-1
    println!("====1-1====");
    let hex_string = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("Input hex string: {}", hex_string);
    let raw_bytes = primitives::hex::hex_bytes_to_raw_bytes(hex_string.as_bytes());
    let b64_bytes = primitives::base64::raw_bytes_to_b64_bytes(&raw_bytes[..]);
    let b64_string = String::from_utf8(b64_bytes).expect("failed to convert b64_bytes to string!");
    println!("Input converted to base 64 string: {}", b64_string);
    let raw_bytes = primitives::base64::b64_bytes_to_raw_bytes(b64_string.as_bytes());
    let hex_bytes = primitives::hex::raw_bytes_to_hex_bytes(&raw_bytes);
    let hex_string = String::from_utf8(hex_bytes).expect("Failed to converted hex bytes to string!");
    println!("Base 64 converted back to hex string: {}", hex_string);
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
    let (most_likely_string, _, _) = decrypt::most_likely_char_xor(orig_bytes.as_slice());
    println!("{}", most_likely_string);
    println!();

    // 1-4
    println!("====1-4====");
    let filename = "material/4.txt";
    let file = File::open(filename).expect("unable to open file!");
    let reader = BufReader::new(file);

    let mut best_score = -1;
    let mut decrypted_string = String::new();
    let mut l = 0;
    let mut key = 0;
    for line in reader.lines() {
        let s = line.expect("problem reading line");
        let xored_bytes = primitives::hex::hex_bytes_to_raw_bytes(s.as_bytes());
        let (most_likely_string, score, k) = decrypt::most_likely_char_xor(xored_bytes.as_slice());
        if score > best_score {
            best_score = score;
            decrypted_string = most_likely_string;
            key = k;
        }
        l += 1;
    }
    println!("Found decrypted string with score {} on line {} with key {}.", best_score, l, key);
    println!("Decrypted string:");
    println!("{}", decrypted_string);
    println!();

    // 1-5
    println!("====1-5====");
    let orig_string = "Burning 'em, if you ain't quick and nimble\n\
                            I go crazy when I hear a cymbal";
    let key = "ICE";
    let ciphertext_bytes = primitives::xor::xor_bytes_repeating_key(orig_string.as_bytes(), key.as_bytes());
    let ciphertext_hex_bytes = primitives::hex::raw_bytes_to_hex_bytes(&ciphertext_bytes);
    let ciphertext_string = String::from_utf8(ciphertext_hex_bytes).expect("Failed to encode ciphertext as hex string!");
    println!("Encrypted string: {}", ciphertext_string);
    println!();


    // 1-6
    println!("====1-6====");
    let s1 = "this is a test";
    let s2 = "wokka wokka!!!";
    println!("Hamming distance between '{}' and '{}' is {}", s1, s2, decrypt::hamming_distance(s1.as_bytes(), s2.as_bytes()));

    let ciphertext_b64 = util::file_to_string_strip_newlines("material/6.txt");
    let ciphertext_bytes = primitives::base64::b64_bytes_to_raw_bytes(ciphertext_b64.as_bytes());
    let key_len = decrypt::most_likely_key_len(&ciphertext_bytes);
    println!("Best key length: {}", key_len);
    let key = decrypt::crack_key(&ciphertext_bytes, key_len);
    println!("Key: {}", String::from_utf8(key.clone()).expect("Cannot interpret key as utf8!"));
    let plaintext_bytes = primitives::xor::xor_bytes_repeating_key(&ciphertext_bytes, &key);
    let plaintext_string = String::from_utf8(plaintext_bytes).expect("Failed to interpet plaintext as utf8!");
    println!("Decrypted message:");
    println!("{}", plaintext_string);
    println!();

    // 1-7
    println!("====1-7====");
    let ciphertext_b64 = util::file_to_string_strip_newlines("material/7.txt");
    let ciphertext_bytes = primitives::base64::b64_bytes_to_raw_bytes(ciphertext_b64.as_bytes());
    let key = b"YELLOW SUBMARINE";
    let cipher = Cipher::aes_128_ecb();
    let plaintext = decrypt(cipher, key, Option::None, &ciphertext_bytes).unwrap();
    println!("{}", String::from_utf8(plaintext).unwrap());
    println!();
}