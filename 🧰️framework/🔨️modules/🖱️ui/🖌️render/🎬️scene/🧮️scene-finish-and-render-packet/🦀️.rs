//! 🧮️ Scene finish and render packet: the layer state, draw batch, scene errors, `Scene::finish` with its validate, snap, order, batch and hash passes and the `RenderPacket`. See [`super`].

use super::*;

//#region 🔖️Scene
//#region 🔖️Finish
/// 🏷️ The scissor/clip/glass-membership/overlay state a [`DrawBatch`] renders under.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LayerState {
    pub scissor: Option<ScissorRect>,
    pub clip: Option<ClipRegion>,
    pub foreground_of: Option<usize>,
    pub overlay: bool,
}

/// 🎞️ One replayable draw call: a pipeline, the state to render under, an instance range into
/// [`RenderPacket::quad_instances`] or [`RenderPacket::vector_vertices`] (backend picks the array by
/// `pipeline`), and an optional precomputed stencil-mask range (also into `quad_instances`) a backend
/// paints with [`StencilPolicy::WriteMask`] before painting `instance_range` with
/// [`StencilPolicy::RequireMaskEquality`]. A backend replays ranges; it makes no clipping decisions.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawBatch {
    pub pipeline: PipelineKind,
    pub layer_state: LayerState,
    pub instance_range: (u32, u32),
    pub mask_range: Option<(u32, u32)>,
    pub texture: Option<TextureId>,
}

/// ⚠️ Why [`Scene::finish`] refused to build a [`RenderPacket`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneError {
    UnbalancedScissorStack,
    UnbalancedClipStack,
    UnbalancedGlassStack,
    NonFiniteGeometry,
    NegativeRect,
}

/// 🎬️ Per-frame inputs that are not part of the builder's own accumulated state.
pub struct FinishParams {
    pub viewport: [f32; 2],
    pub dpr: f32,
    pub time_seconds_origin: f64,
    pub resource_ops: Vec<ResourceOp>,
}

/// 🎬️ The empty namespace for [`Scene::finish`]'s five-step pipeline.
pub struct Scene;

impl Scene {
    /// 🏁️ validate → snap → order → batch → hash. Consumes `builder`; an equal `content_hash` on two
    /// packets built from equal input lets a host skip submission entirely.
    #[cfg(any(test, feature = "backend-testing"))]
    pub fn finish(builder: SceneBuilder, params: FinishParams) -> Result<RenderPacket, SceneError> {
        validate(&builder)?;
        let mut snapped = snap(builder, params.dpr);
        let layers = std::mem::take(&mut snapped.layers);
        let mut scene_passes = std::mem::take(&mut snapped.scene_passes);
        let ordered_layers = order(layers, &mut scene_passes);
        let (quad_instances, vector_vertices, glass_instances, batches) = batch(&ordered_layers, &snapped.glass_regions, params.viewport);
        let has_animated_primitives = quad_instances.iter().any(|quad| is_animated_kind(quad.params[2]));
        let content_hash = content_hash(&quad_instances, &vector_vertices, &glass_instances, &batches);
        Ok(RenderPacket {
            viewport: params.viewport,
            dpr: params.dpr,
            time_seconds_origin: params.time_seconds_origin,
            quad_instances,
            vector_vertices,
            glass_instances,
            batches,
            surface_passes: scene_passes,
            resource_ops: params.resource_ops,
            has_animated_primitives,
            content_hash,
        })
    }
}

//#region Validate
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(any(test, feature = "backend-testing"))]
fn validate(builder: &SceneBuilder) -> Result<(), SceneError> {
    if !builder.scissor_stack.is_empty() {
        return Err(SceneError::UnbalancedScissorStack);
    }
    if !builder.clip_stack.is_empty() {
        return Err(SceneError::UnbalancedClipStack);
    }
    if !builder.glass_content_stack.is_empty() {
        return Err(SceneError::UnbalancedGlassStack);
    }
    for layer in &builder.layers {
        for quad in layer.quad_instances.iter().chain(layer.overlay_quad_instances.iter()).chain(layer.raster_instances.iter().map(|(_, quad)| quad)) {
            validate_quad(quad)?;
        }
        for vertex in layer.vector_vertices.iter().chain(layer.overlay_vector_vertices.iter()) {
            if !vertex.position.iter().chain(vertex.color.iter()).all(|value| value.is_finite()) {
                return Err(SceneError::NonFiniteGeometry);
            }
        }
    }
    for region in &builder.glass_regions {
        let finite = region.rect.iter().chain(region.tint.iter()).chain([&region.radius, &region.alpha, &region.blur_px, &region.saturate]).all(|value| value.is_finite());
        if !finite {
            return Err(SceneError::NonFiniteGeometry);
        }
        if region.rect[2] < 0.0 || region.rect[3] < 0.0 {
            return Err(SceneError::NegativeRect);
        }
    }
    Ok(())
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(any(test, feature = "backend-testing"))]
fn validate_quad(quad: &QuadInstance) -> Result<(), SceneError> {
    let finite = quad.rect.iter().chain(quad.color.iter()).chain(quad.params.iter()).chain(quad.uv_rect.iter()).all(|value| value.is_finite());
    if !finite {
        return Err(SceneError::NonFiniteGeometry);
    }
    if quad.rect[2] < 0.0 || quad.rect[3] < 0.0 {
        return Err(SceneError::NegativeRect);
    }
    Ok(())
}
//#endregion Validate

//#region Snap
/// 🎯️ Logical→physical pixel rounding. Kills a class of shimmer (an edge that lands on a fractional
/// physical pixel dithers between neighboring pixels as content scrolls) and makes goldens
/// deterministic: two builders with the same logical geometry snap to the exact same physical rects
/// at a given `dpr`, regardless of what produced the fractional logical values upstream.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(any(test, feature = "backend-testing"))]
fn snap(mut builder: SceneBuilder, dpr: f32) -> SceneBuilder {
    for layer in &mut builder.layers {
        for quad in layer.quad_instances.iter_mut().chain(layer.overlay_quad_instances.iter_mut()).chain(layer.raster_instances.iter_mut().map(|(_, quad)| quad)) {
            quad.rect = tessellate::snap_rect(quad.rect, dpr);
        }
        for vertex in layer.vector_vertices.iter_mut().chain(layer.overlay_vector_vertices.iter_mut()) {
            vertex.position = tessellate::snap_point(vertex.position, dpr);
        }
    }
    for region in &mut builder.glass_regions {
        region.rect = tessellate::snap_rect(region.rect, dpr);
    }
    builder
}
//#endregion Snap

//#region Order
/// 🗜️ Merges consecutive layers that share identical `(scissor, clip, foreground_of)` state and drops
/// layers left empty by upstream push/pop pairs that never drew anything between them — the wgpu
/// target's `push_scissor`/`pop_scissor`/`begin_silhouette_clip`/`end_silhouette_clip` each start a new
/// layer unconditionally, so returning to an outer clip state that was already active produces an
/// empty boundary layer plus a content layer identical in state to one seen before; dropping the empty
/// one lets the identical-state survivors coalesce into a single batch. A layer referenced by a
/// [`SurfacePass::layer_index`] (an "anchor") is kept even if empty and is never merged with a
/// neighbor in either direction, because its exact byte offset is what `quad_watermark`/
/// `vector_watermark` point into; `scene_passes` has its `layer_index` remapped to the anchor's new
/// position afterward.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(any(test, feature = "backend-testing"))]
fn order(layers: Vec<SceneLayer>, scene_passes: &mut [SurfacePass]) -> Vec<SceneLayer> {
    let anchors: HashSet<usize> = scene_passes.iter().map(|pass| pass.layer_index).collect();
    let survivors: Vec<(usize, SceneLayer)> = layers.into_iter().enumerate().filter(|(old_index, layer)| anchors.contains(old_index) || !layer.is_empty()).collect();

    let mut merged: Vec<SceneLayer> = Vec::new();
    let mut last_was_anchor = false;
    let mut anchor_new_index: HashMap<usize, usize> = HashMap::new();

    for (old_index, layer) in survivors {
        let is_anchor = anchors.contains(&old_index);
        let can_merge = !is_anchor && !last_was_anchor && merged.last().is_some_and(|prev: &SceneLayer| prev.scissor == layer.scissor && prev.clip == layer.clip && prev.foreground_of == layer.foreground_of);
        if can_merge {
            merged.last_mut().expect("checked by can_merge").merge_from(layer);
        } else {
            if is_anchor {
                anchor_new_index.insert(old_index, merged.len());
            }
            merged.push(layer);
        }
        last_was_anchor = is_anchor;
    }

    for pass in scene_passes.iter_mut() {
        if let Some(&new_index) = anchor_new_index.get(&pass.layer_index) {
            pass.layer_index = new_index;
        }
    }
    merged
}
//#endregion Order

//#region Batch
#[cfg(any(test, feature = "backend-testing"))]
struct LayerSpan {
    pub(super) scissor: Option<ScissorRect>,
    pub(super) clip: Option<ClipRegion>,
    pub(super) foreground_of: Option<usize>,
    quad_range: (u32, u32),
    raster_runs: Vec<(TextureId, u32, u32)>,
    vec_range: (u32, u32),
}

/// 📦️ Copies one stream (backdrop-vs-foreground × normal-vs-overlay, selected by the caller) out of
/// `layers` into the flat `quad_instances`/`vector_vertices` arrays, recording each contributing
/// layer's ranges. Ported from the wgpu target's `build_layer_batches`/`build_overlay_layer_batches`,
/// generalized to also chunk `raster_instances` into contiguous same-[`TextureId`] runs (each run
/// needs its own batch — a backend binds one texture at a time).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(any(test, feature = "backend-testing"))]
fn collect_spans<'a>(layers: impl Iterator<Item = &'a SceneLayer>, overlay: bool, quad_instances: &mut Vec<QuadInstance>, vector_vertices: &mut Vec<VectorVertex>) -> Vec<LayerSpan> {
    let mut spans = Vec::new();
    for layer in layers {
        let (quads, raster, vertices): (&[QuadInstance], &[(TextureId, QuadInstance)], &[VectorVertex]) =
            if overlay { (&layer.overlay_quad_instances, &[], &layer.overlay_vector_vertices) } else { (&layer.quad_instances, &layer.raster_instances, &layer.vector_vertices) };
        if quads.is_empty() && raster.is_empty() && vertices.is_empty() {
            continue;
        }

        let quad_start = quad_instances.len() as u32;
        quad_instances.extend_from_slice(quads);
        let quad_range = (quad_start, quads.len() as u32);

        let mut raster_runs = Vec::new();
        let mut index = 0;
        while index < raster.len() {
            let texture = raster[index].0;
            let run_start = quad_instances.len() as u32;
            let mut count = 0u32;
            while index < raster.len() && raster[index].0 == texture {
                quad_instances.push(raster[index].1);
                index += 1;
                count += 1;
            }
            raster_runs.push((texture, run_start, count));
        }

        let vec_start = vector_vertices.len() as u32;
        vector_vertices.extend_from_slice(vertices);
        let vec_range = (vec_start, vertices.len() as u32);

        spans.push(LayerSpan { scissor: layer.scissor, clip: layer.clip.clone(), foreground_of: layer.foreground_of, quad_range, raster_runs, vec_range });
    }
    spans
}

/// 🎞️ Builds every [`DrawBatch`] across the four independent streams (backdrop/foreground ×
/// normal/overlay — each renders in its own pass with its own stencil buffer, so each gets its own
/// mask chain) plus one [`PipelineKind::Glass`] batch per glass region, then appends the precomputed
/// stencil-mask quads for every batch straight into `quad_instances` (ported from `build_batch_masks`)
/// so a backend replays `mask_range` with [`StencilPolicy::WriteMask`] and needs no mask geometry of
/// its own.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(any(test, feature = "backend-testing"))]
fn batch(layers: &[SceneLayer], glass_regions: &[GlassRegion], viewport: [f32; 2]) -> (Vec<QuadInstance>, Vec<VectorVertex>, Vec<GlassInstance>, Vec<DrawBatch>) {
    let mut quad_instances = Vec::new();
    let mut vector_vertices = Vec::new();
    let mut batches = Vec::new();

    for overlay in [false, true] {
        for want_foreground in [false, true] {
            let spans = collect_spans(layers.iter().filter(|layer| layer.foreground_of.is_some() == want_foreground), overlay, &mut quad_instances, &mut vector_vertices);
            let mut previous_bounds = None;
            for span in spans {
                let (mask_quads, current_bounds) = tessellate::mask_instances(span.scissor, span.clip.as_ref(), previous_bounds, viewport[0], viewport[1]);
                let mask_range = if mask_quads.is_empty() {
                    None
                } else {
                    let start = quad_instances.len() as u32;
                    let count = mask_quads.len() as u32;
                    quad_instances.extend(mask_quads);
                    Some((start, count))
                };
                previous_bounds = current_bounds;

                let layer_state = LayerState { scissor: span.scissor, clip: span.clip.clone(), foreground_of: span.foreground_of, overlay };
                if span.quad_range.1 > 0 {
                    batches.push(DrawBatch { pipeline: PipelineKind::UiQuad, layer_state: layer_state.clone(), instance_range: span.quad_range, mask_range, texture: None });
                }
                for (texture, start, count) in span.raster_runs {
                    batches.push(DrawBatch { pipeline: PipelineKind::UiRasterTextured, layer_state: layer_state.clone(), instance_range: (start, count), mask_range, texture: Some(texture) });
                }
                if span.vec_range.1 > 0 {
                    batches.push(DrawBatch { pipeline: PipelineKind::Vector, layer_state, instance_range: span.vec_range, mask_range, texture: None });
                }
            }
        }
    }

    let glass_instances: Vec<GlassInstance> = glass_regions.iter().copied().map(GlassInstance::from).collect();
    for index in 0..glass_instances.len() {
        batches.push(DrawBatch { pipeline: PipelineKind::Glass, layer_state: LayerState::default(), instance_range: (index as u32, 1), mask_range: None, texture: None });
    }

    (quad_instances, vector_vertices, glass_instances, batches)
}
//#endregion Batch

//#region Hash
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(any(test, feature = "backend-testing"))]
fn fnv1a64(bytes: &[u8], mut state: u64) -> u64 {
    for &byte in bytes {
        state ^= u64::from(byte);
        state = state.wrapping_mul(0x0000_0100_0000_01b3);
    }
    state
}

/// 🔢️ A 64-bit content hash over packed instance bytes plus the batch list, so a host can compare two
/// packets' `content_hash` and skip submission entirely when they match.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(any(test, feature = "backend-testing"))]
pub(super) fn content_hash(quad_instances: &[QuadInstance], vector_vertices: &[VectorVertex], glass_instances: &[GlassInstance], batches: &[DrawBatch]) -> u64 {
    let mut state = 0xcbf2_9ce4_8422_2325u64;
    state = fnv1a64(bytemuck::cast_slice(quad_instances), state);
    state = fnv1a64(bytemuck::cast_slice(vector_vertices), state);
    state = fnv1a64(bytemuck::cast_slice(glass_instances), state);
    for draw_batch in batches {
        state = fnv1a64(&(draw_batch.pipeline as u32).to_le_bytes(), state);
        state = fnv1a64(&draw_batch.instance_range.0.to_le_bytes(), state);
        state = fnv1a64(&draw_batch.instance_range.1.to_le_bytes(), state);
        if let Some((start, count)) = draw_batch.mask_range {
            state = fnv1a64(&start.to_le_bytes(), state);
            state = fnv1a64(&count.to_le_bytes(), state);
        }
    }
    state
}
//#endregion Hash
//#endregion 🔖️Finish

//#region 🔖️RenderPacket
/// 📦️ Everything a [`crate::backend`] `GraphicsBackend` needs to paint one frame, and nothing it needs
/// a device to compute. Apply `resource_ops` before replaying `batches`; skip the whole submission
/// when `content_hash` matches the previously submitted packet.
#[derive(Debug)]
pub struct RenderPacket {
    pub viewport: [f32; 2],
    pub dpr: f32,
    pub time_seconds_origin: f64,
    pub quad_instances: Vec<QuadInstance>,
    pub vector_vertices: Vec<VectorVertex>,
    pub glass_instances: Vec<GlassInstance>,
    pub batches: Vec<DrawBatch>,
    pub surface_passes: Vec<SurfacePass>,
    pub resource_ops: Vec<ResourceOp>,
    pub has_animated_primitives: bool,
    pub content_hash: u64,
}
//#endregion 🔖️RenderPacket
//#endregion 🔖️Scene
