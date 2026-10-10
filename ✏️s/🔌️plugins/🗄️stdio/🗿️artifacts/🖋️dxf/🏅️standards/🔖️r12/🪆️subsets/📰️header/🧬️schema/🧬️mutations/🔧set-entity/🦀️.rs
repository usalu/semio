//! 🔧️ `set-entity` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEntity {
    pub index: usize,
    pub entity: DxfEntity,
}

impl protocol::MutationKind<DxfSnapshot, DxfMutation> for SetEntity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "entity", kind: "set-entity", record: "SetEntity" };

    fn diff(&self, base: &DxfSnapshot) -> protocol::MutationOutcome<<DxfMutation as Mutation<DxfSnapshot>>::Diff> {
        let Self { index, entity } = self;
        protocol::MutationOutcome::new(match base.entities.get(*index) {
            Some(old) => diff_set_entity(*index, entity_field_changes(old, entity)),
            None => diff_insert_entity(*index, entity.clone()),
        })
    }
    fn inverse(&self, base: &DxfSnapshot) -> Result<Vec<DxfMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok({
            match base.entities.get(*index) {
                Some(e) => vec![DxfMutation::SetEntity(set_entity::SetEntity { index: *index, entity: e.clone() })],
                None => vec![DxfMutation::RemoveEntity(remove_entity::RemoveEntity { index: *index })],
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set entity", "Entität setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
