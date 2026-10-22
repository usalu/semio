//! 📏️ `scale` — sets a `Group` node's `transform.scale` (SMO-approved domain spatial transform).
//! Only `Group` carries a scale field -- every other node kind is honestly a no-op.

use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use crate::schema::diff::NodePath;
use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ScaleNode {
    pub at: NodePath,
    pub new_scale: SemioPoint3,
}

impl<Op> protocol::MutationKind<SemioDrawingSnapshot, Op> for ScaleNode
where
    Op: protocol::Mutation<SemioDrawingSnapshot, Diff = crate::schema::diff::SemioDrawingDiff> + From<ScaleNode>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "node", kind: "scale-node", record: "ScaledNode" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale node in layer #{}", self.at.layer), &format!("Knoten in Ebene #{} skalieren", self.at.layer))
    }
    fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioDrawingSnapshot, Op> for ScaleNode
where
    Op: protocol::ColdMutation<SemioDrawingSnapshot, ReferenceDiff = crate::schema::diff::SemioDrawingDiff> + From<ScaleNode>,
{
    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioDrawingSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
