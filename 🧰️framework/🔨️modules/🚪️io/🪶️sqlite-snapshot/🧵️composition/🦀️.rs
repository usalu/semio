//! 🧵️ Compile-time composition of one handcrafted `SQLITE_SCHEMA` from DDL segments owned by separate modules or crates.
//!
//! A snapshot's relational schema is the ordered concatenation of the DDL each projection module owns. Composing it in
//! a `const` keeps every DDL file embedded by the module that projects into it while the snapshot type still exposes one
//! `&'static str`. See <https://sqlite.org/lang_createtable.html>.

/// 🧮️ Total byte length of the ordered DDL segments.
pub const fn sqlite_schema_len(segments: &[&str]) -> usize {
    let mut total = 0;
    let mut index = 0;
    while index < segments.len() {
        total += segments[index].len();
        index += 1;
    }
    total
}

/// 🧩️ Concatenates the ordered DDL segments into one byte array; `N` must equal [`sqlite_schema_len`].
pub const fn sqlite_schema_bytes<const N: usize>(segments: &[&str]) -> [u8; N] {
    let mut out = [0u8; N];
    let mut at = 0;
    let mut index = 0;
    while index < segments.len() {
        let bytes = segments[index].as_bytes();
        let mut offset = 0;
        while offset < bytes.len() {
            out[at] = bytes[offset];
            at += 1;
            offset += 1;
        }
        index += 1;
    }
    assert!(at == N, "composed SQLite schema length differs from its declared length");
    out
}

/// 🔤️ Views composed schema bytes as schema text; refuses non-UTF-8 bytes when the constant is evaluated.
pub const fn sqlite_schema_text(bytes: &'static [u8]) -> &'static str {
    match core::str::from_utf8(bytes) {
        Ok(text) => text,
        Err(_) => panic!("composed SQLite schema is not UTF-8"),
    }
}
