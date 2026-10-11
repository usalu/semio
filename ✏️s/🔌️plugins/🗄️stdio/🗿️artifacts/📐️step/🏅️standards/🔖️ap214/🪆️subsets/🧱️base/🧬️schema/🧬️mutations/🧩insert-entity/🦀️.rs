//! 🧩️ `insert-entity` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertEntity {
    pub index: usize,
    pub entity: StepEntity,
}

impl protocol::MutationKind<StepSnapshot, StepMutation> for InsertEntity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "entity", kind: "insert-entity", record: "InsertEntity" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
        let Self { index, entity } = self;
        protocol::MutationOutcome::new(StepDiff { entities: Some(StepEntitiesDiff { added: vec![StepEntityAdded { index: *index, entity: entity.clone() }], ..Default::default() }), ..Default::default() })
    }
    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepMutation>, semio_framework_value::ValueError> {
        let Self { entity, .. } = self;
        Ok(vec![StepMutation::RemoveEntity(remove_entity::RemoveEntity { id: entity.id })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert entity", "Entität einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
