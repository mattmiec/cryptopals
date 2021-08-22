pub fn hex_bytes_to_raw_bytes(hex_bytes: &[u8]) -> Vec<u8> {
    if hex_bytes.len() % 2 != 0 {
        panic!("hex bytes must be in pairs of two");
    }
    let mut ret: Vec<u8> = Vec::with_capacity(hex_bytes.len()/2);
    let mut i = 0;
    while i + 1 < hex_bytes.len() {
        let s0 = hex_bytes[i];
        let s1 = hex_bytes[i + 1];
        let val = hex_byte_to_val_byte(s0) * 16 + hex_byte_to_val_byte(s1);
        ret.push(val);
        i += 2;
    }
    return ret;
}

pub fn raw_bytes_to_hex_bytes(raw_bytes: &[u8]) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(raw_bytes.len()*2);
    for byte in raw_bytes {
        ret.push(val_byte_to_hex_byte(byte >> 4));
        ret.push(val_byte_to_hex_byte(byte & 0b00001111));
    }
    return ret;
}

// assumes lower case a-f
fn hex_byte_to_val_byte(hex: u8) -> u8 {
    if hex >= 48 && hex < 58 {
        return hex - 48;
    }
    if hex >= 97 && hex < 123 {
        return hex - 87;
    }
    eprintln!("trying to convert non hex value {}", hex);
    panic!("trying to convert non hex value");
}

fn val_byte_to_hex_byte(val: u8) -> u8 {
    if val < 10 {
        return val + 48;
    }
    if val < 16 {
        return val + 87;
    }
    eprintln!("trying to convert non hex value{}", val);
    panic!("trying to convert non hex value");
}