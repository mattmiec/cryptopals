pub fn convert_hex_string_to_b64_string(hex_string: &String) -> String {

    let hex_bytes = hex_string.as_bytes();
    let raw_bytes = hex_bytes_to_raw_bytes(hex_bytes);
    let b64_bytes = raw_bytes_to_b64_bytes(&raw_bytes[..]);
    let b64_string = String::from_utf8(b64_bytes).expect("failed to convert b64_bytes to string!");
    return b64_string;
}

fn hex_bytes_to_raw_bytes(hex_bytes: &[u8]) -> Vec<u8> {
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

fn raw_bytes_to_b64_bytes(raw_bytes: &[u8]) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity((raw_bytes.len() / 3) * 4);
    let mut i = 0;
    while i < raw_bytes.len() {

        let offset = if raw_bytes.len() - i > 3 { 3 } else { raw_bytes.len() - i };

        let base64_chunk = byte_chunk_to_base64_chunk(&raw_bytes[i..i+offset]);

        for byte in &base64_chunk {
            ret.push(*byte);
        }
        i += 3;
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

fn byte_chunk_to_base64_chunk(bytes: &[u8]) -> [u8; 4] {

    if bytes.len() == 0 || bytes.len() > 3 {
        panic!("bytes_to_base64 requires >= 1 and <= 3 bytes!")
    }

    let mut ret: [u8; 4] = [b'='; 4];
    let sextet_filter: u8 = 0b0011_1111;

    ret[0] = sextet_val_to_byte(bytes[0] >> 2);
    
    if bytes.len() == 1 {
        ret[1] = sextet_val_to_byte((bytes[0] << 4) & sextet_filter);
        return ret;
    }

    ret[1] = sextet_val_to_byte(((bytes[0] << 4) | (bytes[1] >> 4)) & sextet_filter);

    if bytes.len() == 2 {
        ret[2] = sextet_val_to_byte((bytes[1] << 2) & sextet_filter);
        return ret;
    }

    ret[2] = sextet_val_to_byte(((bytes[1] << 2) | (bytes[2] >> 6)) & sextet_filter);
    ret[3] = sextet_val_to_byte(bytes[2] & sextet_filter);

    return ret;
}

fn sextet_val_to_byte(sextet: u8) -> u8 {
    if sextet < 26 {
        return sextet + 65;
    }
    if sextet < 52 {
        return sextet - 26 + 97;
    }
    if sextet < 62 {
        return sextet - 52 + 48;
    }
    if sextet == 62 {
        return b'+';
    }
    if sextet == 63 {
        return b'/';
    }
    eprintln!("invalid sextet {}", sextet);
    panic!("invalid sextet!");
}