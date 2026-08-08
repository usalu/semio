//! 🔧 playbook artifact — OpText/OpBinary bridge for `PlaybookMutation`.

pub use crate::artifacts::playbook::mutations::{add_block_operation, add_step_operation, apply_playbook_edit_mutation, move_block_operation, move_step_operation, remove_block_operation, remove_step_operation, update_playbook_title_operation, inverse_playbook_mutation, PlaybookMutation};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::playbook::engine::empty_playbook_projection;

    #[test]
    fn update_playbook_op_sets_title() {
        let spec = empty_playbook_projection();
        let mutation = PlaybookMutation::UpdatePlaybook { title: Some("Renamed".into()) };
        let next = apply_playbook_edit_mutation(&spec, &mutation);
        assert_eq!(next.title.as_deref(), Some("Renamed"));
    }

    #[test]
    fn apply_playbook_edit_op_roundtrip() {
        use crate::artifacts::playbook::PlaybookStep;

        let spec = empty_playbook_projection();
        let step = PlaybookStep { id: "step-test".into(), title: "Review".into(), description: None, blocks: Vec::new() };
        let next = apply_playbook_edit_mutation(&spec, &PlaybookMutation::AddStep { step, index: None });
        assert_eq!(next.steps.len(), 2);
    }
}
//#endregion 🧪️Tests

