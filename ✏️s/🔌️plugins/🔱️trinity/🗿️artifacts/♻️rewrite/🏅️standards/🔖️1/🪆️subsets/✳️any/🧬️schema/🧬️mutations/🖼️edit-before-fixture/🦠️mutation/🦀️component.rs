//! 🖼️ Rewrite mutation — `EditBeforeFixture`: replaces the "before" working-graph body (a whole
//! `trinity.graph` fixture, authored/computed as JSON).
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::mutations::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🖼️ `edit-before-fixture` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "edit-before-fixture")]
pub struct EditBeforeFixture {
    pub new_before_fixture_json: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_before_fixture(new_before_fixture_json: String) -> RewriteRuleMutation {
    RewriteRuleMutation::EditBeforeFixture(EditBeforeFixture { new_before_fixture_json })
}

impl protocol::MutationKind<RewriteSnapshot, RewriteRuleMutation> for EditBeforeFixture {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "before-fixture", kind: "edit-before-fixture", record: "EditedBeforeFixture" };

    fn diff(&self, base: &RewriteSnapshot) -> protocol::MutationOutcome<RewriteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewriteSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Edit before-fixture".to_string()
    }
}
//#endregion 🔖️Mutation
