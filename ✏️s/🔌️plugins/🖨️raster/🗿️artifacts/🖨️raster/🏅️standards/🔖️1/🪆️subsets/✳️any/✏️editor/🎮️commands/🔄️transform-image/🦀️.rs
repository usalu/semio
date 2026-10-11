//! 🔄️ Raster play app commands — `transform-image`: one change of a layer image's pixel grid from the editing menu — a
//! quarter turn, a resize or a crop. A host names the layer, the operation and its extent; the command builds ONE
//! parametric `transform-image` leaf — one edit, one history row whose operation and extent time travel edits. A grid
//! change moves the layer's pixels, so a session painting the mask is refused. The image is changed by the leaf on its
//! base, never by the host (design §17.2, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use super::paint_stroke::paint_target_refusal;
use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::mutations::transform_image::{invariant, TransformImage as TransformImageLeaf};
use crate::op::RasterMutation;
use crate::RasterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🔄️ One grid change as both hosts dispatch it: the layer, the operation, the crop origin, the resize or crop extent
/// and the resize sampling — zero and false where the operation has none.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "transform-image")]
pub struct TransformImage {
    pub layer_id: String,
    pub operation: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub bilinear: bool,
}

/// 🧱️ The one leaf a grid change means on `document` under the session `config`, refused (zero trace) when the session
/// paints the mask, when the layer cannot take it — no such layer, a hidden or locked one, a layer without pixels — or
/// when the operation or its extent is not one the leaf admits.
pub fn transform_image_leaf(payload: &TransformImage, document: &RasterSnapshot, config: &RasterConfig) -> Result<RasterMutation, Fault> {
    if config.paint_target != "pixels" {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("raster.transform.pixels-only"), "A rotation, resize or crop changes a layer's pixel grid; switch the edit target to pixels."));
    }
    paint_target_refusal(&payload.layer_id, document, config).map_or(Ok(()), Err)?;
    let leaf = TransformImageLeaf { layer_id: payload.layer_id.clone(), operation: payload.operation.clone(), x: payload.x, y: payload.y, width: payload.width, height: payload.height, bilinear: payload.bilinear };
    invariant(&leaf).map_err(|(field, message)| Fault::new(FaultOrigin::App, FaultCode::new("raster.transform.invalid"), format!("{field}: {message}")))?;
    Ok(RasterMutation::TransformImage(leaf))
}

/// 🔄️ One grid change: ONE edit holding its leaf, or a refusal that leaves zero trace.
pub fn handle(payload: &TransformImage, doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![transform_image_leaf(payload, doc.snapshot, cfg.snapshot)?]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
