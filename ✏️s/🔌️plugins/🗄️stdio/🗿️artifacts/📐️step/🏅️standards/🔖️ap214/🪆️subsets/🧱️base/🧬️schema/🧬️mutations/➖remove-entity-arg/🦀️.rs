//! ➖️ `remove-entity-arg` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveEntityArg {
    pub id: u64,
    pub arg_index: usize,
}

impl protocol::MutationKind<StepSnapshot, StepMutation> for RemoveEntityArg {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "entity-arg", kind: "remove-entity-arg", record: "RemoveEntityArg" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
        let Self { id, arg_index } = self;
        protocol::MutationOutcome::new(StepDiff {
            entities: Some(StepEntitiesDiff { modified: vec![StepEntityModified { id: *id, diff: StepEntityDiff { args: Some(StepArgsDiff { removed: vec![*arg_index], ..Default::default() }), ..Default::default() } }], ..Default::default() }),
            ..Default::default()
        })
    }
    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepMutation>, semio_framework_value::ValueError> {
        let Self { id, arg_index } = self;
        Ok(match base.entities.iter().find(|e| e.id == *id).and_then(|e| e.args.get(*arg_index)) {
            Some(v) => vec![StepMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: *id, arg_index: *arg_index, value: v.clone() })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove entity arg", "Entitätsargument entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
