//! 🧱️ `upsert-instance` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct UpsertInstance {
    pub instance: Part21Instance,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3Mutation> for UpsertInstance {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "instance", kind: "upsert-instance", record: "UpsertInstance" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
        let Self { instance, index } = self;
        let order = || base.document.instances.iter().map(|existing| existing.id).collect::<Vec<_>>();
        protocol::MutationOutcome::new(match base.document.instance(instance.id) {
            Some(existing) if existing == instance => Ifc2x3Diff::default(),
            Some(_) => Ifc2x3Diff { upserted_instances: vec![instance.clone()], ..Default::default() },
            None => {
                let mut diff = Ifc2x3Diff { upserted_instances: vec![instance.clone()], ..Default::default() };
                if let Some(at) = index.filter(|at| *at < base.document.instances.len()) {
                    let mut ids = order();
                    ids.insert(at, instance.id);
                    diff.instance_order = Some(ids);
                }
                diff
            }
        })
    }
    fn inverse(&self, base: &Ifc2x3Snapshot) -> Result<Vec<Ifc2x3Mutation>, semio_framework_value::ValueError> {
        let Self { instance, .. } = self;
        Ok(vec![match base.document.instance(instance.id) {
            Some(existing) => Ifc2x3Mutation::UpsertInstance(Self { instance: existing.clone(), index: None }),
            None => Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id: instance.id }),
        }])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Upsert instance", "Instanz einfügen oder aktualisieren")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
