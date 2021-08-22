fn main() {
    let s: &str = "YELLOW SUBMARINE";
    println!("Input: \"{}\"", s);
    let s: &[u8] = s.as_bytes();
    let mut s: Vec<u8> = s.to_vec();
    pkcs7_pad(&mut s, 20);
    let s: String = String::from_utf8(s).unwrap();
    println!("Result: \"{}\"", s.escape_debug());
}

fn pkcs7_pad(buffer: &mut Vec<u8>, blocksize: u8) {
    let p = blocksize - ((buffer.len() % blocksize as usize) as u8);
    for _ in 0..p {
        buffer.push(p)
    }
}