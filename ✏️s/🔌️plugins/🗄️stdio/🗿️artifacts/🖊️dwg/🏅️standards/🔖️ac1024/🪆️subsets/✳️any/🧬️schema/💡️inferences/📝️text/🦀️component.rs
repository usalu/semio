//! 📖️ s.stdio.dwg.inference (ac1024) — the normative handcrafted text grammar for this facet.
//! Inference values are never authored via DSL text (they are always computed from a snapshot,
//! never a source of truth), so — unlike `📸️snapshot/📝️text`'s live `parse_dsl`/`print_dsl`
//! pair — this leaf is declaration-only. It still names every logical metric rather than hiding
//! the derived record behind an opaque payload.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar
