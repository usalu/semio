//! 🗑️ `remove-instance` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveInstance {
    pub id: u64,
}

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3Mutation> for RemoveInstance {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "instance", kind: "remove-instance", record: "RemoveInstance" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
        let Self { id } = self;
        protocol::MutationOutcome::new(if base.document.instance(*id).is_some() { Ifc2x3Diff { removed_instances: vec![*id], ..Default::default() } } else { Ifc2x3Diff::default() })
    }
    fn inverse(&self, base: &Ifc2x3Snapshot) -> Result<Vec<Ifc2x3Mutation>, semio_framework_value::ValueError> {
        let Self { id } = self;
        Ok(base.document.instances.iter().position(|instance| instance.id == *id).map(|index| Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance: base.document.instances[index].clone(), index: Some(index) })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove instance", "Instanz entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
