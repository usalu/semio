//! 🧱️ Pipeline state and world passes: the pipeline set, the grow and frame buffers, the layer batches and scissor helpers, the accessors and the world pass preparation. See [`super`].

use super::*;

pub struct UiPipelines{pub(super) resources:PipelineResources}

#[derive(Default)]
pub struct GrowBuffer {
    pub(super) buffer: Option<gpu_device::Buffer>,
    capacity: usize,
}

impl GrowBuffer {
    pub fn slice(&self) -> Option<gpu_device::BufferSlice<'_>> {
        self.buffer.as_ref().map(|buffer| buffer.slice(..))
    }

    pub(super) fn upload<T: Pod>(&mut self, device: &gpu_device::Device, queue: &gpu_device::Queue, data: &[T], usage: gpu_device::BufferUsages, label: &str) -> Option<gpu_device::BufferSlice<'_>> {
        if data.is_empty() {
            return None;
        }
        let bytes = bytemuck::cast_slice(data);
        let required = bytes.len();
        if self.capacity < required {
            self.capacity = required.next_power_of_two().max(256);
            self.buffer = Some(device.create_buffer(&gpu_device::BufferDescriptor { label: Some(label), size: self.capacity as u64, usage, mapped_at_creation: false }));
        }
        let buffer = self.buffer.as_ref()?;
        queue.write_buffer(buffer, 0, bytes);
        Some(buffer.slice(..))
    }
}

#[derive(Default)]
pub struct FrameBuffers {
    pub world_instances: GrowBuffer,
    pub world_celebration_instances: GrowBuffer,
    pub world_textured_instances: GrowBuffer,
    pub world_lines: GrowBuffer,
    pub ui_instances: GrowBuffer,
    pub mask_instances: GrowBuffer,
    pub vector_vertices: GrowBuffer,
    pub glass_instances: GrowBuffer,
}

#[cfg(test)]
pub(super) struct WorldDrawRange {
    pub(super) mesh_key: String,
    pub(super) mesh_version: u64,
    pub(super) instance_offset: u32,
    pub(super) instance_count: u32,
}

#[cfg(test)]
pub(super) struct PreparedWorldPass {
    pub(super) globals: World3dGlobals,
    pub(super) viewport: [f32; 4],
    pub(super) draws: Vec<WorldDrawRange>,
    pub(super) translucent_draws: Vec<WorldDrawRange>,
    pub(super) line_start: u32,
    pub(super) line_count: u32,
}

#[cfg(test)]
pub(super) struct LayerBatch {
    pub(super) layer_index: usize,
    pub(super) scissor: Option<ScissorRect>,
    pub(super) clip: Option<ClipRegion>,
    pub(super) ui_start: u32,
    pub(super) ui_count: u32,
    pub(super) vec_start: u32,
    pub(super) vec_count: u32,
}

#[cfg(test)]
pub(super) enum LayerBatchFilter {
    Backdrop,
    Foreground,
}

#[cfg(test)]
impl Copy for LayerBatchFilter {}

#[cfg(test)]
impl Clone for LayerBatchFilter {
    fn clone(&self) -> Self {
        *self
    }
}

#[cfg(test)]
pub(super) fn layer_matches_filter(layer: &DrawLayer, filter: LayerBatchFilter) -> bool {
    match filter {
        LayerBatchFilter::Backdrop => layer.foreground_of.is_none(),
        LayerBatchFilter::Foreground => layer.foreground_of.is_some(),
    }
}

#[cfg(test)]
pub(super) fn build_layer_batches(draw: &DrawList, filter: LayerBatchFilter) -> (Vec<UiInstance>, Vec<VectorVertex>, Vec<LayerBatch>) {
    let mut all_ui = Vec::new();
    let mut all_vec = Vec::new();
    let mut batches = Vec::new();
    let scene_layers: std::collections::HashSet<usize> = draw.scene_passes.iter().filter(|pass| layer_matches_filter(&draw.layers[pass.layer_index], filter)).map(|pass| pass.layer_index).collect();
    for (layer_index, layer) in draw.layers.iter().enumerate() {
        if !layer_matches_filter(layer, filter) {
            continue;
        }
        if layer.ui_instances.is_empty() && layer.vector_vertices.is_empty() && !scene_layers.contains(&layer_index) {
            continue;
        }
        let ui_start = all_ui.len() as u32;
        all_ui.extend_from_slice(&layer.ui_instances);
        let vec_start = all_vec.len() as u32;
        all_vec.extend_from_slice(&layer.vector_vertices);
        batches.push(LayerBatch { layer_index, scissor: layer.scissor, clip: layer.clip.clone(), ui_start, ui_count: layer.ui_instances.len() as u32, vec_start, vec_count: layer.vector_vertices.len() as u32 });
    }
    (all_ui, all_vec, batches)
}

#[cfg(test)]
pub(super) fn build_overlay_layer_batches(draw: &DrawList, filter: LayerBatchFilter) -> (Vec<UiInstance>, Vec<VectorVertex>, Vec<LayerBatch>) {
    let mut all_ui = Vec::new();
    let mut all_vec = Vec::new();
    let mut batches = Vec::new();
    for (layer_index, layer) in draw.layers.iter().enumerate() {
        if !layer_matches_filter(layer, filter) {
            continue;
        }
        if layer.overlay_ui_instances.is_empty() && layer.overlay_vector_vertices.is_empty() {
            continue;
        }
        let ui_start = all_ui.len() as u32;
        all_ui.extend_from_slice(&layer.overlay_ui_instances);
        let vec_start = all_vec.len() as u32;
        all_vec.extend_from_slice(&layer.overlay_vector_vertices);
        batches.push(LayerBatch { layer_index, scissor: layer.scissor, clip: layer.clip.clone(), ui_start, ui_count: layer.overlay_ui_instances.len() as u32, vec_start, vec_count: layer.overlay_vector_vertices.len() as u32 });
    }
    (all_ui, all_vec, batches)
}

/// ✂️ One LOGICAL scissor as the PHYSICAL rectangle `wgpu` clips against. An already-empty rect
/// stays empty, so "clip everything away" never becomes a one-pixel sliver at 2×.
pub fn physical_scissor_rect(scissor: ScissorRect, scale: f32) -> ScissorRect {
    if scissor.w == 0 || scissor.h == 0 {
        return ScissorRect { x: 0, y: 0, w: 0, h: 0 };
    }
    ScissorRect { x: (scissor.x as f32 * scale) as u32, y: (scissor.y as f32 * scale) as u32, w: ((scissor.w as f32 * scale).round() as u32).max(1), h: ((scissor.h as f32 * scale).round() as u32).max(1) }
}

pub fn physical_viewport_rect(viewport: [f32; 4], scale: f32) -> [f32; 4] {
    [viewport[0] * scale, viewport[1] * scale, viewport[2] * scale, viewport[3] * scale]
}

/// 🪚️ `scissor` and `width`/`height` are LOGICAL pixels; `scale` turns them into the physical
/// rectangle `wgpu` clips against, so a clip lands on the same CSS box at any scale factor.
pub(super) fn set_pass_scissor(pass: &mut gpu_device::RenderPass<'_>, scissor: Option<ScissorRect>, scale: f32, width: f32, height: f32) {
    if let Some(scissor) = scissor {
        let scissor = physical_scissor_rect(scissor, scale);
        pass.set_scissor_rect(scissor.x, scissor.y, scissor.w, scissor.h);
    } else {
        pass.set_scissor_rect(0, 0, (width * scale).max(1.0) as u32, (height * scale).max(1.0) as u32);
    }
}

#[cfg(test)]
fn layer_scissors(scissor: Option<ScissorRect>, clip: Option<&ClipRegion>, width: f32, height: f32) -> Vec<Option<ScissorRect>> {
    if let Some(clip) = clip {
        return clip.effective_scissors(scissor, width, height).into_iter().map(Some).collect();
    }
    let viewport = ScissorRect { x: 0, y: 0, w: width.max(0.0) as u32, h: height.max(0.0) as u32 };
    match scissor.map(|value| value.intersect(&viewport)) {
        Some(value) if value.w > 0 && value.h > 0 => vec![Some(value)],
        Some(_) => Vec::new(),
        None => vec![None],
    }
}

pub(super) fn stencil_attachment<'a>(view: &'a gpu_device::TextureView, depth_load: gpu_device::LoadOp<f32>, stencil_load: gpu_device::LoadOp<u32>) -> gpu_device::RenderPassDepthStencilAttachment<'a> {
    gpu_device::RenderPassDepthStencilAttachment { view, depth_ops: Some(gpu_device::Operations { load: depth_load, store: gpu_device::StoreOp::Store }), stencil_ops: Some(gpu_device::Operations { load: stencil_load, store: gpu_device::StoreOp::Store }) }
}

#[cfg(test)]
fn union_scissors(scissors: &[ScissorRect]) -> Option<ScissorRect> {
    let first = *scissors.first()?;
    let (mut x0, mut y0, mut x1, mut y1) = (first.x, first.y, first.x + first.w, first.y + first.h);
    for scissor in &scissors[1..] {
        x0 = x0.min(scissor.x);
        y0 = y0.min(scissor.y);
        x1 = x1.max(scissor.x + scissor.w);
        y1 = y1.max(scissor.y + scissor.h);
    }
    Some(ScissorRect { x: x0, y: y0, w: x1 - x0, h: y1 - y0 })
}

#[cfg(test)]
fn merge_scissor_bounds(a: Option<ScissorRect>, b: Option<ScissorRect>) -> Option<ScissorRect> {
    match (a, b) {
        (Some(a), Some(b)) => union_scissors(&[a, b]),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    }
}

#[cfg(test)]
pub(super) fn mask_instances(scissor: Option<ScissorRect>, clip: Option<&ClipRegion>, previous_bounds: Option<ScissorRect>, width: f32, height: f32) -> (Vec<UiInstance>, Option<ScissorRect>) {
    let white = Rgba::new(1.0, 1.0, 1.0, 1.0);
    let viewport = ScissorRect { x: 0, y: 0, w: width.max(0.0) as u32, h: height.max(0.0) as u32 };
    let pieces: Vec<ScissorRect> = layer_scissors(scissor, clip, width, height).into_iter().map(|piece| piece.unwrap_or(viewport)).collect();
    let current_bounds = union_scissors(&pieces);
    let Some(reset_bounds) = merge_scissor_bounds(previous_bounds, current_bounds) else {
        return (Vec::new(), None);
    };
    let mut instances = vec![UiInstance::solid([reset_bounds.x as f32, reset_bounds.y as f32, reset_bounds.w as f32, reset_bounds.h as f32], white)];
    instances.extend(pieces.into_iter().map(|piece| UiInstance::solid([piece.x as f32, piece.y as f32, piece.w as f32, piece.h as f32], white)));
    (instances, current_bounds)
}

#[cfg(test)]
pub(super) fn build_batch_masks(batches: &[LayerBatch], width: f32, height: f32) -> (Vec<UiInstance>, Vec<(u32, u32)>) {
    let mut instances = Vec::new();
    let mut ranges = Vec::with_capacity(batches.len());
    let mut previous_bounds = None;
    for batch in batches {
        let start = instances.len() as u32;
        let (batch_instances, current_bounds) = mask_instances(batch.scissor, batch.clip.as_ref(), previous_bounds, width, height);
        let count = batch_instances.len() as u32;
        instances.extend(batch_instances);
        ranges.push((start, count));
        previous_bounds = current_bounds;
    }
    (instances, ranges)
}

impl UiPipelines {
    pub fn new(device:&gpu_device::Device,queue:&gpu_device::Queue,format:gpu_device::TextureFormat)->Self{Self{resources:PipelineResources::new(device,queue,format)}}
    /// 📐️ Publishes the surface scale factor every scissor and scene viewport is multiplied by.
    /// Ignores a non-positive or non-finite value rather than collapsing every clip rectangle.
    pub fn set_surface_scale(&mut self, surface_scale: f32) {
        if surface_scale.is_finite() && surface_scale > 0.0 {
            self.resources.surface_scale = surface_scale;
        }
    }

    /// 📏️ One logical rectangle as the physical scissor `wgpu` takes. An empty logical rect stays
    /// empty — "clip everything away" must not become a one-pixel sliver at 2×.
    pub(super) fn physical_scissor(&self, scissor: ScissorRect) -> ScissorRect {
        physical_scissor_rect(scissor, self.resources.surface_scale)
    }

    pub fn bind_group_layout(&self) -> &gpu_device::BindGroupLayout {
        &self.resources.bind_group_layout
    }

    pub fn globals_buffer(&self) -> &gpu_device::Buffer {
        &self.resources.globals_buffer
    }

    pub fn glyph_view(&self) -> gpu_device::TextureView {
        self.resources.glyph_texture.create_view(&gpu_device::TextureViewDescriptor::default())
    }

    pub fn glyph_sampler(&self) -> &gpu_device::Sampler {
        &self.resources.glyph_sampler
    }

    pub fn icon_view(&self) -> gpu_device::TextureView {
        self.resources.icon_texture.create_view(&gpu_device::TextureViewDescriptor::default())
    }

    pub fn icon_sampler(&self) -> &gpu_device::Sampler {
        &self.resources.icon_sampler
    }

    pub fn depth_format(&self) -> gpu_device::TextureFormat {
        gpu_device::TextureFormat::Depth24PlusStencil8
    }

    #[cfg(test)]
    pub(super) fn prepare_world_passes(draw: &DrawList, filter: LayerBatchFilter) -> (Vec<PreparedWorldPass>, Vec<World3dGpuInstance>, Vec<WorldLineGpuVertex>, Vec<Option<usize>>) {
        let mut prepared = Vec::new();
        let mut all_instances = Vec::new();
        let mut all_lines = Vec::new();
        let mut pass_index_map = vec![None; draw.scene_passes.len()];
        for (source_index, scene) in draw.scene_passes.iter().enumerate() {
            if !layer_matches_filter(&draw.layers[scene.layer_index], filter) {
                continue;
            }
            let mut pass_draws = Vec::new();
            for draw_call in &scene.draws {
                if draw_call.instances.is_empty() {
                    continue;
                }
                let instance_offset = all_instances.len() as u32;
                let instance_count = draw_call.instances.len() as u32;
                for instance in &draw_call.instances {
                    all_instances.push(World3dGpuInstance::from_instance(
                        instance.model.to_cols_array_m(),
                        instance.color,
                        instance.material.preserve_vertex_color,
                        instance.material.emissive_intensity,
                        instance.material.metalness,
                        instance.material.roughness,
                        draw_call.shadow_role.receives,
                    ));
                }
                pass_draws.push(WorldDrawRange { mesh_key: draw_call.mesh_key.clone(), mesh_version: draw_call.mesh_version, instance_offset, instance_count });
            }
            let mut translucent_draws = Vec::new();
            for draw_call in &scene.translucent_draws {
                if draw_call.instances.is_empty() {
                    continue;
                }
                let instance_offset = all_instances.len() as u32;
                let instance_count = draw_call.instances.len() as u32;
                for instance in &draw_call.instances {
                    all_instances.push(World3dGpuInstance::from_instance(
                        instance.model.to_cols_array_m(),
                        instance.color,
                        instance.material.preserve_vertex_color,
                        instance.material.emissive_intensity,
                        instance.material.metalness,
                        instance.material.roughness,
                        draw_call.shadow_role.receives,
                    ));
                }
                translucent_draws.push(WorldDrawRange { mesh_key: draw_call.mesh_key.clone(), mesh_version: draw_call.mesh_version, instance_offset, instance_count });
            }
            let line_start = all_lines.len() as u32;
            for line_draw in &scene.line_draws {
                for vertex in &line_draw.vertices {
                    all_lines.push(WorldLineGpuVertex { position: vertex.position, color: vertex.color });
                }
            }
            let line_count = all_lines.len() as u32 - line_start;
            pass_index_map[source_index] = Some(prepared.len());
            prepared.push(PreparedWorldPass { globals: World3dGlobals::from_pass(scene), viewport: scene.viewport, draws: pass_draws, translucent_draws, line_start, line_count });
        }
        (prepared, all_instances, all_lines, pass_index_map)
    }

}
