//! 🔁 Drawing mutation — `ReplaceLayerFill`: whole-value swap of one layer's structured `fill`
//! sub-payload (a tagged `FillStyle` union — solid/linear/radial — not a scalar).
use semio_s_artifact_draw_drawing_diff::DrawingDiff;

use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::FillStyle;

//#region 🔖️Mutation
/// 🔁 `replace-layer-fill` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree,semio_framework_value::NativeReceive)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "replace-layer-fill")]
pub struct ReplaceLayerFill {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(statements, block)]
    pub fill: Option<FillStyle>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_layer_fill<Op: From<ReplaceLayerFill>>(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, fill: Option<FillStyle>) -> Op {
    Op::from(ReplaceLayerFill { layer_id, fill })
}

impl<Op> protocol::MutationKind<DrawingSnapshot, Op> for ReplaceLayerFill where Op: protocol::Mutation<DrawingSnapshot, Diff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<ReplaceLayerFill> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "layer", kind: "replace-layer-fill", record: "ReplacedLayerFill" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace layer \"{}\" fill", self.layer_id), &format!("Füllung von Ebene \"{}\" ersetzen", self.layer_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.to_string_owner()]
    }
}

impl<Op> protocol::ColdMutationKind<DrawingSnapshot, Op> for ReplaceLayerFill where Op: protocol::ColdMutation<DrawingSnapshot, ReferenceDiff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<ReplaceLayerFill> {
    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
}
//#endregion 🔖️Mutation
