//! 📝️ Text representation grammar surface for `stdio.semio.video` (mutations): the `<keyword>
//! arg=value ...` op grammar — actual print/parse lives on `SemioVideoMutation`'s
//! `protocol::OpText` impl in the facet root `🦀️component.rs`; this leaf carries the normative
//! grammar description.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
