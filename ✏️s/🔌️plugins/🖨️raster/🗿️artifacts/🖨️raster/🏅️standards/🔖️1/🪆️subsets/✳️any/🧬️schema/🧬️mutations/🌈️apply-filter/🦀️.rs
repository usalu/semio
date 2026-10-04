//! 🌈️ `apply-filter` — one image filter on a layer's pixels, stated as its intent: the layer, the filter, its amount and
//! the pixel selection it is clipped to. A filter changes colour channels, so it has no meaning on a mask's coverage. The filter runs on every application through the ONE
//! pixel engine (`semio_framework_pixels::editing::PixelEditJob`, byte-equal to the TypeScript twin), so a history edit of
//! the filter or its amount replays onto whatever base it lands on. The filtered image is filed exactly like a painted
//! stroke's (`paint-stroke`'s canvas: a new content-addressed asset in the lossless image-pack carrier, the replaced image
//! released once nothing else shows it). Geometry that changes the image's extent (rotate, resize, crop) is not a filter.
//! Design §17.2, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING.

use crate::mutations::paint_stroke::{canvas, painted, painted_diff, painted_inverse, refused, selection_invariant, selection_mask, Painted, RasterSelectionSpan, Refusal};
use crate::diff::RasterDiff;
use crate::{RasterMutation, RasterSnapshot};
use semio_framework_pixels::editing::{PixelEditJob, PixelOperation};
use semio_framework_pixels::RasterImage;

//#region 🔖️Payload
/// 🌈️ Every filter a layer image takes in place, with the closed range its amount lies within; a filter without a range
/// takes no amount, which is then `0`. Posterize levels and the blur radius are whole numbers.
pub const RASTER_FILTERS: &[(&str, Option<(f64, f64)>)] = &[
    ("invert", None),
    ("grayscale", None),
    ("clear", None),
    ("flipHorizontal", None),
    ("flipVertical", None),
    ("brightness", Some((-1.0, 1.0))),
    ("contrast", Some((-1.0, 1.0))),
    ("saturation", Some((-1.0, 1.0))),
    ("gamma", Some((0.01, 10.0))),
    ("threshold", Some((0.0, 255.0))),
    ("posterize", Some((2.0, 256.0))),
    ("blur", Some((0.0, 16.0))),
    ("sharpen", Some((0.0, 5.0))),
];

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ApplyFilter {
    pub layer_id: String,
    pub filter: String,
    pub amount: f64,
    pub selection: Option<Vec<RasterSelectionSpan>>,
}

/// 🏗️ Builder — wraps an unclipped filter in its dispatch variant.
pub fn apply_filter(layer_id: impl Into<String>, filter: &str, amount: f64) -> RasterMutation {
    RasterMutation::ApplyFilter(ApplyFilter { layer_id: layer_id.into(), filter: filter.to_string(), amount, selection: None })
}

/// 🎯️ The target every filter repaints: the layer's own pixels.
const FILTER_TARGET: &str = "pixels";
//#endregion 🔖️Payload

//#region 🌈️Filter
/// 🧯️ The payload-intrinsic laws the schema states (vocabulary, per-filter bounds, a flip takes no selection, ordered
/// selection runs), checked again before any pixel moves; the first one broken, as the field it names.
pub(crate) fn invariant(payload: &ApplyFilter) -> Result<(), (&'static str, String)> {
    let Some((_, range)) = RASTER_FILTERS.iter().find(|(filter, _)| *filter == payload.filter) else {
        return Err(("filter", format!("\"{}\" is not a filter.", payload.filter)));
    };
    let admitted = match range {
        None => payload.amount == 0.0,
        Some((min, max)) => payload.amount.is_finite() && payload.amount >= *min && payload.amount <= *max && (!matches!(payload.filter.as_str(), "posterize" | "blur") || payload.amount.fract() == 0.0),
    };
    if !admitted {
        return Err(("amount", format!("{} is not an amount the {} filter takes.", payload.amount, payload.filter)));
    }
    if payload.selection.is_some() && matches!(payload.filter.as_str(), "flipHorizontal" | "flipVertical") {
        return Err(("selection", "A flip mirrors the whole image; it takes no selection.".to_string()));
    }
    selection_invariant(payload.selection.as_deref())
}

/// 🎛️ The pixel-engine operation an admitted `payload` means.
fn operation(payload: &ApplyFilter) -> PixelOperation {
    let amount = payload.amount;
    match payload.filter.as_str() {
        "invert" => PixelOperation::Invert,
        "grayscale" => PixelOperation::Grayscale,
        "clear" => PixelOperation::Clear,
        "flipHorizontal" => PixelOperation::FlipHorizontal,
        "flipVertical" => PixelOperation::FlipVertical,
        "brightness" => PixelOperation::Brightness(amount),
        "contrast" => PixelOperation::Contrast(amount),
        "saturation" => PixelOperation::Saturation(amount),
        "gamma" => PixelOperation::Gamma(amount),
        "threshold" => PixelOperation::Threshold(amount),
        "posterize" => PixelOperation::Posterize(amount as u16),
        "blur" => PixelOperation::Blur(amount as u8),
        _ => PixelOperation::Sharpen(amount),
    }
}

/// 🌈️ Runs `payload` on `base`'s target image through the pixel engine; `Ok(None)` when no pixel changes.
fn filter(payload: &ApplyFilter, base: &RasterSnapshot) -> Result<Option<Painted>, Refusal> {
    let canvas = canvas(&payload.layer_id, FILTER_TARGET, base)?;
    let Some(frame) = canvas.source.frames.first() else { return Err(Refusal::Fatal("mutation.apply.image-invalid", "The target image has no frame.".to_string())) };
    let image = RasterImage { width: canvas.source.width, height: canvas.source.height, pixels: frame.rgba8.clone() };
    let selection = match &payload.selection {
        Some(spans) => Some(selection_mask(spans, image.pixels.len() / 4).ok_or_else(|| Refusal::Error("mutation.target-mismatch", format!("The selection reaches past the {}×{} image.", image.width, image.height)))?),
        None => None,
    };
    let rasterize = |error: semio_framework_pixels::editing::PixelEditError| Refusal::Fatal("mutation.apply.rasterize", error.to_string());
    let mut job = PixelEditJob::new(image, operation(payload), selection).map_err(rasterize)?;
    while !job.advance(65_536).map_err(rasterize)?.done {}
    let filtered = job.into_result().map_err(rasterize)?;
    if canvas.source.frames.first().is_some_and(|frame| frame.rgba8 == filtered.pixels) {
        return Ok(None);
    }
    Ok(Some(painted(canvas, filtered.pixels, &payload.layer_id)))
}

/// 🏷️ The filter's English and German name.
fn filter_name(filter: &str) -> (&'static str, &'static str) {
    match filter {
        "invert" => ("Invert", "Invertieren"),
        "grayscale" => ("Grayscale", "Graustufen"),
        "clear" => ("Clear", "Leeren"),
        "flipHorizontal" => ("Flip horizontally", "Horizontal spiegeln"),
        "flipVertical" => ("Flip vertically", "Vertikal spiegeln"),
        "brightness" => ("Brightness", "Helligkeit"),
        "contrast" => ("Contrast", "Kontrast"),
        "saturation" => ("Saturation", "Sättigung"),
        "gamma" => ("Gamma", "Gamma"),
        "threshold" => ("Threshold", "Schwellenwert"),
        "posterize" => ("Posterize", "Tontrennung"),
        "blur" => ("Blur", "Weichzeichnen"),
        "sharpen" => ("Sharpen", "Schärfen"),
        _ => ("Filter", "Filter"),
    }
}
//#endregion 🌈️Filter

//#region 🔖️Diff
/// 🔺️ The filter's diff on `base`: the filtered image filed under its content key, the layer (or its mask) pointed at it,
/// the replaced image released when nothing else shows it.
pub fn diff(payload: &ApplyFilter, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    if let Err((field, message)) = invariant(payload) {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, [field.to_string()]);
    }
    match filter(payload, base) {
        Ok(Some(filtered)) => painted_diff(filtered, base, &payload.layer_id, FILTER_TARGET, None),
        Ok(None) => protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The {} filter changes no pixel of layer \"{}\".", payload.filter, payload.layer_id)),
        Err(refusal) => refused(refusal, &payload.layer_id),
    }
}
//#endregion 🔖️Diff

//#region 🔖️Inverse
/// ↩️ The steps that undo the filter on `base` — the same restoration a painted stroke's inverse runs. Nothing when the
/// filter changed nothing.
pub fn inverse(payload: &ApplyFilter, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    Ok({
    if invariant(payload).is_err() {
        return Ok(Vec::new());
    }
    let Ok(Some(filtered)) = filter(payload, base) else { return Ok(Vec::new() )};
    painted_inverse(filtered, base, &payload.layer_id, FILTER_TARGET, None)?

    })
}
//#endregion 🔖️Inverse

//#region 🔖️Kind
impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ApplyFilter {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "apply", entity: "filter", kind: "apply-filter", record: "FilteredImage" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (en, de) = filter_name(&self.filter);
        let (en, de) = match RASTER_FILTERS.iter().find(|(filter, _)| *filter == self.filter).and_then(|(_, range)| *range) {
            Some(_) => (format!("{en} {}", self.amount), format!("{de} {}", self.amount)),
            None => (en.to_string(), de.to_string()),
        };
        let layer = &self.layer_id;
        semio_framework_ui_locale::LocalizedLabel::native(&format!("{en} on layer {layer}"), &format!("{de} auf Ebene {layer}"))
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
