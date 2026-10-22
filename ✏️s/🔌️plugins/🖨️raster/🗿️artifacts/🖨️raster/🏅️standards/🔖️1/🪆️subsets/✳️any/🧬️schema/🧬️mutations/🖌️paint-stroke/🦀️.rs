//! 🖌️ `paint-stroke` — one brush or eraser stroke on a layer's pixels or on its mask, stated as its intent: the layer,
//! the target, the tool, the brush and the points in the target image's pixels, in drawing order. The pixels are
//! rasterized on every application by the ONE deterministic rasterizer
//! (`semio_framework_pixels::editing::paint_stroke_in_place`), so a history edit of the brush or of the points
//! replays onto whatever base it lands on. The painted image becomes a new content-addressed asset in the lossless
//! image-pack carrier, the layer (or its mask) points at it, and the image it replaced leaves the pool once nothing
//! else shows it. The canvas — resolving the target image, filing a repainted one and restoring it — is shared with
//! `🪣️fill-region`. Design §17.2, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING.

use semio_s_artifact_raster_raster_diff::Painted;
use semio_s_artifact_raster_raster_diff::Refusal;
use semio_s_artifact_raster_raster_diff::Reshape;
use semio_s_artifact_raster_raster_diff::asset_image;
use semio_s_artifact_raster_raster_diff::canvas;
use semio_s_artifact_raster_raster_diff::inverse_pixel_region;
use semio_s_artifact_raster_raster_diff::painted;
use semio_s_artifact_raster_raster_diff::painted_diff;
use semio_s_artifact_raster_raster_diff::rect_samples;
use semio_s_artifact_raster_raster_diff::released;
use semio_s_artifact_raster_raster_diff::{RasterDiff, RasterPixelRegion};

#[cfg(test)] use semio_s_artifact_raster_raster_model::find_layer;
use semio_s_artifact_raster_raster_model::{RasterLayerMask, RasterPixelContent};
#[cfg(test)] use semio_s_artifact_raster_raster_model::RasterLayerNode;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;
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
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct RasterStrokePoint {
    pub x: f64,
    pub y: f64,
}

/// 🖌️ The brush a stroke paints with: its diameter in pixels, the fraction of the radius painted at full strength, its
/// opacity, and its straight-alpha sRGB colour (four channels, each 0..1). On a mask the brush paints the colour's grey
/// level as coverage; the eraser ignores the colour.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct RasterBrush {
    pub size: f64,
    pub hardness: f64,
    pub opacity: f64,
    pub color: Vec<f64>,
}

/// ✂️ One run of a pixel selection the stroke is clipped to: `length` pixels from row-major pixel index `start`, each
/// covered `coverage` out of 255.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, schema::ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword="selection-span")]
#[artifact_schema(id="s.raster.raster.selectionspan")]
pub struct RasterSelectionSpan {
    #[state(config)]
    pub start: u32,
    #[state(config)]
    pub length: u32,
    #[state(config)]
    pub coverage: u32,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
pub fn paint_stroke<Op: From<crate::add_layer_asset::mutation::AddLayerAsset> + From<crate::change_layer_mask::ChangeLayerMask> + From<crate::change_layer_pixels::ChangeLayerPixels> + From<crate::paint_stroke::PaintStroke> + From<crate::remove_layer_asset::mutation::RemoveLayerAsset> + From<crate::write_pixel_region::WritePixelRegion>>(layer_id: impl Into<String>, target: &str, tool: &str, brush: RasterBrush, points: Vec<RasterStrokePoint>) -> Op {
    Op::from(PaintStroke { layer_id: layer_id.into(), target: target.to_string(), tool: tool.to_string(), brush, points, selection: None })
}
//#endregion 🔖️Payload

//#region 🎨️Raster
/// 🔢️ One unit channel as the byte the rasterizer paints.
pub fn channel_byte(value: f64) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// 🌗️ The coverage a mask brush of `color` paints: the colour's Rec. 709 grey level as a byte.
pub fn grey_byte(color: &[f64]) -> u8 {
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
pub fn selection_invariant(selection: Option<&[RasterSelectionSpan]>) -> Result<(), (&'static str, String)> {
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
pub fn selection_mask(spans: &[RasterSelectionSpan], count: usize) -> Option<Vec<u8>> {
    let mut mask = vec![0_u8; count];
    for span in spans {
        let start = span.start as usize;
        mask.get_mut(start..start.checked_add(span.length as usize)?)?.fill(span.coverage as u8);
    }
    Some(mask)
}

/// 🖌️ The rasterizer operation `payload` means on its target.
pub fn operation(payload: &PaintStroke) -> PixelOperation {
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

//#endregion 🎨️Raster

//#region 🔖️Region

/// 🧱️ The smallest rectangle holding every pixel that differs between `before` and `after` (same extent), with `after`'s
/// samples; `None` when nothing differs.
fn changed_region(image_width: u32, before: &[u8], after: &[u8], layer_id: &str, target: &str) -> Option<RasterPixelRegion> {
    if before.len() != after.len() || image_width == 0 {
        return None;
    }
    let width = image_width as usize;
    let mut bounds: Option<(usize, usize, usize, usize)> = None;
    for (index, (old, new)) in before.chunks_exact(4).zip(after.chunks_exact(4)).enumerate() {
        if old != new {
            let (x, y) = (index % width, index / width);
            bounds = Some(match bounds {
                Some((left, top, right, bottom)) => (left.min(x), top.min(y), right.max(x), bottom.max(y)),
                None => (x, y, x, y),
            });
        }
    }
    let (left, top, right, bottom) = bounds?;
    let (x, y, region_width, region_height) = (left as u32, top as u32, (right - left + 1) as u32, (bottom - top + 1) as u32);
    Some(RasterPixelRegion { layer_id: layer_id.to_string(), target: target.to_string(), x, y, width: region_width, height: region_height, samples: rect_samples(after, image_width, x, y, region_width, region_height)? })
}

/// 🔺️ The diff a repaint makes: one pixel region (the changed rectangle with its new samples) when the layer already shows an
/// image, otherwise the first image filed whole (`painted_diff`).
pub fn repainted_diff(painted: Painted, base: &RasterSnapshot, layer_id: &str, target: &str) -> protocol::MutationOutcome<RasterDiff> {
    if painted.canvas.previous.is_some() {
        if let (Some(before), Some(after)) = (painted.canvas.source.frames.first(), painted.asset.frames.first()) {
            if let Some(region) = changed_region(painted.canvas.source.width, &before.rgba8, &after.rgba8, layer_id, target) {
                return protocol::MutationOutcome::new(RasterDiff { pixels: vec![region], ..Default::default() });
            }
        }
    }
    painted_diff(painted, base, layer_id, target, None)
}


/// ↩️ The steps that undo a repaint: one `write-pixel-region` with `base`'s samples of the changed rectangle when the layer
/// already showed an image, otherwise the whole-image restoration (`painted_inverse`).
pub fn repainted_inverse<Op: From<crate::add_layer_asset::mutation::AddLayerAsset> + From<crate::change_layer_mask::ChangeLayerMask> + From<crate::change_layer_pixels::ChangeLayerPixels> + From<crate::paint_stroke::PaintStroke> + From<crate::remove_layer_asset::mutation::RemoveLayerAsset> + From<crate::write_pixel_region::WritePixelRegion>>(painted: Painted, base: &RasterSnapshot, layer_id: &str, target: &str) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if painted.canvas.previous.is_some() {
        if let (Some(before), Some(after)) = (painted.canvas.source.frames.first(), painted.asset.frames.first()) {
            if let Some(region) = changed_region(painted.canvas.source.width, &before.rgba8, &after.rgba8, layer_id, target) {
                if let Some(restore) = inverse_pixel_region(&region, base) {
                    return Ok(vec![Op::from(crate::write_pixel_region::WritePixelRegion { layer_id: restore.layer_id, target: restore.target, x: restore.x, y: restore.y, width: restore.width, height: restore.height, samples: restore.samples })]);
                }
            }
        }
    }
    painted_inverse(painted, base, layer_id, target, None)
}
//#endregion 🔖️Region

//#region 🔖️Diff
/// 🔺️ The stroke's diff on `base`: the painted image filed under its content key, the layer (or its mask) pointed at
/// it, the replaced image released when nothing else shows it.
pub fn diff(payload: &PaintStroke, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    if let Err((field, message)) = invariant(payload) {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, [field.to_string()]);
    }
    match paint(payload, base) {
        Ok(Some(painted)) => repainted_diff(painted, base, &payload.layer_id, &payload.target),
        Ok(None) => protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The stroke changes no pixel of layer \"{}\".", payload.layer_id)),
        Err(refusal) => refused(refusal, &payload.layer_id),
    }
}

/// ⚠️ The outcome a refused repaint of `layer_id` raises.
pub fn refused(refusal: Refusal, layer_id: &str) -> protocol::MutationOutcome<RasterDiff> {
    match refusal {
        Refusal::Error(code, message) => protocol::MutationOutcome::error(code, message, [layer_id.to_string()]),
        Refusal::Fatal(code, message) => protocol::MutationOutcome::fatal(code, message, [layer_id.to_string()]),
    }
}

//#endregion 🔖️Diff

//#region 🔖️Inverse
/// ↩️ The steps that undo the stroke on `base`: the image it released back into the pool (in the lossless carrier, so
/// the restored child is the exact one), the layer (or its mask) pointed back at it, and the painted image released
/// when the stroke brought it in. Nothing when the stroke changed nothing.
pub fn inverse<Op: From<crate::add_layer_asset::mutation::AddLayerAsset> + From<crate::change_layer_mask::ChangeLayerMask> + From<crate::change_layer_pixels::ChangeLayerPixels> + From<crate::paint_stroke::PaintStroke> + From<crate::remove_layer_asset::mutation::RemoveLayerAsset> + From<crate::write_pixel_region::WritePixelRegion>>(payload: &PaintStroke, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
    if invariant(payload).is_err() {
        return Ok(Vec::new());
    }
    let Ok(Some(painted)) = paint(payload, base) else { return Ok(Vec::new() )};
    repainted_inverse(painted, base, &payload.layer_id, &payload.target)?

    })
}

/// ↩️ The steps that undo a repaint of `layer_id`'s `target` on `base`: the image it released back into the pool (in the
/// lossless carrier, so the restored child is the exact one), the layer (or its mask) pointed back at it — with its prior
/// placement when the repaint was `reshape`d — and the painted image released when the repaint brought it in.
pub fn painted_inverse<Op: From<crate::add_layer_asset::mutation::AddLayerAsset> + From<crate::change_layer_mask::ChangeLayerMask> + From<crate::change_layer_pixels::ChangeLayerPixels> + From<crate::paint_stroke::PaintStroke> + From<crate::remove_layer_asset::mutation::RemoveLayerAsset> + From<crate::write_pixel_region::WritePixelRegion>>(painted: Painted, base: &RasterSnapshot, layer_id: &str, target: &str, reshape: Option<&Reshape>) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    use crate::add_layer_asset::mutation::AddLayerAsset;
    use crate::change_layer_mask::ChangeLayerMask;
    use crate::change_layer_pixels::ChangeLayerPixels;
    use crate::remove_layer_asset::mutation::RemoveLayerAsset;
    let mut steps = Vec::new();
    if let Some(previous) = painted.canvas.previous.as_deref().filter(|previous| *previous != painted.key && released(base, previous, layer_id, target)) {
        let Ok(image) = asset_image(base, previous) else { return Vec::new() };
        steps.push(Op::from(AddLayerAsset { asset_id: previous.to_string(), asset: image }));
    }
    match &painted.canvas.mask {
        Some(mask) => steps.push(Op::from(ChangeLayerMask { layer_id: layer_id.to_string(), expected: Some(RasterLayerMask { image_key: Some(painted.key.clone()), ..mask.clone() }), mask: Some(mask.clone()) })),
        None => steps.push(Op::from(ChangeLayerPixels {
            layer_id: layer_id.to_string(),
            expected_image_key: Some(painted.key.clone()),
            content: RasterPixelContent { image_key: painted.canvas.previous.clone(), width: painted.canvas.extent.0, height: painted.canvas.extent.1 },
            transform: reshape.map(|_| painted.canvas.placement.clone()),
        })),
    }
    if !base.assets.contains_key(&painted.key) {
        steps.push(Op::from(RemoveLayerAsset { asset_id: painted.key }));
    }
    steps

    })())
}
//#endregion 🔖️Inverse

//#region 🔖️Kind
impl<Op> protocol::MutationKind<RasterSnapshot, Op> for PaintStroke where Op: protocol::Mutation<RasterSnapshot, Diff = semio_s_artifact_raster_raster_diff::RasterDiff> + From<crate::add_layer_asset::mutation::AddLayerAsset> + From<crate::change_layer_mask::ChangeLayerMask> + From<crate::change_layer_pixels::ChangeLayerPixels> + From<crate::paint_stroke::PaintStroke> + From<crate::remove_layer_asset::mutation::RemoveLayerAsset> + From<crate::write_pixel_region::WritePixelRegion> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "paint", entity: "stroke", kind: "paint-stroke", record: "PaintedStroke" };

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

impl<Op> protocol::ColdMutationKind<RasterSnapshot, Op> for PaintStroke where Op: protocol::ColdMutation<RasterSnapshot, ReferenceDiff = semio_s_artifact_raster_raster_diff::RasterDiff> + From<crate::add_layer_asset::mutation::AddLayerAsset> + From<crate::change_layer_mask::ChangeLayerMask> + From<crate::change_layer_pixels::ChangeLayerPixels> + From<crate::paint_stroke::PaintStroke> + From<crate::remove_layer_asset::mutation::RemoveLayerAsset> + From<crate::write_pixel_region::WritePixelRegion> {
    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        diff(self, base)
    }
    fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Kind

//#region 🧪️Tests
//#endregion 🧪️Tests
