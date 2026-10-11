//! ➕️ `insert-block` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertBlock {
    pub(crate) path: Vec<MdPathStep>,
    pub(crate) index: usize,
    pub(crate) block: MdBlock,
}

impl protocol::MutationKind<MdSnapshot, MdMutation> for InsertBlock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "block", kind: "insert-block", record: "InsertBlock" };

    fn diff(&self, base: &MdSnapshot) -> protocol::MutationOutcome<<MdMutation as Mutation<MdSnapshot>>::Diff> {
        let Self { path, index, block } = self;
        protocol::MutationOutcome::new(diff_at_path(path, *index, MdBlocksLeafDiff::Added(block.clone())))
    }
    fn inverse(&self, base: &MdSnapshot) -> Result<Vec<MdMutation>, semio_framework_value::ValueError> {
        let Self { path, index, .. } = self;
        Ok(vec![MdMutation::RemoveBlock(remove_block::RemoveBlock { path: path.clone(), index: *index })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert block", "Block einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
