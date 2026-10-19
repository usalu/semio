//! 🧷️ Prepared upload and present steps: the prepared upload step, the begin present and the prepared present advance. See [`super`].

use super::*;

// #region gpu
impl GpuContext {
    pub fn apply_prepared_upload_step(&mut self, packet: &PreparedRenderPacket, index: usize, candidate: RasterTextureWitness, expected: RasterTextureWitness) -> Result<bool, String> {
        let Some(upload) = packet.uploads().get(index) else { return Ok(true) };
        let complete = match upload {
            #[cfg(test)]
            PreparedRenderUpload::GlyphAtlas { pixels, width, height } => {
                self.atlas_upload = None;
                self.pipelines.upload_glyph_atlas(&self.roots.queue, pixels, *width, *height);
                true
            }
            #[cfg(test)]
            PreparedRenderUpload::IconAtlas { pixels, width, height } => {
                self.atlas_upload = None;
                self.pipelines.upload_icon_atlas(&self.roots.queue, pixels, *width, *height);
                true
            }
            PreparedRenderUpload::GlyphAtlasPages { pixels } => {
                let cursor = match self.atlas_upload {
                    Some(cursor) if cursor.generation == packet.preview_generation() && cursor.upload == index => cursor,
                    _ => PreparedAtlasUploadCursor { generation: packet.preview_generation(), upload: index, page: 0 },
                };
                let Some((bytes, start_row, rows)) = pixels.page(cursor.page) else {
                    self.atlas_upload = None;
                    return Ok(true);
                };
                self.pipelines.upload_glyph_atlas_page(&self.roots.queue, bytes, pixels.width(), start_row, rows);
                let page = cursor.page.checked_add(1).ok_or_else(|| "glyph atlas page cursor exhausted".to_string())?;
                if page == pixels.len() {
                    self.atlas_upload = None;
                    true
                } else {
                    self.atlas_upload = Some(PreparedAtlasUploadCursor { page, ..cursor });
                    false
                }
            }
            PreparedRenderUpload::IconAtlasPages { pixels } => {
                let cursor = match self.atlas_upload {
                    Some(cursor) if cursor.generation == packet.preview_generation() && cursor.upload == index => cursor,
                    _ => PreparedAtlasUploadCursor { generation: packet.preview_generation(), upload: index, page: 0 },
                };
                let Some((bytes, start_row, rows)) = pixels.page(cursor.page) else {
                    self.atlas_upload = None;
                    return Ok(true);
                };
                self.pipelines.upload_icon_atlas_page(&self.roots.queue, bytes, pixels.width(), start_row, rows);
                let page = cursor.page.checked_add(1).ok_or_else(|| "icon atlas page cursor exhausted".to_string())?;
                if page == pixels.len() {
                    self.atlas_upload = None;
                    true
                } else {
                    self.atlas_upload = Some(PreparedAtlasUploadCursor { page, ..cursor });
                    false
                }
            }
            #[cfg(test)]
            PreparedRenderUpload::Raster { key, pixels, width, height } => {
                let identity = RasterContentIdentity::test_pixels(*width, *height, pixels).ok_or_else(|| "raster test identity overflowed".to_string())?;
                if !self.raster_store.prepare_admission_step(key, *width, *height,1, identity, candidate).map_err(str::to_owned)? {
                    return Ok(false);
                }
                self.ensure_raster_texture_step(key, RasterUploadPixels::Contiguous(pixels), *width, *height, candidate, expected).map_err(str::to_owned)?
            }
            PreparedRenderUpload::RasterPages { key, pixels } => {
                if pixels.frame_generation() != packet.preview_generation() {
                    return Err("prepared raster producer generation is stale".into());
                }
                if !self.raster_store.prepare_admission_step(key, pixels.width(), pixels.height(),1, pixels.content_identity(), candidate).map_err(str::to_owned)? {
                    return Ok(false);
                }
                self.ensure_raster_texture_step(key, RasterUploadPixels::Pages(pixels), pixels.width(), pixels.height(), candidate, expected).map_err(str::to_owned)?
            }
            PreparedRenderUpload::SceneRaster { key, lease } => {
                let descriptor = lease.identity().descriptor();
                if !self.raster_store.prepare_admission_step(key, descriptor.width, descriptor.height,RasterUploadPixels::Scene(lease).mip_levels(descriptor.width,descriptor.height), lease.identity().content(), candidate).map_err(str::to_owned)? {
                    return Ok(false);
                }
                self.ensure_raster_texture_step(key, RasterUploadPixels::Scene(lease), descriptor.width, descriptor.height, candidate, expected).map_err(str::to_owned)?
            }
            PreparedRenderUpload::Mesh { key, version, lease } => self.ensure_mesh_step(key, *version, *lease).map_err(str::to_owned)?,
        };
        Ok(complete)
    }

    pub fn begin_prepared_present(&mut self, packet: &PreparedRenderPacket, witness: RasterTextureWitness) -> Result<PreparedGpuPresentCursor, String> {
        let cursor = PreparedGpuPresentCursor::begin(packet.scene_revision(), packet.preview_generation()).ok_or_else(|| "prepared GPU cursor generation or abandonment admission was exhausted".to_string())?;
        self.raster_store.begin_presenting(witness).map_err(str::to_owned)?;
        self.accepted_offscreen = None;
        Ok(cursor)
    }

    /// 🚦 Advances one fixed command scalar or one bounded platform submission opportunity.
    ///
    /// 🖼️ **The swapchain is touched by exactly one phase.** `Present` acquires the surface texture,
    /// views it, blits the finished [`PreparedCompositeTarget`] onto it, submits and presents inside
    /// ONE call, because a browser expires the canvas's current texture at the end of the task that
    /// obtained it and presents whatever it held then. Every earlier phase composites offscreen, so
    /// a host pump may yield between any two of them without ever publishing an empty frame.
    ///
    /// ⚖️ A single over-ceiling wall sample is a MEASUREMENT, not a verdict — the ratified law of
    /// [`semio_framework_job::SUSTAINED_OVERRUN_QUARANTINE_STEPS`]. The first frame of a cold surface
    /// pays the platform's pipeline warm-up inside one opportunity (measured on Apple metal-3 through
    /// a browser `OffscreenCanvas`: `ClearScene` 2 601 µs against the 2 000 µs ceiling), and failing
    /// that one sample quarantined the whole surface before it had ever presented
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Only a RUN of consecutive over-ceiling
    /// opportunities is terminal; any under-ceiling opportunity clears the run.
    pub fn prepared_present_step(&mut self, packet: &PreparedRenderPacket, cursor: &mut PreparedGpuPresentCursor) -> Result<bool, String> {
        if !cursor.matches(packet) || cursor.phase == PreparedGpuPresentPhase::Closing {
            return Err("prepared GPU cursor was stale, uncredited, or closing".to_string());
        }
        if cursor.phase == PreparedGpuPresentPhase::Complete {
            return Ok(true);
        }
        let command = packet.command_pages().get(cursor.command).map(|command| (command.kind().code(), command.draw_cursor()));
        measure_prepared_gpu_opportunity(cursor, command, semio_framework_job::default_now_us, |cursor| {
            let complete = self.advance_prepared_present(packet, cursor)?;
            if !cursor.matches(packet) {
                return Err("prepared GPU cursor became stale after a platform call".to_string());
            }
            Ok(complete)
        })
    }

    fn advance_prepared_present(&mut self, packet: &PreparedRenderPacket, cursor: &mut PreparedGpuPresentCursor) -> Result<bool, String> {
        match cursor.phase {
            PreparedGpuPresentPhase::EnsureTarget => {
                self.ensure_prepared_targets();
                cursor.phase = PreparedGpuPresentPhase::ClearScene;
            }
            PreparedGpuPresentPhase::ClearScene => {
                let Some(scene) = self.scene_color.as_ref() else { return Err("prepared scene target was missing".to_string()) };
                let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_scene_packet") });
                let clear = if self.surface.is_none() { gpu_device::Color::TRANSPARENT } else { gpu_device::Color { r: 0.05, g: 0.05, b: 0.06, a: 1.0 } };
                self.pipelines.clear_prepared_scene(&mut encoder, scene, self.depth_view.as_ref(), clear);
                self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
                cursor.phase = PreparedGpuPresentPhase::InitializeComposite;
            }
            PreparedGpuPresentPhase::InitializeComposite => {
                let Some(scene) = self.scene_color.as_ref() else { return Err("prepared scene target was missing".to_string()) };
                let Some(composite) = self.composite_color.as_ref() else { return Err("prepared composite target was missing".to_string()) };
                let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_composite_packet") });
                self.pipelines.blit_prepared_scene(&self.roots.device, &mut encoder, composite.view(), scene);
                self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
                cursor.phase = PreparedGpuPresentPhase::Commands;
            }
            PreparedGpuPresentPhase::Commands => {
                let Some(command) = packet.command_pages().get(cursor.command) else {
                    cursor.phase = PreparedGpuPresentPhase::Present;
                    return Ok(false);
                };
                if cursor.clip_piece == 0 {
                    let source = u32::try_from(command.source()).map_err(|_| "prepared command source exceeded fixed GPU record".to_string())?;
                    let digest = command.digest();
                    let record = [command.kind().code(), source, digest as u32, (digest >> 32) as u32];
                    let offset = u64::try_from(cursor.command).ok().and_then(|value| value.checked_mul(16)).ok_or_else(|| "prepared command buffer offset exhausted".to_string())?;
                    self.roots.queue.write_buffer(&self.roots.commands, offset, bytemuck::cast_slice(&record));
                }
                let Some(draw_cursor) = command.draw_cursor() else {
                    cursor.command = cursor.command.checked_add(1).ok_or_else(|| "prepared command cursor exhausted".to_string())?;
                    cursor.clip_piece = 0;
                    return Ok(false);
                };
                let draw = if command.packet_overlay() { packet.overlay().ok_or_else(|| "prepared command overlay owner was missing".to_string())? } else { packet.draw() };
                match prepared_command_clip_piece(draw, draw_cursor, cursor.clip_piece, self.logical_width, self.logical_height)? {
                    PreparedCommandClipPiece::Unclipped => {
                        self.encode_prepared_draw_scalar(packet, draw_cursor, command.packet_overlay(), None)?;
                        cursor.command = cursor.command.checked_add(1).ok_or_else(|| "prepared command cursor exhausted".to_string())?;
                        cursor.clip_piece = 0;
                    }
                    PreparedCommandClipPiece::Empty => {
                        cursor.clip_piece = cursor.clip_piece.checked_add(1).ok_or_else(|| "prepared clip-piece cursor exhausted".to_string())?;
                    }
                    PreparedCommandClipPiece::Scissor(_scissor) if matches!(draw_cursor, DrawMeasureCursor::Glass(_)) => {
                        cursor.phase = PreparedGpuPresentPhase::SnapshotBackdrop;
                    }
                    PreparedCommandClipPiece::Scissor(scissor) => {
                        self.encode_prepared_draw_scalar(packet, draw_cursor, command.packet_overlay(), Some(scissor))?;
                        cursor.clip_piece = cursor.clip_piece.checked_add(1).ok_or_else(|| "prepared clip-piece cursor exhausted".to_string())?;
                    }
                    PreparedCommandClipPiece::Complete => {
                        cursor.command = cursor.command.checked_add(1).ok_or_else(|| "prepared command cursor exhausted".to_string())?;
                        cursor.clip_piece = 0;
                    }
                }
            }
            PreparedGpuPresentPhase::SnapshotBackdrop => {
                let glass = prepared_glass_command(packet, cursor.command)?;
                let Some(scene) = self.scene_color.as_ref() else { return Err("prepared scene target was missing".to_string()) };
                let Some(composite) = self.composite_color.as_ref() else { return Err("prepared composite target was missing".to_string()) };
                let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_glass_backdrop") });
                self.pipelines.blit_prepared_composite(&self.roots.device, &mut encoder, scene.mip_view(0), composite);
                self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
                cursor.blur_mip = 1;
                cursor.phase = if glass.blur_px > 0.0 { PreparedGpuPresentPhase::BlurScene } else { PreparedGpuPresentPhase::CompositeGlass };
            }
            PreparedGpuPresentPhase::BlurScene => {
                if cursor.blur_mip >= SCENE_MIP_LEVELS {
                    cursor.phase = PreparedGpuPresentPhase::CompositeGlass;
                    return Ok(false);
                }
                let Some(scene) = self.scene_color.as_ref() else { return Err("prepared scene target was missing".to_string()) };
                self.pipelines.encode_prepared_blur_mip(&self.roots.device, &self.roots.queue, scene, cursor.blur_mip).map_err(str::to_owned)?;
                cursor.blur_mip = cursor.blur_mip.checked_add(1).ok_or_else(|| "prepared blur cursor exhausted".to_string())?;
            }
            PreparedGpuPresentPhase::CompositeGlass => {
                let glass = prepared_glass_command(packet, cursor.command)?;
                let command = packet.command_pages().get(cursor.command).ok_or_else(|| "prepared glass command was missing".to_string())?;
                let draw = if command.packet_overlay() { packet.overlay().ok_or_else(|| "prepared glass overlay owner was missing".to_string())? } else { packet.draw() };
                let Some(draw_cursor) = command.draw_cursor() else { return Err("prepared glass command cursor was missing".to_string()) };
                let piece = prepared_command_clip_piece(draw, draw_cursor, cursor.clip_piece, self.logical_width, self.logical_height)?;
                if matches!(piece, PreparedCommandClipPiece::Empty) {
                    cursor.clip_piece = cursor.clip_piece.checked_add(1).ok_or_else(|| "prepared glass clip-piece cursor exhausted".to_string())?;
                    return Ok(false);
                }
                if matches!(piece, PreparedCommandClipPiece::Complete) {
                    cursor.command = cursor.command.checked_add(1).ok_or_else(|| "prepared command cursor exhausted".to_string())?;
                    cursor.clip_piece = 0;
                    cursor.phase = PreparedGpuPresentPhase::Commands;
                    return Ok(false);
                }
                let PreparedCommandClipPiece::Scissor(scissor) = piece else { return Err("prepared glass command did not own a clipped color scalar".to_string()) };
                let Some(scene) = self.scene_color.as_ref() else { return Err("prepared scene target was missing".to_string()) };
                let Some(composite) = self.composite_color.as_ref() else { return Err("prepared composite target was missing".to_string()) };
                let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_glass_scalar") });
                self.pipelines.encode_prepared_glass_scalar(&self.roots.device, &self.roots.queue, &mut encoder, composite.view(), scene, &mut self.frame_buffers, glass, scissor, self.logical_width, self.logical_height).map_err(str::to_owned)?;
                self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
                cursor.clip_piece = cursor.clip_piece.checked_add(1).ok_or_else(|| "prepared glass clip-piece cursor exhausted".to_string())?;
            }
            PreparedGpuPresentPhase::Present => {
                let Some(composite) = self.composite_color.as_ref() else { return Err("prepared composite target was missing".to_string()) };
                let Some(surface) = self.surface.as_ref() else {
                    cursor.phase = PreparedGpuPresentPhase::Complete;
                    self.accepted_offscreen = Some((cursor.scene_revision, cursor.preview_generation));
                    return Ok(true);
                };
                cursor.frame = Some(match surface.get_current_texture() {
                    gpu_device::CurrentSurfaceTexture::Success(frame) | gpu_device::CurrentSurfaceTexture::Suboptimal(frame) => frame,
                    outcome => return Err(format!("prepared surface acquisition: {outcome:?}")),
                });
                let Some(frame) = cursor.frame.as_ref() else { return Err("prepared surface owner was missing".to_string()) };
                cursor.view = Some(frame.texture.create_view(&gpu_device::TextureViewDescriptor { format: Some(self.color_target_format), ..Default::default() }));
                let Some(view) = cursor.view.as_ref() else { return Err("prepared surface view was missing".to_string()) };
                let mut encoder = self.roots.device.create_command_encoder(&gpu_device::CommandEncoderDescriptor { label: Some("prepared_surface_present") });
                self.pipelines.blit_prepared_composite(&self.roots.device, &mut encoder, view, composite);
                self.roots.queue.submit(encoder.finish()).expect("same-session recorded command");
                cursor.view = None;
                let Some(frame) = cursor.frame.take() else { return Err("prepared surface owner was missing before present".to_string()) };
                frame.present();
                cursor.phase = PreparedGpuPresentPhase::Complete;
            }
            PreparedGpuPresentPhase::Complete => return Ok(true),
            PreparedGpuPresentPhase::Closing => return Err("prepared GPU cursor was closing".to_string()),
        }
        Ok(cursor.phase == PreparedGpuPresentPhase::Complete)
    }

}
// #endregion gpu
