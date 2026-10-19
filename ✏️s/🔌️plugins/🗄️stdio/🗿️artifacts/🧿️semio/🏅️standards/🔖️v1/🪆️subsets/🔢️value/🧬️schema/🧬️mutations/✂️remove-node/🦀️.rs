//! ✂️ `remove-node` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::diff::SemioValueTreeDiff;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::ValueId;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveNode {
    pub id: ValueId,
}

impl<Op> protocol::MutationKind<SemioValueSnapshot, Op> for RemoveNode
where
    Op: protocol::Mutation<SemioValueSnapshot, Diff = SemioValueTreeDiff> + From<RemoveNode>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "node", kind: "remove-node", record: "RemoveNode" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove node", "Knoten entfernen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioValueSnapshot, Op> for RemoveNode
where
    Op: protocol::ColdMutation<SemioValueSnapshot, ReferenceDiff = SemioValueTreeDiff> + From<RemoveNode> + From<crate::set_node::SetNode>,
{
    fn diff(&self, base: &SemioValueSnapshot) -> protocol::MutationOutcome<SemioValueTreeDiff> {
        diff::diff(self, base)
    }

    fn inverse(&self, base: &SemioValueSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
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
