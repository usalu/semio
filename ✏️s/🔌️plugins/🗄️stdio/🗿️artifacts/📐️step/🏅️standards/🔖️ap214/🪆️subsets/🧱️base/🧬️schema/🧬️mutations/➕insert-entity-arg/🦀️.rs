//! ➕️ `insert-entity-arg` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertEntityArg {
    pub id: u64,
    pub arg_index: usize,
    pub value: StepValue,
}

impl protocol::MutationKind<StepSnapshot, StepMutation> for InsertEntityArg {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "entity-arg", kind: "insert-entity-arg", record: "InsertEntityArg" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
        let Self { id, arg_index, value } = self;
        protocol::MutationOutcome::new(StepDiff {
            entities: Some(StepEntitiesDiff {
                modified: vec![StepEntityModified { id: *id, diff: StepEntityDiff { args: Some(StepArgsDiff { added: vec![StepArgAdded { index: *arg_index, value: value.clone() }], ..Default::default() }), ..Default::default() } }],
                ..Default::default()
            }),
            ..Default::default()
        })
    }
    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepMutation>, semio_framework_value::ValueError> {
        let Self { id, arg_index, .. } = self;
        Ok(vec![StepMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: *id, arg_index: *arg_index })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert entity arg", "Entitätsargument einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
