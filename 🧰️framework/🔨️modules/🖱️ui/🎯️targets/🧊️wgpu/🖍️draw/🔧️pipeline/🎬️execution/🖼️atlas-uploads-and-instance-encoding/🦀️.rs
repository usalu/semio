//! 🖼️ Atlas uploads and instance encoding: the globals update, the glyph and icon atlas uploads, the prepared scene clear and the ui, vector, shadow and world instance encoders. See [`super`].

use super::*;

impl UiPipelines {
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

}
