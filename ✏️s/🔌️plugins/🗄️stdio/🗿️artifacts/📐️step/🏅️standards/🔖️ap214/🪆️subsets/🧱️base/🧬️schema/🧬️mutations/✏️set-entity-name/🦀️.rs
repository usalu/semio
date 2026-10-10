//! ✏️️ `set-entity-name` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEntityName {
    pub id: u64,
    pub name: String,
}

impl protocol::MutationKind<StepSnapshot, StepMutation> for SetEntityName {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "entity-name", kind: "set-entity-name", record: "SetEntityName" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
        let Self { id, name } = self;
        protocol::MutationOutcome::new(match base.entities.iter().find(|e| e.id == *id) {
            Some(e) if e.name == *name => StepDiff::default(),
            _ => StepDiff { entities: Some(StepEntitiesDiff { modified: vec![StepEntityModified { id: *id, diff: StepEntityDiff { name: Some(name.clone()), ..Default::default() } }], ..Default::default() }), ..Default::default() },
        })
    }
    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepMutation>, semio_framework_value::ValueError> {
        let Self { id, .. } = self;
        Ok(match base.entities.iter().find(|e| e.id == *id) {
            Some(e) => vec![StepMutation::SetEntityName(set_entity_name::SetEntityName { id: *id, name: e.name.clone() })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set entity name", "Entitätsname setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
