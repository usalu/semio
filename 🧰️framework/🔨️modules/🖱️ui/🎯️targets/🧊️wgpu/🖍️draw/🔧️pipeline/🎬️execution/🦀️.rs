//! 🎬️ Actual draw encoding and progressive resource uploads consume prepared pipeline resources.
use semio_framework_ui_wgpu_device as gpu_device;
use semio_framework_ui_draw_contract::*;
use bytemuck::{Pod,Zeroable};
use std::mem::size_of;
use gpu_device::util::DeviceExt;
mod kernel_3d_scene{pub use semio_framework_ui_scene_algebra::*;pub use semio_framework_ui_scene_camera::*;pub use semio_framework_ui_scene_mesh::*;pub use semio_framework_ui_scene_pass::*;}
use kernel_3d_scene::{Mat4Math,ProceduralGrid3d,ScenePass3d,SceneRenderProfile3d,PROCEDURAL_GRID_CELL_THICKNESS,PROCEDURAL_GRID_FADE_STRENGTH};
use semio_framework_ui_prepared as prepared;
use semio_framework_ui_raster_owner as raster_ownership;
use semio_framework_ui_theme as theme;
use semio_framework_ui_theme::{geometry,Theme};
#[cfg(test)]
use semio_framework_ui_theme::Rgba;
pub use semio_framework_ui_wgpu_mesh::*;
pub use semio_framework_ui_wgpu_raster::*;
pub use semio_framework_ui_wgpu_pipelines::*;
use prepared::{PreparedRasterPages,RasterContentIdentity};
pub struct UiPipelines{resources:PipelineResources}
#[derive(Default)]
pub struct GrowBuffer {
    buffer: Option<gpu_device::Buffer>,
    capacity: usize,
}

impl GrowBuffer {
    pub fn slice(&self) -> Option<gpu_device::BufferSlice<'_>> {
        self.buffer.as_ref().map(|buffer| buffer.slice(..))
    }

    fn upload<T: Pod>(&mut self, device: &gpu_device::Device, queue: &gpu_device::Queue, data: &[T], usage: gpu_device::BufferUsages, label: &str) -> Option<gpu_device::BufferSlice<'_>> {
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
struct WorldDrawRange {
    mesh_key: String,
    mesh_version: u64,
    instance_offset: u32,
    instance_count: u32,
}

#[cfg(test)]
struct PreparedWorldPass {
    globals: World3dGlobals,
    viewport: [f32; 4],
    draws: Vec<WorldDrawRange>,
    translucent_draws: Vec<WorldDrawRange>,
    line_start: u32,
    line_count: u32,
}

#[cfg(test)]
struct LayerBatch {
    layer_index: usize,
    scissor: Option<ScissorRect>,
    clip: Option<ClipRegion>,
    ui_start: u32,
    ui_count: u32,
    vec_start: u32,
    vec_count: u32,
}

#[cfg(test)]
enum LayerBatchFilter {
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
fn layer_matches_filter(layer: &DrawLayer, filter: LayerBatchFilter) -> bool {
    match filter {
        LayerBatchFilter::Backdrop => layer.foreground_of.is_none(),
        LayerBatchFilter::Foreground => layer.foreground_of.is_some(),
    }
}

#[cfg(test)]
fn build_layer_batches(draw: &DrawList, filter: LayerBatchFilter) -> (Vec<UiInstance>, Vec<VectorVertex>, Vec<LayerBatch>) {
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
fn build_overlay_layer_batches(draw: &DrawList, filter: LayerBatchFilter) -> (Vec<UiInstance>, Vec<VectorVertex>, Vec<LayerBatch>) {
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
pub(crate) fn physical_scissor_rect(scissor: ScissorRect, scale: f32) -> ScissorRect {
    if scissor.w == 0 || scissor.h == 0 {
        return ScissorRect { x: 0, y: 0, w: 0, h: 0 };
    }
    ScissorRect { x: (scissor.x as f32 * scale) as u32, y: (scissor.y as f32 * scale) as u32, w: ((scissor.w as f32 * scale).round() as u32).max(1), h: ((scissor.h as f32 * scale).round() as u32).max(1) }
}

pub(crate) fn physical_viewport_rect(viewport: [f32; 4], scale: f32) -> [f32; 4] {
    [viewport[0] * scale, viewport[1] * scale, viewport[2] * scale, viewport[3] * scale]
}

/// ✂️ `scissor` and `width`/`height` are LOGICAL pixels; `scale` turns them into the physical
/// rectangle `wgpu` clips against, so a clip lands on the same CSS box at any scale factor.
fn set_pass_scissor(pass: &mut gpu_device::RenderPass<'_>, scissor: Option<ScissorRect>, scale: f32, width: f32, height: f32) {
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





fn stencil_attachment<'a>(view: &'a gpu_device::TextureView, depth_load: gpu_device::LoadOp<f32>, stencil_load: gpu_device::LoadOp<u32>) -> gpu_device::RenderPassDepthStencilAttachment<'a> {
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
fn mask_instances(scissor: Option<ScissorRect>, clip: Option<&ClipRegion>, previous_bounds: Option<ScissorRect>, width: f32, height: f32) -> (Vec<UiInstance>, Option<ScissorRect>) {
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
fn build_batch_masks(batches: &[LayerBatch], width: f32, height: f32) -> (Vec<UiInstance>, Vec<(u32, u32)>) {
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

    /// 📐️ One logical rectangle as the physical scissor `wgpu` takes. An empty logical rect stays
    /// empty — "clip everything away" must not become a one-pixel sliver at 2×.
    fn physical_scissor(&self, scissor: ScissorRect) -> ScissorRect {
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
    fn prepare_world_passes(draw: &DrawList, filter: LayerBatchFilter) -> (Vec<PreparedWorldPass>, Vec<World3dGpuInstance>, Vec<WorldLineGpuVertex>, Vec<Option<usize>>) {
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

    #[cfg(test)]
    fn upload_world_passes(&mut self, device: &gpu_device::Device, queue: &gpu_device::Queue, draw: &DrawList, frame_buffers: &mut FrameBuffers, filter: LayerBatchFilter) -> Option<(Vec<PreparedWorldPass>, Vec<Option<usize>>)> {
        if draw.scene_passes.is_empty() {
            return None;
        }
        let (prepared, all_instances, all_lines, pass_index_map) = Self::prepare_world_passes(draw, filter);
        if prepared.is_empty() {
            return None;
        }
        if all_instances.is_empty() && all_lines.is_empty() {
            return Some((prepared, pass_index_map));
        }
        self.resources.world_globals_ring.ensure_slots(device, &self.resources.world_bind_group_layout, prepared.len() as u32);
        let globals: Vec<World3dGlobals> = prepared.iter().map(|pass| pass.globals).collect();
        self.resources.world_globals_ring.write_passes(queue, &globals);
        if !all_instances.is_empty() {
            frame_buffers.world_instances.upload(device, queue, &all_instances, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "world3d_instances");
        }
        if !all_lines.is_empty() {
            frame_buffers.world_lines.upload(device, queue, &all_lines, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "world3d_lines");
        }
        Some((prepared, pass_index_map))
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    #[cfg(test)]
    fn draw_world_pass_at<'a>(
        &'a self,
        pass: &mut gpu_device::RenderPass<'a>,
        mesh_store: &MeshGpuTable,
        prepared: &PreparedWorldPass,
        slot: u32,
        instance_buffer: gpu_device::BufferSlice<'a>,
        line_buffer: Option<gpu_device::BufferSlice<'a>>,
        screen_w: f32,
        screen_h: f32,
        clip: Option<ScissorRect>,
    ) {
        let instance_stride = size_of::<World3dGpuInstance>() as u64;
        pass.set_pipeline(&self.resources.world_pipeline);
        let scale = self.resources.surface_scale;
        let viewport = prepared.viewport;
        pass.set_viewport(viewport[0] * scale, viewport[1] * scale, viewport[2] * scale, viewport[3] * scale, 0.0, 1.0);
        let scene_scissor = ScissorRect { x: viewport[0] as u32, y: viewport[1] as u32, w: viewport[2] as u32, h: viewport[3] as u32 };
        let scene_scissor = clip.map_or(scene_scissor, |clip| scene_scissor.intersect(&clip));
        if scene_scissor.w == 0 || scene_scissor.h == 0 {
            pass.set_viewport(0.0, 0.0, screen_w * scale, screen_h * scale, 0.0, 1.0);
            set_pass_scissor(pass, clip, scale, screen_w, screen_h);
            pass.set_pipeline(&self.resources.ui_pipeline);
            pass.set_bind_group(0, &self.resources.glyph_bind_group, &[]);
            return;
        }
        let scene_scissor = self.physical_scissor(scene_scissor);
        pass.set_scissor_rect(scene_scissor.x, scene_scissor.y, scene_scissor.w, scene_scissor.h);
        pass.set_bind_group(0, &self.resources.world_globals_ring.bind_group, &[self.resources.world_globals_ring.offset_for_slot(slot)]);
        pass.set_bind_group(1, &self.resources.world_shadow_target.bind_group, &[]);
        for draw_call in &prepared.draws {
            Self::draw_world_range(pass, mesh_store, draw_call, instance_buffer, instance_stride);
        }
        if prepared.line_count > 0 {
            if let Some(line_buffer) = line_buffer {
                pass.set_pipeline(&self.resources.world_line_pipeline);
                pass.set_bind_group(0, &self.resources.world_globals_ring.bind_group, &[self.resources.world_globals_ring.offset_for_slot(slot)]);
                let line_stride = size_of::<WorldLineGpuVertex>() as u64;
                let byte_offset = prepared.line_start as u64 * line_stride;
                pass.set_vertex_buffer(0, line_buffer.slice(byte_offset..byte_offset + prepared.line_count as u64 * line_stride));
                pass.draw(0..prepared.line_count, 0..1);
            }
        }
        if !prepared.translucent_draws.is_empty() {
            pass.set_pipeline(&self.resources.world_pipeline_translucent);
            pass.set_bind_group(0, &self.resources.world_globals_ring.bind_group, &[self.resources.world_globals_ring.offset_for_slot(slot)]);
            for draw_call in &prepared.translucent_draws {
                Self::draw_world_range(pass, mesh_store, draw_call, instance_buffer, instance_stride);
            }
        }
        pass.set_viewport(0.0, 0.0, screen_w * scale, screen_h * scale, 0.0, 1.0);
        set_pass_scissor(pass, clip, scale, screen_w, screen_h);
        pass.set_pipeline(&self.resources.ui_pipeline);
        pass.set_bind_group(0, &self.resources.glyph_bind_group, &[]);
    }

    #[cfg(test)]
    fn draw_world_range<'a>(pass: &mut gpu_device::RenderPass<'a>, mesh_store: &MeshGpuTable, draw_call: &WorldDrawRange, instance_buffer: gpu_device::BufferSlice<'a>, instance_stride: u64) {
        let Some(mesh) = mesh_store.get_versioned(&draw_call.mesh_key, draw_call.mesh_version) else {
            return;
        };
        if mesh.index_count == 0 || draw_call.instance_count == 0 { return; }
        let byte_offset = draw_call.instance_offset as u64 * instance_stride;
        pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.slice(byte_offset..byte_offset + draw_call.instance_count as u64 * instance_stride));
        pass.set_index_buffer(mesh.index_buffer.slice(..), gpu_device::IndexFormat::Uint32);
        pass.draw_indexed(0..mesh.index_count, 0, 0..draw_call.instance_count);
    }

    #[cfg(test)]
    fn draw_ui_instances<'a>(&'a self, pass: &mut gpu_device::RenderPass<'a>, instance_buffer: &gpu_device::BufferSlice<'a>, start: u32, count: u32) {
        if count == 0 {
            return;
        }
        pass.set_pipeline(&self.resources.ui_pipeline);
        pass.set_bind_group(0, &self.resources.glyph_bind_group, &[]);
        pass.set_vertex_buffer(0, self.resources.quad_vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, *instance_buffer);
        pass.draw(0..6, start..start + count);
    }

    #[cfg(test)]
    fn draw_silhouette_mask<'a>(&'a self, pass: &mut gpu_device::RenderPass<'a>, mask_buffer: &gpu_device::BufferSlice<'a>, start: u32, count: u32, width: f32, height: f32) {
        if count == 0 {
            pass.set_stencil_reference(1);
            return;
        }
        set_pass_scissor(pass, None, self.resources.surface_scale, width, height);
        pass.set_pipeline(&self.resources.mask_pipeline);
        pass.set_bind_group(0, &self.resources.glyph_bind_group, &[]);
        pass.set_vertex_buffer(0, self.resources.quad_vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, *mask_buffer);
        pass.set_stencil_reference(0);
        pass.draw(0..6, start..start + 1);
        if count > 1 {
            pass.set_stencil_reference(1);
            pass.draw(0..6, start + 1..start + count);
        }
        pass.set_stencil_reference(1);
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    #[cfg(test)]
    fn draw_raster_layers<'a>(
        &'a self,
        pass: &mut gpu_device::RenderPass<'a>,
        raster_store: &RasterTextureTable,
        draw: &DrawList,
        frame_buffers: &'a mut FrameBuffers,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        width: f32,
        height: f32,
        filter: LayerBatchFilter,
        overlay: bool,
    ) {
        let raster_layers: Vec<&DrawLayer> = draw.layers.iter().filter(|layer| layer_matches_filter(layer, filter) && if overlay { !layer.overlay_raster_instances.is_empty() } else { !layer.raster_instances.is_empty() }).collect();
        let mut mask_data = Vec::new();
        let mut mask_ranges = Vec::with_capacity(raster_layers.len());
        let mut previous_bounds = None;
        for layer in &raster_layers {
            let start = mask_data.len() as u32;
            let (instances, current_bounds) = mask_instances(layer.scissor, layer.clip.as_ref(), previous_bounds, width, height);
            let count = instances.len() as u32;
            mask_data.extend(instances);
            mask_ranges.push((start, count));
            previous_bounds = current_bounds;
        }
        let Some(mask_buffer) = frame_buffers.mask_instances.upload(device, queue, &mask_data, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "raster_silhouette_masks") else {
            return;
        };
        for (layer_index, layer) in raster_layers.into_iter().enumerate() {
            let (mask_start, mask_count) = mask_ranges[layer_index];
            self.draw_silhouette_mask(pass, &mask_buffer, mask_start, mask_count, width, height);
            let mut batch_key: Option<String> = None;
            let mut batch_instances: Vec<UiInstance> = Vec::new();
            let mut flush = |key: &str, instances: &[UiInstance]| {
                if instances.is_empty() {
                    return;
                }
                let Some(rt) = raster_store.get(key) else {
                    return;
                };
                pass.set_pipeline(&self.resources.ui_pipeline);
                pass.set_bind_group(0, &rt.bind_group, &[]);
                let Some(buffer) = frame_buffers.ui_instances.upload(device, queue, instances, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "raster_instances") else {
                    return;
                };
                pass.set_vertex_buffer(0, self.resources.quad_vertex_buffer.slice(..));
                pass.set_vertex_buffer(1, buffer);
                pass.draw(0..6, 0..instances.len() as u32);
            };
            let instances = if overlay { &layer.overlay_raster_instances } else { &layer.raster_instances };
            for (key, instance) in instances {
                if batch_key.as_deref() != Some(key.as_str()) {
                    if let Some(ref prior) = batch_key {
                        flush(prior, &batch_instances);
                    }
                    batch_key = Some(key.clone());
                    batch_instances.clear();
                }
                batch_instances.push(*instance);
            }
            if let Some(ref key) = batch_key {
                flush(key, &batch_instances);
            }
        }
        pass.set_scissor_rect(0, 0, (width * self.resources.surface_scale).max(1.0) as u32, (height * self.resources.surface_scale).max(1.0) as u32);
    }

    #[cfg(test)]
    fn draw_vector_vertices<'a>(&'a self, pass: &mut gpu_device::RenderPass<'a>, vector_buffer: &gpu_device::BufferSlice<'a>, start: u32, count: u32) {
        if count == 0 {
            return;
        }
        pass.set_pipeline(&self.resources.vector_pipeline);
        pass.set_bind_group(0, &self.resources.glyph_bind_group, &[]);
        pass.set_vertex_buffer(0, *vector_buffer);
        pass.draw(start..start + count, 0..1);
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    #[cfg(test)]
    fn render_interleaved_layers<'a>(
        &'a self,
        pass: &mut gpu_device::RenderPass<'a>,
        draw: &DrawList,
        batches: &[LayerBatch],
        mask_buffer: Option<&gpu_device::BufferSlice<'a>>,
        mask_ranges: &[(u32, u32)],
        ui_buffer: Option<&gpu_device::BufferSlice<'a>>,
        vector_buffer: Option<&gpu_device::BufferSlice<'a>>,
        world_prepared: Option<&[PreparedWorldPass]>,
        pass_index_map: &[Option<usize>],
        instance_buffer: Option<gpu_device::BufferSlice<'a>>,
        line_buffer: Option<gpu_device::BufferSlice<'a>>,
        mesh_store: &MeshGpuTable,
        width: f32,
        height: f32,
        depth_enabled: bool,
    ) {
        for (batch_index, batch) in batches.iter().enumerate() {
            if let Some((start, count)) = mask_ranges.get(batch_index).copied() {
                if let Some(mask_buffer) = mask_buffer {
                    self.draw_silhouette_mask(pass, mask_buffer, start, count, width, height);
                } else {
                    pass.set_stencil_reference(1);
                }
            }
            let mut layer_passes: Vec<(usize, usize, usize)> = draw.scene_passes.iter().enumerate().filter(|(_, scene)| scene.layer_index == batch.layer_index).map(|(index, scene)| (index, scene.ui_watermark, scene.vector_watermark)).collect();
            layer_passes.sort_by_key(|(_, ui, vec)| (*ui, *vec));
            if layer_passes.is_empty() {
                if let Some(instance_buffer) = ui_buffer {
                    self.draw_ui_instances(pass, instance_buffer, batch.ui_start, batch.ui_count);
                }
                if let Some(vector_buffer) = vector_buffer {
                    self.draw_vector_vertices(pass, vector_buffer, batch.vec_start, batch.vec_count);
                }
                continue;
            }
            let mut ui_local = 0u32;
            let mut vec_local = 0u32;
            for (pass_index, ui_mark, vec_mark) in layer_passes {
                let ui_mark = ui_mark as u32;
                let vec_mark = vec_mark as u32;
                if ui_mark > ui_local {
                    if let Some(instance_buffer) = ui_buffer {
                        self.draw_ui_instances(pass, instance_buffer, batch.ui_start + ui_local, ui_mark - ui_local);
                    }
                    ui_local = ui_mark;
                }
                if vec_mark > vec_local {
                    if let Some(vector_buffer) = vector_buffer {
                        self.draw_vector_vertices(pass, vector_buffer, batch.vec_start + vec_local, vec_mark - vec_local);
                    }
                    vec_local = vec_mark;
                }
                if depth_enabled {
                    if let (Some(prepared), Some(instance_buffer)) = (world_prepared, instance_buffer.as_ref()) {
                        if let Some(prepared_slot) = pass_index_map.get(pass_index).and_then(|slot| *slot) {
                            if let Some(scene) = prepared.get(prepared_slot) {
                                self.draw_world_pass_at(pass, mesh_store, scene, prepared_slot as u32, *instance_buffer, line_buffer, width, height, None);
                            }
                        }
                    }
                }
            }
            if ui_local < batch.ui_count {
                if let Some(instance_buffer) = ui_buffer {
                    self.draw_ui_instances(pass, instance_buffer, batch.ui_start + ui_local, batch.ui_count - ui_local);
                }
            }
            if vec_local < batch.vec_count {
                if let Some(vector_buffer) = vector_buffer {
                    self.draw_vector_vertices(pass, vector_buffer, batch.vec_start + vec_local, batch.vec_count - vec_local);
                }
            }
        }
        pass.set_scissor_rect(0, 0, (width * self.resources.surface_scale).max(1.0) as u32, (height * self.resources.surface_scale).max(1.0) as u32);
    }

    pub fn update_globals(&self, queue: &gpu_device::Queue, width: f32, height: f32, time_seconds: f32) {
        queue.write_buffer(&self.resources.globals_buffer, 0, bytemuck::bytes_of(&UiGlobals { screen_size: [width, height], _pad: [time_seconds, 0.0] }));
    }

    pub fn upload_glyph_atlas(&self, queue: &gpu_device::Queue, pixels: &[u8], width: u32, height: u32) {
        queue.write_texture(
            gpu_device::TexelCopyTextureInfo { texture: &self.resources.glyph_texture, mip_level: 0, origin: gpu_device::Origin3d::ZERO, aspect: gpu_device::TextureAspect::All },
            pixels,
            gpu_device::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width), rows_per_image: Some(height) },
            gpu_device::Extent3d { width, height, depth_or_array_layers: 1 },
        );
    }

    pub fn upload_glyph_atlas_page(&self, queue: &gpu_device::Queue, pixels: &[u8], width: u32, start_row: u32, rows: u32) {
        queue.write_texture(
            gpu_device::TexelCopyTextureInfo { texture: &self.resources.glyph_texture, mip_level: 0, origin: gpu_device::Origin3d { x: 0, y: start_row, z: 0 }, aspect: gpu_device::TextureAspect::All },
            pixels,
            gpu_device::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width), rows_per_image: Some(rows) },
            gpu_device::Extent3d { width, height: rows, depth_or_array_layers: 1 },
        );
    }

    pub fn upload_icon_atlas(&self, queue: &gpu_device::Queue, pixels: &[u8], width: u32, height: u32) {
        queue.write_texture(
            gpu_device::TexelCopyTextureInfo { texture: &self.resources.icon_texture, mip_level: 0, origin: gpu_device::Origin3d::ZERO, aspect: gpu_device::TextureAspect::All },
            pixels,
            gpu_device::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width * 4), rows_per_image: Some(height) },
            gpu_device::Extent3d { width, height, depth_or_array_layers: 1 },
        );
    }

    pub fn upload_icon_atlas_page(&self, queue: &gpu_device::Queue, pixels: &[u8], width: u32, start_row: u32, rows: u32) {
        queue.write_texture(
            gpu_device::TexelCopyTextureInfo { texture: &self.resources.icon_texture, mip_level: 0, origin: gpu_device::Origin3d { x: 0, y: start_row, z: 0 }, aspect: gpu_device::TextureAspect::All },
            pixels,
            gpu_device::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width * 4), rows_per_image: Some(rows) },
            gpu_device::Extent3d { width, height: rows, depth_or_array_layers: 1 },
        );
    }

    /// 🌑 Encodes the fixed scene clear packet without traversing retained draw owners.
    pub fn clear_prepared_scene<'a>(&'a self, encoder: &mut gpu_device::CommandEncoder, scene: &'a SceneColorTarget, depth_view: Option<&'a gpu_device::TextureView>, clear: gpu_device::Color) {
        let pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("prepared_scene_clear"),
            color_attachments: &[Some(gpu_device::RenderPassColorAttachment {
                view: scene.mip_view(0),
                resolve_target: None,
                ops: gpu_device::Operations { load: gpu_device::LoadOp::Clear(clear), store: gpu_device::StoreOp::Store },
                depth_slice: None,
            })],
            depth_stencil_attachment: depth_view.map(|depth| stencil_attachment(depth, gpu_device::LoadOp::Clear(1.0), gpu_device::LoadOp::Clear(0))),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        drop(pass);
    }

    /// 🔹 Encodes one retained UI instance without collecting or traversing sibling layers.
    #[allow(clippy::too_many_arguments, reason = "one fixed owner per GPU boundary")]
    pub fn encode_prepared_ui_scalar<'a>(
        &'a self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        encoder: &mut gpu_device::CommandEncoder,
        color_view: &'a gpu_device::TextureView,
        depth_view: &'a gpu_device::TextureView,
        frame_buffers: &'a mut FrameBuffers,
        raster_store: &'a RasterTextureTable,
        instance: &UiInstance,
        raster_key: Option<&str>,
        scissor: Option<ScissorRect>,
        width: f32,
        height: f32,
        time_seconds: f32,
    ) -> Result<(), &'static str> {
        self.update_globals(queue, width, height, time_seconds);
        let Some(buffer) = frame_buffers.ui_instances.upload(device, queue, std::slice::from_ref(instance), gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "prepared_ui_scalar") else {
            return Err("prepared UI scalar buffer admission failed");
        };
        let bind_group = match raster_key {
            Some(key) => &raster_store.get(key).ok_or("prepared raster scalar texture was missing")?.bind_group,
            None => &self.resources.glyph_bind_group,
        };
        let mut pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("prepared_ui_scalar"),
            color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view: color_view, resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: Some(stencil_attachment(depth_view, gpu_device::LoadOp::Load, gpu_device::LoadOp::Load)),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        set_pass_scissor(&mut pass, scissor, self.resources.surface_scale, width, height);
        pass.set_pipeline(&self.resources.ui_pipeline);
        pass.set_bind_group(0, bind_group, &[]);
        pass.set_vertex_buffer(0, self.resources.quad_vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, buffer);
        pass.draw(0..6, 0..1);
        Ok(())
    }

    /// 🔺 Encodes one retained vector triangle without staging a dynamic batch.
    #[allow(clippy::too_many_arguments, reason = "one fixed owner per GPU boundary")]
    pub fn encode_prepared_vector_triangle<'a>(
        &'a self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        encoder: &mut gpu_device::CommandEncoder,
        color_view: &'a gpu_device::TextureView,
        depth_view: &'a gpu_device::TextureView,
        frame_buffers: &'a mut FrameBuffers,
        vertices: &[VectorVertex],
        scissor: Option<ScissorRect>,
        width: f32,
        height: f32,
        time_seconds: f32,
    ) -> Result<(), &'static str> {
        if vertices.len() != 3 {
            return Err("prepared vector scalar was not one triangle");
        }
        self.update_globals(queue, width, height, time_seconds);
        let Some(buffer) = frame_buffers.vector_vertices.upload(device, queue, vertices, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "prepared_vector_triangle") else {
            return Err("prepared vector triangle buffer admission failed");
        };
        let mut pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("prepared_vector_triangle"),
            color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view: color_view, resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: Some(stencil_attachment(depth_view, gpu_device::LoadOp::Load, gpu_device::LoadOp::Load)),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        set_pass_scissor(&mut pass, scissor, self.resources.surface_scale, width, height);
        pass.set_pipeline(&self.resources.vector_pipeline);
        pass.set_bind_group(0, &self.resources.glyph_bind_group, &[]);
        pass.set_vertex_buffer(0, buffer);
        pass.draw(0..3, 0..1);
        Ok(())
    }

    /// 🌑️ Clears the lazily enlarged directional depth map before this scene's bounded caster
    /// scalars. Disabled shadows leave the one-pixel fallback resident and issue no GPU work.
    pub fn encode_prepared_world_shadow_begin(&mut self, device: &gpu_device::Device, encoder: &mut gpu_device::CommandEncoder, pass_owner: &ScenePass3d) {
        if !pass_owner.shadow.enabled {
            return;
        }
        if self.resources.world_shadow_target.size != pass_owner.shadow.map_size {
            self.resources.world_shadow_target = WorldShadowTarget::new(device, &self.resources.world_shadow_bind_group_layout, pass_owner.shadow.map_size);
        }
        let _pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("prepared_world_shadow_clear"),
            color_attachments: &[],
            depth_stencil_attachment: Some(gpu_device::RenderPassDepthStencilAttachment { view: &self.resources.world_shadow_target.view, depth_ops: Some(gpu_device::Operations { load: gpu_device::LoadOp::Clear(1.0), store: gpu_device::StoreOp::Store }), stencil_ops: None }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    }

    /// 🌑️ Encodes one opaque caster into the scene's shadow map. One instance per call keeps
    /// cancellation and replacement at the same scalar boundary as the color pass.
    #[allow(clippy::too_many_arguments, reason = "one fixed owner per GPU boundary")]
    pub fn encode_prepared_world_shadow_instance<'a>(
        &'a mut self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        encoder: &mut gpu_device::CommandEncoder,
        frame_buffers: &'a mut FrameBuffers,
        mesh_store: &'a MeshGpuTable,
        pass_owner: &ScenePass3d,
        mesh_key: &str,
        mesh_version: u64,
        instance: &crate::kernel_3d_scene::Instance3d,
    ) -> Result<bool, &'static str> {
        if !pass_owner.shadow.enabled {
            return Ok(false);
        }
        if self.resources.world_shadow_target.size != pass_owner.shadow.map_size {
            return Err("prepared world shadow map was not begun");
        }
        let Some(mesh) = mesh_store.get_versioned(mesh_key, mesh_version) else { return Ok(false) };
        if mesh.index_count == 0 { return Ok(false); }
        let globals = World3dGlobals::from_pass(pass_owner);
        self.resources.world_globals_ring.ensure_slots(device, &self.resources.world_bind_group_layout, 1);
        self.resources.world_globals_ring.write_passes(queue, std::slice::from_ref(&globals));
        let gpu_instance =
            World3dGpuInstance::from_instance(instance.model.to_cols_array_m(), instance.color, instance.material.preserve_vertex_color, instance.material.emissive_intensity, instance.material.metalness, instance.material.roughness, false);
        let Some(instance_buffer) = frame_buffers.world_instances.upload(device, queue, std::slice::from_ref(&gpu_instance), gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "prepared_world_shadow_instance") else {
            return Err("prepared world shadow instance buffer admission failed");
        };
        let mut pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("prepared_world_shadow_instance"),
            color_attachments: &[],
            depth_stencil_attachment: Some(gpu_device::RenderPassDepthStencilAttachment { view: &self.resources.world_shadow_target.view, depth_ops: Some(gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }), stencil_ops: None }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&self.resources.world_shadow_pipeline);
        pass.set_bind_group(0, &self.resources.world_globals_ring.bind_group, &[self.resources.world_globals_ring.offset_for_slot(0)]);
        pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer);
        pass.set_index_buffer(mesh.index_buffer.slice(..), gpu_device::IndexFormat::Uint32);
        pass.draw_indexed(0..mesh.index_count, 0, 0..1);
        Ok(true)
    }

    /// 🌐 Encodes one retained mesh instance under one scene-pass owner, answering whether it drew.
    ///
    /// 🕰️ A mesh key/version that is not resident draws NOTHING and is not a fault, exactly as
    /// [`Self::encode_prepared_world_textured`] answers a raster whose pixels have not landed. A miss
    /// used to be a FATAL `present_step` fault that quarantined the whole surface for the rest of the
    /// session — one unbacked ghost draw killed the puzzle3d journey mid-flight (ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w11a-prepared-world-mesh-missing.md`). React simply
    /// renders nothing until its loader resolves, so the skip is the parity behaviour; the source-side
    /// residency law lives in `♾️infinite/🌍️world`'s `retain_ensured_world_draws`, and the skip is
    /// reported once per transition by the host so a regression there stays readable.
    #[allow(clippy::too_many_arguments, reason = "one fixed owner per GPU boundary")]
    pub fn encode_prepared_world_instance<'a>(
        &'a mut self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        encoder: &mut gpu_device::CommandEncoder,
        color_view: &'a gpu_device::TextureView,
        depth_view: &'a gpu_device::TextureView,
        frame_buffers: &'a mut FrameBuffers,
        mesh_store: &'a MeshGpuTable,
        pass_owner: &ScenePass3d,
        mesh_key: &str,
        mesh_version: u64,
        instance: &crate::kernel_3d_scene::Instance3d,
        receives_shadow: bool,
        translucent: bool,
        scissor: ScissorRect,
        width: f32,
        height: f32,
    ) -> Result<bool, &'static str> {
        let Some(mesh) = mesh_store.get_versioned(mesh_key, mesh_version) else { return Ok(false) };
        if mesh.index_count == 0 { return Ok(true); }
        let globals = World3dGlobals::from_pass(pass_owner);
        self.resources.world_globals_ring.ensure_slots(device, &self.resources.world_bind_group_layout, 1);
        self.resources.world_globals_ring.write_passes(queue, std::slice::from_ref(&globals));
        let gpu_instance =
            World3dGpuInstance::from_instance(instance.model.to_cols_array_m(), instance.color, instance.material.preserve_vertex_color, instance.material.emissive_intensity, instance.material.metalness, instance.material.roughness, receives_shadow).with_section(instance.material.section);
        let Some(instance_buffer) = frame_buffers.world_instances.upload(device, queue, std::slice::from_ref(&gpu_instance), gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "prepared_world_instance") else {
            return Err("prepared world instance buffer admission failed");
        };
        let scale = self.resources.surface_scale;
        let viewport = pass_owner.viewport;
        let scene_scissor = self.physical_scissor(scissor);
        if scene_scissor.w == 0 || scene_scissor.h == 0 {
            return Ok(true);
        }
        let mut pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("prepared_world_instance"),
            color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view: color_view, resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: Some(stencil_attachment(depth_view, gpu_device::LoadOp::Load, gpu_device::LoadOp::Load)),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_viewport(viewport[0] * scale, viewport[1] * scale, viewport[2] * scale, viewport[3] * scale, 0.0, 1.0);
        pass.set_scissor_rect(scene_scissor.x, scene_scissor.y, scene_scissor.w, scene_scissor.h);
        match instance.material.section {
            crate::kernel_3d_scene::SceneSectionRole3d::StencilToggle => pass.set_pipeline(&self.resources.world_section_stencil_pipeline),
            crate::kernel_3d_scene::SceneSectionRole3d::Cap => {
                pass.set_pipeline(&self.resources.world_section_cap_pipeline);
                pass.set_stencil_reference(SECTION_STENCIL_BIT);
            }
            _ => pass.set_pipeline(if translucent { &self.resources.world_pipeline_translucent } else { &self.resources.world_pipeline }),
        }
        pass.set_bind_group(0, &self.resources.world_globals_ring.bind_group, &[self.resources.world_globals_ring.offset_for_slot(0)]);
        pass.set_bind_group(1, &self.resources.world_shadow_target.bind_group, &[]);
        pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer);
        pass.set_index_buffer(mesh.index_buffer.slice(..), gpu_device::IndexFormat::Uint32);
        pass.draw_indexed(0..mesh.index_count, 0, 0..1);
        pass.set_viewport(0.0, 0.0, width * scale, height * scale, 0.0, 1.0);
        Ok(true)
    }

    #[allow(clippy::too_many_arguments, reason = "one fixed owner per GPU boundary")]
    pub fn encode_prepared_world_material<'a>(
        &'a mut self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        encoder: &mut gpu_device::CommandEncoder,
        color_view: &'a gpu_device::TextureView,
        depth_view: &'a gpu_device::TextureView,
        frame_buffers: &'a mut FrameBuffers,
        mesh_store: &'a MeshGpuTable,
        raster_store: &'a RasterTextureTable,
        pass_owner: &ScenePass3d,
        draw_owner: &crate::kernel_3d_scene::SceneMaterialDraw3d,
        instance: &crate::kernel_3d_scene::Instance3d,
        scissor: ScissorRect,
        width: f32,
        height: f32,
    ) -> Result<bool, &'static str> {
        let Some(mesh) = mesh_store.get_versioned(&draw_owner.mesh_key, draw_owner.mesh_version) else { return Ok(false) };
        if mesh.index_count == 0 { return Ok(true); }
        let globals = World3dGlobals::from_pass(pass_owner);
        self.resources.world_globals_ring.ensure_slots(device, &self.resources.world_bind_group_layout, 1);
        self.resources.world_globals_ring.write_passes(queue, std::slice::from_ref(&globals));
        let painted_bind_group;
        let instance_buffer = match &draw_owner.material {
            crate::kernel_3d_scene::SceneMaterialKind3d::Standard => {
                painted_bind_group = None;
                let gpu_instance = World3dGpuInstance::from_instance(
                    instance.model.to_cols_array_m(),
                    instance.color,
                    instance.material.preserve_vertex_color,
                    instance.material.emissive_intensity,
                    instance.material.metalness,
                    instance.material.roughness,
                    pass_owner.shadow.enabled,
                )
                .with_section(instance.material.section);
                frame_buffers
                    .world_instances
                    .upload(device, queue, std::slice::from_ref(&gpu_instance), gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "prepared_world_standard_translucent")
                    .ok_or("prepared world standard translucent buffer admission failed")?
            }
            crate::kernel_3d_scene::SceneMaterialKind3d::Painted { texture_key } => {
                let Some(raster) = raster_store.get(texture_key) else { return Ok(true) };
                painted_bind_group = Some(device.create_bind_group(&gpu_device::BindGroupDescriptor {
                    label: Some("world3d_painted_bind_group"),
                    layout: &self.resources.scene_bind_group_layout,
                    entries: &[gpu_device::BindGroupEntry { binding: 0, resource: gpu_device::BindingResource::TextureView(&raster.view) }, gpu_device::BindGroupEntry { binding: 1, resource: gpu_device::BindingResource::Sampler(&self.resources.world_plane_sampler) }],
                }));
                let gpu_instance = World3dGpuInstance::from_instance(
                    instance.model.to_cols_array_m(),
                    instance.color,
                    instance.material.preserve_vertex_color,
                    instance.material.emissive_intensity,
                    instance.material.metalness,
                    instance.material.roughness,
                    pass_owner.shadow.enabled,
                )
                .with_section(instance.material.section);
                frame_buffers.world_instances.upload(device, queue, std::slice::from_ref(&gpu_instance), gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "prepared_world_painted").ok_or("prepared world painted buffer admission failed")?
            }
            crate::kernel_3d_scene::SceneMaterialKind3d::Celebration { stops, angle } => {
                painted_bind_group = None;
                let gpu_instance = World3dCelebrationGpuInstance::from_instance(instance.model.to_cols_array_m(), *stops, *angle, instance.color[3]);
                frame_buffers
                    .world_celebration_instances
                    .upload(device, queue, std::slice::from_ref(&gpu_instance), gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "prepared_world_celebration")
                    .ok_or("prepared world celebration buffer admission failed")?
            }
            crate::kernel_3d_scene::SceneMaterialKind3d::Authored(material) => {
                let keys = [&material.base_color_texture, &material.metallic_roughness_texture, &material.normal_texture, &material.occlusion_texture, &material.emissive_texture];
                painted_bind_group = if keys.iter().any(|key| key.is_some()) {
                    let mut entries = Vec::with_capacity(10);
                    for (role,key) in keys.into_iter().enumerate() {
                        let view = if let Some(key) = key { let Some(raster) = raster_store.get(key) else { return Ok(true) }; &raster.view } else { &self.resources.world_authored_white };
                        let descriptor = if role == 0 { material.texture_sampler } else { material.additional_texture_samplers[role-1] };
                        let sampler = &self.resources.world_material_samplers.iter().find(|(candidate,_)| *candidate == descriptor).ok_or("authored material sampler was not prepared")?.1;
                        entries.push(gpu_device::BindGroupEntry { binding: role as u32 * 2, resource: gpu_device::BindingResource::TextureView(view) });
                        entries.push(gpu_device::BindGroupEntry { binding: role as u32 * 2 + 1, resource: gpu_device::BindingResource::Sampler(sampler) });
                    }
                    Some(device.create_bind_group(&gpu_device::BindGroupDescriptor { label: Some("world3d_authored_bind_group"), layout: &self.resources.world_authored_bind_group_layout, entries: &entries }))
                } else { None };
                let gpu_instance = World3dGpuInstance::from_authored(instance.model.to_cols_array_m(), material, pass_owner.shadow.enabled).with_section(instance.material.section);
                frame_buffers.world_instances.upload(device, queue, std::slice::from_ref(&gpu_instance), gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "prepared_world_authored").ok_or("prepared world authored buffer admission failed")?
            }
        };
        let scale = self.resources.surface_scale;
        let viewport = pass_owner.viewport;
        let scene_scissor = self.physical_scissor(scissor);
        if scene_scissor.w == 0 || scene_scissor.h == 0 {
            return Ok(true);
        }
        let mut pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("prepared_world_material"),
            color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view: color_view, resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: Some(stencil_attachment(depth_view, gpu_device::LoadOp::Load, gpu_device::LoadOp::Load)),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_viewport(viewport[0] * scale, viewport[1] * scale, viewport[2] * scale, viewport[3] * scale, 0.0, 1.0);
        pass.set_scissor_rect(scene_scissor.x, scene_scissor.y, scene_scissor.w, scene_scissor.h);
        match &draw_owner.material {
            crate::kernel_3d_scene::SceneMaterialKind3d::Standard => {
                pass.set_pipeline(&self.resources.world_standard_translucent_pipeline);
                pass.set_bind_group(0, &self.resources.world_globals_ring.bind_group, &[self.resources.world_globals_ring.offset_for_slot(0)]);
                pass.set_bind_group(1, &self.resources.world_shadow_target.bind_group, &[]);
            }
            crate::kernel_3d_scene::SceneMaterialKind3d::Painted { .. } => {
                pass.set_pipeline(if draw_owner.translucent { &self.resources.world_painted_pipeline_translucent } else { &self.resources.world_painted_pipeline });
                pass.set_bind_group(0, &self.resources.world_globals_ring.bind_group, &[self.resources.world_globals_ring.offset_for_slot(0)]);
                pass.set_bind_group(1, &self.resources.world_shadow_target.bind_group, &[]);
                pass.set_bind_group(2, painted_bind_group.as_ref().expect("painted branch owns its texture bind group"), &[]);
            }
            crate::kernel_3d_scene::SceneMaterialKind3d::Celebration { .. } => {
                pass.set_pipeline(if draw_owner.translucent { &self.resources.world_celebration_pipeline_translucent } else { &self.resources.world_celebration_pipeline });
                pass.set_bind_group(0, &self.resources.world_globals_ring.bind_group, &[self.resources.world_globals_ring.offset_for_slot(0)]);
            }
            crate::kernel_3d_scene::SceneMaterialKind3d::Authored(material) => {
                if painted_bind_group.is_some() {
                    pass.set_pipeline(match (draw_owner.translucent, material.double_sided) {
                        (false, false) => &self.resources.world_authored_painted_front_pipeline,
                        (false, true) => &self.resources.world_authored_painted_double_pipeline,
                        (true, false) => &self.resources.world_authored_painted_front_translucent_pipeline,
                        (true, true) => &self.resources.world_authored_painted_double_translucent_pipeline,
                    });
                    pass.set_bind_group(0, &self.resources.world_globals_ring.bind_group, &[self.resources.world_globals_ring.offset_for_slot(0)]);
                    pass.set_bind_group(1, &self.resources.world_shadow_target.bind_group, &[]);
                    pass.set_bind_group(2, painted_bind_group.as_ref().expect("authored texture owns its bind group"), &[]);
                } else {
                    pass.set_pipeline(match (draw_owner.translucent, material.double_sided) {
                        (false, false) => &self.resources.world_authored_front_pipeline,
                        (false, true) => &self.resources.world_pipeline,
                        (true, false) => &self.resources.world_pipeline_translucent,
                        (true, true) => &self.resources.world_authored_double_translucent_pipeline,
                    });
                    pass.set_bind_group(0, &self.resources.world_globals_ring.bind_group, &[self.resources.world_globals_ring.offset_for_slot(0)]);
                    pass.set_bind_group(1, &self.resources.world_shadow_target.bind_group, &[]);
                }
            }
        }
        pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer);
        pass.set_index_buffer(mesh.index_buffer.slice(..), gpu_device::IndexFormat::Uint32);
        let first = draw_owner.first_index.min(mesh.index_count);
        let end = first.saturating_add(draw_owner.index_count).min(mesh.index_count);
        if first < end {
            pass.draw_indexed(first..end, 0, 0..1);
        }
        pass.set_viewport(0.0, 0.0, width * scale, height * scale, 0.0, 1.0);
        Ok(true)
    }

    /// 🖼️ Encodes one retained TEXTURED world quad — the reference underlay — under one scene-pass
    /// owner.
    ///
    /// 🩸️ `ScenePass3d::textured_draws` has been measured by `advance_pipeline` and carried in every
    /// prepared packet since the scene pass existed, and `DrawMeasureCursor::PassTextured*` walked it
    /// page by page — but nothing ever ENCODED it: `encode_prepared_draw_scalar` had no arm for those
    /// cursors and `WORLD3D_TEXTURED_PIPELINE` was declared in the shader contract as "inferred —
    /// unwired in draw.rs". The plan underlay the puzzle3d playground decodes, admits and submits
    /// (`textured=1` on both surfaces) could therefore never paint on any target, ever
    /// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W5c).
    ///
    /// 🕰️ A key whose pixels have not landed yet draws NOTHING and is not a fault: the underlay's
    /// instance is published the frame its url appears, long before the image is fetched and decoded,
    /// exactly as React returns `null` from `WorldReferencePlaneItem` until `media` resolves.
    #[allow(clippy::too_many_arguments, reason = "one fixed owner per GPU boundary")]
    pub fn encode_prepared_world_textured<'a>(
        &'a mut self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        encoder: &mut gpu_device::CommandEncoder,
        color_view: &'a gpu_device::TextureView,
        depth_view: &'a gpu_device::TextureView,
        frame_buffers: &'a mut FrameBuffers,
        raster_store: &'a RasterTextureTable,
        pass_owner: &ScenePass3d,
        instance: &crate::kernel_3d_scene::TexturedInstance3d,
        scissor: ScissorRect,
    ) -> Result<(), &'static str> {
        let Some(raster) = raster_store.get(&instance.texture_key) else { return Ok(()) };
        let globals = World3dGlobals::from_pass(pass_owner);
        self.resources.world_globals_ring.ensure_slots(device, &self.resources.world_bind_group_layout, 1);
        self.resources.world_globals_ring.write_passes(queue, std::slice::from_ref(&globals));
        let gpu_instance = World3dTexturedGpuInstance::from_instance(instance.model.to_cols_array_m(), instance.background, instance.appearance);
        let Some(instance_buffer) = frame_buffers.world_textured_instances.upload(device, queue, std::slice::from_ref(&gpu_instance), gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "prepared_world_textured") else {
            return Err("prepared world textured buffer admission failed");
        };
        let texture_bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
            label: Some("world3d_textured_bind_group"),
            layout: &self.resources.scene_bind_group_layout,
            entries: &[gpu_device::BindGroupEntry { binding: 0, resource: gpu_device::BindingResource::TextureView(&raster.view) }, gpu_device::BindGroupEntry { binding: 1, resource: gpu_device::BindingResource::Sampler(&self.resources.world_plane_sampler) }],
        });
        let scale = self.resources.surface_scale;
        let viewport = pass_owner.viewport;
        let scene_scissor = self.physical_scissor(scissor);
        if scene_scissor.w == 0 || scene_scissor.h == 0 {
            return Ok(());
        }
        let mut pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("prepared_world_textured"),
            color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view: color_view, resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: Some(stencil_attachment(depth_view, gpu_device::LoadOp::Load, gpu_device::LoadOp::Load)),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_viewport(viewport[0] * scale, viewport[1] * scale, viewport[2] * scale, viewport[3] * scale, 0.0, 1.0);
        pass.set_scissor_rect(scene_scissor.x, scene_scissor.y, scene_scissor.w, scene_scissor.h);
        pass.set_pipeline(&self.resources.world_textured_pipeline);
        pass.set_bind_group(0, &self.resources.world_globals_ring.bind_group, &[self.resources.world_globals_ring.offset_for_slot(0)]);
        pass.set_bind_group(1, &texture_bind_group, &[]);
        pass.set_vertex_buffer(0, self.resources.world_plane_vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer);
        pass.draw(0..6, 0..1);
        Ok(())
    }

    pub fn encode_prepared_world_grid<'a>(
        &'a mut self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        encoder: &mut gpu_device::CommandEncoder,
        color_view: &'a gpu_device::TextureView,
        depth_view: &'a gpu_device::TextureView,
        pass_owner: &ScenePass3d,
        scissor: ScissorRect,
    ) -> Result<(), &'static str> {
        let grid = pass_owner.procedural_grid.as_ref().ok_or("prepared world grid cursor was stale")?;
        let globals = World3dGlobals::from_pass(pass_owner);
        self.resources.world_globals_ring.ensure_slots(device, &self.resources.world_bind_group_layout, 1);
        self.resources.world_globals_ring.write_passes(queue, std::slice::from_ref(&globals));
        queue.write_buffer(&self.resources.world_grid_uniform_buffer, 0, bytemuck::bytes_of(&World3dGridUniforms::from_grid(grid)));
        let viewport = physical_viewport_rect(pass_owner.viewport, self.resources.surface_scale);
        let scene_scissor = self.physical_scissor(scissor);
        if scene_scissor.w == 0 || scene_scissor.h == 0 {
            return Ok(());
        }
        let mut pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("prepared_world_grid"),
            color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view: color_view, resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: Some(stencil_attachment(depth_view, gpu_device::LoadOp::Load, gpu_device::LoadOp::Load)),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_viewport(viewport[0], viewport[1], viewport[2], viewport[3], 0.0, 1.0);
        pass.set_scissor_rect(scene_scissor.x, scene_scissor.y, scene_scissor.w, scene_scissor.h);
        pass.set_pipeline(&self.resources.world_grid_pipeline);
        pass.set_bind_group(0, &self.resources.world_globals_ring.bind_group, &[self.resources.world_globals_ring.offset_for_slot(0)]);
        pass.set_bind_group(1, &self.resources.world_grid_bind_group, &[]);
        pass.set_vertex_buffer(0, self.resources.world_plane_vertex_buffer.slice(..));
        pass.draw(0..6, 0..1);
        Ok(())
    }

    /// 📏 Encodes one retained world line segment under one scene-pass owner.
    #[allow(clippy::too_many_arguments, reason = "one fixed owner per GPU boundary")]
    pub fn encode_prepared_world_line<'a>(
        &'a mut self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        encoder: &mut gpu_device::CommandEncoder,
        color_view: &'a gpu_device::TextureView,
        depth_view: &'a gpu_device::TextureView,
        frame_buffers: &'a mut FrameBuffers,
        pass_owner: &ScenePass3d,
        vertices: &[crate::kernel_3d_scene::LineVertex3d],
        scissor: ScissorRect,
    ) -> Result<(), &'static str> {
        if vertices.len() != 2 {
            return Err("prepared world line scalar was not one segment");
        }
        let globals = World3dGlobals::from_pass(pass_owner);
        self.resources.world_globals_ring.ensure_slots(device, &self.resources.world_bind_group_layout, 1);
        self.resources.world_globals_ring.write_passes(queue, std::slice::from_ref(&globals));
        let gpu_vertices = [WorldLineGpuVertex { position: vertices[0].position, color: vertices[0].color }, WorldLineGpuVertex { position: vertices[1].position, color: vertices[1].color }];
        let Some(line_buffer) = frame_buffers.world_lines.upload(device, queue, &gpu_vertices, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "prepared_world_line") else {
            return Err("prepared world line buffer admission failed");
        };
        let scale = self.resources.surface_scale;
        let viewport = pass_owner.viewport;
        let scene_scissor = self.physical_scissor(scissor);
        if scene_scissor.w == 0 || scene_scissor.h == 0 {
            return Ok(());
        }
        let mut pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("prepared_world_line"),
            color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view: color_view, resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: Some(stencil_attachment(depth_view, gpu_device::LoadOp::Load, gpu_device::LoadOp::Load)),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_viewport(viewport[0] * scale, viewport[1] * scale, viewport[2] * scale, viewport[3] * scale, 0.0, 1.0);
        pass.set_scissor_rect(scene_scissor.x, scene_scissor.y, scene_scissor.w, scene_scissor.h);
        pass.set_pipeline(&self.resources.world_line_pipeline);
        pass.set_bind_group(0, &self.resources.world_globals_ring.bind_group, &[self.resources.world_globals_ring.offset_for_slot(0)]);
        pass.set_vertex_buffer(0, line_buffer);
        pass.draw(0..2, 0..1);
        Ok(())
    }

    /// 🎭️ Replays one completed World viewport through its image remap and physical viewport mask.
    pub fn encode_prepared_world_postprocess(
        &self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        encoder: &mut gpu_device::CommandEncoder,
        composite: &PreparedCompositeTarget,
        pass_owner: &ScenePass3d,
        scene_scissor: ScissorRect,
    ) -> Result<(), &'static str> {
        let curvilinear = pass_owner.curvilinear;
        let ellipse = pass_owner.viewport_mask == crate::kernel_3d_scene::SceneViewportMask3d::Ellipse;
        if curvilinear.is_none() && !ellipse {
            return Err("prepared world postprocess pass had no descriptor");
        }
        let background = if ellipse { composite.postprocess_background_view().ok_or("prepared ellipse background snapshot was missing")? } else { &composite.postprocess_scene_scratch_view };
        let scale = self.resources.surface_scale;
        let viewport = [pass_owner.viewport[0] * scale, pass_owner.viewport[1] * scale, pass_owner.viewport[2] * scale, pass_owner.viewport[3] * scale];
        let fov_strength = curvilinear.map_or([0.0, 0.0], |value| [value.fov_radians, value.strength]);
        queue.write_buffer(
            &self.resources.postprocess_globals_buffer,
            0,
            bytemuck::bytes_of(&WorldPostprocessGlobals { viewport, surface_size: [composite.width as f32, composite.height as f32], fov_strength, modes: [u32::from(curvilinear.is_some()), u32::from(ellipse), 0, 0], clear_color: [0.0; 4] }),
        );
        composite.copy_to_postprocess_scene_scratch(encoder);
        let bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
            label: Some("prepared_world_postprocess_bind_group"),
            layout: &self.resources.postprocess_bind_group_layout,
            entries: &[
                gpu_device::BindGroupEntry { binding: 0, resource: self.resources.postprocess_globals_buffer.as_entire_binding() },
                gpu_device::BindGroupEntry { binding: 1, resource: gpu_device::BindingResource::TextureView(&composite.postprocess_scene_scratch_view) },
                gpu_device::BindGroupEntry { binding: 2, resource: gpu_device::BindingResource::Sampler(&composite.sampler) },
                gpu_device::BindGroupEntry { binding: 3, resource: gpu_device::BindingResource::TextureView(background) },
            ],
        });
        let mut render = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("prepared_world_postprocess"),
            color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view: composite.world_encoded_view(), resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        render.set_viewport(viewport[0], viewport[1], viewport[2].max(1.0), viewport[3].max(1.0), 0.0, 1.0);
        let scene_scissor = self.physical_scissor(scene_scissor);
        render.set_scissor_rect(scene_scissor.x, scene_scissor.y, scene_scissor.w, scene_scissor.h);
        render.set_pipeline(&self.resources.world_postprocess_pipeline);
        render.set_bind_group(0, &bind_group, &[]);
        render.draw(0..6, 0..1);
        Ok(())
    }

    /// 🫧️ Paints an authored World background after the backdrop snapshot and before any 3D payload.
    pub fn encode_prepared_world_scene_clear(
        &self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        encoder: &mut gpu_device::CommandEncoder,
        composite: &PreparedCompositeTarget,
        pass_owner: &ScenePass3d,
        scene_scissor: ScissorRect,
    ) -> Result<(), &'static str> {
        let clear_color = pass_owner.clear_color.ok_or("prepared world scene clear had no color")?;
        let background = composite.postprocess_background_view().ok_or("prepared world scene clear had no background snapshot")?;
        let scale = self.resources.surface_scale;
        let viewport = [pass_owner.viewport[0] * scale, pass_owner.viewport[1] * scale, pass_owner.viewport[2] * scale, pass_owner.viewport[3] * scale];
        queue.write_buffer(
            &self.resources.postprocess_globals_buffer,
            0,
            bytemuck::bytes_of(&WorldPostprocessGlobals { viewport, surface_size: [composite.width as f32, composite.height as f32], fov_strength: [0.0; 2], modes: [0, 0, 1, 0], clear_color }),
        );
        let bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
            label: Some("prepared_world_scene_clear_bind_group"),
            layout: &self.resources.postprocess_bind_group_layout,
            entries: &[
                gpu_device::BindGroupEntry { binding: 0, resource: self.resources.postprocess_globals_buffer.as_entire_binding() },
                gpu_device::BindGroupEntry { binding: 1, resource: gpu_device::BindingResource::TextureView(background) },
                gpu_device::BindGroupEntry { binding: 2, resource: gpu_device::BindingResource::Sampler(&composite.sampler) },
                gpu_device::BindGroupEntry { binding: 3, resource: gpu_device::BindingResource::TextureView(background) },
            ],
        });
        let mut render = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("prepared_world_scene_clear"),
            color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view: composite.world_encoded_view(), resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        render.set_viewport(viewport[0], viewport[1], viewport[2].max(1.0), viewport[3].max(1.0), 0.0, 1.0);
        let scene_scissor = self.physical_scissor(scene_scissor);
        render.set_scissor_rect(scene_scissor.x, scene_scissor.y, scene_scissor.w, scene_scissor.h);
        render.set_pipeline(&self.resources.world_postprocess_pipeline);
        render.set_bind_group(0, &bind_group, &[]);
        render.draw(0..6, 0..1);
        Ok(())
    }

    /// 🌫️ Builds one fixed blur mip for retained glass sampling.
    pub fn encode_prepared_blur_mip(&self, device: &gpu_device::Device, queue: &gpu_device::Queue, scene: &SceneColorTarget, mip: u32) -> Result<(), &'static str> {
        if mip == 0 || mip >= SCENE_MIP_LEVELS {
            return Err("prepared blur mip cursor was invalid");
        }
        let source = mip - 1;
        queue.write_buffer(&self.resources.blur_globals_buffer, 0, bytemuck::bytes_of(&BlurGlobals { src_mip: 0.0, _pad: [0.0; 7] }));
        let bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
            label: Some("prepared_blur_mip"),
            layout: &self.resources.blur_bind_group_layout,
            entries: &[
                gpu_device::BindGroupEntry { binding: 0, resource: self.resources.blur_globals_buffer.as_entire_binding() },
                gpu_device::BindGroupEntry { binding: 1, resource: gpu_device::BindingResource::TextureView(scene.blur_scratch_mip_view(source)) },
                gpu_device::BindGroupEntry { binding: 2, resource: gpu_device::BindingResource::Sampler(scene.sampler()) },
            ],
        });
        let mut copy = device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_blur_copy") });
        scene.copy_mip_to_blur_scratch(&mut copy, source);
        queue.submit(copy.finish()).expect("same-session recorded command");
        let mut encoder = device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_blur_mip") });
        let mut pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("prepared_blur_mip"),
            color_attachments: &[Some(gpu_device::RenderPassColorAttachment {
                view: scene.mip_view(mip),
                resolve_target: None,
                ops: gpu_device::Operations { load: gpu_device::LoadOp::Clear(gpu_device::Color::TRANSPARENT), store: gpu_device::StoreOp::Store },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&self.resources.blur_downsample_pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.draw(0..6, 0..1);
        drop(pass);
        queue.submit(encoder.finish()).expect("same-session recorded command");
        Ok(())
    }

    /// 🪟 Encodes one retained glass region after the scene snapshot is resident.
    #[allow(clippy::too_many_arguments, reason = "one fixed owner per GPU boundary")]
    pub fn encode_prepared_glass_scalar<'a>(
        &'a self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        encoder: &mut gpu_device::CommandEncoder,
        view: &'a gpu_device::TextureView,
        scene: &'a SceneColorTarget,
        frame_buffers: &'a mut FrameBuffers,
        region: &GlassRegion,
        scissor: ScissorRect,
        width: f32,
        height: f32,
    ) -> Result<(), &'static str> {
        let instance = GlassInstance { rect: region.rect, tint: [region.tint.r, region.tint.g, region.tint.b, region.tint.a], params: [region.radius, region.alpha, Theme::glass_mip_level(region.blur_px, SCENE_MIP_LEVELS - 1), region.saturate] };
        let Some(buffer) = frame_buffers.glass_instances.upload(device, queue, std::slice::from_ref(&instance), gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "prepared_glass_scalar") else {
            return Err("prepared glass scalar buffer admission failed");
        };
        let scene_bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
            label: Some("prepared_glass_scene"),
            layout: &self.resources.scene_bind_group_layout,
            entries: &[gpu_device::BindGroupEntry { binding: 0, resource: gpu_device::BindingResource::TextureView(scene.sample_view()) }, gpu_device::BindGroupEntry { binding: 1, resource: gpu_device::BindingResource::Sampler(scene.sampler()) }],
        });
        let mut pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("prepared_glass_scalar"),
            color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view, resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        set_pass_scissor(&mut pass, Some(scissor), self.resources.surface_scale, width, height);
        pass.set_pipeline(&self.resources.glass_pipeline);
        pass.set_bind_group(0, &self.resources.glyph_bind_group, &[]);
        pass.set_bind_group(1, &scene_bind_group, &[]);
        pass.set_vertex_buffer(0, self.resources.quad_vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, buffer);
        pass.draw(0..6, 0..1);
        Ok(())
    }

    /// 🖼️ Encodes one fixed scene-to-surface packet after all command pages are resident.
    pub fn blit_prepared_scene<'a>(&'a self, device: &gpu_device::Device, encoder: &mut gpu_device::CommandEncoder, view: &'a gpu_device::TextureView, scene: &'a SceneColorTarget) {
        self.blit_sampled_color(device, encoder, view, scene.sample_view(), scene.sampler());
    }

    /// 🖼️ Copies the accumulated composite into a backdrop snapshot or the terminal surface.
    pub fn blit_prepared_composite<'a>(&'a self, device: &gpu_device::Device, encoder: &mut gpu_device::CommandEncoder, view: &'a gpu_device::TextureView, composite: &'a PreparedCompositeTarget) {
        self.blit_sampled_color(device, encoder, view, &composite.view, &composite.sampler);
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    #[cfg(test)]
    pub fn render_scene_content<'a>(
        &'a mut self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        encoder: &mut gpu_device::CommandEncoder,
        scene: &'a SceneColorTarget,
        depth_view: Option<&'a gpu_device::TextureView>,
        draw: &DrawList,
        mesh_store: &MeshGpuTable,
        raster_store: &RasterTextureTable,
        frame_buffers: &mut FrameBuffers,
        width: f32,
        height: f32,
        time_seconds: f32,
    ) {
        self.update_globals(queue, width, height, time_seconds);
        let scene_view = scene.mip_view(0);
        let world_upload = if depth_view.is_some() { self.upload_world_passes(device, queue, draw, frame_buffers, LayerBatchFilter::Backdrop) } else { None };
        let (prepared_holder, pass_index_map) = match world_upload {
            Some((prepared, map)) => (Some(prepared), map),
            None => (None, vec![None; draw.scene_passes.len()]),
        };
        let world_prepared = prepared_holder.as_deref();
        let (all_ui, all_vec, batches) = build_layer_batches(draw, LayerBatchFilter::Backdrop);
        let (mask_data, mask_ranges) = build_batch_masks(&batches, width, height);
        let mask_buffer = frame_buffers.mask_instances.upload(device, queue, &mask_data, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "silhouette_masks");
        let ui_buffer = if all_ui.is_empty() { None } else { frame_buffers.ui_instances.upload(device, queue, &all_ui, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "ui_instances") };
        let vector_buffer = if all_vec.is_empty() { None } else { frame_buffers.vector_vertices.upload(device, queue, &all_vec, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "vector_vertices") };
        let instance_buffer = frame_buffers.world_instances.slice();
        let line_buffer = frame_buffers.world_lines.slice();
        let mut pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("ui_pass"),
            color_attachments: &[Some(gpu_device::RenderPassColorAttachment {
                view: scene_view,
                resolve_target: None,
                ops: gpu_device::Operations { load: gpu_device::LoadOp::Clear(gpu_device::Color { r: 0.05, g: 0.05, b: 0.06, a: 1.0 }), store: gpu_device::StoreOp::Store },
                depth_slice: None,
            })],
            depth_stencil_attachment: depth_view.map(|depth| stencil_attachment(depth, gpu_device::LoadOp::Clear(1.0), gpu_device::LoadOp::Clear(0))),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        self.render_interleaved_layers(
            &mut pass,
            draw,
            &batches,
            mask_buffer.as_ref(),
            &mask_ranges,
            ui_buffer.as_ref(),
            vector_buffer.as_ref(),
            world_prepared,
            &pass_index_map,
            instance_buffer,
            line_buffer,
            mesh_store,
            width,
            height,
            depth_view.is_some(),
        );
        drop(pass);
        if draw.layers.iter().any(|layer| layer_matches_filter(layer, LayerBatchFilter::Backdrop) && !layer.raster_instances.is_empty()) {
            let mut raster_pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
                label: Some("ui_raster_pass"),
                color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view: scene_view, resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
                depth_stencil_attachment: depth_view.map(|depth| stencil_attachment(depth, gpu_device::LoadOp::Load, gpu_device::LoadOp::Clear(0))),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            self.draw_raster_layers(&mut raster_pass, raster_store, draw, frame_buffers, device, queue, width, height, LayerBatchFilter::Backdrop, false);
        }
        let (overlay_ui, overlay_vec, overlay_batches) = build_overlay_layer_batches(draw, LayerBatchFilter::Backdrop);
        if !overlay_ui.is_empty() || !overlay_vec.is_empty() {
            let (overlay_mask_data, overlay_mask_ranges) = build_batch_masks(&overlay_batches, width, height);
            let overlay_mask_buffer = frame_buffers.mask_instances.upload(device, queue, &overlay_mask_data, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "overlay_silhouette_masks");
            let overlay_ui_buffer = if overlay_ui.is_empty() { None } else { frame_buffers.ui_instances.upload(device, queue, &overlay_ui, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "overlay_ui_instances") };
            let overlay_vector_buffer = if overlay_vec.is_empty() { None } else { frame_buffers.vector_vertices.upload(device, queue, &overlay_vec, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "overlay_vector_vertices") };
            let mut overlay_pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
                label: Some("ui_overlay_pass"),
                color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view: scene_view, resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
                depth_stencil_attachment: depth_view.map(|depth| stencil_attachment(depth, gpu_device::LoadOp::Load, gpu_device::LoadOp::Clear(0))),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            self.render_interleaved_layers(
                &mut overlay_pass,
                draw,
                &overlay_batches,
                overlay_mask_buffer.as_ref(),
                &overlay_mask_ranges,
                overlay_ui_buffer.as_ref(),
                overlay_vector_buffer.as_ref(),
                None,
                &[],
                None,
                None,
                mesh_store,
                width,
                height,
                depth_view.is_some(),
            );
        }
        if draw.layers.iter().any(|layer| layer_matches_filter(layer, LayerBatchFilter::Backdrop) && !layer.overlay_raster_instances.is_empty()) {
            let mut raster_pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
                label: Some("ui_overlay_raster_pass"),
                color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view: scene_view, resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
                depth_stencil_attachment: depth_view.map(|depth| stencil_attachment(depth, gpu_device::LoadOp::Load, gpu_device::LoadOp::Clear(0))),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            self.draw_raster_layers(&mut raster_pass, raster_store, draw, frame_buffers, device, queue, width, height, LayerBatchFilter::Backdrop, true);
        }
    }

    #[cfg(test)]
    fn has_glass_foreground(draw: &DrawList) -> bool {
        let layer_content = draw.layers.iter().any(|layer| {
            layer.foreground_of.is_some()
                && (!layer.ui_instances.is_empty()
                    || !layer.vector_vertices.is_empty()
                    || !layer.raster_instances.is_empty()
                    || !layer.overlay_ui_instances.is_empty()
                    || !layer.overlay_vector_vertices.is_empty()
                    || !layer.overlay_raster_instances.is_empty())
        });
        let scene_content = draw.scene_passes.iter().any(|pass| layer_matches_filter(&draw.layers[pass.layer_index], LayerBatchFilter::Foreground));
        layer_content || scene_content
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    #[cfg(test)]
    fn render_glass_foreground<'a>(
        &'a mut self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        encoder: &mut gpu_device::CommandEncoder,
        view: &'a gpu_device::TextureView,
        draw: &DrawList,
        depth_view: Option<&'a gpu_device::TextureView>,
        mesh_store: &MeshGpuTable,
        raster_store: &RasterTextureTable,
        frame_buffers: &mut FrameBuffers,
        width: f32,
        height: f32,
    ) {
        if !Self::has_glass_foreground(draw) {
            return;
        }
        let world_upload = if depth_view.is_some() { self.upload_world_passes(device, queue, draw, frame_buffers, LayerBatchFilter::Foreground) } else { None };
        let (prepared_holder, pass_index_map) = match world_upload {
            Some((prepared, map)) => (Some(prepared), map),
            None => (None, vec![None; draw.scene_passes.len()]),
        };
        let world_prepared = prepared_holder.as_deref();
        let (all_ui, all_vec, batches) = build_layer_batches(draw, LayerBatchFilter::Foreground);
        let has_overlay =
            draw.layers.iter().any(|layer| layer_matches_filter(layer, LayerBatchFilter::Foreground) && (!layer.overlay_ui_instances.is_empty() || !layer.overlay_vector_vertices.is_empty() || !layer.overlay_raster_instances.is_empty()));
        if all_ui.is_empty() && all_vec.is_empty() && batches.is_empty() && world_prepared.is_none() && !has_overlay {
            return;
        }
        let (mask_data, mask_ranges) = build_batch_masks(&batches, width, height);
        let mask_buffer = frame_buffers.mask_instances.upload(device, queue, &mask_data, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "glass_foreground_silhouette_masks");
        let ui_buffer = if all_ui.is_empty() { None } else { frame_buffers.ui_instances.upload(device, queue, &all_ui, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "glass_foreground_ui_instances") };
        let vector_buffer = if all_vec.is_empty() { None } else { frame_buffers.vector_vertices.upload(device, queue, &all_vec, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "glass_foreground_vector_vertices") };
        let instance_buffer = frame_buffers.world_instances.slice();
        let line_buffer = frame_buffers.world_lines.slice();
        let mut pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("glass_foreground_pass"),
            color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view, resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: depth_view.map(|depth| stencil_attachment(depth, gpu_device::LoadOp::Load, gpu_device::LoadOp::Clear(0))),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        self.render_interleaved_layers(
            &mut pass,
            draw,
            &batches,
            mask_buffer.as_ref(),
            &mask_ranges,
            ui_buffer.as_ref(),
            vector_buffer.as_ref(),
            world_prepared,
            &pass_index_map,
            instance_buffer,
            line_buffer,
            mesh_store,
            width,
            height,
            depth_view.is_some(),
        );
        drop(pass);
        if draw.layers.iter().any(|layer| layer_matches_filter(layer, LayerBatchFilter::Foreground) && !layer.raster_instances.is_empty()) {
            let mut raster_pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
                label: Some("glass_foreground_raster_pass"),
                color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view, resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
                depth_stencil_attachment: depth_view.map(|depth| stencil_attachment(depth, gpu_device::LoadOp::Load, gpu_device::LoadOp::Clear(0))),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            self.draw_raster_layers(&mut raster_pass, raster_store, draw, frame_buffers, device, queue, width, height, LayerBatchFilter::Foreground, false);
        }
        let (overlay_ui, overlay_vec, overlay_batches) = build_overlay_layer_batches(draw, LayerBatchFilter::Foreground);
        if !overlay_ui.is_empty() || !overlay_vec.is_empty() {
            let (overlay_mask_data, overlay_mask_ranges) = build_batch_masks(&overlay_batches, width, height);
            let overlay_mask_buffer = frame_buffers.mask_instances.upload(device, queue, &overlay_mask_data, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "glass_foreground_overlay_silhouette_masks");
            let overlay_ui_buffer = if overlay_ui.is_empty() { None } else { frame_buffers.ui_instances.upload(device, queue, &overlay_ui, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "glass_foreground_overlay_ui_instances") };
            let overlay_vector_buffer =
                if overlay_vec.is_empty() { None } else { frame_buffers.vector_vertices.upload(device, queue, &overlay_vec, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "glass_foreground_overlay_vector_vertices") };
            let mut overlay_pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
                label: Some("glass_foreground_overlay_pass"),
                color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view, resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
                depth_stencil_attachment: depth_view.map(|depth| stencil_attachment(depth, gpu_device::LoadOp::Load, gpu_device::LoadOp::Clear(0))),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            self.render_interleaved_layers(
                &mut overlay_pass,
                draw,
                &overlay_batches,
                overlay_mask_buffer.as_ref(),
                &overlay_mask_ranges,
                overlay_ui_buffer.as_ref(),
                overlay_vector_buffer.as_ref(),
                None,
                &[],
                None,
                None,
                mesh_store,
                width,
                height,
                depth_view.is_some(),
            );
        }
        if draw.layers.iter().any(|layer| layer_matches_filter(layer, LayerBatchFilter::Foreground) && !layer.overlay_raster_instances.is_empty()) {
            let mut raster_pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
                label: Some("glass_foreground_overlay_raster_pass"),
                color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view, resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
                depth_stencil_attachment: depth_view.map(|depth| stencil_attachment(depth, gpu_device::LoadOp::Load, gpu_device::LoadOp::Clear(0))),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            self.draw_raster_layers(&mut raster_pass, raster_store, draw, frame_buffers, device, queue, width, height, LayerBatchFilter::Foreground, true);
        }
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    #[cfg(test)]
    pub fn composite_to_swapchain<'a>(
        &'a mut self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        encoder: &mut gpu_device::CommandEncoder,
        view: &'a gpu_device::TextureView,
        scene: &'a SceneColorTarget,
        depth_view: Option<&'a gpu_device::TextureView>,
        draw: &DrawList,
        overlay: Option<&DrawList>,
        mesh_store: &MeshGpuTable,
        raster_store: &RasterTextureTable,
        frame_buffers: &mut FrameBuffers,
        width: f32,
        height: f32,
    ) {
        self.run_blur_chain(device, queue, scene);
        self.blit_sampled_color(device, encoder, view, scene.sample_view(), scene.sampler());
        let max_mip = SCENE_MIP_LEVELS - 1;
        self.composite_glass_regions(device, queue, encoder, view, scene, frame_buffers, &draw.glass_regions, max_mip, width, height);
        self.render_glass_foreground(device, queue, encoder, view, draw, depth_view, mesh_store, raster_store, frame_buffers, width, height);
        if let Some(overlay) = overlay {
            if !overlay.glass_regions.is_empty() {
                self.composite_glass_regions(device, queue, encoder, view, scene, frame_buffers, &overlay.glass_regions, max_mip, width, height);
            }
            self.render_glass_foreground(device, queue, encoder, view, overlay, depth_view, mesh_store, raster_store, frame_buffers, width, height);
            let mut overlay_pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
                label: Some("ui_overlay_pass"),
                color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view, resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
                depth_stencil_attachment: depth_view.map(|depth| stencil_attachment(depth, gpu_device::LoadOp::Load, gpu_device::LoadOp::Clear(0))),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            self.render_overlay(device, queue, &mut overlay_pass, overlay, frame_buffers, width, height);
        }
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    #[allow(dead_code, reason = "top-level UiPipelines render entrypoint; not yet called internally, likely wired externally by framework/renderer/wgpu")]
    #[cfg(test)]
    pub fn render<'a>(
        &'a mut self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        encoder: &mut gpu_device::CommandEncoder,
        view: &'a gpu_device::TextureView,
        scene: &'a SceneColorTarget,
        depth_view: Option<&'a gpu_device::TextureView>,
        draw: &DrawList,
        overlay: Option<&DrawList>,
        mesh_store: &MeshGpuTable,
        raster_store: &RasterTextureTable,
        frame_buffers: &mut FrameBuffers,
        width: f32,
        height: f32,
        time_seconds: f32,
    ) {
        self.render_scene_content(device, queue, encoder, scene, depth_view, draw, mesh_store, raster_store, frame_buffers, width, height, time_seconds);
        self.composite_to_swapchain(device, queue, encoder, view, scene, depth_view, draw, overlay, mesh_store, raster_store, frame_buffers, width, height);
    }

    #[cfg(test)]
    fn run_blur_chain(&self, device: &gpu_device::Device, queue: &gpu_device::Queue, scene: &SceneColorTarget) {
        for mip in 1..SCENE_MIP_LEVELS {
            let src_mip = mip - 1;
            queue.write_buffer(&self.resources.blur_globals_buffer, 0, bytemuck::bytes_of(&BlurGlobals { src_mip: 0.0, _pad: [0.0; 7] }));
            let blur_bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
                label: Some("blur_downsample_bind_group"),
                layout: &self.resources.blur_bind_group_layout,
                entries: &[
                    gpu_device::BindGroupEntry { binding: 0, resource: self.resources.blur_globals_buffer.as_entire_binding() },
                    gpu_device::BindGroupEntry { binding: 1, resource: gpu_device::BindingResource::TextureView(scene.blur_scratch_mip_view(src_mip)) },
                    gpu_device::BindGroupEntry { binding: 2, resource: gpu_device::BindingResource::Sampler(scene.sampler()) },
                ],
            });
            let mut copy_encoder = device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("blur_copy_encoder") });
            scene.copy_mip_to_blur_scratch(&mut copy_encoder, src_mip);
            queue.submit(copy_encoder.finish()).expect("same-session recorded command");
            let mut encoder = device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("blur_downsample_encoder") });
            let mut pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
                label: Some("blur_downsample_pass"),
                color_attachments: &[Some(gpu_device::RenderPassColorAttachment {
                    view: scene.mip_view(mip),
                    resolve_target: None,
                    ops: gpu_device::Operations { load: gpu_device::LoadOp::Clear(gpu_device::Color::TRANSPARENT), store: gpu_device::StoreOp::Store },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&self.resources.blur_downsample_pipeline);
            pass.set_bind_group(0, &blur_bind_group, &[]);
            pass.draw(0..6, 0..1);
            drop(pass);
            queue.submit(encoder.finish()).expect("same-session recorded command");
        }
    }

    fn blit_sampled_color(&self, device: &gpu_device::Device, encoder: &mut gpu_device::CommandEncoder, view: &gpu_device::TextureView, source: &gpu_device::TextureView, sampler: &gpu_device::Sampler) {
        let scene_bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
            label: Some("scene_blit_bind_group"),
            layout: &self.resources.scene_bind_group_layout,
            entries: &[gpu_device::BindGroupEntry { binding: 0, resource: gpu_device::BindingResource::TextureView(source) }, gpu_device::BindGroupEntry { binding: 1, resource: gpu_device::BindingResource::Sampler(sampler) }],
        });
        let mut pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("scene_blit_pass"),
            color_attachments: &[Some(gpu_device::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: gpu_device::Operations { load: gpu_device::LoadOp::Clear(gpu_device::Color { r: 0.05, g: 0.05, b: 0.06, a: 1.0 }), store: gpu_device::StoreOp::Store },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&self.resources.scene_blit_pipeline);
        pass.set_bind_group(0, &scene_bind_group, &[]);
        pass.draw(0..6, 0..1);
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    #[cfg(test)]
    fn composite_glass_regions(
        &self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        encoder: &mut gpu_device::CommandEncoder,
        view: &gpu_device::TextureView,
        scene: &SceneColorTarget,
        frame_buffers: &mut FrameBuffers,
        regions: &[GlassRegion],
        max_mip: u32,
        width: f32,
        height: f32,
    ) {
        if regions.is_empty() {
            return;
        }
        let instances: Vec<GlassInstance> = regions
            .iter()
            .map(|region| GlassInstance { rect: region.rect, tint: [region.tint.r, region.tint.g, region.tint.b, region.tint.a], params: [region.radius, region.alpha, Theme::glass_mip_level(region.blur_px, max_mip), region.saturate] })
            .collect();
        let glass_buffer = frame_buffers.glass_instances.upload(device, queue, &instances, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "glass_instances");
        let Some(glass_buffer) = glass_buffer else {
            return;
        };
        let scene_bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
            label: Some("glass_scene_bind_group"),
            layout: &self.resources.scene_bind_group_layout,
            entries: &[gpu_device::BindGroupEntry { binding: 0, resource: gpu_device::BindingResource::TextureView(scene.sample_view()) }, gpu_device::BindGroupEntry { binding: 1, resource: gpu_device::BindingResource::Sampler(scene.sampler()) }],
        });
        let mut pass = encoder.begin_render_pass(&gpu_device::RenderPassDescriptor {
            label: Some("glass_composite_pass"),
            color_attachments: &[Some(gpu_device::RenderPassColorAttachment { view, resolve_target: None, ops: gpu_device::Operations { load: gpu_device::LoadOp::Load, store: gpu_device::StoreOp::Store }, depth_slice: None })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&self.resources.glass_pipeline);
        pass.set_bind_group(0, &self.resources.glyph_bind_group, &[]);
        pass.set_bind_group(1, &scene_bind_group, &[]);
        pass.set_vertex_buffer(0, self.resources.quad_vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, glass_buffer.slice(..));
        pass.draw(0..6, 0..instances.len() as u32);
        let _ = (width, height);
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    #[cfg(test)]
    pub fn render_overlay<'a>(&'a self, device: &gpu_device::Device, queue: &gpu_device::Queue, pass: &mut gpu_device::RenderPass<'a>, overlay: &DrawList, frame_buffers: &'a mut FrameBuffers, width: f32, height: f32) {
        pass.set_pipeline(&self.resources.ui_pipeline);
        pass.set_bind_group(0, &self.resources.glyph_bind_group, &[]);

        let (all_ui, all_vec, batches) = build_layer_batches(overlay, LayerBatchFilter::Backdrop);
        let (mask_data, mask_ranges) = build_batch_masks(&batches, width, height);
        let mask_buffer = frame_buffers.mask_instances.upload(device, queue, &mask_data, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "top_overlay_silhouette_masks");
        let ui_buffer = if all_ui.is_empty() { None } else { frame_buffers.ui_instances.upload(device, queue, &all_ui, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "overlay_ui_instances") };
        let vector_buffer = if all_vec.is_empty() { None } else { frame_buffers.vector_vertices.upload(device, queue, &all_vec, gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, "overlay_vector_vertices") };

        for (batch_index, batch) in batches.iter().enumerate() {
            if let Some((start, count)) = mask_ranges.get(batch_index).copied() {
                if let Some(mask_buffer) = mask_buffer.as_ref() {
                    self.draw_silhouette_mask(pass, mask_buffer, start, count, width, height);
                } else {
                    pass.set_stencil_reference(1);
                }
            }
            if batch.ui_count > 0 {
                if let Some(instance_buffer) = &ui_buffer {
                    pass.set_pipeline(&self.resources.ui_pipeline);
                    pass.set_bind_group(0, &self.resources.glyph_bind_group, &[]);
                    pass.set_vertex_buffer(0, self.resources.quad_vertex_buffer.slice(..));
                    pass.set_vertex_buffer(1, *instance_buffer);
                    pass.draw(0..6, batch.ui_start..batch.ui_start + batch.ui_count);
                }
            }
            if batch.vec_count > 0 {
                if let Some(vector_buffer) = &vector_buffer {
                    pass.set_pipeline(&self.resources.vector_pipeline);
                    pass.set_vertex_buffer(0, *vector_buffer);
                    pass.draw(batch.vec_start..batch.vec_start + batch.vec_count, 0..1);
                }
            }
        }
    }
}

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
