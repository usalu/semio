//! 🖼️ Direct rewriting mutation — `EditBeforeFixture`: replaces the "before" working-graph body (a whole
//! `trinity.graph` fixture, authored/computed as JSON).
use crate::artifacts::rewriting::diff::RewritingDiff;
use crate::artifacts::rewriting::mutations::RewriteRuleMutation;
use crate::artifacts::rewriting::RewritingSnapshot;

//#region 🔖️Mutation
/// 🖼️ `edit-before-fixture` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "edit-before-fixture")]
pub struct EditBeforeFixture {
    pub new_before_fixture_json: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_before_fixture(new_before_fixture_json: String) -> RewriteRuleMutation {
    RewriteRuleMutation::EditBeforeFixture(EditBeforeFixture { new_before_fixture_json })
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for EditBeforeFixture {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "before-fixture", kind: "edit-before-fixture", record: "EditedBeforeFixture" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Edit before-fixture".to_string()
    }
}
//#endregion 🔖️Mutation
