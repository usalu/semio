//! 🗑️ `remove-entity` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveEntity {
    pub id: u64,
}

impl protocol::MutationKind<StepSnapshot, StepMutation> for RemoveEntity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "entity", kind: "remove-entity", record: "RemoveEntity" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
        let Self { id } = self;
        protocol::MutationOutcome::new(StepDiff { entities: Some(StepEntitiesDiff { removed: vec![*id], ..Default::default() }), ..Default::default() })
    }
    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepMutation>, semio_framework_value::ValueError> {
        let Self { id } = self;
        Ok(match base.entities.iter().position(|e| e.id == *id) {
            Some(idx) => vec![StepMutation::InsertEntity(insert_entity::InsertEntity { index: idx, entity: base.entities[idx].clone() })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove entity", "Entität entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
