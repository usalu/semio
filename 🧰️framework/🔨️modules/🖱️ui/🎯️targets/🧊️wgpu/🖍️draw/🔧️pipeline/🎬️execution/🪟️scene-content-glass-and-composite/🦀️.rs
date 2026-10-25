//! 🪟️ Scene content, glass and composite: the prepared scene blits, the scene content and glass foreground rendering, the swapchain composite, the blur chain and the overlay render. See [`super`].

use super::*;

impl UiPipelines {
    /// 🖼️ Encodes one fixed scene-to-surface packet after all command pages are resident.
    pub fn blit_prepared_scene<'a>(&'a self, device: &gpu_device::Device, encoder: &mut gpu_device::CommandEncoder, view: &'a gpu_device::TextureView, scene: &'a SceneColorTarget) {
        self.blit_sampled_color(device, encoder, view, scene.sample_view(), scene.sampler());
    }

    /// 📸️ Copies the accumulated composite into a backdrop snapshot or the terminal surface.
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
