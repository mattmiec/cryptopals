mod base64;

fn main() {
    // 1-1
    let hex_string = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let b64_string = base64::convert_hex_string_to_b64_string(&hex_string);
    println!("{}", b64_string);
}