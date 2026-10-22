//! 🎛️ Conflict-checked, undoable editing of one non-destructive tone parameter.
use semio_s_artifact_raster_raster_model::{RasterAdjustmentParameter, RasterLayerNode, RasterLayerPatch};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_diff::{diff_patch_layer, RasterDiff};
use semio_s_artifact_raster_raster_model::find_layer;
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeLayerAdjustmentParameter {
    pub layer_id: String,
    pub parameter: String,
    pub expected: Option<semio_s_artifact_raster_raster_model::RasterAdjustmentNumber>,
    pub value: Option<semio_s_artifact_raster_raster_model::RasterAdjustmentNumber>,
}
pub fn validate(payload: &ChangeLayerAdjustmentParameter, base: &RasterSnapshot) -> Result<(), protocol::OutcomeCode> {
    let Some(RasterLayerNode::Adjustment { adjustment_kind, params, .. }) = find_layer(&base.layers, &payload.layer_id) else { return Err(protocol::OutcomeCode::TargetMissing); };
    if !matches!(payload.parameter.as_str(), "brightness" | "contrast") || [payload.expected,payload.value].into_iter().flatten().any(|value| !value.get().is_finite() || !(-1.0..=1.0).contains(&value.get())) { return Err(protocol::OutcomeCode::Invariant); }
    if adjustment_kind != "brightnessContrast" { return Err(protocol::OutcomeCode::TargetMismatch); }
    let previous = params.get(&payload.parameter);
    if previous.is_some_and(|value| value.as_f64().is_none()) || previous.and_then(semio_s_artifact_raster_raster_model::RasterAdjustmentNumber::from_parameter) != payload.expected { return Err(protocol::OutcomeCode::TargetMismatch); }
    if previous.is_none() && payload.value.is_some() && params.len() >= semio_s_artifact_raster_raster_model::RASTER_OWNED_MAP_CAPACITY { return Err(protocol::OutcomeCode::TargetMismatch); }
    Ok(())
}

impl<Op> protocol::MutationKind<RasterSnapshot, Op> for ChangeLayerAdjustmentParameter where Op: protocol::Mutation<RasterSnapshot, Diff = semio_s_artifact_raster_raster_diff::RasterDiff> + From<crate::change_layer_adjustment_parameter::ChangeLayerAdjustmentParameter> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb:"change",entity:"layer-adjustment-parameter",kind:"change-layer-adjustment-parameter",record:"ChangedLayerAdjustmentParameter" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {semio_framework_ui_locale::LocalizedLabel::native("Change adjustment parameter","Korrekturparameter ändern")}
    fn target(&self) -> Vec<String> {vec![self.layer_id.clone()]}
}

impl<Op> protocol::ColdMutationKind<RasterSnapshot, Op> for ChangeLayerAdjustmentParameter where Op: protocol::ColdMutation<RasterSnapshot, ReferenceDiff = semio_s_artifact_raster_raster_diff::RasterDiff> + From<crate::change_layer_adjustment_parameter::ChangeLayerAdjustmentParameter> {
    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        if let Err(code)=validate(self,base) { return protocol::MutationOutcome::refuse(code,"Adjustment parameter cannot be applied to this layer revision.",[self.layer_id.clone()]); }
        protocol::MutationOutcome::new(diff_patch_layer(&self.layer_id,RasterLayerPatch { adjustment_parameters:Some(vec![RasterAdjustmentParameter {parameter:self.parameter.clone(),value:self.value}]),..Default::default() }))
    }
    fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        if validate(self,base).is_err() { return Vec::new(); }
        vec![Op::from(Self {layer_id:self.layer_id.clone(),parameter:self.parameter.clone(),expected:self.value,value:self.expected})]
    
    })())
}
}
