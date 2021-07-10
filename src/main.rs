mod primitives;
mod set1;

fn main() {
    // 1-1
    println!("====1-1====");
    let hex_string = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let b64_string = set1::convert_hex_string_to_b64_string(&hex_string);
    println!("{}", b64_string);
    println!();

    // 1-2
    println!("====1-2====");
    let hex_string_1 = String::from("1c0111001f010100061a024b53535009181c");
    let hex_string_2 = String::from("686974207468652062756c6c277320657965");
    let hex_string_xor = set1::xor_hex_strings(&hex_string_1, &hex_string_2);
    println!("{}", hex_string_xor);
    println!();

    // 1-3
    println!("====1-3====");
    let orig_string = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let orig_bytes = primitives::hex::hex_bytes_to_raw_bytes(orig_string.as_bytes());
    let (most_likely_string, _) = set1::most_likely_char_xor(orig_bytes.as_slice());
    println!("{}", most_likely_string);
    println!();

    // 1-4
    println!("====1-4====");
    let (original_string, decrypted_string, score, line) = set1::most_likely_string_from_file("material/4.txt");
    println!("Found decrypted string with score {} on line {}.", score, line);
    println!("Decrypted string:");
    println!("{}", decrypted_string);
    println!("Original string:");
    println!("{}", original_string);

    // 1-5
    println!("====1-4====");
    let orig_string = "Burning 'em, if you ain't quick and nimble\n\
                            I go crazy when I hear a cymbal";
    let encrypted_bytes = set1::encrypt_repeating_key_xor(orig_string, "ICE");
}