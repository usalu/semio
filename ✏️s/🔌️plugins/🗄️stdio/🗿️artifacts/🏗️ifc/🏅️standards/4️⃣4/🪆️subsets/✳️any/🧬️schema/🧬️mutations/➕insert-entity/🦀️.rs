//! ➕️ `insert-entity` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertEntity {
    pub index: usize,
    pub entity: IfcEntity,
}

impl protocol::MutationKind<IfcSnapshot, IfcMutation> for InsertEntity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "entity", kind: "insert-entity", record: "InsertEntity" };

    fn diff(&self, base: &IfcSnapshot) -> protocol::MutationOutcome<IfcDiff> {
        let Self { index, entity } = self;
        protocol::MutationOutcome::new(diff::diff_insert_entity(*index, entity.clone()))
    }
    fn inverse(&self, base: &IfcSnapshot) -> Result<Vec<IfcMutation>, semio_framework_value::ValueError> {
        let entity = |id: u64| base.entities.iter().find(|e| e.id == id);
        let Self { entity, .. } = self;
        Ok(vec![IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id: entity.id })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert entity", "Entität einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
