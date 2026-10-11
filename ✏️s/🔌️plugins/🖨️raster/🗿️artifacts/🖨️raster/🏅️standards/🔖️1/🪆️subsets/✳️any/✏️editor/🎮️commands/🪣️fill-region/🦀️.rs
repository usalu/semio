//! 🪣️ Raster play app commands — `fill-region`: the bucket TOOL. A host dispatches one click in the layer image's pixels;
//! the tool builds ONE parametric `fill-region` leaf — the layer, the target, the session colour
//! (or mask value), the seed pixel, the session tolerance and the session pixel selection the region is clipped to — and publishes
//! it as ONE `ToolTransaction` through the one-shot pixel tool the brush runs (`🖌️paint-stroke`): one edit, one history
//! row, whose seed, tolerance and colour time travel edits. The region is flooded by the leaf on its base, never by the
//! host. Tool state is never history; the yielded leaf is (design §5, §17.2, ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use super::paint_stroke::{hex_color, paint_target_refusal, raster_fault, raster_tool_commit, selection};
use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::mutations::fill_region::{FillRegion as FillRegionLeaf, RasterSeed};
use crate::op::RasterMutation;
use crate::RasterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🪪️ The editor whose bucket tool authors every fill transaction: `<appId>#fillRegion`.
pub const RASTER_FILL_TOOL_ID: &str = "s.raster.raster@1/*#editor#fillRegion";

/// 🪣️ One bucket click as both hosts dispatch it: the layer and the clicked point in its image's pixels. The colour
/// tolerance is the session's (`setFillTolerance`), one value both hosts read.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "fill-region")]
pub struct FillRegion {
    pub layer_id: String,
    pub x: f64,
    pub y: f64,
}

/// 🧱️ The one leaf a bucket click means on `document` under the session `config`, refused (zero trace) when the layer
/// cannot take it: no such layer, a hidden or locked one, a layer without pixels (or, filling the mask, without a mask),
/// a point off the image's pixel grid or a session tolerance past 255.
pub fn fill_region_leaf(payload: &FillRegion, document: &RasterSnapshot, config: &RasterConfig) -> Result<RasterMutation, Fault> {
    if !(payload.x.is_finite() && payload.y.is_finite() && payload.x >= 0.0 && payload.y >= 0.0 && config.fill_tolerance <= 255) {
        return Err(raster_fault("raster.fill.seed-invalid"));
    }
    paint_target_refusal(&payload.layer_id, document, config).map_or(Ok(()), Err)?;
    let target = config.paint_target.as_str();
    let color = match target {
        "mask" => {
            let grey = f64::from(config.mask_value.min(255)) / 255.0;
            vec![grey, grey, grey, 1.0]
        }
        _ => hex_color(&config.brush_color).ok_or_else(|| raster_fault("raster.brush.color-invalid"))?,
    };
    let seed = RasterSeed { x: payload.x.floor() as u32, y: payload.y.floor() as u32 };
    Ok(RasterMutation::FillRegion(FillRegionLeaf { layer_id: payload.layer_id.clone(), target: target.to_string(), seed, tolerance: config.fill_tolerance, color, selection: selection(config, &payload.layer_id)? }))
}

/// 🪣️ One bucket click: ONE edit stamped with its `TransactionRef` (plain without an admission — a render or test view),
/// or a refusal that leaves zero trace.
pub fn handle(payload: &FillRegion, doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let fill = fill_region_leaf(payload, doc.snapshot, cfg.snapshot)?;
    let seed = doc.operation_optional().map(|operation| operation.authoring_seed.as_str()).unwrap_or_default();
    Ok(match raster_tool_commit(RASTER_FILL_TOOL_ID, seed, fill) {
        Some((transaction, mutations)) if !seed.is_empty() => Emit::commit_transaction(transaction, mutations),
        Some((_, mutations)) => Emit::mutations(mutations),
        None => Emit::default(),
    })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
