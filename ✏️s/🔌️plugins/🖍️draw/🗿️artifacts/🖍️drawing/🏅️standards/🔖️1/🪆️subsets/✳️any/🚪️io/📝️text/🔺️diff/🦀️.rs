//! 📖️ Drawing diff — the normative handcrafted text grammar for this facet. `DrawingDiff` is never
//! parsed from authored DSL text (it is only ever produced by `Mutation::diff`/absorbed via VCS
//! replay, never a source of truth a user hand-authors), so — like `💡️inferences/📝️text`'s
//! declaration-only shape — this leaf declares the wire grammar only; the real `apply`/`absorb` law
//! and every `diff_*` builder live on `DrawingDiff` itself in `🧬️schema/🔺️diff/🦀️.rs`
//! (design.md rule: `🧬️schema` keeps types + pure transforms, `🚪️io` keeps codecs — `apply`/`absorb`
//! are pure transforms over already-decoded types, not a byte-boundary codec, so they stayed put;
//! only this facet's grammar/protocol spec assets relocated here).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

#[cfg(test)]
#[path="🧪️tests/🔬️typed-patches/🦀️.rs"]
mod typed_patches;

impl semio_framework_os_kernel::DiffText for crate::standards::v1::subsets::any::schema::diff::DrawingDiff {
    fn print_diff(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_diff(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
