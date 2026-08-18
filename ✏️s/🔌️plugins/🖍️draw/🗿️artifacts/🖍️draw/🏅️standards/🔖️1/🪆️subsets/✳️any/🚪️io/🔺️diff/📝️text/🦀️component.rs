//! 📖️ Draw diff — the normative handcrafted text grammar for this facet. `DrawDiff` is never
//! parsed from authored DSL text (it is only ever produced by `Mutation::diff`/absorbed via VCS
//! replay, never a source of truth a user hand-authors), so — like `💡️inferences/📝️text`'s
//! declaration-only shape — this leaf declares the wire grammar only; the real `apply`/`absorb` law
//! and every `diff_*` builder live on `DrawDiff` itself in `🧬️schema/🔺️diff/🦀️component.rs`
//! (design.md rule: `🧬️schema` keeps types + pure transforms, `🚪️io` keeps codecs — `apply`/`absorb`
//! are pure transforms over already-decoded types, not a byte-boundary codec, so they stayed put;
//! only this facet's grammar/protocol spec assets relocated here).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar
