//! 📍️ `set-node-position` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::diff::SemioFlowDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioFlowSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetNodePosition {
    pub id: String,
    pub position: SemioPoint2,
}

impl<Op> protocol::MutationKind<SemioFlowSnapshot, Op> for SetNodePosition
where
    Op: protocol::Mutation<SemioFlowSnapshot, Diff = crate::schema::diff::SemioFlowDiff> + From<SetNodePosition>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "node-position", kind: "set-node-position", record: "SetNodePosition" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set node position", "Knotenposition setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioFlowSnapshot, Op> for SetNodePosition
where
    Op: protocol::ColdMutation<SemioFlowSnapshot, ReferenceDiff = crate::schema::diff::SemioFlowDiff> + From<SetNodePosition>,
{
    fn diff(&self, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioFlowSnapshot>>::ReferenceDiff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioFlowSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload

//#region 🪢️TaxonomyMounts
#[path = "🔺️diff/🦀️.rs"]
mod diff;
#[path = "↩️inverse/🦀️.rs"]
mod inverse;
//#endregion 🪢️TaxonomyMounts
