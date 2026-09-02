//! 📝️ Text representation codec surface for `stdio.html` (mutations). The REAL codec is the
//! hand-rolled `print_op`/`parse_op` in the sibling `🦀️.rs` two levels up.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
