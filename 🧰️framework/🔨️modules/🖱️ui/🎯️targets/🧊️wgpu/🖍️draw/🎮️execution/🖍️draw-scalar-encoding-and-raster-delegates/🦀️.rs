//! 🖍️ Draw scalar encoding and raster delegates: the prepared draw scalar encoder, the atlas uploads, the raster texture delegates and the size accessors. See [`super`].

use super::*;

// #region gpu
impl GpuContext {
    pub(super) fn encode_prepared_draw_scalar(&mut self, packet: &PreparedRenderPacket, cursor: DrawMeasureCursor, packet_overlay: bool, scissor: Option<semio_framework_ui_draw_contract::ScissorRect>) -> Result<(), String> {
        let draw = if packet_overlay { packet.overlay().ok_or_else(|| "prepared overlay owner was missing".to_string())? } else { packet.draw() };
        if let DrawMeasureCursor::PassBackdropSnapshot { pass } = cursor {
            let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared backdrop snapshot pass cursor was stale".to_string())?;
            if pass_owner.viewport_mask != crate::kernel_3d_scene::SceneViewportMask3d::Ellipse {
                return Err("prepared backdrop snapshot did not own an ellipse mask".to_string());
            }
            let composite = self.composite_color.as_mut().ok_or_else(|| "prepared backdrop snapshot composite target was missing".to_string())?;
            let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_world_backdrop_snapshot") });
            composite.copy_to_postprocess_background_scratch(&self.roots.device, &mut encoder);
            self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
            return Ok(());
        }
        let world_encoded = prepared_draw_scalar_uses_world_encoded_attachment(cursor);
        let color_view = self.composite_color.as_ref().map(|composite| if world_encoded { composite.world_encoded_view() } else { composite.view() }).ok_or_else(|| "prepared composite target was missing".to_string())?;
        let Some(depth) = self.depth_view.as_ref() else { return Err("prepared depth owner was missing".to_string()) };
        let width = self.logical_width;
        let height = self.logical_height;
        match cursor {
            DrawMeasureCursor::LayerUi { layer, item, overlay } => {
                let layer = draw.layers.get(layer).ok_or_else(|| "prepared UI layer cursor was stale".to_string())?;
                let instances = if overlay { &layer.overlay_ui_instances } else { &layer.ui_instances };
                let instance = instances.get(item).ok_or_else(|| "prepared UI scalar cursor was stale".to_string())?;
                let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_ui_scalar") });
                self.pipelines.encode_prepared_ui_scalar(&self.roots.device, &self.roots.queue, &mut encoder, color_view, depth, &mut self.frame_buffers, &self.raster_store, instance, None, scissor, width, height, packet.time_seconds()).map_err(str::to_owned)?;
                self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
            }
            DrawMeasureCursor::LayerVector { layer, item, overlay } if item % 3 == 2 => {
                let layer = draw.layers.get(layer).ok_or_else(|| "prepared vector layer cursor was stale".to_string())?;
                let vertices = if overlay { &layer.overlay_vector_vertices } else { &layer.vector_vertices };
                let start = item.checked_sub(2).ok_or_else(|| "prepared vector triangle cursor underflowed".to_string())?;
                let triangle = vertices.get(start..=item).ok_or_else(|| "prepared vector triangle cursor was stale".to_string())?;
                let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_vector_triangle") });
                self.pipelines.encode_prepared_vector_triangle(&self.roots.device, &self.roots.queue, &mut encoder, color_view, depth, &mut self.frame_buffers, triangle, scissor, width, height, packet.time_seconds()).map_err(str::to_owned)?;
                self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
            }
            DrawMeasureCursor::LayerRaster { layer, raster, overlay } => {
                let layer = draw.layers.get(layer).ok_or_else(|| "prepared raster layer cursor was stale".to_string())?;
                let instances = if overlay { &layer.overlay_raster_instances } else { &layer.raster_instances };
                let (key, instance) = instances.get(raster).ok_or_else(|| "prepared raster scalar cursor was stale".to_string())?;
                let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_raster_scalar") });
                self.pipelines
                    .encode_prepared_ui_scalar(&self.roots.device, &self.roots.queue, &mut encoder, color_view, depth, &mut self.frame_buffers, &self.raster_store, instance, Some(key), scissor, width, height, packet.time_seconds())
                    .map_err(str::to_owned)?;
                self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
            }
            DrawMeasureCursor::PassShadowBegin(pass) => {
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared shadow pass cursor was stale".to_string())?;
                let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_world_shadow_clear") });
                self.pipelines.encode_prepared_world_shadow_begin(&self.roots.device, &mut encoder, pass_owner);
                self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
            }
            DrawMeasureCursor::PassSceneClear { pass } => {
                let scissor = scissor.ok_or_else(|| "prepared world scene clear clip piece was missing".to_string())?;
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared world scene clear pass cursor was stale".to_string())?;
                let composite = self.composite_color.as_ref().ok_or_else(|| "prepared world scene clear composite target was missing".to_string())?;
                let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_world_scene_clear") });
                self.pipelines.encode_prepared_world_scene_clear(&self.roots.device, &self.roots.queue, &mut encoder, composite, pass_owner, scissor).map_err(str::to_owned)?;
                self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
            }
            DrawMeasureCursor::PassShadowInstance { pass, draw: draw_index, instance } => {
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared shadow pass cursor was stale".to_string())?;
                let draw_owner = pass_owner.shadow_draws.get(draw_index).ok_or_else(|| "prepared shadow draw cursor was stale".to_string())?;
                if !draw_owner.shadow_role.casts {
                    return Err("prepared shadow draw did not own the caster role".to_string());
                }
                let instance_owner = draw_owner.instances.get(instance).ok_or_else(|| "prepared shadow instance cursor was stale".to_string())?;
                let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_world_shadow_instance") });
                let drawn = self
                    .pipelines
                    .encode_prepared_world_shadow_instance(&self.roots.device, &self.roots.queue, &mut encoder, &mut self.frame_buffers, &self.mesh_store, pass_owner, &draw_owner.mesh_key, draw_owner.mesh_version, instance_owner)
                    .map_err(str::to_owned)?;
                if drawn {
                    self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
                }
            }
            DrawMeasureCursor::PassInstance { pass, draw: draw_index, instance, translucent } => {
                let scissor = scissor.ok_or_else(|| "prepared world instance clip piece was missing".to_string())?;
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared world pass cursor was stale".to_string())?;
                let draws = if translucent { &pass_owner.translucent_draws } else { &pass_owner.draws };
                let draw_owner = draws.get(draw_index).ok_or_else(|| "prepared world draw cursor was stale".to_string())?;
                let instance_owner = draw_owner.instances.get(instance).ok_or_else(|| "prepared world instance cursor was stale".to_string())?;
                let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_world_instance") });
                let drawn = self
                    .pipelines
                    .encode_prepared_world_instance(
                        &self.roots.device,
                        &self.roots.queue,
                        &mut encoder,
                        color_view,
                        depth,
                        &mut self.frame_buffers,
                        &self.mesh_store,
                        pass_owner,
                        &draw_owner.mesh_key,
                        draw_owner.mesh_version,
                        instance_owner,
                        draw_owner.shadow_role.receives,
                        translucent,
                        scissor,
                        width,
                        height,
                    )
                    .map_err(str::to_owned)?;
                if !drawn {
                    self.missing_world_mesh = Some((draw_owner.mesh_key.clone(), draw_owner.mesh_version));
                    return Ok(());
                }
                self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
            }
            DrawMeasureCursor::PassMaterialInstance { pass, draw: draw_index, instance, .. } => {
                let scissor = scissor.ok_or_else(|| "prepared world material clip piece was missing".to_string())?;
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared material pass cursor was stale".to_string())?;
                let draw_owner = pass_owner.material_draws.get(draw_index).ok_or_else(|| "prepared material draw cursor was stale".to_string())?;
                let instance_owner = draw_owner.instances.get(instance).ok_or_else(|| "prepared material instance cursor was stale".to_string())?;
                let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_world_material") });
                let drawn = self
                    .pipelines
                    .encode_prepared_world_material(&self.roots.device, &self.roots.queue, &mut encoder, color_view, depth, &mut self.frame_buffers, &self.mesh_store, &self.raster_store, pass_owner, draw_owner, instance_owner, scissor, width, height)
                    .map_err(str::to_owned)?;
                if !drawn {
                    self.missing_world_mesh = Some((draw_owner.mesh_key.clone(), draw_owner.mesh_version));
                    return Ok(());
                }
                self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
            }
            DrawMeasureCursor::PassTexturedInstance { pass, draw: draw_index, instance } => {
                let scissor = scissor.ok_or_else(|| "prepared world textured clip piece was missing".to_string())?;
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared textured pass cursor was stale".to_string())?;
                let draw_owner = pass_owner.textured_draws.get(draw_index).ok_or_else(|| "prepared textured draw cursor was stale".to_string())?;
                let instance_owner = draw_owner.instances.get(instance).ok_or_else(|| "prepared textured instance cursor was stale".to_string())?;
                let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_world_textured") });
                self.pipelines.encode_prepared_world_textured(&self.roots.device, &self.roots.queue, &mut encoder, color_view, depth, &mut self.frame_buffers, &self.raster_store, pass_owner, instance_owner, scissor).map_err(str::to_owned)?;
                self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
            }
            DrawMeasureCursor::PassGrid { pass } => {
                let scissor = scissor.ok_or_else(|| "prepared world grid clip piece was missing".to_string())?;
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared grid pass cursor was stale".to_string())?;
                let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_world_grid") });
                self.pipelines.encode_prepared_world_grid(&self.roots.device, &self.roots.queue, &mut encoder, color_view, depth, pass_owner, scissor).map_err(str::to_owned)?;
                self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
            }
            DrawMeasureCursor::PassLineVertex { pass, draw: draw_index, vertex } if vertex % 2 == 1 => {
                let scissor = scissor.ok_or_else(|| "prepared world line clip piece was missing".to_string())?;
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared line pass cursor was stale".to_string())?;
                let line_owner = pass_owner.line_draws.get(draw_index).ok_or_else(|| "prepared line draw cursor was stale".to_string())?;
                let start = vertex.checked_sub(1).ok_or_else(|| "prepared line segment cursor underflowed".to_string())?;
                let segment = line_owner.vertices.get(start..=vertex).ok_or_else(|| "prepared line segment cursor was stale".to_string())?;
                let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_world_line") });
                self.pipelines.encode_prepared_world_line(&self.roots.device, &self.roots.queue, &mut encoder, color_view, depth, &mut self.frame_buffers, pass_owner, segment, scissor).map_err(str::to_owned)?;
                self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
            }
            DrawMeasureCursor::PassPostprocess { pass } => {
                let scissor = scissor.ok_or_else(|| "prepared world postprocess clip piece was missing".to_string())?;
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared world postprocess pass cursor was stale".to_string())?;
                let composite = self.composite_color.as_ref().ok_or_else(|| "prepared world postprocess composite target was missing".to_string())?;
                let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_world_postprocess") });
                self.pipelines.encode_prepared_world_postprocess(&self.roots.device, &self.roots.queue, &mut encoder, composite, pass_owner, scissor).map_err(str::to_owned)?;
                self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
            }
            _ => {}
        }
        Ok(())
    }

    pub fn upload_font_atlas(&self, atlas: &FontAtlas) {
        self.pipelines.upload_glyph_atlas(&self.roots.queue, &atlas.pixels, atlas.width, atlas.height);
    }

    pub fn upload_icon_atlas(&self, atlas: &semio_framework_ui_draw_contract::IconAtlas) {
        self.pipelines.upload_icon_atlas(&self.roots.queue, &atlas.pixels, atlas.width, atlas.height);
    }

    pub(super) fn ensure_raster_texture_step(&mut self, key: &str, pixels: RasterUploadPixels<'_>, width: u32, height: u32, candidate: RasterTextureWitness, expected: RasterTextureWitness) -> Result<bool, &'static str> {
        self.raster_store.ensure_raster_step(
            &self.roots.device,
            &self.roots.queue,
            self.pipelines.globals_buffer(),
            &self.pipelines.glyph_view(),
            self.pipelines.glyph_sampler(),
            &self.pipelines.icon_view(),
            self.pipelines.icon_sampler(),
            key,
            pixels,
            width,
            height,
            candidate,
            expected,
        )
    }

    pub fn commit_presented_rasters_step(&mut self, witness: RasterTextureWitness) -> Result<bool, String> {
        self.raster_store.commit_presented_step(witness).map_err(|fault| {
            let (candidate, presenting, staged) = self.raster_store.presentation_witnesses();
            format!("{fault} (commit {witness:?}, candidate {candidate:?}, presenting {presenting:?}, staged {staged})")
        })
    }

    pub fn abort_presented_rasters_step(&mut self, witness: RasterTextureWitness) -> Result<bool, String> {
        self.raster_store.abort_presented_step(witness).map_err(str::to_owned)
    }

    pub fn close_raster_upload_step(&mut self) -> RasterTextureCleanupStep {
        self.raster_store.close_upload_step()
    }

    pub fn close_raster_table_step(&mut self) -> Result<bool, String> {
        self.raster_store.close_step().map_err(str::to_owned)
    }

    pub fn raster_table_terminal_is_empty(&self) -> bool {
        self.raster_store.terminal_is_empty()
    }

    pub fn dpr(&self) -> f32 {
        self.dpr
    }

    pub fn reserve_engine_texture(&mut self, key: &str, width: u32, height: u32,mip_levels:u32, identity: RasterContentIdentity, candidate: RasterTextureWitness, expected: RasterTextureWitness) -> Result<RasterTextureAdmission, String> {
        self.raster_store.reserve_engine_texture(key, width, height,mip_levels, identity, candidate, expected).map_err(str::to_owned)
    }

    pub fn begin_raster_ownership(&mut self, witness: RasterTextureWitness) -> Result<(), String> {
        self.raster_store.begin_candidate_ownership(witness).map_err(str::to_owned)
    }

    pub fn publish_raster_ownership(&mut self, witness: RasterTextureWitness, key: &str) -> Result<(), String> {
        self.raster_store.publish_candidate_ownership(witness, key).map_err(str::to_owned)
    }

    pub fn seal_raster_ownership(&mut self, witness: RasterTextureWitness) -> Result<(), String> {
        self.raster_store.seal_candidate_ownership(witness).map_err(str::to_owned)
    }

    pub fn prepare_raster_admission_step(&mut self, key: &str, width: u32, height: u32, identity: RasterContentIdentity, witness: RasterTextureWitness) -> Result<bool, String> {
        self.raster_store.prepare_admission_step(key, width, height,1, identity, witness).map_err(str::to_owned)
    }

    pub fn raster_content_is_reusable(&self, key: &str, identity: RasterContentIdentity, candidate: RasterTextureWitness, expected: RasterTextureWitness) -> Result<bool, String> {
        self.raster_store.content_is_reusable(key, identity, candidate, expected).map_err(str::to_owned)
    }

    pub fn release_previous_raster_ownership(&mut self) {
        self.raster_store.release_previous_ownership();
    }

    pub fn retire_unowned_raster_step(&mut self) -> Result<bool, String> {
        self.raster_store.retire_unowned_step().map_err(str::to_owned)
    }

    pub fn begin_exact_raster_close(&mut self, key: &str) -> Result<bool, String> {
        self.raster_store.begin_exact_close(key).map_err(str::to_owned)
    }

    pub fn close_exact_raster_step(&mut self, key: &str) -> Result<bool, String> {
        self.raster_store.close_exact_step(key).map_err(str::to_owned)
    }

    pub fn exact_raster_terminal_is_empty(&self, key: &str) -> bool {
        self.raster_store.exact_close_terminal_is_empty(key)
    }

    #[expect(clippy::result_large_err, reason = "The table returns its exact original admission on a full close slot.")]
    pub fn cancel_engine_texture_admission(&mut self, admission: RasterTextureAdmission) -> Result<(), RasterTextureCancelFault> {
        self.raster_store.cancel_engine_texture_admission(admission)
    }

    pub fn validate_engine_renderer_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), String> {
        self.raster_store.validate_engine_renderer_allocation(admission, expected).map_err(str::to_owned)
    }

    pub fn validate_engine_target_texture_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), String> {
        self.raster_store.validate_engine_target_texture_allocation(admission, expected).map_err(str::to_owned)
    }

    pub fn validate_engine_target_view_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), String> {
        self.raster_store.validate_engine_target_view_allocation(admission, expected).map_err(str::to_owned)
    }

    pub fn retain_engine_allocation_fault(&mut self, admission: RasterTextureAdmission, texture: Option<gpu_device::Texture>, view: Option<gpu_device::TextureView>) {
        self.raster_store.retain_engine_allocation_fault(admission, texture, view);
    }

    #[expect(clippy::result_large_err, reason = "GPU admission returns the exact resource and reservation owner without allocating during refusal.")]
    pub fn stage_engine_texture(&mut self, admission: RasterTextureAdmission, texture: gpu_device::Texture, view: gpu_device::TextureView, expected: RasterTextureWitness) -> Result<(), RasterTextureStageFault> {
        self.raster_store.stage_gpu_bind_group(&self.roots.device, self.pipelines.globals_buffer(), &self.pipelines.glyph_view(), self.pipelines.glyph_sampler(), admission, view, texture, expected)
    }

    /// 💻️ PHYSICAL surface width in device pixels.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// 📱️ PHYSICAL surface height in device pixels.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// 📏️ LOGICAL surface width — the unit every draw list, hit test and chrome constant speaks.
    pub fn logical_width(&self) -> f32 {
        self.logical_width
    }

    /// ↕️ LOGICAL surface height — the vertical twin of [`Self::logical_width`].
    pub fn logical_height(&self) -> f32 {
        self.logical_height
    }

    /// 📐️ Device pixels per logical pixel for this surface.
    pub fn scale_factor(&self) -> f32 {
        self.dpr
    }
}
// #endregion gpu
