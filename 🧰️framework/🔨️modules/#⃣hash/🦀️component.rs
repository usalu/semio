//! 🪪️ Blake3 content-hash and Merkle utilities for operation envelopes and assets.

//#region 🔖️Hash
pub fn hash_parts<S: AsRef<[u8]>>(parts: &[S]) -> String {
    let mut hasher = blake3::Hasher::new();
    for part in parts {
        hasher.update(part.as_ref());
        hasher.update(b"\x1f");
    }
    hasher.finalize().to_hex().to_string()
}

pub fn hash_bytes(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

pub fn format_number_for_hash(value: f64) -> String {
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value.is_sign_positive() { "inf".into() } else { "-inf".into() };
    }
    if value == 0.0 {
        return "0".into();
    }
    if (value - value.round()).abs() < 1e-9 && value.abs() < 1e15 {
        return format!("{:.0}", value);
    }
    let mut text = format!("{value:.12}");
    if text.contains('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
    }
    if text == "-0" {
        "0".into()
    } else {
        text
    }
}

pub fn merkle_node(own: &[&str], mut children: Vec<String>) -> String {
    children.sort();
    let mut hasher = blake3::Hasher::new();
    for entry in own {
        hasher.update(entry.as_bytes());
        hasher.update(b"\x1f");
    }
    for child in &children {
        hasher.update(child.as_bytes());
        hasher.update(b"\x1f");
    }
    hasher.finalize().to_hex().to_string()
}

pub fn merkle_collection(children: Vec<String>) -> String {
    merkle_node(&["RelayCollection"], children)
}
//#endregion 🔖️Hash

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_bytes_deterministically() {
        let first = hash_bytes(b"hello");
        let second = hash_bytes(b"hello");
        assert_eq!(first, second);
        assert_ne!(first, hash_bytes(b"world"));
    }

    #[test]
    fn normalizes_hash_numbers() {
        assert_eq!(format_number_for_hash(-0.0), "0");
        assert_eq!(format_number_for_hash(42.0), "42");
        assert_eq!(format_number_for_hash(1.25), "1.25");
    }

    #[test]
    fn separates_hash_parts_with_a_delimiter() {
        assert_ne!(hash_parts(&["ab", "c"]), hash_parts(&["a", "bc"]));
    }

    #[test]
    fn orders_merkle_children_deterministically() {
        assert_eq!(merkle_node(&["root"], vec!["child-b".into(), "child-a".into()]), merkle_node(&["root"], vec!["child-a".into(), "child-b".into()]),);
    }

    #[test]
    fn normalizes_special_hash_numbers() {
        assert_eq!(format_number_for_hash(f64::NAN), "nan");
        assert_eq!(format_number_for_hash(f64::INFINITY), "inf");
        assert_eq!(format_number_for_hash(f64::NEG_INFINITY), "-inf");
    }
}
