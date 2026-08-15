//! 📝️ Text representation codec surface for `stdio.mp4` (mutations) — the real op text
//! codec is `protocol::OpText` in ../🦀️component.rs (`print_op`/`parse_op`, one structured
//! named-record line per op); this leaf documents that shape via the grammar file below.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
