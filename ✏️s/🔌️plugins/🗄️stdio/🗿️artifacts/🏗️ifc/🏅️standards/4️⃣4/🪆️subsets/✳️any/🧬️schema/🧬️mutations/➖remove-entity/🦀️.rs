//! ➖️ `remove-entity` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveEntity {
    pub id: u64,
}

impl protocol::MutationKind<IfcSnapshot, IfcMutation> for RemoveEntity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "entity", kind: "remove-entity", record: "RemoveEntity" };

    fn diff(&self, base: &IfcSnapshot) -> protocol::MutationOutcome<IfcDiff> {
        let Self { id } = self;
        protocol::MutationOutcome::new(diff::diff_remove_entity(*id))
    }
    fn inverse(&self, base: &IfcSnapshot) -> Result<Vec<IfcMutation>, semio_framework_value::ValueError> {
        let entity = |id: u64| base.entities.iter().find(|e| e.id == id);
        let Self { id } = self;
        Ok(vec![match base.entities.iter().position(|e| e.id == *id) {
            Some(index) => IfcMutation::InsertEntity(insert_entity::InsertEntity { index, entity: base.entities[index].clone() }),
            None => return Ok(Vec::new()),
        }])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove entity", "Entität entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
