//! 👤️ Block2d mutation — `AddAuthor`: a credited author.

use semio_s_plugin_block::BlockAuthor;
use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Mutation
/// 👤️ `add-author` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "add-author")]
pub struct AddAuthor {
    #[dsl(block)]
    pub author: BlockAuthor,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_author<Op: From<crate::add_author::AddAuthor>>(author: BlockAuthor) -> Op {
    Op::from(AddAuthor { author, index: None })
}

/// 📍️ Builder — like [`add_author`] but inserts the row at `index`.
pub fn add_author_at<Op: From<crate::add_author::AddAuthor>>(author: BlockAuthor, index: u32) -> Op {
    Op::from(AddAuthor { author, index: Some(index) })
}

impl<Op> protocol::MutationKind<Block2dSnapshot, Op> for AddAuthor where Op: protocol::Mutation<Block2dSnapshot, Diff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::remove_author::RemoveAuthor> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "author", kind: "add-author", record: "AddedAuthor" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add author \"{}\"", self.author.name), &format!("Autor \"{}\" hinzufügen", self.author.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.author.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Block2dSnapshot, Op> for AddAuthor where Op: protocol::ColdMutation<Block2dSnapshot, ReferenceDiff = semio_s_artifact_block_2d_diff::Block2dDiff> + From<crate::remove_author::RemoveAuthor> {
    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        crate::add_author::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::add_author::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
