//! 🪣️ `fill-region` — one bucket fill on a layer's pixels or on its mask, stated as its intent: the layer, the target,
//! the seed pixel, the colour tolerance, the fill colour and the pixel selection the region is clipped to. The region is
//! flooded and filled on every application by the ONE deterministic pixel engine
//! (`semio_framework_pixels::editing::{flood_selection, fill_in_place}`, byte-equal to the TypeScript twin), so a history
//! edit of the seed, the tolerance or the colour replays onto whatever base it lands on. The filled image is filed exactly
//! like a painted stroke's (`paint-stroke`'s canvas: a new content-addressed asset in the lossless image-pack carrier, the
//! replaced image released once nothing else shows it). Design §17.2, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING.

use crate::mutations::paint_stroke::{canvas, channel_byte, grey_byte, painted, repainted_diff, repainted_inverse, refused, selection_invariant, selection_mask, Painted, RasterSelectionSpan, Refusal, RASTER_PAINT_TARGETS};
use crate::diff::RasterDiff;
use crate::{RasterMutation, RasterSnapshot};
use semio_framework_pixels::editing::{fill_in_place, flood_selection, PixelOperation};
use semio_framework_pixels::RasterImage;

//#region 🔖️Payload
/// 🪣️ The pixel a fill floods from, in the target image's whole pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterSeed {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct FillRegion {
    pub layer_id: String,
    pub target: String,
    pub seed: RasterSeed,
    pub tolerance: u32,
    pub color: Vec<f64>,
    pub selection: Option<Vec<RasterSelectionSpan>>,
}

/// 🏗️ Builder — wraps an unclipped fill in its dispatch variant.
pub fn fill_region(layer_id: impl Into<String>, target: &str, seed: RasterSeed, tolerance: u32, color: [f64; 4]) -> RasterMutation {
    RasterMutation::FillRegion(FillRegion { layer_id: layer_id.into(), target: target.to_string(), seed, tolerance, color: color.to_vec(), selection: None })
}
//#endregion 🔖️Payload

//#region 🪣️Fill
/// 🧯️ The payload-intrinsic laws the schema states (vocabulary, bounds, ordered selection runs), checked again before any
/// pixel moves; the first one broken, as the field it names.
fn invariant(payload: &FillRegion) -> Result<(), (&'static str, String)> {
    if !RASTER_PAINT_TARGETS.contains(&payload.target.as_str()) {
        return Err(("target", format!("Fill target \"{}\" is neither pixels nor mask.", payload.target)));
    }
    if payload.tolerance > 255 {
        return Err(("tolerance", format!("A colour tolerance lies within 0..255, not {}.", payload.tolerance)));
    }
    fill_color_invariant(&payload.color)?;
    selection_invariant(payload.selection.as_deref())
}

/// 🎨️ The fill operation `color` means on `target`: the colour over the layer's pixels, or the colour's grey level as
/// coverage on a mask, at the colour's alpha — shared by the region fill and the selection fill (`🫗️fill-selection`).
pub(crate) fn fill_operation(target: &str, color: &[f64]) -> PixelOperation {
    match target {
        "mask" => PixelOperation::AlphaFill { alpha: grey_byte(color), opacity: color[3] },
        _ => PixelOperation::Fill([channel_byte(color[0]), channel_byte(color[1]), channel_byte(color[2]), channel_byte(color[3])]),
    }
}

/// 🎨️ The unit-channel colour law every fill leaf states: four channels within 0..1.
pub(crate) fn fill_color_invariant(color: &[f64]) -> Result<(), (&'static str, String)> {
    match color.len() == 4 && color.iter().all(|channel| channel.is_finite() && (0.0..=1.0).contains(channel)) {
        true => Ok(()),
        false => Err(("color", "The fill colour needs four channels within 0..1.".to_string())),
    }
}

/// 🪣️ Floods and fills `payload` on `base`; `Ok(None)` when no pixel changes (already that colour).
fn fill(payload: &FillRegion, base: &RasterSnapshot) -> Result<Option<Painted>, Refusal> {
    let canvas = canvas(&payload.layer_id, &payload.target, base)?;
    let Some(frame) = canvas.source.frames.first() else { return Err(Refusal::Fatal("mutation.apply.image-invalid", "The target image has no frame.".to_string())) };
    let mut image = RasterImage { width: canvas.source.width, height: canvas.source.height, pixels: frame.rgba8.clone() };
    if payload.seed.x >= image.width || payload.seed.y >= image.height {
        return Err(Refusal::Error("mutation.target-mismatch", format!("The seed ({}, {}) lies outside the {}×{} image.", payload.seed.x, payload.seed.y, image.width, image.height)));
    }
    let selection = match &payload.selection {
        Some(spans) => Some(selection_mask(spans, image.pixels.len() / 4).ok_or_else(|| Refusal::Error("mutation.target-mismatch", format!("The selection reaches past the {}×{} image.", image.width, image.height)))?),
        None => None,
    };
    let region = flood_selection(&image, payload.seed.x, payload.seed.y, payload.tolerance as u8, selection.as_deref()).map_err(|error| Refusal::Fatal("mutation.apply.rasterize", error.to_string()))?;
    if !fill_in_place(&mut image, &fill_operation(&payload.target, &payload.color), &region).map_err(|error| Refusal::Fatal("mutation.apply.rasterize", error.to_string()))? {
        return Ok(None);
    }
    Ok(Some(painted(canvas, image.pixels, &payload.layer_id)))
}
//#endregion 🪣️Fill

//#region 🔖️Diff
/// 🔺️ The fill's diff on `base`: the filled image filed under its content key, the layer (or its mask) pointed at it, the
/// replaced image released when nothing else shows it.
pub fn diff(payload: &FillRegion, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
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
pub fn inverse(payload: &FillRegion, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
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
impl protocol::MutationKind<RasterSnapshot, RasterMutation> for FillRegion {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "paint", entity: "region", kind: "fill-region", record: "PaintedRegion" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (en, de) = match self.target.as_str() {
            "mask" => ("Mask fill", "Maskenfüllung"),
            _ => ("Bucket fill", "Farbfüllung"),
        };
        let (x, y) = (self.seed.x, self.seed.y);
        semio_framework_ui_locale::LocalizedLabel::native(&format!("{en} from ({x}, {y}) on layer {}", self.layer_id), &format!("{de} ab ({x}, {y}) auf Ebene {}", self.layer_id))
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
