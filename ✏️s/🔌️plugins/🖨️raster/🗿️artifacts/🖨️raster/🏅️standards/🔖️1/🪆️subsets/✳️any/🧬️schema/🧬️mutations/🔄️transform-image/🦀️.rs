//! 🔄️ `transform-image` — one change of a layer image's pixel grid, stated as its intent: the layer and the operation —
//! a quarter turn either way, a resize to `width × height` (nearest or bilinear sampling) or a crop to the `width ×
//! height` window at `(x, y)`. Every application runs the operation on the layer image through the ONE pixel engine
//! (`semio_framework_pixels::editing::PixelEditJob`), files the result as a new content-addressed asset, and moves the
//! layer so the image keeps its place and scale on the canvas: the display extent becomes the new pixel extent with the
//! old display scale folded into the placement (swapped for a quarter turn), and a crop keeps its window where it was.
//! A history edit of the operation or its extent replays on whatever base it lands on. Design §17.2, ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING.

use crate::mutations::paint_stroke::{canvas, painted_diff, painted_inverse, refused, reshaped, Painted, Refusal, Reshape};
use crate::diff::RasterDiff;
use crate::{RasterMutation, RasterSnapshot, RasterTransform};
use semio_framework_pixels::editing::{validate_extent, PixelEditJob, PixelOperation};
use semio_framework_pixels::RasterImage;

//#region 🔖️Payload
/// 🔄️ Every operation that moves a layer image's pixel grid.
pub const RASTER_IMAGE_TRANSFORMS: [&str; 4] = ["rotateClockwise", "rotateCounterclockwise", "resize", "crop"];
/// 🧮️ The longest side a transformed image may have, as the payload schema bounds it.
pub const RASTER_IMAGE_MAXIMUM_SIDE: u32 = 16_384;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct TransformImage {
    pub layer_id: String,
    pub operation: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub bilinear: bool,
}

/// 🏗️ Builder — a quarter turn of `layer_id`'s image (`clockwise` or not).
pub fn rotate_image(layer_id: impl Into<String>, clockwise: bool) -> RasterMutation {
    let operation = if clockwise { "rotateClockwise" } else { "rotateCounterclockwise" };
    RasterMutation::TransformImage(TransformImage { layer_id: layer_id.into(), operation: operation.to_string(), x: 0, y: 0, width: 0, height: 0, bilinear: false })
}

/// 🏗️ Builder — `layer_id`'s image resized to `width × height`.
pub fn resize_image(layer_id: impl Into<String>, width: u32, height: u32, bilinear: bool) -> RasterMutation {
    RasterMutation::TransformImage(TransformImage { layer_id: layer_id.into(), operation: "resize".to_string(), x: 0, y: 0, width, height, bilinear })
}

/// 🏗️ Builder — `layer_id`'s image cropped to the `width × height` window at `(x, y)`.
pub fn crop_image(layer_id: impl Into<String>, x: u32, y: u32, width: u32, height: u32) -> RasterMutation {
    RasterMutation::TransformImage(TransformImage { layer_id: layer_id.into(), operation: "crop".to_string(), x, y, width, height, bilinear: false })
}
//#endregion 🔖️Payload

//#region 🔄️Transform
/// 🧯️ The payload-intrinsic laws the schema states (vocabulary; a quarter turn carries no window, extent or sampling; a
/// resize no origin; a crop no sampling; every extent within 1..=16384), checked again before any pixel moves; the first
/// one broken, as the field it names.
pub(crate) fn invariant(payload: &TransformImage) -> Result<(), (&'static str, String)> {
    if !RASTER_IMAGE_TRANSFORMS.contains(&payload.operation.as_str()) {
        return Err(("operation", format!("\"{}\" is not an image transform.", payload.operation)));
    }
    let sized = |side: u32| (1..=RASTER_IMAGE_MAXIMUM_SIDE).contains(&side);
    match payload.operation.as_str() {
        "resize" | "crop" if !sized(payload.width) => Err(("width", format!("A width lies within 1..={RASTER_IMAGE_MAXIMUM_SIDE}, not {}.", payload.width))),
        "resize" | "crop" if !sized(payload.height) => Err(("height", format!("A height lies within 1..={RASTER_IMAGE_MAXIMUM_SIDE}, not {}.", payload.height))),
        "resize" | "rotateClockwise" | "rotateCounterclockwise" if payload.x != 0 || payload.y != 0 => Err(("x", "Only a crop has an origin.".to_string())),
        "rotateClockwise" | "rotateCounterclockwise" if payload.width != 0 || payload.height != 0 => Err(("width", "A quarter turn keeps the image's own extent.".to_string())),
        "crop" | "rotateClockwise" | "rotateCounterclockwise" if payload.bilinear => Err(("bilinear", "Only a resize samples.".to_string())),
        _ => Ok(()),
    }
}

/// 🎛️ The pixel-engine operation an admitted `payload` means.
fn operation(payload: &TransformImage) -> PixelOperation {
    match payload.operation.as_str() {
        "rotateClockwise" => PixelOperation::RotateClockwise,
        "rotateCounterclockwise" => PixelOperation::RotateCounterclockwise,
        "resize" => PixelOperation::Resize { width: payload.width, height: payload.height, bilinear: payload.bilinear },
        _ => PixelOperation::Crop { x: payload.x, y: payload.y, width: payload.width, height: payload.height },
    }
}

/// 📐️ Where the layer goes so the transformed image keeps its place and scale: the old display scale per image pixel
/// folded into the placement (its axes swapped by a quarter turn), and a crop's window centre kept where it was.
fn reshape(payload: &TransformImage, placement: &RasterTransform, display: (Option<u32>, Option<u32>), source: (u32, u32), result: (u32, u32)) -> Reshape {
    let (scale_x, scale_y) = (f64::from(display.0.unwrap_or(source.0)) / f64::from(source.0.max(1)), f64::from(display.1.unwrap_or(source.1)) / f64::from(source.1.max(1)));
    let (scale_x, scale_y) = match payload.operation.as_str() {
        "rotateClockwise" | "rotateCounterclockwise" => (scale_y, scale_x),
        _ => (scale_x, scale_y),
    };
    let mut transform = RasterTransform { a: placement.a * scale_x, b: placement.b * scale_x, c: placement.c * scale_y, d: placement.d * scale_y, ..placement.clone() };
    if payload.operation == "crop" {
        let dx = f64::from(payload.x) + f64::from(payload.width) / 2.0 - f64::from(source.0) / 2.0;
        let dy = f64::from(payload.y) + f64::from(payload.height) / 2.0 - f64::from(source.1) / 2.0;
        transform.x += transform.a * dx + transform.c * dy;
        transform.y += transform.b * dx + transform.d * dy;
    }
    Reshape { width: result.0, height: result.1, transform }
}

/// 🔄️ Runs `payload` on `base`'s layer image through the pixel engine: the filed image and the layer's new grid.
fn transform(payload: &TransformImage, base: &RasterSnapshot) -> Result<(Painted, Reshape), Refusal> {
    let canvas = canvas(&payload.layer_id, "pixels", base)?;
    let Some(frame) = canvas.source.frames.first() else { return Err(Refusal::Fatal("mutation.apply.image-invalid", "The target image has no frame.".to_string())) };
    let source = (canvas.source.width, canvas.source.height);
    if payload.operation == "crop" && (u64::from(payload.x) + u64::from(payload.width) > u64::from(source.0) || u64::from(payload.y) + u64::from(payload.height) > u64::from(source.1)) {
        return Err(Refusal::Error("mutation.target-mismatch", format!("The {}×{} window at ({}, {}) reaches past the {}×{} image.", payload.width, payload.height, payload.x, payload.y, source.0, source.1)));
    }
    let rasterize = |error: semio_framework_pixels::editing::PixelEditError| Refusal::Fatal("mutation.apply.rasterize", error.to_string());
    let image = RasterImage { width: source.0, height: source.1, pixels: frame.rgba8.clone() };
    let mut job = PixelEditJob::new(image, operation(payload), None).map_err(rasterize)?;
    while !job.advance(65_536).map_err(rasterize)?.done {}
    let result = job.into_result().map_err(rasterize)?;
    validate_extent(result.width, result.height).map_err(rasterize)?;
    let grid = reshape(payload, &canvas.placement, canvas.extent, source, (result.width, result.height));
    Ok((reshaped(canvas, result, &payload.layer_id), grid))
}
//#endregion 🔄️Transform

//#region 🔖️Diff
/// 🔺️ The transform's diff on `base`: the transformed image filed under its content key, the layer pointed at it at its
/// new extent and placement, the replaced image released when nothing else shows it.
pub fn diff(payload: &TransformImage, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    if let Err((field, message)) = invariant(payload) {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, [field.to_string()]);
    }
    match transform(payload, base) {
        Ok((transformed, grid)) => painted_diff(transformed, base, &payload.layer_id, "pixels", Some(&grid)),
        Err(refusal) => refused(refusal, &payload.layer_id),
    }
}
//#endregion 🔖️Diff

//#region 🔖️Inverse
/// ↩️ The steps that undo the transform on `base`: the prior image back in the pool and the layer back on it at its prior
/// extent and placement.
pub fn inverse(payload: &TransformImage, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    Ok({
    if invariant(payload).is_err() {
        return Ok(Vec::new());
    }
    let Ok((transformed, grid)) = transform(payload, base) else { return Ok(Vec::new() )};
    painted_inverse(transformed, base, &payload.layer_id, "pixels", Some(&grid))?

    })
}
//#endregion 🔖️Inverse

//#region 🔖️Kind
impl protocol::MutationKind<RasterSnapshot, RasterMutation> for TransformImage {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "image", kind: "transform-image", record: "TransformedImage" };

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
        let (width, height, x, y) = (self.width, self.height, self.x, self.y);
        let (en, de) = match self.operation.as_str() {
            "rotateClockwise" => (format!("Rotate layer {layer} right"), format!("Ebene {layer} nach rechts drehen")),
            "rotateCounterclockwise" => (format!("Rotate layer {layer} left"), format!("Ebene {layer} nach links drehen")),
            "resize" => (format!("Resize layer {layer} to {width} × {height}"), format!("Ebene {layer} auf {width} × {height} skalieren")),
            _ => (format!("Crop layer {layer} to {width} × {height} at ({x}, {y})"), format!("Ebene {layer} auf {width} × {height} bei ({x}, {y}) zuschneiden")),
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
