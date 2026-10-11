//! 🖼️ Direct rewriting mutation — `EditWorkingGraph`: replaces the "before" working-graph body (a whole
//! `trinity.graph` snapshot, authored/computed as JSON).
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// 🖼️ `edit-working-graph` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "edit-working-graph")]
pub struct EditWorkingGraph {
    pub new_working_graph: semio_s_artifact_trinity_jack::JackSnapshot,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_working_graph(new_working_graph: semio_s_artifact_trinity_jack::JackSnapshot) -> RewriteRuleMutation {
    RewriteRuleMutation::EditWorkingGraph(EditWorkingGraph { new_working_graph })
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for EditWorkingGraph {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "working-graph", kind: "edit-working-graph", record: "EditedWorkingGraph" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Result<Vec<RewriteRuleMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Edit working-graph", "Ausgangszustand bearbeiten")
    }
}
//#endregion 🔖️Mutation
