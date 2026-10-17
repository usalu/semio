//! 🗑️ Drawing mutation — `DeleteLayer`: removes an id-keyed layer (captures its full subtree +
//! location for undo).
use semio_s_artifact_draw_drawing_diff::DrawingDiff;

use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Mutation
/// 🗑️ `delete-layer` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "delete-layer")]
pub struct DeleteLayer {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_layer<Op: From<DeleteLayer>>(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>) -> Op {
    Op::from(DeleteLayer { layer_id })
}

impl<Op> protocol::MutationKind<DrawingSnapshot, Op> for DeleteLayer where Op: protocol::Mutation<DrawingSnapshot, Diff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<crate::create_layer::mutation::CreateLayer> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "layer", kind: "delete-layer", record: "DeletedLayer" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete layer \"{}\"", self.layer_id), &format!("Ebene \"{}\" löschen", self.layer_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
