//! 🔧️ `set-entity-arg` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetEntityArg {
    pub id: u64,
    pub arg_index: usize,
    pub value: StepValue,
}

impl protocol::MutationKind<StepSnapshot, StepMutation> for SetEntityArg {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "entity-arg", kind: "set-entity-arg", record: "SetEntityArg" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
        let Self { id, arg_index, value } = self;
        protocol::MutationOutcome::new(match base.entities.iter().find(|e| e.id == *id) {
            Some(e) if e.args.get(*arg_index).is_some_and(|v| v == value) => StepDiff::default(),
            _ => StepDiff {
                entities: Some(StepEntitiesDiff {
                    modified: vec![StepEntityModified { id: *id, diff: StepEntityDiff { args: Some(StepArgsDiff { modified: vec![StepArgModified { index: *arg_index, value: value.clone() }], ..Default::default() }), ..Default::default() } }],
                    ..Default::default()
                }),
                ..Default::default()
            },
        })
    }
    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepMutation>, semio_framework_value::ValueError> {
        let Self { id, arg_index, .. } = self;
        Ok(match base.entities.iter().find(|e| e.id == *id).and_then(|e| e.args.get(*arg_index)) {
            Some(v) => vec![StepMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: *id, arg_index: *arg_index, value: v.clone() })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set entity arg", "Entitätsargument setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
