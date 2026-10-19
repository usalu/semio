//! 🌍️ World pass drawing: the world pass upload and draw, the instance, silhouette, raster and vector draws and the interleaved layer rendering. See [`super`].

use super::*;

impl UiPipelines {
    #[cfg(test)]
    pub(super) fn upload_world_passes(&mut self, device: &gpu_device::Device, queue: &gpu_device::Queue, draw: &DrawList, frame_buffers: &mut FrameBuffers, filter: LayerBatchFilter) -> Option<(Vec<PreparedWorldPass>, Vec<Option<usize>>)> {
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
    pub(super) fn draw_silhouette_mask<'a>(&'a self, pass: &mut gpu_device::RenderPass<'a>, mask_buffer: &gpu_device::BufferSlice<'a>, start: u32, count: u32, width: f32, height: f32) {
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
    pub(super) fn draw_raster_layers<'a>(
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
    pub(super) fn render_interleaved_layers<'a>(
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

}
