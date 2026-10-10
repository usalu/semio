//! 🔲️ `set-block` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetBlock {
    pub index: usize,
    pub block: DxfBlock,
}

impl protocol::MutationKind<DxfSnapshot, DxfMutation> for SetBlock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "block", kind: "set-block", record: "SetBlock" };

    fn diff(&self, base: &DxfSnapshot) -> protocol::MutationOutcome<<DxfMutation as Mutation<DxfSnapshot>>::Diff> {
        let Self { index, block } = self;
        protocol::MutationOutcome::new(match base.blocks.get(*index) {
            Some(old) => diff_set_block(*index, block_field_changes(old, block)),
            None => diff_insert_block(*index, block.clone()),
        })
    }
    fn inverse(&self, base: &DxfSnapshot) -> Result<Vec<DxfMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok({
            match base.blocks.get(*index) {
                Some(b) => vec![DxfMutation::SetBlock(set_block::SetBlock { index: *index, block: b.clone() })],
                None => vec![DxfMutation::RemoveBlock(remove_block::RemoveBlock { index: *index })],
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set block", "Block setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
