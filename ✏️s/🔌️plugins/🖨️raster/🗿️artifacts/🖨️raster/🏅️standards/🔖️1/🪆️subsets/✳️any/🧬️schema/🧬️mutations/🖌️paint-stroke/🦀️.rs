//! 🖌️ `paint-stroke` — one brush or eraser stroke on a layer's pixels or on its mask, stated as its intent: the layer,
//! the target, the tool, the brush and the points in the target image's pixels, in drawing order. The pixels are
//! rasterized on every application by the ONE deterministic rasterizer
//! (`semio_framework_pixels::editing::paint_stroke_in_place`), so a history edit of the brush or of the points
//! replays onto whatever base it lands on. The painted image becomes a new content-addressed asset in the lossless
//! image-pack carrier, the layer (or its mask) points at it, and the image it replaced leaves the pool once nothing
//! else shows it. The canvas — resolving the target image, filing a repainted one and restoring it — is shared with
//! `🪣️fill-region`. Design §17.2, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING.

use crate::diff::{RasterAssetsDelta, RasterDiff, RasterLayerPatchEntry, RasterLayersDelta};
use crate::io::{raster_image_pack_asset, semio_image_from_rgba8};
use crate::standards::v1::subsets::any::schema::{find_layer, flatten_raster_layers, layer_node_id, layer_protection};
use crate::{RasterLayerMask, RasterLayerNode, RasterLayerPatch, RasterMaskContent, RasterMutation, RasterPixelContent, RasterSnapshot, RasterTransform, SemioImageSnapshot};
use semio_framework_pixels::editing::{paint_stroke_in_place, validate_extent, PixelAlphaBrush, PixelBrush, PixelOperation};
use semio_framework_pixels::RasterImage;

//#region 🔖️Payload
/// 🧮️ The most points one stroke carries — a host flushes a longer drag as several strokes.
pub const RASTER_STROKE_MAXIMUM_POINTS: usize = 2_048;
/// 🎯️ The images a stroke paints: the layer's own pixels, or its mask's coverage.
pub const RASTER_PAINT_TARGETS: [&str; 2] = ["pixels", "mask"];
/// 🖌️ The tools a stroke paints with: the brush lays its colour down, the eraser takes alpha (a mask: coverage) away.
pub const RASTER_PAINT_TOOLS: [&str; 2] = ["brush", "eraser"];

/// 📍️ One stroke point in the target image's pixels: pixel edges at whole numbers, pixel centres at `+ 0.5`.
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterStrokePoint {
    pub x: f64,
    pub y: f64,
}

/// 🖌️ The brush a stroke paints with: its diameter in pixels, the fraction of the radius painted at full strength, its
/// opacity, and its straight-alpha sRGB colour (four channels, each 0..1). On a mask the brush paints the colour's grey
/// level as coverage; the eraser ignores the colour.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterBrush {
    pub size: f64,
    pub hardness: f64,
    pub opacity: f64,
    pub color: Vec<f64>,
}

/// ✂️ One run of a pixel selection the stroke is clipped to: `length` pixels from row-major pixel index `start`, each
/// covered `coverage` out of 255.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterSelectionSpan {
    pub start: u32,
    pub length: u32,
    pub coverage: u32,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct PaintStroke {
    pub layer_id: String,
    pub target: String,
    pub tool: String,
    pub brush: RasterBrush,
    pub points: Vec<RasterStrokePoint>,
    pub selection: Option<Vec<RasterSelectionSpan>>,
}

/// 🏗️ Builder — wraps an unclipped stroke in its dispatch variant.
pub fn paint_stroke(layer_id: impl Into<String>, target: &str, tool: &str, brush: RasterBrush, points: Vec<RasterStrokePoint>) -> RasterMutation {
    RasterMutation::PaintStroke(PaintStroke { layer_id: layer_id.into(), target: target.to_string(), tool: tool.to_string(), brush, points, selection: None })
}
//#endregion 🔖️Payload

//#region 🎨️Raster
/// 🔢️ One unit channel as the byte the rasterizer paints.
pub(crate) fn channel_byte(value: f64) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// 🌗️ The coverage a mask brush of `color` paints: the colour's Rec. 709 grey level as a byte.
pub(crate) fn grey_byte(color: &[f64]) -> u8 {
    (255.0 * (0.2126 * color[0] + 0.7152 * color[1] + 0.0722 * color[2])).clamp(0.0, 255.0).round() as u8
}

/// 🧯️ The payload-intrinsic laws the schema states (vocabularies, bounds, finite points), checked again before any pixel
/// moves; the first one broken, as the field it names.
fn invariant(payload: &PaintStroke) -> Result<(), (&'static str, String)> {
    let unit = |value: f64| value.is_finite() && (0.0..=1.0).contains(&value);
    if !RASTER_PAINT_TARGETS.contains(&payload.target.as_str()) {
        return Err(("target", format!("Paint target \"{}\" is neither pixels nor mask.", payload.target)));
    }
    if !RASTER_PAINT_TOOLS.contains(&payload.tool.as_str()) {
        return Err(("tool", format!("Paint tool \"{}\" is neither brush nor eraser.", payload.tool)));
    }
    if payload.points.is_empty() || payload.points.len() > RASTER_STROKE_MAXIMUM_POINTS || !payload.points.iter().all(|point| point.x.is_finite() && point.y.is_finite()) {
        return Err(("points", format!("A stroke carries 1 to {RASTER_STROKE_MAXIMUM_POINTS} finite points, not {}.", payload.points.len())));
    }
    let brush = &payload.brush;
    if !(brush.size.is_finite() && (0.1..=4096.0).contains(&brush.size)) || !unit(brush.hardness) || !unit(brush.opacity) || brush.color.len() != 4 || !brush.color.iter().all(|channel| unit(*channel)) {
        return Err(("brush", "The brush needs a size of 0.1 to 4096 pixels, hardness and opacity within 0..1 and four colour channels within 0..1.".to_string()));
    }
    selection_invariant(payload.selection.as_deref())
}

/// ✂️ The selection law every repaint states (`ordered-selection-runs`): runs are non-empty, ordered, disjoint and cover at
/// most 255.
pub(crate) fn selection_invariant(selection: Option<&[RasterSelectionSpan]>) -> Result<(), (&'static str, String)> {
    let mut next = 0_u64;
    for span in selection.into_iter().flatten() {
        if span.length == 0 || span.coverage > 255 || u64::from(span.start) < next {
            return Err(("selection", "Selection runs are non-empty, ordered, disjoint and cover at most 255.".to_string()));
        }
        next = u64::from(span.start) + u64::from(span.length);
    }
    Ok(())
}

/// ✂️ The per-pixel coverage `spans` select on an image of `count` pixels; `None` when a run reaches past the image.
pub(crate) fn selection_mask(spans: &[RasterSelectionSpan], count: usize) -> Option<Vec<u8>> {
    let mut mask = vec![0_u8; count];
    for span in spans {
        let start = span.start as usize;
        mask.get_mut(start..start.checked_add(span.length as usize)?)?.fill(span.coverage as u8);
    }
    Some(mask)
}

/// 🖌️ The rasterizer operation `payload` means on its target.
fn operation(payload: &PaintStroke) -> PixelOperation {
    let points = payload.points.iter().map(|point| [point.x, point.y]).collect();
    let brush = &payload.brush;
    let erase = payload.tool == "eraser";
    match payload.target.as_str() {
        "mask" => PixelOperation::AlphaStroke(PixelAlphaBrush { points, size: brush.size, opacity: brush.opacity, hardness: brush.hardness, alpha: if erase { 0 } else { grey_byte(&brush.color) } }),
        _ => PixelOperation::Stroke(PixelBrush {
            points,
            size: brush.size,
            opacity: brush.opacity,
            hardness: brush.hardness,
            color: [channel_byte(brush.color[0]), channel_byte(brush.color[1]), channel_byte(brush.color[2]), channel_byte(brush.color[3])],
            erase,
        }),
    }
}

/// 🖼️ The image a repaint (a stroke, a region fill) paints into and where it sits: the source image (decoded, so its
/// profile and metadata ride along), its asset key, the key prefix of the image the repaint mints, and the layer's mask
/// when the target is one.
pub(crate) struct Canvas {
    pub(crate) source: SemioImageSnapshot,
    previous: Option<String>,
    prefix: &'static str,
    mask: Option<RasterLayerMask>,
    pub(crate) extent: (Option<u32>, Option<u32>),
    pub(crate) placement: RasterTransform,
}

/// 📐️ A repaint that also moves the layer's pixel grid (a rotation, a resize, a crop): the image's new display extent and
/// the layer's new placement.
pub(crate) struct Reshape {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) transform: RasterTransform,
}

/// ⚠️ Why a repaint cannot land: a state-dependent Error (the layer or its image is not there, or does not take this
/// repaint) or a Fatal application fault, with the target it names.
pub(crate) enum Refusal {
    Error(&'static str, String),
    Fatal(&'static str, String),
}

/// 🧩️ The decoded image behind `key`, from the asset pool's own materialized child.
fn asset_image(base: &RasterSnapshot, key: &str) -> Result<SemioImageSnapshot, Refusal> {
    let child = base.assets.get(key).ok_or_else(|| Refusal::Error("mutation.target-missing", format!("Image asset \"{key}\" is not in the document.")))?;
    let image = child.local_owner::<SemioImageSnapshot>().ok_or_else(|| Refusal::Fatal("mutation.apply.image-unmaterialized", format!("Image asset \"{key}\" has no materialized pixels.")))?;
    let frame = image.frames.first().ok_or_else(|| Refusal::Fatal("mutation.apply.image-invalid", format!("Image asset \"{key}\" has no frame.")))?;
    if validate_extent(image.width, image.height).ok().map(|count| count * 4) != Some(frame.rgba8.len()) {
        return Err(Refusal::Fatal("mutation.apply.image-invalid", format!("Image asset \"{key}\" does not hold one RGBA8 pixel per pixel of its extent.")));
    }
    Ok(image.as_ref().clone())
}

/// 🫥️ A blank `width × height` image filled with `pixel`, refused when the extent is outside the pixel budget.
fn blank(width: u32, height: u32, pixel: [u8; 4]) -> Result<SemioImageSnapshot, Refusal> {
    let count = validate_extent(width, height).map_err(|_| Refusal::Fatal("mutation.apply.image-invalid", format!("A {width}×{height} image is outside the pixel budget.")))?;
    Ok(semio_image_from_rgba8(width, height, pixel.repeat(count)))
}

/// 🖼️ Resolves the image a repaint of `layer_id`'s `target` (`pixels` or `mask`) paints into on `base`.
pub(crate) fn canvas(layer_id: &str, target: &str, base: &RasterSnapshot) -> Result<Canvas, Refusal> {
    let layer = find_layer(&base.layers, layer_id).ok_or_else(|| Refusal::Error("mutation.target-missing", format!("Layer \"{layer_id}\" is not in the document.")))?;
    if !layer_protection(&base.layers, layer_id).is_some_and(|protection| protection.editable) {
        return Err(Refusal::Error("mutation.target-mismatch", format!("Layer \"{layer_id}\" is locked.")));
    }
    let layer_extent = match layer {
        RasterLayerNode::Pixel { width, height, .. } => (width.unwrap_or(512), height.unwrap_or(512)),
        _ => (512, 512),
    };
    let placement = match layer {
        RasterLayerNode::Pixel { transform, .. } | RasterLayerNode::Group { transform, .. } | RasterLayerNode::Adjustment { transform, .. } => transform.clone(),
    };
    if target == "mask" {
        let mask = match layer {
            RasterLayerNode::Pixel { mask, .. } | RasterLayerNode::Group { mask, .. } => mask.clone(),
            RasterLayerNode::Adjustment { .. } => None,
        }
        .ok_or_else(|| Refusal::Error("mutation.target-mismatch", format!("Layer \"{layer_id}\" has no mask.")))?;
        let source = match &mask.image_key {
            Some(key) => {
                let mut image = asset_image(base, key)?;
                if let Some(frame) = image.frames.first_mut() {
                    for pixel in frame.rgba8.chunks_exact_mut(4) {
                        let coverage = semio_framework_pixels::compositing::layers::mask_coverage([pixel[0], pixel[1], pixel[2], pixel[3]]);
                        pixel.copy_from_slice(&[255, 255, 255, coverage]);
                    }
                }
                image
            }
            None => blank(mask.width.unwrap_or(layer_extent.0), mask.height.unwrap_or(layer_extent.1), [255; 4])?,
        };
        return Ok(Canvas { source, previous: mask.image_key.clone(), prefix: "mask", mask: Some(mask), extent: (None, None), placement });
    }
    let RasterLayerNode::Pixel { image_key, width, height, .. } = layer else {
        return Err(Refusal::Error("mutation.target-mismatch", format!("Layer \"{layer_id}\" holds no pixels.")));
    };
    let source = match image_key {
        Some(key) => asset_image(base, key)?,
        None => blank(layer_extent.0, layer_extent.1, [0; 4])?,
    };
    Ok(Canvas { source, previous: image_key.clone(), prefix: "pixels", mask: None, extent: (*width, *height), placement })
}

/// 🎨️ What one repaint did: the canvas it painted, the painted image as an asset, and the key it is filed under.
pub(crate) struct Painted {
    canvas: Canvas,
    asset: crate::RasterImageAsset,
    key: String,
}

/// 🗄️ Files `pixels` repainted on `canvas` as a new content-addressed asset of `layer_id` in the lossless carrier.
pub(crate) fn painted(canvas: Canvas, pixels: Vec<u8>, layer_id: &str) -> Painted {
    let mut image = canvas.source.clone();
    image.frames[0].rgba8 = pixels;
    filed(canvas, image, layer_id)
}

/// 🗄️ Files `image` — of any extent — computed from `canvas` as a new content-addressed asset of `layer_id`.
pub(crate) fn reshaped(canvas: Canvas, image: RasterImage, layer_id: &str) -> Painted {
    let image = semio_image_from_rgba8(image.width, image.height, image.pixels);
    filed(canvas, image, layer_id)
}

fn filed(canvas: Canvas, image: SemioImageSnapshot, layer_id: &str) -> Painted {
    let asset = raster_image_pack_asset(&image);
    let key = store::content_id(&format!("{}-{layer_id}", canvas.prefix), &asset.data);
    Painted { canvas, asset, key }
}

/// 🖌️ Rasterizes `payload` on `base`; `Ok(None)` when no pixel changes (out of reach, or already painted).
fn paint(payload: &PaintStroke, base: &RasterSnapshot) -> Result<Option<Painted>, Refusal> {
    let canvas = canvas(&payload.layer_id, &payload.target, base)?;
    let Some(frame) = canvas.source.frames.first() else { return Err(Refusal::Fatal("mutation.apply.image-invalid", "The target image has no frame.".to_string())) };
    let mut image = RasterImage { width: canvas.source.width, height: canvas.source.height, pixels: frame.rgba8.clone() };
    let selection = match &payload.selection {
        Some(spans) => Some(selection_mask(spans, image.pixels.len() / 4).ok_or_else(|| Refusal::Error("mutation.target-mismatch", format!("The selection reaches past the {}×{} image.", image.width, image.height)))?),
        None => None,
    };
    if paint_stroke_in_place(&mut image, &operation(payload), selection.as_deref()).map_err(|error| Refusal::Fatal("mutation.apply.rasterize", error.to_string()))?.is_none() || image.pixels == frame.rgba8 {
        return Ok(None);
    }
    Ok(Some(painted(canvas, image.pixels, &payload.layer_id)))
}

/// 🗑️ Whether the image `key` that `layer_id`'s `target` stops showing is shown by nothing else and so leaves the pool.
fn released(base: &RasterSnapshot, key: &str, layer_id: &str, target: &str) -> bool {
    base.assets.contains_key(key)
        && !flatten_raster_layers(&base.layers).iter().any(|layer| {
            let own = layer_node_id(layer) == layer_id;
            let pixels = matches!(layer, RasterLayerNode::Pixel { image_key: Some(shown), .. } if shown == key) && !(own && target == "pixels");
            let mask = match layer {
                RasterLayerNode::Pixel { mask: Some(mask), .. } | RasterLayerNode::Group { mask: Some(mask), .. } => mask.image_key.as_deref() == Some(key) && !(own && target == "mask"),
                _ => false,
            };
            pixels || mask
        })
}
//#endregion 🎨️Raster

//#region 🔖️Diff
/// 🔺️ The stroke's diff on `base`: the painted image filed under its content key, the layer (or its mask) pointed at
/// it, the replaced image released when nothing else shows it.
pub fn diff(payload: &PaintStroke, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    if let Err((field, message)) = invariant(payload) {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, [field.to_string()]);
    }
    match paint(payload, base) {
        Ok(Some(painted)) => painted_diff(painted, base, &payload.layer_id, &payload.target, None),
        Ok(None) => protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The stroke changes no pixel of layer \"{}\".", payload.layer_id)),
        Err(refusal) => refused(refusal, &payload.layer_id),
    }
}

/// ⚠️ The outcome a refused repaint of `layer_id` raises.
pub(crate) fn refused(refusal: Refusal, layer_id: &str) -> protocol::MutationOutcome<RasterDiff> {
    match refusal {
        Refusal::Error(code, message) => protocol::MutationOutcome::error(code, message, [layer_id.to_string()]),
        Refusal::Fatal(code, message) => protocol::MutationOutcome::fatal(code, message, [layer_id.to_string()]),
    }
}

/// 🔺️ The diff a repaint of `layer_id`'s `target` makes on `base`: the painted image filed under its content key, the
/// layer (or its mask) pointed at it — at its `reshape`d extent and placement when the repaint moved the pixel grid — the
/// replaced image released when nothing else shows it.
pub(crate) fn painted_diff(painted: Painted, base: &RasterSnapshot, layer_id: &str, target: &str, reshape: Option<&Reshape>) -> protocol::MutationOutcome<RasterDiff> {
    let release = painted.canvas.previous.as_deref().filter(|previous| *previous != painted.key && released(base, previous, layer_id, target)).map(str::to_string);
    if !base.assets.contains_key(&painted.key) && base.assets.len() >= crate::RASTER_OWNED_MAP_CAPACITY {
        return protocol::MutationOutcome::fatal("mutation.apply.capacity", format!("The asset pool already holds {} images.", base.assets.len()), [layer_id.to_string()]);
    }
    let patch = match painted.canvas.mask {
        Some(mask) => RasterLayerPatch { mask_content: Some(RasterMaskContent { mask: Some(RasterLayerMask { image_key: Some(painted.key.clone()), ..mask }) }), ..Default::default() },
        None => RasterLayerPatch {
            pixel_content: Some(RasterPixelContent { image_key: Some(painted.key.clone()), width: reshape.map_or(painted.canvas.extent.0, |reshape| Some(reshape.width)), height: reshape.map_or(painted.canvas.extent.1, |reshape| Some(reshape.height)) }),
            transform: reshape.map(|reshape| reshape.transform.clone()),
            ..Default::default()
        },
    };
    let mut entries = std::collections::BTreeMap::new();
    entries.insert(painted.key, Some(painted.asset));
    if let Some(release) = release {
        entries.insert(release, None);
    }
    protocol::MutationOutcome::new(RasterDiff {
        layers: Some(RasterLayersDelta { patched: vec![RasterLayerPatchEntry { id: layer_id.to_string(), patch }], ..Default::default() }),
        assets: Some(RasterAssetsDelta { entries }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff

//#region 🔖️Inverse
/// ↩️ The steps that undo the stroke on `base`: the image it released back into the pool (in the lossless carrier, so
/// the restored child is the exact one), the layer (or its mask) pointed back at it, and the painted image released
/// when the stroke brought it in. Nothing when the stroke changed nothing.
pub fn inverse(payload: &PaintStroke, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    Ok({
    if invariant(payload).is_err() {
        return Ok(Vec::new());
    }
    let Ok(Some(painted)) = paint(payload, base) else { return Ok(Vec::new() )};
    painted_inverse(painted, base, &payload.layer_id, &payload.target, None)?

    })
}

/// ↩️ The steps that undo a repaint of `layer_id`'s `target` on `base`: the image it released back into the pool (in the
/// lossless carrier, so the restored child is the exact one), the layer (or its mask) pointed back at it — with its prior
/// placement when the repaint was `reshape`d — and the painted image released when the repaint brought it in.
pub(crate) fn painted_inverse(painted: Painted, base: &RasterSnapshot, layer_id: &str, target: &str, reshape: Option<&Reshape>) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    use crate::mutations::{add_layer_asset::AddLayerAsset, change_layer_mask::ChangeLayerMask, change_layer_pixels::ChangeLayerPixels, remove_layer_asset::RemoveLayerAsset};
    let mut steps = Vec::new();
    if let Some(previous) = painted.canvas.previous.as_deref().filter(|previous| *previous != painted.key && released(base, previous, layer_id, target)) {
        let Ok(image) = asset_image(base, previous) else { return Vec::new() };
        steps.push(RasterMutation::AddLayerAsset(AddLayerAsset { asset_id: previous.to_string(), asset: raster_image_pack_asset(&image) }));
    }
    match &painted.canvas.mask {
        Some(mask) => steps.push(RasterMutation::ChangeLayerMask(ChangeLayerMask { layer_id: layer_id.to_string(), expected: Some(RasterLayerMask { image_key: Some(painted.key.clone()), ..mask.clone() }), mask: Some(mask.clone()) })),
        None => steps.push(RasterMutation::ChangeLayerPixels(ChangeLayerPixels {
            layer_id: layer_id.to_string(),
            expected_image_key: Some(painted.key.clone()),
            content: RasterPixelContent { image_key: painted.canvas.previous.clone(), width: painted.canvas.extent.0, height: painted.canvas.extent.1 },
            transform: reshape.map(|_| painted.canvas.placement.clone()),
        })),
    }
    if !base.assets.contains_key(&painted.key) {
        steps.push(RasterMutation::RemoveLayerAsset(RemoveLayerAsset { asset_id: painted.key }));
    }
    steps

    })())
}
//#endregion 🔖️Inverse

//#region 🔖️Kind
impl protocol::MutationKind<RasterSnapshot, RasterMutation> for PaintStroke {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "paint", entity: "stroke", kind: "paint-stroke", record: "PaintedStroke" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let count = self.points.len();
        let (en_points, de_points) = if count == 1 { ("1 point".to_string(), "1 Punkt".to_string()) } else { (format!("{count} points"), format!("{count} Punkten")) };
        let (en, de) = match (self.target.as_str(), self.tool.as_str()) {
            ("mask", "eraser") => ("Mask eraser stroke", "Masken-Radierstrich"),
            ("mask", _) => ("Mask stroke", "Maskenstrich"),
            (_, "eraser") => ("Eraser stroke", "Radierstrich"),
            _ => ("Brush stroke", "Pinselstrich"),
        };
        semio_framework_ui_locale::LocalizedLabel::native(&format!("{en} of {en_points} on layer {}", self.layer_id), &format!("{de} mit {de_points} auf Ebene {}", self.layer_id))
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
