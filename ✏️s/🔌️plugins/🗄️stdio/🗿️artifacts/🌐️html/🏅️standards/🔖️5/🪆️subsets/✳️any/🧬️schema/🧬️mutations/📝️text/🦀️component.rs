//! 📝️ Text representation codec surface for `stdio.html` (mutations). The REAL codec is the
//! hand-rolled `print_op`/`parse_op` in the sibling `🦀️component.rs` two levels up.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
