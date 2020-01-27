fn atbash(plain: &str) -> String {
    plain
        .chars()
        .enumerate()
        .filter_map(|(_, c)| {
            if c.is_ascii_alphabetic() {
                let ascii_code = c.to_ascii_lowercase() as u8;
                Some((122 - (ascii_code - 97)) as char)
            } else if c.is_numeric() {
                Some(c)
            } else {
                None
            }
        })
        .collect()
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let group_length = 5;
    // Output cipher in groups of 5
    atbash(plain)
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % group_length == 0 {
                Some(' ')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    atbash(cipher)
}
