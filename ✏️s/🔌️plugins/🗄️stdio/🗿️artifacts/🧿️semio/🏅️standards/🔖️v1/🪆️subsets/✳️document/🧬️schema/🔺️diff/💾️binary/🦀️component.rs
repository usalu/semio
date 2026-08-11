//! 💾️ Binary representation marker for `s.stdio.semio.document.diff`: `SemioDocumentDiff`'s
//! `protocol::DiffCodec::encode_diff` is the `print_diff()` text bytes verbatim (see
//! `🦀️component.rs`'s `HandcraftedDiffCodec` region) — no separate binary framing.
pub const BINARY_MAGIC: &str = "s.stdio.semio.document.diff";
