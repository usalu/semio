//! 🔲 `write-pixel-region` — writes one rectangle of RGBA8 samples into the image a layer (or its mask) shows. It is the
//! sparse restoration `paint-stroke`, `fill-region` and `fill-selection` invert to (the samples the repaint overwrote, read
//! from the base image), and a gesture of its own for a host that already holds the pixels to write.

use crate::diff::{RasterDiff, RasterPixelRegion};
use crate::mutations::paint_stroke::{canvas, inverse_pixel_region, refused, rect_samples, RASTER_PAINT_TARGETS};
use crate::{RasterMutation, RasterSnapshot};

//#region 🔖️Payload
/// 🔲 `width × height` RGBA8 `samples`, row-major, written at `(x, y)` of `layer_id`'s `target` image (`pixels` or `mask`).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct WritePixelRegion {
    pub layer_id: String,
    pub target: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub samples: Vec<u8>,
}

impl WritePixelRegion {
    fn region(&self) -> RasterPixelRegion {
        RasterPixelRegion { layer_id: self.layer_id.clone(), target: self.target.clone(), x: self.x, y: self.y, width: self.width, height: self.height, samples: self.samples.clone() }
    }
}
//#endregion 🔖️Payload

//#region 🔖️Diff
/// 🔺️ The sparse diff on `base`: one pixel region. Error `target-missing`/`target-mismatch` when the layer or its image is
/// not there; Fatal `invariant` when the payload's vocabulary or rectangle is broken; Warning `no-op` when the rectangle
/// already holds the samples.
pub fn diff(payload: &WritePixelRegion, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    if !RASTER_PAINT_TARGETS.contains(&payload.target.as_str()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Write target \"{}\" is neither pixels nor mask.", payload.target), ["target".to_string()]);
    }
    let target = match canvas(&payload.layer_id, &payload.target, base) {
        Ok(target) => target,
        Err(refusal) => return refused(refusal, &payload.layer_id),
    };
    let Some(frame) = target.source.frames.first() else {
        return protocol::MutationOutcome::fatal("mutation.apply.image-invalid", "The target image has no frame.", [payload.layer_id.clone()]);
    };
    let (width, height) = (target.source.width, target.source.height);
    let fits = payload.width > 0
        && payload.height > 0
        && payload.x.checked_add(payload.width).is_some_and(|end| end <= width)
        && payload.y.checked_add(payload.height).is_some_and(|end| end <= height)
        && payload.samples.len() == (payload.width as usize) * (payload.height as usize) * 4;
    if !fits {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("The {}×{} region at ({}, {}) must lie inside the {width}×{height} image and carry 4 samples per pixel.", payload.width, payload.height, payload.x, payload.y), ["region".to_string()]);
    }
    if rect_samples(&frame.rgba8, width, payload.x, payload.y, payload.width, payload.height).as_deref() == Some(payload.samples.as_slice()) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The region of layer \"{}\" already holds those samples.", payload.layer_id));
    }
    protocol::MutationOutcome::new(RasterDiff { pixels: vec![payload.region()], ..Default::default() })
}
//#endregion 🔖️Diff

//#region 🔖️Inverse
/// ↩️ The same kind carrying `base`'s samples of the rectangle. Nothing when `base` shows no image there or already holds the
/// samples.
pub fn inverse(payload: &WritePixelRegion, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    let Some(restore) = inverse_pixel_region(&payload.region(), base) else { return Ok(Vec::new()) };
    if restore.samples == payload.samples {
        return Ok(Vec::new());
    }
    Ok(vec![RasterMutation::WritePixelRegion(WritePixelRegion { layer_id: restore.layer_id, target: restore.target, x: restore.x, y: restore.y, width: restore.width, height: restore.height, samples: restore.samples })])
}
//#endregion 🔖️Inverse

//#region 🔖️Kind
impl protocol::MutationKind<RasterSnapshot, RasterMutation> for WritePixelRegion {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "pixel-region", kind: "write-pixel-region", record: "WrittenPixelRegion" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        diff(self, base)
    }

    fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
        inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Write {}×{} pixels at ({}, {}) on layer {}", self.width, self.height, self.x, self.y, self.layer_id), &format!("{}×{} Pixel bei ({}, {}) auf Ebene {} schreiben", self.width, self.height, self.x, self.y, self.layer_id))
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
