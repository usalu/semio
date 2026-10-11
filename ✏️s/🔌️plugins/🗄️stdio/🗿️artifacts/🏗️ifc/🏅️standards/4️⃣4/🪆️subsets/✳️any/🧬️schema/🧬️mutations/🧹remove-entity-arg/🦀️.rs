//! 🧹️ `remove-entity-arg` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveEntityArg {
    pub id: u64,
    pub index: usize,
}

impl protocol::MutationKind<IfcSnapshot, IfcMutation> for RemoveEntityArg {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "entity-arg", kind: "remove-entity-arg", record: "RemoveEntityArg" };

    fn diff(&self, base: &IfcSnapshot) -> protocol::MutationOutcome<IfcDiff> {
        let Self { id, index } = self;
        protocol::MutationOutcome::new(diff::diff_remove_entity_arg(*id, *index))
    }
    fn inverse(&self, base: &IfcSnapshot) -> Result<Vec<IfcMutation>, semio_framework_value::ValueError> {
        let entity = |id: u64| base.entities.iter().find(|e| e.id == id);
        let Self { id, index } = self;
        Ok(vec![match entity(*id).and_then(|e| e.args.get(*index)) {
            Some(v) => IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: *id, index: *index, value: v.clone() }),
            None => return Ok(Vec::new()),
        }])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove entity arg", "Entitätsargument entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
