//! 🫗️ `fill-selection` — one fill of a layer's pixels or of its mask over the pixel selection (the whole image without
//! one), stated as its intent: the layer, the target, the fill colour and the selection. Every application fills through
//! the ONE pixel engine (`semio_framework_pixels::editing::fill_in_place`, the same fill operation the region fill uses)
//! and files the filled image exactly like a painted stroke's, so a history edit of the colour or the selection replays
//! on whatever base it lands on. Design §17.2, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING.

use crate::mutations::fill_region::{fill_color_invariant, fill_operation};
use crate::mutations::paint_stroke::{canvas, painted, repainted_diff, repainted_inverse, refused, selection_invariant, selection_mask, Painted, RasterSelectionSpan, Refusal, RASTER_PAINT_TARGETS};
use crate::diff::RasterDiff;
use crate::{RasterMutation, RasterSnapshot};
use semio_framework_pixels::editing::fill_in_place;
use semio_framework_pixels::RasterImage;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct FillSelection {
    pub layer_id: String,
    pub target: String,
    pub color: Vec<f64>,
    pub selection: Option<Vec<RasterSelectionSpan>>,
}

/// 🏗️ Builder — fills the whole image of `layer_id`'s `target` with `color`.
pub fn fill_selection(layer_id: impl Into<String>, target: &str, color: [f64; 4]) -> RasterMutation {
    RasterMutation::FillSelection(FillSelection { layer_id: layer_id.into(), target: target.to_string(), color: color.to_vec(), selection: None })
}
//#endregion 🔖️Payload

//#region 🫗️Fill
/// 🧯️ The payload-intrinsic laws the schema states (vocabulary, unit colour channels, ordered selection runs), checked
/// again before any pixel moves; the first one broken, as the field it names.
pub(crate) fn invariant(payload: &FillSelection) -> Result<(), (&'static str, String)> {
    if !RASTER_PAINT_TARGETS.contains(&payload.target.as_str()) {
        return Err(("target", format!("Fill target \"{}\" is neither pixels nor mask.", payload.target)));
    }
    fill_color_invariant(&payload.color)?;
    selection_invariant(payload.selection.as_deref())
}

/// 🫗️ Fills `payload` on `base`; `Ok(None)` when no pixel changes (already that colour, or an empty selection).
fn fill(payload: &FillSelection, base: &RasterSnapshot) -> Result<Option<Painted>, Refusal> {
    let canvas = canvas(&payload.layer_id, &payload.target, base)?;
    let Some(frame) = canvas.source.frames.first() else { return Err(Refusal::Fatal("mutation.apply.image-invalid", "The target image has no frame.".to_string())) };
    let mut image = RasterImage { width: canvas.source.width, height: canvas.source.height, pixels: frame.rgba8.clone() };
    let count = image.pixels.len() / 4;
    let coverage = match &payload.selection {
        Some(spans) => selection_mask(spans, count).ok_or_else(|| Refusal::Error("mutation.target-mismatch", format!("The selection reaches past the {}×{} image.", image.width, image.height)))?,
        None => vec![255; count],
    };
    if !fill_in_place(&mut image, &fill_operation(&payload.target, &payload.color), &coverage).map_err(|error| Refusal::Fatal("mutation.apply.rasterize", error.to_string()))? {
        return Ok(None);
    }
    Ok(Some(painted(canvas, image.pixels, &payload.layer_id)))
}
//#endregion 🫗️Fill

//#region 🔖️Diff
/// 🔺️ The fill's diff on `base`: the filled image filed under its content key, the layer (or its mask) pointed at it, the
/// replaced image released when nothing else shows it.
pub fn diff(payload: &FillSelection, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    if let Err((field, message)) = invariant(payload) {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, [field.to_string()]);
    }
    match fill(payload, base) {
        Ok(Some(filled)) => repainted_diff(filled, base, &payload.layer_id, &payload.target),
        Ok(None) => protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The fill changes no pixel of layer \"{}\".", payload.layer_id)),
        Err(refusal) => refused(refusal, &payload.layer_id),
    }
}
//#endregion 🔖️Diff

//#region 🔖️Inverse
/// ↩️ The steps that undo the fill on `base` — the same restoration a painted stroke's inverse runs. Nothing when the fill
/// changed nothing.
pub fn inverse(payload: &FillSelection, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    Ok({
    if invariant(payload).is_err() {
        return Ok(Vec::new());
    }
    let Ok(Some(filled)) = fill(payload, base) else { return Ok(Vec::new() )};
    repainted_inverse(filled, base, &payload.layer_id, &payload.target)?

    })
}
//#endregion 🔖️Inverse

//#region 🔖️Kind
impl protocol::MutationKind<RasterSnapshot, RasterMutation> for FillSelection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "paint", entity: "selection", kind: "fill-selection", record: "FilledSelection" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let layer = &self.layer_id;
        let (en, de) = match (self.target.as_str(), self.selection.is_some()) {
            ("mask", true) => (format!("Fill the selection of the mask of layer {layer}"), format!("Auswahl der Maske von Ebene {layer} füllen")),
            ("mask", false) => (format!("Fill the mask of layer {layer}"), format!("Maske von Ebene {layer} füllen")),
            (_, true) => (format!("Fill the selection on layer {layer}"), format!("Auswahl auf Ebene {layer} füllen")),
            _ => (format!("Fill layer {layer}"), format!("Ebene {layer} füllen")),
        };
        semio_framework_ui_locale::LocalizedLabel::native(&en, &de)
    }

    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Kind

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
