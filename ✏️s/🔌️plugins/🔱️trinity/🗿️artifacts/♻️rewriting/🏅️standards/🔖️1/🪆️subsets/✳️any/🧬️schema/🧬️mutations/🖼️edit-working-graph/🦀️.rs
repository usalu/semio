//! 🖼️ Direct rewriting mutation — `EditWorkingGraph`: replaces the "before" working-graph body (a whole
//! `trinity.graph` snapshot, authored/computed as JSON).
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;

//#region 🔖️Mutation
/// 🖼️ `edit-working-graph` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "edit-working-graph")]
pub struct EditWorkingGraph {
    pub new_working_graph: semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_working_graph<Op>(new_working_graph: semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::edit_working_graph::EditWorkingGraph>,
{
    Op::from(EditWorkingGraph { new_working_graph })
}

impl<Op> protocol::MutationKind<RewritingSnapshot, Op> for EditWorkingGraph
where
    Op: protocol::Mutation<RewritingSnapshot, Diff = RewritingDiff> + From<EditWorkingGraph>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "working-graph", kind: "edit-working-graph", record: "EditedWorkingGraph" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Edit working-graph", "Ausgangszustand bearbeiten")
    }
}

impl<Op> protocol::ColdMutationKind<RewritingSnapshot, Op> for EditWorkingGraph
where
    Op: protocol::ColdMutation<RewritingSnapshot, ReferenceDiff = RewritingDiff> + From<EditWorkingGraph>,
{
    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
