//! 📏️ `scale` — sets a `Group` node's `transform.scale` (SMO-approved domain spatial transform).
//! Only `Group` carries a scale field -- every other node kind is honestly a no-op.

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::standards::v1::subsets::drawing::schema::diff::NodePath;
use crate::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ScaleNode {
    pub at: NodePath,
    pub new_scale: SemioPoint3,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for ScaleNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "node", kind: "scale-node", record: "ScaledNode" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Result<Vec<SemioDrawingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale node in layer #{}", self.at.layer), &format!("Knoten in Ebene #{} skalieren", self.at.layer))
    }
    fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}
//#endregion 🔖️Payload
