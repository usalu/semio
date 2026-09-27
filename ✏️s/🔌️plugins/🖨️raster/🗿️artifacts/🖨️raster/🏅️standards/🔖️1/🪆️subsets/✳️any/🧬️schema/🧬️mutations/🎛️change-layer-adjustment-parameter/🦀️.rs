//! 🎛️ Conflict-checked, undoable editing of one non-destructive tone parameter.
use crate::{RasterAdjustmentParameter, RasterLayerNode, RasterLayerPatch, RasterMutation, RasterSnapshot};
use crate::diff::{diff_patch_layer, RasterDiff};
use crate::standards::v1::subsets::any::schema::find_layer;
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeLayerAdjustmentParameter {
    pub layer_id: String,
    pub parameter: String,
    pub expected: Option<f64>,
    pub value: Option<f64>,
}
pub fn validate(payload: &ChangeLayerAdjustmentParameter, base: &RasterSnapshot) -> Result<(), &'static str> {
    let Some(RasterLayerNode::Adjustment { adjustment_kind, params, .. }) = find_layer(&base.layers, &payload.layer_id) else { return Err("mutation.target-missing"); };
    if adjustment_kind != "brightnessContrast" || !matches!(payload.parameter.as_str(), "brightness" | "contrast") || [payload.expected,payload.value].into_iter().flatten().any(|value| !value.is_finite() || !(-1.0..=1.0).contains(&value)) { return Err("mutation.adjustment-invalid"); }
    let previous = params.get(&payload.parameter);
    if previous.is_some_and(|value| value.as_f64().is_none()) || previous.and_then(dsl::DslValue::as_f64) != payload.expected { return Err("mutation.adjustment-conflict"); }
    if previous.is_none() && payload.value.is_some() && params.len() >= crate::RASTER_OWNED_MAP_CAPACITY { return Err("mutation.adjustment-capacity"); }
    Ok(())
}
impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ChangeLayerAdjustmentParameter {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb:"change",entity:"layer-adjustment-parameter",kind:"change-layer-adjustment-parameter",record:"ChangedLayerAdjustmentParameter" };
    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        if let Err(code)=validate(self,base) { return protocol::MutationOutcome::error(code,"Adjustment parameter cannot be applied to this layer revision.",[self.layer_id.clone()]); }
        protocol::MutationOutcome::new(diff_patch_layer(&self.layer_id,RasterLayerPatch { adjustment_parameter:Some(RasterAdjustmentParameter {parameter:self.parameter.clone(),value:self.value}),..Default::default() }))
    }
    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        if validate(self,base).is_err() { return Vec::new(); }
        vec![RasterMutation::ChangeLayerAdjustmentParameter(Self {layer_id:self.layer_id.clone(),parameter:self.parameter.clone(),expected:self.value,value:self.expected})]
    }
    fn label(&self) -> protocol::LocalizedLabel {protocol::LocalizedLabel::native("Change adjustment parameter","Korrekturparameter ändern")}
    fn target(&self) -> Vec<String> {vec![self.layer_id.clone()]}
}
