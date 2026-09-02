//! 👤️ Block2d mutation — `AddAuthor`: a credited author.

use crate::BlockAuthor;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::{Block2dAuthorList, Block2dDiff};
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Mutation
/// 👤️ `add-author` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "add-author")]
pub struct AddAuthor {
    #[dsl(block)]
    pub author: BlockAuthor,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn add_author(author: BlockAuthor) -> Block2dMutation {
    Block2dMutation::AddAuthor(AddAuthor { author })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for AddAuthor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "author", kind: "add-author", record: "AddedAuthor" };

    async fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Add author \"{}\"", self.author.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.author.id.clone()]
    }
}
//#endregion 🔖️Mutation
