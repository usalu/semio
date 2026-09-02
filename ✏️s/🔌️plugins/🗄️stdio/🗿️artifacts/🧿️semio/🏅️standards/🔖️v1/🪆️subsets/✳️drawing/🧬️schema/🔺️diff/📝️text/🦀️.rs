//! 📝️ Text representation codec surface for `stdio.semio.drawing` (diff). The real parse/print
//! is `SemioDrawingDiff`'s hand-rolled `protocol::DiffCodec` impl (../🦀️.rs) -- this
//! module exposes the grammar source for tooling/introspection.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
