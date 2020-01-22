/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut encoded_string = String::from("");
    for c in plain.chars() {
        if c.is_ascii_whitespace() || c.is_numeric() {
            encoded_string.push(c);
        } else if c.is_ascii_alphanumeric() {
            let ascii_code = c.to_ascii_lowercase() as u8;
            let ascii_char = (122 - (ascii_code - 97)) as char;
            encoded_string.push(ascii_char)
        }
    }
    encoded_string
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    unimplemented!("Decoding of {:?} in Atbash cipher.", cipher);
}
