//! 🔧️ Playbook artifact — the operation enum + constructors (constitutional: op).
//!
//! `PlaybookOperation`, its `protocol::Operation<PlaybookSpec>` impl and the private
//! `apply_playbook_edit_operation` match all live in the shared `playbook` kernel crate alongside the
//! `PlaybookSpec` projection they mutate — see `🗿️artifacts/📖️playbook/🦀️component.rs` for why. Re-exported
//! here so every taxonomy node names an artifact-owned symbol instead of reaching into the kernel path.

//#region 🔖️Types
pub use playbook::{add_block_operation, add_step_operation, apply_playbook_edit_operation, move_block_operation, move_step_operation, remove_block_operation, remove_step_operation, update_playbook_title_operation, PlaybookOperation};
//#endregion 🔖️Types

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::playbook::engine::empty_playbook_projection;

    #[test]
    fn update_playbook_op_sets_title() {
        let spec = empty_playbook_projection();
        let operation = PlaybookOperation::UpdatePlaybook { title: Some("Renamed".into()) };
        let next = apply_playbook_edit_operation(&spec, &operation);
        assert_eq!(next.title.as_deref(), Some("Renamed"));
    }

    #[test]
    fn apply_playbook_edit_op_roundtrip() {
        use crate::artifacts::playbook::PlaybookStep;

        let spec = empty_playbook_projection();
        let step = PlaybookStep { id: "step-test".into(), title: "Review".into(), description: None, blocks: Vec::new() };
        let next = apply_playbook_edit_operation(&spec, &PlaybookOperation::AddStep { step, index: None });
        assert_eq!(next.steps.len(), 2);
    }
}
//#endregion 🧪️Tests
