//! 🖌️ Repaint support: the canvas a stroke or fill paints into, its refusals and the painted diff.

use super::*;

/// 🖼️ The image a repaint (a stroke, a region fill) paints into and where it sits: the source image (decoded, so its
/// profile and metadata ride along), its asset key, the key prefix of the image the repaint mints, and the layer's mask
/// when the target is one.
pub struct Canvas {
    pub source: SemioImageSnapshot,
    previous: Option<String>,
    prefix: &'static str,
    pub(super) mask: Option<RasterLayerMask>,
    pub extent: (Option<u32>, Option<u32>),
    pub placement: RasterTransform,
}

/// 📐️ A repaint that also moves the layer's pixel grid (a rotation, a resize, a crop): the image's new display extent and
/// the layer's new placement.
pub struct Reshape {
    pub width: u32,
    pub height: u32,
    pub transform: RasterTransform,
}

/// ⚠️ Why a repaint cannot land: a state-dependent Error (the layer or its image is not there, or does not take this
/// repaint) or a Fatal application fault, with the target it names.
pub enum Refusal {
    Error(&'static str, String),
    Fatal(&'static str, String),
}

/// 🧩️ The decoded image behind `key`, from the asset pool's own materialized child.
pub fn asset_image(base: &RasterSnapshot, key: &str) -> Result<SemioImageSnapshot, Refusal> {
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
pub fn canvas(layer_id: &str, target: &str, base: &RasterSnapshot) -> Result<Canvas, Refusal> {
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
pub struct Painted {
    canvas: Canvas,
    pub(super) asset: SemioImageSnapshot,
    pub(super) key: String,
}

/// 🗄️ Files `pixels` repainted on `canvas` as a new content-addressed asset of `layer_id` in the lossless carrier.
pub fn painted(canvas: Canvas, pixels: Vec<u8>, layer_id: &str) -> Painted {
    let mut image = canvas.source.clone();
    image.frames[0].rgba8 = pixels;
    filed(canvas, image, layer_id)
}

/// 🗄️ Files `image` — of any extent — computed from `canvas` as a new content-addressed asset of `layer_id`.
pub fn reshaped(canvas: Canvas, image: RasterImage, layer_id: &str) -> Painted {
    let image = semio_image_from_rgba8(image.width, image.height, image.pixels);
    filed(canvas, image, layer_id)
}

pub(super) fn filed(canvas: Canvas, image: SemioImageSnapshot, layer_id: &str) -> Painted {
    let key = semio_s_artifact_raster_raster_model::raster_image_content_id(&format!("{}-{layer_id}", canvas.prefix), &image);
    let asset = image;
    Painted { canvas, asset, key }
}

/// 🗑️ Whether the image `key` that `layer_id`'s `target` stops showing is shown by nothing else and so leaves the pool.
pub fn released(base: &RasterSnapshot, key: &str, layer_id: &str, target: &str) -> bool {
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

/// 🧱️ `width × height` RGBA8 samples at `(x, y)` of a row-major `pixels` buffer `image_width` pixels wide; `None` when the
/// rectangle leaves the buffer.
pub fn rect_samples(pixels: &[u8], image_width: u32, x: u32, y: u32, width: u32, height: u32) -> Option<Vec<u8>> {
    let row_bytes = (width as usize) * 4;
    let mut samples = Vec::with_capacity(row_bytes * height as usize);
    for row in 0..height as usize {
        let at = ((y as usize + row) * image_width as usize + x as usize) * 4;
        samples.extend_from_slice(pixels.get(at..at + row_bytes)?);
    }
    Some(samples)
}

/// ↩️ The region that restores the rectangle `region` writes, read from `base`'s own samples; `None` when `base` shows no
/// image there (the layer's pointer and pool are then restored by the whole-image inverse).
pub fn inverse_pixel_region(region: &RasterPixelRegion, base: &RasterSnapshot) -> Option<RasterPixelRegion> {
    let target = canvas(&region.layer_id, &region.target, base).ok()?;
    target.previous.as_ref()?;
    let frame = target.source.frames.first()?;
    Some(RasterPixelRegion { samples: rect_samples(&frame.rgba8, target.source.width, region.x, region.y, region.width, region.height)?, ..region.clone() })
}

/// 🔺️ The diff a repaint of `layer_id`'s `target` makes on `base`: the painted image filed under its content key, the
/// layer (or its mask) pointed at it — at its `reshape`d extent and placement when the repaint moved the pixel grid — the
/// replaced image released when nothing else shows it.
pub fn painted_diff(painted: Painted, base: &RasterSnapshot, layer_id: &str, target: &str, reshape: Option<&Reshape>) -> protocol::MutationOutcome<RasterDiff> {
    let release = painted.canvas.previous.as_deref().filter(|previous| *previous != painted.key && released(base, previous, layer_id, target)).map(str::to_string);
    if !base.assets.contains_key(&painted.key) && base.assets.len() >= semio_s_artifact_raster_raster_model::RASTER_OWNED_MAP_CAPACITY {
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
        layers: Some(RasterLayersDelta { modified: vec![RasterLayerModification { id: layer_id.to_string(), patch }], ..Default::default() }),
        assets: Some(RasterAssetsDelta { entries }),
        ..Default::default()
    })
}
