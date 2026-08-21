//! 🔧 playbook artifact — OpText/OpBinary codecs + grammar for serializing `PlaybookMutation`.
//! Mutation diff/inverse live in the `🧬️mutations/<slug>/` triad leaves; this facet only
//! handcrafts the op wire forms.

pub use crate::artifacts::playbook::mutations::{
    add_block_operation, add_step_operation, apply_playbook_mutation, change_title_operation, inverse_playbook_mutation, move_block_operation, move_step_operation, remove_block_operation, remove_step_operation, replace_block_operation,
    update_step_operation, AddBlock, AddStep, ChangeTitle, MoveBlock, MoveStep, PlaybookMutation, RemoveBlock, RemoveStep, ReplaceBlock, UpdateStep,
};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits). Delegates to
/// `PlaybookMutation`'s own `#[derive(dsl::DslEnum)]`-generated `DslVariants` impl — every variant
/// is a single-field tuple, so each payload's own `#[dsl(keyword = "...")]` IS the wire keyword.
impl protocol::OpText for PlaybookMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for PlaybookMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::playbook::empty_playbook_snapshot;

    #[semio_framework_async_macros::async_test]
    async fn change_title_op_sets_title() {
        let spec = empty_playbook_snapshot();
        let mutation = change_title_operation(Some("Renamed".into()));
        let next = apply_playbook_mutation(&spec, &mutation).expect("valid mutation diff");
        assert_eq!(next.title.as_deref(), Some("Renamed"));
    }

    #[semio_framework_async_macros::async_test]
    async fn apply_playbook_add_step_roundtrip() {
        let spec = empty_playbook_snapshot();
        let next = apply_playbook_mutation(&spec, &add_step_operation(&spec, "step-test".into())).expect("valid mutation diff");
        assert_eq!(next.steps().len(), 2);
    }

    async fn sample_block() -> crate::artifacts::playbook::PlaybookBlock {
        crate::artifacts::playbook::PlaybookBlock {
            id: "b1".into(),
            label: "Team size".into(),
            kind: "number".into(),
            description: None,
            required: None,
            placeholder: None,
            default: None,
            min: None,
            max: None,
            step: None,
            unit: None,
            text: None,
            options: None,
            fields: None,
            schema: None,
            src: None,
            accept: None,
            fixture_slug: None,
            params: None,
            condition: None,
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trips_for_every_kind() {
        use protocol::OpText;
        let ops = vec![
            add_step_operation(&empty_playbook_snapshot(), "s2".into()),
            remove_step_operation("s1"),
            move_step_operation("s1", 2),
            add_block_operation("s1", sample_block(), None),
            remove_block_operation("s1", "b1"),
            move_block_operation("b1", "s1", "s2", 0),
            replace_block_operation("s1", sample_block()),
            update_step_operation("s1", "Basics".into(), Some("d".into())),
            change_title_operation(Some("Recipe".into())),
        ];
        for op in ops {
            let line = op.print_op();
            assert!(!line.contains('\n'));
            assert_eq!(PlaybookMutation::parse_op(&line).expect("parse"), op);
        }
    }
}
//#endregion 🧪️Tests
