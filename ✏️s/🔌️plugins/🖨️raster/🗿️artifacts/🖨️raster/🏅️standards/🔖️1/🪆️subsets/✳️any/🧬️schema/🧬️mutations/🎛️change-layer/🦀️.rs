//! 🎛️ Conflict-checked, undoable editing of one non-destructive tone parameter.
use crate::{RasterAdjustmentParameter, RasterLayerNode, RasterLayerPatch, RasterMutation, RasterSnapshot};
use crate::diff::{diff_patch_layer, RasterDiff};
use crate::standards::v1::subsets::any::schema::find_layer;
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeLayerAdjustmentParameter {
    pub layer_id: String,
    pub parameter: String,
    pub expected: Option<crate::RasterAdjustmentNumber>,
    pub value: Option<crate::RasterAdjustmentNumber>,
}
pub fn validate(payload: &ChangeLayerAdjustmentParameter, base: &RasterSnapshot) -> Result<(), protocol::OutcomeCode> {
    let Some(RasterLayerNode::Adjustment { adjustment_kind, params, .. }) = find_layer(&base.layers, &payload.layer_id) else { return Err(protocol::OutcomeCode::TargetMissing); };
    if !matches!(payload.parameter.as_str(), "brightness" | "contrast") || [payload.expected,payload.value].into_iter().flatten().any(|value| !value.get().is_finite() || !(-1.0..=1.0).contains(&value.get())) { return Err(protocol::OutcomeCode::Invariant); }
    if adjustment_kind != "brightnessContrast" { return Err(protocol::OutcomeCode::TargetMismatch); }
    let previous = params.get(&payload.parameter);
    if previous.is_some_and(|value| value.as_f64().is_none()) || previous.and_then(crate::RasterAdjustmentNumber::from_parameter) != payload.expected { return Err(protocol::OutcomeCode::TargetMismatch); }
    if previous.is_none() && payload.value.is_some() && params.len() >= crate::RASTER_OWNED_MAP_CAPACITY { return Err(protocol::OutcomeCode::TargetMismatch); }
    Ok(())
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ChangeLayerAdjustmentParameter {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb:"change",entity:"layer-adjustment-parameter",kind:"change-layer-adjustment-parameter",record:"ChangedLayerAdjustmentParameter" };
    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        if let Err(code)=validate(self,base) { return protocol::MutationOutcome::refuse(code,"Adjustment parameter cannot be applied to this layer revision.",[self.layer_id.clone()]); }
        protocol::MutationOutcome::new(diff_patch_layer(&self.layer_id,RasterLayerPatch { adjustment_parameters:Some(vec![RasterAdjustmentParameter {parameter:self.parameter.clone(),value:self.value}]),..Default::default() }))
    }
    fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        if validate(self,base).is_err() { return Vec::new(); }
        vec![RasterMutation::ChangeLayerAdjustmentParameter(Self {layer_id:self.layer_id.clone(),parameter:self.parameter.clone(),expected:self.value,value:self.expected})]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {semio_framework_ui_locale::LocalizedLabel::native("Change adjustment parameter","Korrekturparameter ändern")}
    fn target(&self) -> Vec<String> {vec![self.layer_id.clone()]}
}
