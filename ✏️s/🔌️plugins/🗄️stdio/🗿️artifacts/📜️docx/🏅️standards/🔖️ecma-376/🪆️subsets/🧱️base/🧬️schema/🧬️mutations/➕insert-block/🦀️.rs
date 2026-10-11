//! ➕️ `insert-block` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertBlock {
    pub(crate) path: DocxBlockPath,
    pub(crate) block: DocxBlock,
}

impl protocol::MutationKind<DocxSnapshot, DocxMutation> for InsertBlock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "block", kind: "insert-block", record: "InsertBlock" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<DocxDiff> {
        plan_outcome(insert_block_plan(base, &self.path, &self.block))
    }

    fn inverse(&self, base: &DocxSnapshot) -> Result<Vec<DocxMutation>, semio_framework_value::ValueError> {
        Ok(plan_inverse(insert_block_plan(base, &self.path, &self.block)))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert block", "Block einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
