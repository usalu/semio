//! 🔺️ Playbook artifact — the operation diff (constitutional: diff).
//!
//! `PlaybookDiff` and its `protocol::MutationDiff<PlaybookSpec>` impl are implemented directly in the
//! shared `playbook` kernel crate alongside the `PlaybookSpec` projection it patches — see
//! `🗿️artifacts/📖️playbook/🦀️component.rs` for why. Re-exported here so the artifact's diff slot names an
//! artifact-owned symbol.

//#region 🔖️Types

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


pub use crate::playbook::PlaybookDiff;
//#endregion 🔖️Types

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::playbook::{mutations::PlaybookMutation, PlaybookSpec, PlaybookStep, PLAYBOOK_DOCUMENT_SCHEMA};
    use protocol::{Mutation, MutationDiff};

    /// ⚖️ LAW: `op.diff(base)` applied to `base` equals applying the operation, and the diff carries only
    /// the touched slot — the `MutationDiff` contract undo/redo rides on.
    #[test]
    fn add_step_diff_applies_onto_the_base_projection() {
        let base = PlaybookSpec { schema: PLAYBOOK_DOCUMENT_SCHEMA.into(), id: "playbook".into(), version: "1".into(), title: None, steps: Vec::new() };
        let step = PlaybookStep { id: "s".into(), title: "Basics".into(), description: None, blocks: Vec::new() };
        let operation = PlaybookMutation::AddStep { step, index: None };
        let diff: PlaybookDiff = operation.diff(&base);
        assert_eq!(diff.apply(&base).steps.len(), 1);
    }
}
//#endregion 🧪️Tests
