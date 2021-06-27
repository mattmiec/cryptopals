mod primitives;
mod set1;

fn main() {
    // 1-1
    let hex_string = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let b64_string = set1::convert_hex_string_to_b64_string(&hex_string);
    println!("{}", b64_string);

    // 2-2
    let hex_string_1 = String::from("1c0111001f010100061a024b53535009181c");
    let hex_string_2 = String::from("686974207468652062756c6c277320657965");
    let hex_string_xor = set1::xor_hex_strings(&hex_string_1, &hex_string_2);
    println!("{}", hex_string_xor);

}