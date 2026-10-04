//! 🫗️ Raster play app commands — `fill-selection`: one fill of the session pixel selection from the editing menu (the
//! whole image without one). A host names the layer; the command builds ONE parametric `fill-selection` leaf — the
//! layer, the session target, the session colour (the mask value at the brush opacity, filling the mask) and the session
//! selection — one edit, one history row whose colour and selection time travel edits. The image is filled by the leaf on
//! its base, never by the host (design §17.2, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use super::paint_stroke::{hex_color, paint_target_refusal, raster_fault, selection};
use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::mutations::fill_selection::{invariant, FillSelection as FillSelectionLeaf};
use crate::op::RasterMutation;
use crate::RasterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🫗️ One selection fill as both hosts dispatch it: the layer. Target, colour and selection are the session's.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "fill-selection")]
pub struct FillSelection {
    pub layer_id: String,
}

/// 🧱️ The one leaf a selection fill means on `document` under the session `config`, refused (zero trace) when the layer
/// cannot take it — no such layer, a hidden or locked one, a layer without pixels (or, filling the mask, without a
/// mask) — or when the session colour is not one the leaf admits.
pub fn fill_selection_leaf(payload: &FillSelection, document: &RasterSnapshot, config: &RasterConfig) -> Result<RasterMutation, Fault> {
    paint_target_refusal(&payload.layer_id, document, config).map_or(Ok(()), Err)?;
    let target = config.paint_target.as_str();
    let color = match target {
        "mask" => {
            let grey = f64::from(config.mask_value.min(255)) / 255.0;
            vec![grey, grey, grey, if config.brush_opacity.is_finite() { config.brush_opacity.clamp(0.0, 1.0) } else { 1.0 }]
        }
        _ => hex_color(&config.brush_color).ok_or_else(|| raster_fault("raster.brush.color-invalid"))?,
    };
    let leaf = FillSelectionLeaf { layer_id: payload.layer_id.clone(), target: target.to_string(), color, selection: selection(config, &payload.layer_id)? };
    invariant(&leaf).map_err(|(field, message)| Fault::new(FaultOrigin::App, FaultCode::new("raster.fill.invalid"), format!("{field}: {message}")))?;
    Ok(RasterMutation::FillSelection(leaf))
}

/// 🫗️ One selection fill: ONE edit holding its leaf, or a refusal that leaves zero trace.
pub fn handle(payload: &FillSelection, doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![fill_selection_leaf(payload, doc.snapshot, cfg.snapshot)?]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
