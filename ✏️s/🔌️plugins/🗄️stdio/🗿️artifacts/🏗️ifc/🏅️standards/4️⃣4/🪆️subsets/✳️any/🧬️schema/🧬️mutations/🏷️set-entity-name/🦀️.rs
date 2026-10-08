//! 🏷️ `set-entity-name` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEntityName {
    pub id: u64,
    pub name: String,
}

impl protocol::MutationKind<IfcSnapshot, IfcMutation> for SetEntityName {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "entity-name", kind: "set-entity-name", record: "SetEntityName" };

    fn diff(&self, base: &IfcSnapshot) -> protocol::MutationOutcome<IfcDiff> {
        let Self { id, name } = self;
        protocol::MutationOutcome::new(diff::diff_set_entity_name(*id, name))
    }
    fn inverse(&self, base: &IfcSnapshot) -> Result<Vec<IfcMutation>, semio_framework_value::ValueError> {
        let entity = |id: u64| base.entities.iter().find(|e| e.id == id);
        let Self { id, .. } = self;
        Ok(vec![match entity(*id) {
            Some(e) => IfcMutation::SetEntityName(set_entity_name::SetEntityName { id: *id, name: e.name.clone() }),
            None => return Ok(Vec::new()),
        }])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set entity name", "Entitätsname setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
