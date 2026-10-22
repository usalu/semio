//! 🔄️ `rotate` — sets a `Group` node's `transform.rotation` (SMO-approved domain spatial
//! transform). Only `Group` carries a rotation field -- `Path`/`Text`/`Image` are honestly a
//! no-op, matching `move-node`'s own reach limits.

use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
use crate::schema::diff::NodePath;
use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RotateNode {
    pub at: NodePath,
    pub new_rotation: SemioQuaternion,
}

impl<Op> protocol::MutationKind<SemioDrawingSnapshot, Op> for RotateNode
where
    Op: protocol::Mutation<SemioDrawingSnapshot, Diff = crate::schema::diff::SemioDrawingDiff> + From<RotateNode>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "node", kind: "rotate-node", record: "RotatedNode" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate node in layer #{}", self.at.layer), &format!("Knoten in Ebene #{} drehen", self.at.layer))
    }
    fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioDrawingSnapshot, Op> for RotateNode
where
    Op: protocol::ColdMutation<SemioDrawingSnapshot, ReferenceDiff = crate::schema::diff::SemioDrawingDiff> + From<RotateNode>,
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
