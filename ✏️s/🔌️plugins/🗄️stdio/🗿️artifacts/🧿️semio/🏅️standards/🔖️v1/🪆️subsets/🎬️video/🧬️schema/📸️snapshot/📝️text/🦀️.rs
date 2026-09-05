//! 📝️ Text representation grammar surface for `stdio.semio.video` (snapshot): real structured DSL
//! body — `schema=<hex>` then `streams=[<stream>,...]`, every leaf its own hex/bracket-encoded
//! token (video wave, replacing the old envelope-header-plus-hex(JSON) scaffold) — actual
//! parse/print lives on `SemioVideoSnapshot`'s `store::ArtifactDsl` impl in the facet root
//! `🦀️.rs`; this leaf carries the normative grammar description.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
