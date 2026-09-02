//! 📝️ Text representation codec surface for `stdio.mp4` (mutations) — the real op text
//! codec is `protocol::OpText` in ../🦀️.rs (`print_op`/`parse_op`, one structured
//! named-record line per op); this leaf documents that shape via the grammar file below.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
