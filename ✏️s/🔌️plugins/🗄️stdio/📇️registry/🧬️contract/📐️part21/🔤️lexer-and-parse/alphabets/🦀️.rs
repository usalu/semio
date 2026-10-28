//! 🔤 STEP PAGE decoding over pinned owned ISO 8859 data.

#[path = "data.rs"]
mod data;

/// 🗺️ Undefined pages, payloads and scalar positions have no decoded character.
pub fn part21_alphabet_character(selector: char, payload: char) -> Option<char> {
    if !('A'..='I').contains(&selector) || !(' '..='~').contains(&payload) { return None; }
    let code = data::ALPHABETS[(selector as u32 - 'A' as u32) as usize][(payload as u32 - 32) as usize];
    if code == 0 { None } else { char::from_u32(code) }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
