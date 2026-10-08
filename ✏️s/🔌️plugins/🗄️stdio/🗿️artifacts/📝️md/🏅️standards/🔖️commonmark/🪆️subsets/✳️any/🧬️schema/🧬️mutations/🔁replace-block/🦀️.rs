//! 🔁️ `replace-block` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceBlock {
    pub(crate) path: Vec<MdPathStep>,
    pub(crate) index: usize,
    pub(crate) block: MdBlock,
}

impl protocol::MutationKind<MdSnapshot, MdMutation> for ReplaceBlock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "block", kind: "replace-block", record: "ReplaceBlock" };

    fn diff(&self, base: &MdSnapshot) -> protocol::MutationOutcome<<MdMutation as Mutation<MdSnapshot>>::Diff> {
        let Self { path, index, block } = self;
        protocol::MutationOutcome::new(diff_at_path(path, *index, MdBlocksLeafDiff::Modified(MdBlockDiff::Replace { block: block.clone() })))
    }
    fn inverse(&self, base: &MdSnapshot) -> Result<Vec<MdMutation>, semio_framework_value::ValueError> {
        let Self { path, index, .. } = self;
        Ok(match navigate_container(&base.blocks, path).and_then(|c| c.get(*index)).cloned() {
            Some(block) => vec![MdMutation::ReplaceBlock(replace_block::ReplaceBlock { path: path.clone(), index: *index, block })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace block", "Block ersetzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
