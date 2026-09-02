//! 🆔 Puzzle2d mutation — `ChangeManifestId`: changes the fixture's catalog-manifest reference.

use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// 🆔 `change-manifest-id` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-manifest-id")]
pub struct ChangeManifestId {
    pub new_manifest_id: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_manifest_id(new_manifest_id: Option<String>) -> Puzzle2dMutation {
    Puzzle2dMutation::ChangeManifestId(ChangeManifestId { new_manifest_id })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ChangeManifestId {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "manifest-id", kind: "change-manifest-id", record: "ChangedManifestId" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Change manifest id".to_string()
    }
}
//#endregion 🔖️Mutation
