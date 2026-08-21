//! 👤 Block5d mutation — `AddAuthor`: a credited author.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::BlockAuthor;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 👤 `add-author` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-author")]
pub struct AddAuthor {
    #[dsl(block)]
    pub author: BlockAuthor,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn add_author(author: BlockAuthor) -> Block5dMutation {
    Block5dMutation::AddAuthor(AddAuthor { author })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for AddAuthor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "author", kind: "add-author", record: "AddedAuthor" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
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
