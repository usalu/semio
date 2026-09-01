//! 🌱 Puzzle5d mutation — `CreatePart`: brings a new id-keyed part into existence.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::{Puzzle5dPart, Puzzle5dSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱 `create-part` payload — full initial payload at an optional FINAL-state `index` (`None`
/// appends). A duplicate `part.id` is a no-op.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-part")]
pub struct CreatePart {
    #[dsl(block)]
    pub part: Puzzle5dPart,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_part(part: Puzzle5dPart, index: Option<usize>) -> Puzzle5dMutation {
    Puzzle5dMutation::CreatePart(CreatePart { part, index })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for CreatePart {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "part", kind: "create-part", record: "CreatedPart" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create part \"{}\"", self.part.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.part.id.clone()]
    }
}
//#endregion 🔖️Mutation
