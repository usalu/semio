//! 🪪️ Identity pending: the emoji rune table and entity emoji extraction awaiting the identity crate.
//!
//! Holds the `🔖️IdentityPending` region of the crate root; the root re-exports its items.

// #region 🔖️IdentityPending

// 🚚️ `is_emoji_rune` / `extract_entity_emoji` belong to `🔨️modules/🪪️identity` per the taxonomy plan.
// They live here until that crate exists; re-export from identity and delete this region then.

/// 🎭️ Reports whether a scalar is an emoji base character, a joiner or a presentation selector.
pub fn is_emoji_rune(r: char) -> bool {
    let c = r as u32;
    const RANGES: &[(u32, u32)] = &[
        (0x1F600, 0x1F64F),
        (0x1F300, 0x1F5FF),
        (0x1F680, 0x1F6FF),
        (0x1F700, 0x1F77F),
        (0x1F780, 0x1F7FF),
        (0x1F800, 0x1F8FF),
        (0x1F900, 0x1F9FF),
        (0x1FA00, 0x1FA6F),
        (0x1FA70, 0x1FAFF),
        (0x2600, 0x26FF),
        (0x2700, 0x27BF),
        (0x2300, 0x23FF),
        (0x2B50, 0x2B55),
        (0x200D, 0x200D),
        (0xFE00, 0xFE0F),
        (0x2196, 0x2199),
        (0x21A9, 0x21AA),
        (0x231A, 0x231B),
        (0x25AA, 0x25AB),
        (0x25B6, 0x25C0),
        (0x25FB, 0x25FE),
        (0x2614, 0x2615),
        (0x2648, 0x2653),
        (0x267F, 0x267F),
        (0x2693, 0x2693),
        (0x26A1, 0x26A1),
        (0x26AA, 0x26AB),
        (0x26BD, 0x26BE),
        (0x26C4, 0x26C5),
        (0x26CE, 0x26CF),
        (0x26D4, 0x26D4),
        (0x26EA, 0x26EA),
        (0x26F2, 0x26F3),
        (0x26F5, 0x26F5),
        (0x26FA, 0x26FA),
        (0x26FD, 0x26FD),
    ];
    const SINGLES: &[u32] = &[0x2139, 0x2194, 0x2195, 0x203C, 0x2049, 0x20E3, 0x00A9, 0x00AE, 0x2122];
    RANGES.iter().any(|&(lo, hi)| c >= lo && c <= hi) || SINGLES.contains(&c)
}

/// 🧲️ Splits a leading emoji grapheme (with joiners, selectors and skin tones) from the rest.
pub fn extract_entity_emoji(s: &str) -> (String, String) {
    let runes: Vec<char> = s.chars().collect();
    if runes.is_empty() || !is_emoji_rune(runes[0]) {
        return (String::new(), s.to_string());
    }
    let mut i = 1usize;
    while i < runes.len() {
        let r = runes[i] as u32;
        if r == 0xFE0F || r == 0xFE0E || r == 0x20E3 {
            i += 1;
        } else if r == 0x200D {
            i += 1;
            if i < runes.len() {
                i += 1;
                while i < runes.len() && matches!(runes[i] as u32, 0xFE0F | 0xFE0E) {
                    i += 1;
                }
            }
        } else if (0x1F3FB..=0x1F3FF).contains(&r) {
            i += 1;
        } else {
            break;
        }
    }
    (runes[..i].iter().collect(), runes[i..].iter().collect())
}

// #endregion 🔖️IdentityPending
