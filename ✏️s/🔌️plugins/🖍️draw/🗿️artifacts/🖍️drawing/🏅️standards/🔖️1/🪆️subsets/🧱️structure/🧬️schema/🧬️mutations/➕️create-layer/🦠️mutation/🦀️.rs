//! 🌱 Drawing mutation — `CreateLayer`: brings a new id-keyed layer into existence at an address
//! (root when `parent_id` is `None`, `index` FINAL-state — appends when `None`).
use semio_s_artifact_draw_drawing_diff::DrawingDiff;

use semio_s_artifact_draw_drawing_model::DrawingLayerNode;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Mutation
/// 🌱 `create-layer` payload — full initial payload plus optional (parent, index) address.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-layer")]
pub struct CreateLayer {
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    pub parent_id: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    pub index: Option<usize>,
    #[dsl(statements)]
    pub layer: Box<DrawingLayerNode>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_layer<Op: From<CreateLayer>>(parent_id: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>, index: Option<usize>, layer: DrawingLayerNode) -> Op {
    Op::from(CreateLayer { parent_id, index, layer: Box::new(layer) })
}

impl<Op> protocol::MutationKind<DrawingSnapshot, Op> for CreateLayer where Op: protocol::Mutation<DrawingSnapshot, Diff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<crate::delete_layer::mutation::DeleteLayer> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "layer", kind: "create-layer", record: "CreatedLayer" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create layer \"{}\"", semio_s_artifact_draw_drawing_model::layer_id(&self.layer)), &format!("Ebene \"{}\" erstellen", semio_s_artifact_draw_drawing_model::layer_id(&self.layer)))
    }
    fn target(&self) -> Vec<String> {
        vec![semio_s_artifact_draw_drawing_model::layer_id(&self.layer).to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<DrawingSnapshot, Op> for CreateLayer where Op: protocol::ColdMutation<DrawingSnapshot, ReferenceDiff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<crate::delete_layer::mutation::DeleteLayer> {
    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
}
//#endregion 🔖️Mutation
