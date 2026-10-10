//! 🧩️ `insert-entity-arg` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertEntityArg {
    pub id: u64,
    pub index: usize,
    pub value: IfcValue,
}

impl protocol::MutationKind<IfcSnapshot, IfcMutation> for InsertEntityArg {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "entity-arg", kind: "insert-entity-arg", record: "InsertEntityArg" };

    fn diff(&self, base: &IfcSnapshot) -> protocol::MutationOutcome<IfcDiff> {
        let Self { id, index, value } = self;
        protocol::MutationOutcome::new(diff::diff_insert_entity_arg(*id, *index, value.clone()))
    }
    fn inverse(&self, base: &IfcSnapshot) -> Result<Vec<IfcMutation>, semio_framework_value::ValueError> {
        let Self { id, index, .. } = self;
        Ok(vec![IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: *id, index: *index })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert entity arg", "Entitätsargument einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
