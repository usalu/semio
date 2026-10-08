//! ➖️ `remove-block` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveBlock {
    pub(crate) path: Vec<MdPathStep>,
    pub(crate) index: usize,
}

impl protocol::MutationKind<MdSnapshot, MdMutation> for RemoveBlock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "block", kind: "remove-block", record: "RemoveBlock" };

    fn diff(&self, base: &MdSnapshot) -> protocol::MutationOutcome<<MdMutation as Mutation<MdSnapshot>>::Diff> {
        let Self { path, index } = self;
        protocol::MutationOutcome::new(diff_at_path(path, *index, MdBlocksLeafDiff::Removed))
    }
    fn inverse(&self, base: &MdSnapshot) -> Result<Vec<MdMutation>, semio_framework_value::ValueError> {
        let Self { path, index } = self;
        Ok(match navigate_container(&base.blocks, path).and_then(|c| c.get(*index)).cloned() {
            Some(block) => vec![MdMutation::InsertBlock(insert_block::InsertBlock { path: path.clone(), index: *index, block })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove block", "Block entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
