//! 📝️ Text representation grammar surface for `stdio.semio.video` (snapshot): envelope header +
//! hex(JSON) body — actual parse/print lives on `SemioVideoSnapshot`'s `store::ArtifactDsl` impl
//! in the facet root `🦀️component.rs`; this leaf carries the normative grammar description.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
