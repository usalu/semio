//! 🪓️ `remove-block` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveBlock {
    pub index: usize,
}

impl protocol::MutationKind<DxfSnapshot, DxfMutation> for RemoveBlock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "block", kind: "remove-block", record: "RemoveBlock" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove block", "Block entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<DxfSnapshot, DxfMutation> for RemoveBlock {
    fn diff(&self, _base: &DxfSnapshot) -> protocol::MutationOutcome<<DxfMutation as protocol::ColdMutation<DxfSnapshot>>::ReferenceDiff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(diff_remove_block(*index))
    }
    fn inverse(&self, base: &DxfSnapshot) -> Result<Vec<DxfMutation>, semio_framework_value::ValueError> {
        let Self { index } = self;
        Ok({
            match base.blocks.get(*index) {
                Some(b) => vec![DxfMutation::InsertBlock(insert_block::InsertBlock { index: *index, block: b.clone() })],
                None => Vec::new(),
            }
        })
    }
}
//#endregion 🔖️Payload
