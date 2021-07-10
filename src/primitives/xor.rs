pub fn xor_byte_slices(raw_bytes_1: &[u8], raw_bytes_2: &[u8]) -> Vec<u8> {
    let mut raw_bytes_xor = Vec::with_capacity(raw_bytes_1.len());
    if raw_bytes_1.len() != raw_bytes_2.len() {
        panic!("bytes to xor must have equal length")
    }
    let mut i = 0;
    while i < raw_bytes_1.len() {
        raw_bytes_xor.push(raw_bytes_1[i] ^ raw_bytes_2[i]);
        i += 1;
    }
    return raw_bytes_xor;
}

pub fn xor_byte_slice_fixed(raw_bytes: &[u8], key: u8) -> Vec<u8> {
    let mut raw_bytes_xor = Vec::with_capacity(raw_bytes.len());
    let mut i = 0;
    while i < raw_bytes.len() {
        raw_bytes_xor.push(raw_bytes[i] ^ key);
        i += 1;
    }
    return raw_bytes_xor;
}

pub fn encrypt_bytes_repeating_key_xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    if key.len() == 0 {
        panic!("Key length must be >= 0");
    }
    let mut ciphertext = Vec::with_capacity(bytes.len());
    let mut i = 0;
    let mut j = 0;
    while i < bytes.len() {
        ciphertext.push(bytes[i] ^ key[j]);
        i += 1;
        j = (j + 1) % key.len();
    }
    return ciphertext;
}