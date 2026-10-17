//! ♻️ Drawing mutation — `ReplaceLayerStroke`: whole-value swap of one layer's structured `stroke`
//! sub-payload.
use semio_s_artifact_draw_drawing_diff::DrawingDiff;

use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::StrokeStyle;

//#region 🔖️Mutation
/// ♻️ `replace-layer-stroke` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "replace-layer-stroke")]
pub struct ReplaceLayerStroke {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub stroke: Option<StrokeStyle>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_layer_stroke<Op: From<ReplaceLayerStroke>>(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, stroke: Option<StrokeStyle>) -> Op {
    Op::from(ReplaceLayerStroke { layer_id, stroke })
}

impl<Op> protocol::MutationKind<DrawingSnapshot, Op> for ReplaceLayerStroke where Op: protocol::Mutation<DrawingSnapshot, Diff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<ReplaceLayerStroke> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "layer", kind: "replace-layer-stroke", record: "ReplacedLayerStroke" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace layer \"{}\" stroke", self.layer_id), &format!("Kontur von Ebene \"{}\" ersetzen", self.layer_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
