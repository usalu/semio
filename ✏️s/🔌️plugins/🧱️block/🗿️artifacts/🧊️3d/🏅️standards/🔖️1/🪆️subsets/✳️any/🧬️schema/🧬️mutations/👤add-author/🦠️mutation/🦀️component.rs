//! 👤 Block3d mutation — `AddAuthor`: a credited author.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::{BlockAuthor};
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
pub fn add_author(author: BlockAuthor) -> Block3dMutation {
    Block3dMutation::AddAuthor(AddAuthor { author })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for AddAuthor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "author", kind: "add-author", record: "AddedAuthor" };

    fn diff(&self, base: &Block3dSnapshot) -> Block3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
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
