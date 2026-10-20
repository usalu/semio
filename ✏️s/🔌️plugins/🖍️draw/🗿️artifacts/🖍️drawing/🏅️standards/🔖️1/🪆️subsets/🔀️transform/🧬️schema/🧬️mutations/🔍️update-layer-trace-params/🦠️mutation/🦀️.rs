//! 🔧 Drawing mutation — `UpdateLayerTraceParams`: sets a trace layer's `params` facet (threshold +
//! simplify epsilon, always validated/persisted together — the `update` verb's cohesive-facet case).
use semio_s_artifact_draw_drawing_diff::DrawingDiff;

use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::DrawingTraceParams;

//#region 🔖️Mutation
/// 🔧 `update-layer-trace-params` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "update-layer-trace-params")]
pub struct UpdateLayerTraceParams {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[dsl(block)]
    pub params: DrawingTraceParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_layer_trace_params<Op: From<UpdateLayerTraceParams>>(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, params: DrawingTraceParams) -> Op {
    Op::from(UpdateLayerTraceParams { layer_id, params })
}

impl<Op> protocol::MutationKind<DrawingSnapshot, Op> for UpdateLayerTraceParams where Op: protocol::Mutation<DrawingSnapshot, Diff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<UpdateLayerTraceParams> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "layer", kind: "update-layer-trace-params", record: "UpdatedLayerTraceParams" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update layer \"{}\" trace params", self.layer_id), &format!("Nachzeichnungsparameter von Ebene \"{}\" aktualisieren", self.layer_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
