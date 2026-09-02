//! 📝️ Text representation codec surface for `stdio.semio.drawing` (snapshot). The real parse/
//! print lives on `SemioDrawingSnapshot`'s `store::ArtifactDsl` impl (📸️snapshot/🦀️.rs)
//! -- this module exposes the grammar source for tooling/introspection, matching svg's own
//! `📝️text/🦀️.rs` convention.

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
