//! 📐️ Specification text: spec keyword detection and the implementation-syntax leak scan.

//#region 📐️Specification Text

const SPEC_KEYWORDS: [&str; 7] = ["MUST", "SHOULD", "SHALL", "MAY", "REQUIRED", "RECOMMENDED", "OPTIONAL"];

fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn word_at(bytes: &[u8], position: usize, word: &str) -> bool {
    let end = position + word.len();
    if end > bytes.len() || &bytes[position..end] != word.as_bytes() {
        return false;
    }
    if position > 0 && is_word_byte(bytes[position - 1]) {
        return false;
    }
    end == bytes.len() || !is_word_byte(bytes[end])
}

/// 📕️ Whether the text carries a specification keyword as a whole word.
pub fn is_spec_text(text: &str) -> bool {
    let bytes = text.as_bytes();
    (0..bytes.len()).any(|position| SPEC_KEYWORDS.iter().any(|word| word_at(bytes, position, word)))
}

fn backtick_pair(bytes: &[u8]) -> bool {
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'`' {
            let mut scan = index + 1;
            while scan < bytes.len() && bytes[scan] != b'`' {
                scan += 1;
            }
            if scan == bytes.len() {
                return false;
            }
            if scan > index + 1 {
                return true;
            }
            index = scan;
            continue;
        }
        index += 1;
    }
    false
}

fn qualified_call_at(bytes: &[u8], position: usize) -> Option<usize> {
    if !(bytes[position].is_ascii_alphabetic() || bytes[position] == b'_') {
        return None;
    }
    let mut scan = position + 1;
    while scan < bytes.len() && is_word_byte(bytes[scan]) {
        scan += 1;
    }
    while scan > position {
        if bytes.get(scan) == Some(&b'.') {
            let mut tail = scan + 1;
            while tail < bytes.len() && is_word_byte(bytes[tail]) {
                tail += 1;
            }
            if tail > scan + 1 && bytes.get(tail) == Some(&b'(') {
                return Some(tail + 1);
            }
        }
        scan -= 1;
    }
    None
}

fn bare_call_at(bytes: &[u8], position: usize) -> Option<usize> {
    if !bytes[position].is_ascii_uppercase() {
        return None;
    }
    let mut scan = position + 1;
    while scan < bytes.len() && is_word_byte(bytes[scan]) {
        scan += 1;
    }
    if scan > position + 1 && bytes.get(scan) == Some(&b'(') {
        return Some(scan + 1);
    }
    None
}

/// 🧪️ Whether a specification line leaks implementation syntax, and why.
pub fn has_implementation_syntax(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    if backtick_pair(bytes) {
        return Some("backtick-wrapped code".to_string());
    }
    for position in 0..bytes.len() {
        let end = qualified_call_at(bytes, position).or_else(|| bare_call_at(bytes, position));
        if let Some(end) = end {
            let matched = &text[position..end];
            return if matches!(matched, "MUST(" | "SHOULD(" | "SHALL(" | "MAY(") { None } else { Some(format!("function/method call: {matched}")) };
        }
    }
    None
}

pub(super) fn contributor_line(text: &str) -> bool {
    let bytes = text.as_bytes();
    for start in 0..bytes.len() {
        if start + 4 > bytes.len() || !bytes[start..start + 4].iter().all(u8::is_ascii_digit) {
            continue;
        }
        let mut cursor = start + 4;
        let space_start = cursor;
        while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if cursor == space_start {
            continue;
        }
        while cursor < bytes.len() && (is_word_byte(bytes[cursor]) || bytes[cursor].is_ascii_whitespace()) {
            cursor += 1;
        }
        if cursor - space_start >= 2 && bytes.get(cursor) == Some(&b'<') {
            let mut tail = cursor + 1;
            while tail < bytes.len() && (is_word_byte(bytes[tail]) || matches!(bytes[tail], b'.' | b'@' | b'-')) {
                tail += 1;
            }
            if tail > cursor + 1 && bytes.get(tail) == Some(&b'>') {
                return true;
            }
        }
    }
    false
}

//#endregion 📐️Specification Text
