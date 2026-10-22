//! 🌀️ Sets a layer's authored compound-path fill rule.
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

use semio_s_artifact_draw_drawing_diff::DrawingDiff;
use semio_s_artifact_draw_drawing_model::fill_rule::FillRule;
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract=::protocol)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all="camelCase")]
#[cfg_attr(any(test, feature = "testkit"),serde(rename_all="camelCase"))]
#[dsl(keyword="set-layer-fill-rule")]
pub struct SetLayerFillRule {pub layer_id:semio_framework_value::paged::PagedUtf8<{usize::MAX}>,pub fill_rule:FillRule}
pub fn set_layer_fill_rule<Op: From<SetLayerFillRule>>(layer_id:semio_framework_value::paged::PagedUtf8<{usize::MAX}>,fill_rule:FillRule) -> Op {Op::from(SetLayerFillRule {layer_id,fill_rule})}
impl<Op> protocol::MutationKind<DrawingSnapshot, Op> for SetLayerFillRule where Op: protocol::Mutation<DrawingSnapshot, Diff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<SetLayerFillRule> {
    const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor {verb:"set",entity:"layer",kind:"set-layer-fill-rule",record:"SetLayerFillRule"};

    fn label(&self)->semio_framework_ui_locale::LocalizedLabel {semio_framework_ui_locale::LocalizedLabel::native(&format!("Set layer \"{}\" fill rule to {}",self.layer_id,self.fill_rule.as_str()),&format!("Füllregel von Ebene \"{}\" auf {} setzen",self.layer_id,self.fill_rule.as_str()))}
    fn target(&self)->Vec<String> {vec![self.layer_id.to_string_owner()]}
}

impl<Op> protocol::ColdMutationKind<DrawingSnapshot, Op> for SetLayerFillRule where Op: protocol::ColdMutation<DrawingSnapshot, ReferenceDiff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<SetLayerFillRule> {
    fn diff(&self,base:&DrawingSnapshot)->protocol::MutationOutcome<DrawingDiff> {super::diff::diff(self,base)}
    fn inverse(&self,base:&DrawingSnapshot)-> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
}
