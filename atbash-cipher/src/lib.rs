fn atbash(plain: &str) -> String {
    plain
        .bytes()
        .filter_map(|c| match c {
            c if c.is_ascii_alphabetic() => Some(b'z' - (c.to_ascii_lowercase() - b'a')),
            c if c.is_ascii_digit() => Some(c),
            _ => None,
        })
        .map(|c| c as char)
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
