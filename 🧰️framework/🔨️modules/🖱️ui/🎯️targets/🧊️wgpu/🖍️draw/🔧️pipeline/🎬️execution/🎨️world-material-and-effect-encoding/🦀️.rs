//! 🎨️ World material and effect encoding: the material, textured, grid, line, postprocess, scene clear, blur mip and glass scalar encoders. See [`super`].

use super::*;

impl UiPipelines {
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
        let (Some(vertex_buffer), Some(index_buffer)) = (&mesh.vertex_buffer, &mesh.index_buffer) else { return Ok(true) };
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
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer);
        pass.set_index_buffer(index_buffer.slice(..), gpu_device::IndexFormat::Uint32);
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

}
