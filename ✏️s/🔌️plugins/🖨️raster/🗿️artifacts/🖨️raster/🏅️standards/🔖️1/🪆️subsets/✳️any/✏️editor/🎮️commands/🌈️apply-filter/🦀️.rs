//! 🌈️ Raster play app commands — `apply-filter`: one image filter from the editing menu. A host names the layer, the filter
//! and its amount; the command builds ONE parametric `apply-filter` leaf on the layer's pixels with the session pixel
//! selection (a flip mirrors the whole image) — one edit, one history row whose filter and amount time travel edits. A
//! filter changes colour channels, so a session painting the mask is refused. The image is
//! filtered by the leaf on its base, never by the host (design §17.2, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use super::paint_stroke::{paint_target_refusal, selection};
use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::mutations::apply_filter::{invariant, ApplyFilter as ApplyFilterLeaf};
use crate::op::RasterMutation;
use crate::RasterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🌈️ One menu filter as both hosts dispatch it: the layer, the filter and its amount (0 for a filter without one).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "apply-filter")]
pub struct ApplyFilter {
    pub layer_id: String,
    pub filter: String,
    pub amount: f64,
}

/// 🧱️ The one leaf a menu filter means on `document` under the session `config`, refused (zero trace) when the session
/// paints the mask, when the layer cannot take it — no such layer, a hidden or locked one, a layer without pixels — or when
/// the filter or its amount is not one the leaf admits.
pub fn apply_filter_leaf(payload: &ApplyFilter, document: &RasterSnapshot, config: &RasterConfig) -> Result<RasterMutation, Fault> {
    if config.paint_target != "pixels" {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("raster.filter.pixels-only"), "A filter changes the colours of a layer's pixels; switch the edit target to pixels."));
    }
    paint_target_refusal(&payload.layer_id, document, config).map_or(Ok(()), Err)?;
    let whole = matches!(payload.filter.as_str(), "flipHorizontal" | "flipVertical");
    let leaf = ApplyFilterLeaf { layer_id: payload.layer_id.clone(), filter: payload.filter.clone(), amount: payload.amount, selection: if whole { None } else { selection(config, &payload.layer_id)? } };
    invariant(&leaf).map_err(|(field, message)| Fault::new(FaultOrigin::App, FaultCode::new("raster.filter.invalid"), format!("{field}: {message}")))?;
    Ok(RasterMutation::ApplyFilter(leaf))
}

/// 🌈️ One menu filter: ONE edit holding its leaf, or a refusal that leaves zero trace.
pub fn handle(payload: &ApplyFilter, doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![apply_filter_leaf(payload, doc.snapshot, cfg.snapshot)?]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
