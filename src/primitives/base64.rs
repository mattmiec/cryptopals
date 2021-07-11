pub fn raw_bytes_to_b64_bytes(raw_bytes: &[u8]) -> Vec<u8> {
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

pub fn b64_bytes_to_raw_bytes(b64_bytes: &[u8]) -> Vec<u8> {
    let mut i = 0;
    let mut ret = Vec::new();
    while i < b64_bytes.len() {
        if i + 4 > b64_bytes.len() {
            panic!("Invalid base64 string!");
        }
        for byte in base64_chunk_to_byte_chunk(&b64_bytes[i..i+4]) {
            ret.push(byte);
        }
        i += 4;
    }
    return ret;
}

fn byte_chunk_to_base64_chunk(bytes: &[u8]) -> [u8; 4] {
    if bytes.len() == 0 || bytes.len() > 3 {
        panic!("bytes_to_base64 requires >= 1 and <= 3 bytes!")
    }

    let mut ret: [u8; 4] = [b'='; 4];
    let sextet_filter: u8 = 0b0011_1111;

    ret[0] = sextet_val_to_b64_byte(bytes[0] >> 2);
    
    if bytes.len() == 1 {
        ret[1] = sextet_val_to_b64_byte((bytes[0] << 4) & sextet_filter);
        return ret;
    }

    ret[1] = sextet_val_to_b64_byte(((bytes[0] << 4) | (bytes[1] >> 4)) & sextet_filter);

    if bytes.len() == 2 {
        ret[2] = sextet_val_to_b64_byte((bytes[1] << 2) & sextet_filter);
        return ret;
    }

    ret[2] = sextet_val_to_b64_byte(((bytes[1] << 2) | (bytes[2] >> 6)) & sextet_filter);
    ret[3] = sextet_val_to_b64_byte(bytes[2] & sextet_filter);

    return ret;
}

fn base64_chunk_to_byte_chunk(b64_bytes: &[u8]) -> Vec<u8> {
    if b64_bytes.len() != 4 {
        panic!("Base64 chunk must be of length 4!");
    }
    let mut ret = Vec::new();
    ret.push((b64_byte_to_sextet_val(b64_bytes[0]) << 2) | (b64_byte_to_sextet_val(b64_bytes[1]) >> 4));
    if b64_bytes[2] == b'=' {
        return ret;
    }
    ret.push((b64_byte_to_sextet_val(b64_bytes[1]) << 4) | (b64_byte_to_sextet_val(b64_bytes[2]) >> 2));
    if b64_bytes[3] == b'=' {
        return ret;
    }
    ret.push((b64_byte_to_sextet_val(b64_bytes[2]) << 6) | b64_byte_to_sextet_val(b64_bytes[3]));

    return ret;
}

fn sextet_val_to_b64_byte(sextet: u8) -> u8 {
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

fn b64_byte_to_sextet_val(b64_byte: u8) -> u8 {
    if b64_byte >= 65 && b64_byte < 91 {
        return b64_byte - 65;
    }
    if b64_byte >= 97 && b64_byte < 123 {
        return b64_byte - 97 + 26;
    }
    if b64_byte >= 48 && b64_byte < 58 {
        return b64_byte - 48 + 52;
    }
    if b64_byte == b'+' {
        return 62;
    }
    if b64_byte == b'/' {
        return 63;
    }
    if b64_byte == b'=' {
        return 64;
    }
    eprintln!("invalid base64 byte {}", b64_byte);
    panic!("invalid base64 byte!");
}