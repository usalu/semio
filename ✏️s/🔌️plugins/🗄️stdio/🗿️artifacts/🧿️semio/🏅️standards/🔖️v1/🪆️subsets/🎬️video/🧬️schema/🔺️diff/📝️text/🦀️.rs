//! 📝️ Text representation grammar surface for `stdio.semio.video` (diff): the `streams=` triple
//! grammar — actual print/parse lives on `SemioVideoDiff`'s `protocol::DiffCodec` impl in the
//! facet root `🦀️.rs`; this leaf carries the normative grammar description.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
