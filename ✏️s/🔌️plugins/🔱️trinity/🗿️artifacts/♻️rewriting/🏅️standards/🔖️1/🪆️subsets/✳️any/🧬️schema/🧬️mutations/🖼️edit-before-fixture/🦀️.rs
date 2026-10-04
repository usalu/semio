//! 🖼️ Direct rewriting mutation — `EditBeforeFixture`: replaces the "before" working-graph body (a whole
//! `trinity.graph` fixture, authored/computed as JSON).
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// 🖼️ `edit-before-fixture` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "edit-before-fixture")]
pub struct EditBeforeFixture {
    pub new_working_graph: semio_s_artifact_trinity_jack::JackSnapshot,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_before_fixture(new_working_graph: semio_s_artifact_trinity_jack::JackSnapshot) -> RewriteRuleMutation {
    RewriteRuleMutation::EditBeforeFixture(EditBeforeFixture { new_working_graph })
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for EditBeforeFixture {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "before-fixture", kind: "edit-before-fixture", record: "EditedBeforeFixture" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Result<Vec<RewriteRuleMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Edit before-fixture", "Ausgangszustand bearbeiten")
    }
}
//#endregion 🔖️Mutation
