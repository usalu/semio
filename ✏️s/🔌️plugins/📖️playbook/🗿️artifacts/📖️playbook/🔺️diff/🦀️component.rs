//! 🔺️ Playbook artifact — the operation diff (constitutional: diff).
//!
//! `PlaybookDiff` and its `protocol::OperationDiff<PlaybookSpec>` impl are implemented directly in the
//! shared `playbook` kernel crate alongside the `PlaybookSpec` projection it patches — see
//! `🗿️artifacts/📖️playbook/🦀️component.rs` for why. Re-exported here so the artifact's diff slot names an
//! artifact-owned symbol.

//#region 🔖️Types
pub use playbook::PlaybookDiff;
//#endregion 🔖️Types

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::playbook::{op::PlaybookOperation, PlaybookSpec, PlaybookStep, PLAYBOOK_DOCUMENT_SCHEMA};
    use protocol::{Operation, OperationDiff};

    /// ⚖️ LAW: `op.diff(base)` applied to `base` equals applying the operation, and the diff carries only
    /// the touched slot — the `OperationDiff` contract undo/redo rides on.
    #[test]
    fn add_step_diff_applies_onto_the_base_projection() {
        let base = PlaybookSpec { schema: PLAYBOOK_DOCUMENT_SCHEMA.into(), id: "playbook".into(), version: "1".into(), title: None, steps: Vec::new() };
        let step = PlaybookStep { id: "s".into(), title: "Basics".into(), description: None, blocks: Vec::new() };
        let operation = PlaybookOperation::AddStep { step, index: None };
        let diff: PlaybookDiff = operation.diff(&base);
        assert_eq!(diff.apply(&base).steps.len(), 1);
    }
}
//#endregion 🧪️Tests
