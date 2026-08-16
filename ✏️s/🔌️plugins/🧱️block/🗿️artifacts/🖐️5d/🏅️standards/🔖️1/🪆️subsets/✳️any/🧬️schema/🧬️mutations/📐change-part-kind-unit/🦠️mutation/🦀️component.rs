//! 📐 Block5d mutation — `ChangePartKindUnit`: the part kind's optional `unit`.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📐 `change-part-kind-unit` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-part-kind-unit")]
pub struct ChangePartKindUnit {
    pub new_unit: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_part_kind_unit(new_unit: Option<String>) -> Block5dMutation {
    Block5dMutation::ChangePartKindUnit(ChangePartKindUnit { new_unit })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ChangePartKindUnit {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part-kind", kind: "change-part-kind-unit", record: "ChangedPartKindUnit" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change part kind unit to {:?}", self.new_unit)
    }
}
//#endregion 🔖️Mutation
