//! 📝️ `set-entity-arg` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEntityArg {
    pub id: u64,
    pub index: usize,
    pub value: IfcValue,
}

impl protocol::MutationKind<IfcSnapshot, IfcMutation> for SetEntityArg {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "entity-arg", kind: "set-entity-arg", record: "SetEntityArg" };

    fn diff(&self, base: &IfcSnapshot) -> protocol::MutationOutcome<IfcDiff> {
        let Self { id, index, value } = self;
        protocol::MutationOutcome::new(diff::diff_set_entity_arg(*id, *index, value.clone()))
    }
    fn inverse(&self, base: &IfcSnapshot) -> Result<Vec<IfcMutation>, semio_framework_value::ValueError> {
        let entity = |id: u64| base.entities.iter().find(|e| e.id == id);
        let Self { id, index, .. } = self;
        Ok(vec![match entity(*id).and_then(|e| e.args.get(*index)) {
            Some(v) => IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: *id, index: *index, value: v.clone() }),
            None => return Ok(Vec::new()),
        }])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set entity arg", "Entitätsargument setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
