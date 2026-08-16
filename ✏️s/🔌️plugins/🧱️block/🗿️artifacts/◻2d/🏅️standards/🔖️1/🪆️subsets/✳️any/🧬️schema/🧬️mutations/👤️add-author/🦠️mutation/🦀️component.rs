//! 👤️ Block2d mutation — `AddAuthor`: a credited author.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::{BlockAuthor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 👤️ `add-author` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-author")]
pub struct AddAuthor {
    #[dsl(block)]
    pub author: BlockAuthor,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_author(author: BlockAuthor) -> Block2dMutation {
    Block2dMutation::AddAuthor(AddAuthor { author })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for AddAuthor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "author", kind: "add-author", record: "AddedAuthor" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add author \"{}\"", self.author.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.author.id.clone()]
    }
}
//#endregion 🔖️Mutation
