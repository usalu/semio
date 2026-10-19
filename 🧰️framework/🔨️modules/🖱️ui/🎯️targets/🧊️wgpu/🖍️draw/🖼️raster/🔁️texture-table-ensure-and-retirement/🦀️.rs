//! 🔁️ Texture table ensure and retirement: the ensure step, the bind group staging, the presenting, retirement and commit steps and the exact close steps. See [`super`].

use super::*;

impl RasterTextureTable {
    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    pub fn ensure_raster_step(
        &mut self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        globals_buffer: &gpu_device::Buffer,
        glyph_view: &gpu_device::TextureView,
        glyph_sampler: &gpu_device::Sampler,
        _icon_view: &gpu_device::TextureView,
        _icon_sampler: &gpu_device::Sampler,
        key: &str,
        pixels: RasterUploadPixels<'_>,
        width: u32,
        height: u32,
        candidate: RasterTextureWitness,
        expected: RasterTextureWitness,
    ) -> Result<bool, &'static str> {
        if self.closing || self.retirement.is_some() {
            return Err("raster texture table is closing or retiring");
        }
        let row_bytes = usize::try_from(width).ok().and_then(|value| value.checked_mul(4)).ok_or("raster row byte credits overflowed")?;
        let expected_bytes = row_bytes.checked_mul(usize::try_from(height).map_err(|_| "raster height exceeded fixed credits")?).ok_or("raster byte credits overflowed")?;
        let transfer_bytes = pixels.transfer_bytes();
        if width == 0 || height == 0 || row_bytes > transfer_bytes || pixels.len() != expected_bytes || !pixels.dimensions_match(width, height) {
            return Err("raster upload exceeded fixed page or byte credits");
        }
        let content_identity = pixels.content_identity(width, height).ok_or("raster content identity was unavailable")?;
        if self.content_is_reusable(key, content_identity, candidate, expected)? {
            if let Some(live) = self.live.get_mut(key).filter(|entry| entry.content_identity == content_identity && entry.gpu_resident.is_none()) {
                if let Some(release) = live.cpu_release.take().or_else(|| pixels.release_witness()) {
                    match release.commit_gpu(key) {
                        Ok(resident) => live.gpu_resident = Some(resident),
                        Err(_release) => {}
                    }
                }
            }
            pixels.release_reused();
            return Ok(true);
        }
        let mip_levels=pixels.mip_levels(width,height);
        if self.upload.is_none() {
            let admission = self.reserve_engine_texture(key, width, height,mip_levels, content_identity, candidate, expected)?;
            self.upload = Some(RasterTextureUploadCursor {mip_levels,mip:1,mip_row:0, admission: Some(admission), row: 0, texture: None, view: None, bind_group: None, allocation_claim: None, cpu_release: pixels.release_witness() });
            let allocation_claim = {
                let admission = self.upload.as_ref().and_then(|cursor| cursor.admission.as_ref()).expect("retained raster texture admission");
                self.claim_texture_allocation(admission, expected)?
            };
            self.upload.as_mut().expect("retained raster texture claim").allocation_claim = Some(allocation_claim);
            let texture = device.create_texture(&gpu_device::TextureDescriptor {
                label: Some("raster_texture"),
                size: gpu_device::Extent3d { width, height, depth_or_array_layers: 1 },
                mip_level_count: mip_levels,
                sample_count: 1,
                dimension: gpu_device::TextureDimension::D2,
                format: pixels.texture_format(),
                usage: gpu_device::TextureUsages::TEXTURE_BINDING | gpu_device::TextureUsages::COPY_DST | gpu_device::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
            self.upload.as_mut().expect("retained raster texture upload").texture = Some(texture);
            let allocation_claim = {
                let admission = self.upload.as_ref().and_then(|cursor| cursor.admission.as_ref()).expect("retained raster view admission");
                self.claim_view_allocation(admission, expected)?
            };
            self.upload.as_mut().expect("retained raster view claim").allocation_claim = Some(allocation_claim);
            let view = self.upload.as_ref().and_then(|cursor| cursor.texture.as_ref()).expect("retained raster texture owner").create_view(&gpu_device::TextureViewDescriptor::default());
            self.upload.as_mut().expect("retained raster view upload").view = Some(view);
            let allocation_claim = {
                let admission = self.upload.as_ref().and_then(|cursor| cursor.admission.as_ref()).expect("retained raster bind-group admission");
                self.claim_bind_group_allocation(admission, expected)?
            };
            self.upload.as_mut().expect("retained raster bind-group claim").allocation_claim = Some(allocation_claim);
            let view = self.upload.as_ref().and_then(|cursor| cursor.view.as_ref()).expect("retained raster view owner");
            let bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
                label: Some("raster_texture_bind_group"),
                layout: &self.layout,
                entries: &[
                    gpu_device::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                    gpu_device::BindGroupEntry { binding: 1, resource: gpu_device::BindingResource::TextureView(glyph_view) },
                    gpu_device::BindGroupEntry { binding: 2, resource: gpu_device::BindingResource::Sampler(glyph_sampler) },
                    gpu_device::BindGroupEntry { binding: 3, resource: gpu_device::BindingResource::TextureView(view) },
                    gpu_device::BindGroupEntry { binding: 4, resource: gpu_device::BindingResource::Sampler(&self.sampler) },
                ],
            });
            let upload = self.upload.as_mut().expect("retained raster bind-group upload");
            upload.bind_group = Some(bind_group);
        }
        let cursor = self.upload.as_mut().expect("raster upload cursor initialized above");
        let admission = cursor.admission.as_ref().ok_or("raster upload admission was retired")?;
        if admission.key.as_str() != key || admission.witness != candidate || admission.content_identity != content_identity || admission.width != width || admission.height != height || candidate != expected {
            return Err("raster upload authority was occupied by another generation");
        }
        if cursor.row < height {
            let rows = (transfer_bytes / row_bytes).max(1).min(usize::try_from(height - cursor.row).unwrap_or(usize::MAX));
            let start = usize::try_from(cursor.row).unwrap_or(usize::MAX).saturating_mul(row_bytes);
            let end = start.saturating_add(rows.saturating_mul(row_bytes));
            let texture = cursor.texture.as_ref().ok_or("raster upload texture was retired")?;
            let page_rows = pixels
                .with_rows(cursor.row, start, end, rows as u32, |page, page_rows| {
                    queue.write_texture(
                        gpu_device::TexelCopyTextureInfo { texture, mip_level: 0, origin: gpu_device::Origin3d { x: 0, y: cursor.row, z: 0 }, aspect: gpu_device::TextureAspect::All },
                        page,
                        gpu_device::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width * 4), rows_per_image: Some(page_rows) },
                        gpu_device::Extent3d { width, height: page_rows, depth_or_array_layers: 1 },
                    );
                    page_rows
                })
                .ok_or("raster prepared page ownership was incomplete")?;
            cursor.row += page_rows;
            return Ok(false);
        }
        if cursor.mip<cursor.mip_levels {
            let texture=cursor.texture.as_ref().ok_or("raster mip texture was retired")?;let width=(width>>cursor.mip).max(1);let height=(height>>cursor.mip).max(1);
            let view=|level|texture.create_view(&gpu_device::TextureViewDescriptor{base_mip_level:level,mip_level_count:Some(1),..Default::default()});let source=view(cursor.mip-1);let target=view(cursor.mip);
            let group=device.create_bind_group(&gpu_device::BindGroupDescriptor{label:Some("raster_mip_bind_group"),layout:&self.mip_layout,entries:&[gpu_device::BindGroupEntry{binding:0,resource:gpu_device::BindingResource::TextureView(&source)},gpu_device::BindGroupEntry{binding:1,resource:gpu_device::BindingResource::Sampler(&self.sampler)}]});
            let mut encoder=device.create_command_encoder(&gpu_device::CommandEncoderDescriptor{label:Some("raster_mip_encoder")});let rows=(height-cursor.mip_row).min(8);
            {let mut pass=encoder.begin_render_pass(&gpu_device::RenderPassDescriptor{label:Some("raster_mip_pass"),color_attachments:&[Some(gpu_device::RenderPassColorAttachment{view:&target,resolve_target:None,ops:gpu_device::Operations{load:gpu_device::LoadOp::Load,store:gpu_device::StoreOp::Store},depth_slice:None})],depth_stencil_attachment:None,timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});pass.set_pipeline(&self.mip_pipelines[usize::from(pixels.texture_format()==gpu_device::TextureFormat::Rgba8UnormSrgb)]);pass.set_bind_group(0,&group,&[]);pass.set_scissor_rect(0,cursor.mip_row,width,rows);pass.draw(0..3,0..1);}
            queue.submit(encoder.finish()).expect("same-session recorded command");cursor.mip_row+=rows;if cursor.mip_row==height{cursor.mip_row=0;cursor.mip+=1;}return Ok(false);
        }
        let mut cursor = self.upload.take().expect("completed raster upload cursor");
        let admission = cursor.admission.take().expect("completed raster upload admission");
        let allocation_claim = cursor.allocation_claim.take().expect("completed raster allocation claim");
        let value = RasterTexture {
            texture: cursor.texture.take().expect("completed raster upload texture"),
            view: cursor.view.take().expect("completed raster upload view"),
            bind_group: cursor.bind_group.take().expect("completed raster upload bind group"),
            width,
            height,
        };
        let cpu_release = cursor.cpu_release.take();
        if let Err((fault, admission, value, cpu_release)) = self.stage_claimed_texture(admission, value, allocation_claim, cpu_release) {
            self.upload_close = Some(RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor {mip_levels:1,mip:1,mip_row:0,
                admission: Some(admission),
                row: height,
                texture: Some(value.texture),
                view: Some(value.view),
                bind_group: Some(value.bind_group),
                allocation_claim: Some(allocation_claim),
                cpu_release,
            }));
            return Err(fault);
        }
        Ok(true)
    }

    pub fn get(&self, key: &str) -> Option<&RasterTexture> {
        if let Some(witness) = self.witnesses.presenting() {
            if let Some(entry) = self.staged.get(key).filter(|entry| entry.witness == witness) {
                return Some(&entry.value);
            }
        }
        self.live.get(key).map(|entry| &entry.value)
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    #[expect(clippy::result_large_err, reason = "GPU admission returns the exact resource and reservation owner without allocating during refusal.")]
    pub fn stage_gpu_bind_group(
        &mut self,
        device: &gpu_device::Device,
        globals_buffer: &gpu_device::Buffer,
        glyph_view: &gpu_device::TextureView,
        glyph_sampler: &gpu_device::Sampler,
        admission: RasterTextureAdmission,
        raster_view: gpu_device::TextureView,
        texture: gpu_device::Texture,
        expected: RasterTextureWitness,
    ) -> Result<(), RasterTextureStageFault> {
        let allocation_claim = match self.claim_bind_group_allocation(&admission, expected) {
            Ok(claim) => claim,
            Err(fault) => return Err(RasterTextureStageFault::Returned { fault, admission, texture, view: raster_view }),
        };
        let bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
            label: Some("raster_bind_group"),
            layout: &self.layout,
            entries: &[
                gpu_device::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                gpu_device::BindGroupEntry { binding: 1, resource: gpu_device::BindingResource::TextureView(glyph_view) },
                gpu_device::BindGroupEntry { binding: 2, resource: gpu_device::BindingResource::Sampler(glyph_sampler) },
                gpu_device::BindGroupEntry { binding: 3, resource: gpu_device::BindingResource::TextureView(&raster_view) },
                gpu_device::BindGroupEntry { binding: 4, resource: gpu_device::BindingResource::Sampler(&self.sampler) },
            ],
        });
        let width = admission.width;
        let height = admission.height;
        let value = RasterTexture { texture, view: raster_view, bind_group, width, height };
        if let Err((fault, admission, value, cpu_release)) = self.stage_claimed_texture(admission, value, allocation_claim, None) {
            assert!(self.upload.is_none() && self.upload_close.is_none(), "matching raster publication fault must own its reserved close slot");
            self.upload_close = Some(RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor {mip_levels:1,mip:1,mip_row:0,
                admission: Some(admission),
                row: height,
                texture: Some(value.texture),
                view: Some(value.view),
                bind_group: Some(value.bind_group),
                allocation_claim: Some(allocation_claim),
                cpu_release,
            }));
            return Err(RasterTextureStageFault::Retained(fault));
        }
        Ok(())
    }

    pub fn begin_presenting(&mut self, witness: RasterTextureWitness) -> Result<bool, &'static str> {
        if self.closing || self.retirement.is_some() || self.upload.is_some() {
            return Err("raster candidate was not terminal before presentation");
        }
        if !self.residency.candidate_is_sealed(witness) {
            return Err("raster keep-set candidate was not sealed before presentation");
        }
        if self.staged.is_empty() {
            return Ok(false);
        }
        self.witnesses.arm_presentation(witness)?;
        Ok(true)
    }

    /// 🧾️ Admits a retirement ONCE, then hands the witness slots to its cursor.
    ///
    /// ⚖️ The candidate/presentation witnesses are an ADMISSION fact, not a per-step invariant: the
    /// cursor retires those very slots one field at a time, so re-reading them on every later step
    /// compares a half-consumed slot against the operation that owns it and refuses its own progress
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END — `raster commit candidate witness was stale`).
    /// An EMPTY presentation is admitted: an operation that reserved a raster the surface never
    /// presented owes a candidate retirement and nothing else — "not yet", never "stale".
    pub(super) fn begin_retirement(&mut self, mode: RasterTextureRetirementMode) -> Result<(), &'static str> {
        if let Some(cursor) = self.retirement.as_ref() {
            return (cursor.mode == mode).then_some(()).ok_or("raster retirement authority was occupied");
        }
        self.retirement = Some(RasterTextureRetirementCursor { mode, scan: 0, owner: None });
        Ok(())
    }

    fn retirement_step(&mut self) -> Result<bool, &'static str> {
        let Some(cursor) = self.retirement.as_mut() else { return Ok(true) };
        if let Some(owner) = cursor.owner.as_mut() {
            return match owner.step() {
                RasterTextureCleanupStep::Pending { .. } => Ok(false),
                RasterTextureCleanupStep::Blocked(fault) => Err(fault),
                RasterTextureCleanupStep::Complete => {
                    assert!(owner.terminal_is_empty(), "completed raster retirement owner must be terminal-empty");
                    cursor.owner = None;
                    Ok(false)
                }
            };
        }
        if cursor.scan >= RASTER_TEXTURE_TABLE_CAPACITY {
            if !matches!(cursor.mode, RasterTextureRetirementMode::Unowned | RasterTextureRetirementMode::Exact(_)) && !self.witnesses.retire_step() {
                return Ok(false);
            }
            self.retirement = None;
            return Ok(true);
        }
        let index = cursor.scan;
        cursor.scan += 1;
        match cursor.mode {
            RasterTextureRetirementMode::Abort(witness) => {
                let selected = self.staged.slots[index].as_ref().is_some_and(|entry| entry.witness == witness);
                if selected {
                    let entry = self.staged.take(index).expect("selected staged raster");
                    cursor.owner = Some(RasterTextureRetirementOwner::new(entry));
                }
            }
            RasterTextureRetirementMode::Commit(witness) => {
                let selected = self.staged.slots[index].as_ref().is_some_and(|entry| entry.witness == witness);
                if selected {
                    let entry = self.staged.take(index).expect("selected staged raster");
                    let key = entry.key;
                    match self.live.insert(entry) {
                        Ok(previous) => {
                            let live = self.live.get_mut(key.as_str()).expect("committed raster remains live under its exact key");
                            if let Some(release) = live.cpu_release.take() {
                                match release.commit_gpu(key.as_str()) {
                                    Ok(resident) => live.gpu_resident = Some(resident),
                                    Err(_release) => {}
                                }
                            }
                            if let Some(previous) = previous {
                                cursor.owner = Some(RasterTextureRetirementOwner::new(previous));
                            }
                        }
                        Err(entry) => {
                            if let Err(entry) = self.staged.insert_vacant(index, entry) {
                                cursor.owner = Some(RasterTextureRetirementOwner::new(entry));
                            }
                            return Err("raster live table capacity exhausted");
                        }
                    }
                }
            }
            RasterTextureRetirementMode::Unowned => {
                if let Some(entry) = take_unowned_live_entry(&mut self.live, &self.staged, &self.residency, index) {
                    cursor.owner = Some(RasterTextureRetirementOwner::new(entry));
                }
            }
            RasterTextureRetirementMode::Exact(key) => {
                let staged = self.staged.slots[index].as_ref().is_some_and(|entry| entry.key == key).then(|| self.staged.take(index)).flatten();
                if staged.is_some() && self.live.slots[index].as_ref().is_some_and(|entry| entry.key == key) {
                    cursor.scan = index;
                }
                let live = if staged.is_none() && self.live.slots[index].as_ref().is_some_and(|entry| entry.key == key) { self.live.take(index) } else { None };
                if let Some(entry) = staged.or(live) {
                    cursor.owner = Some(RasterTextureRetirementOwner::new(entry));
                }
            }
            RasterTextureRetirementMode::Close => {
                let staged = self.staged.take(index);
                if staged.is_some() && self.live.slots[index].is_some() {
                    cursor.scan = index;
                }
                if let Some(entry) = staged.or_else(|| self.live.take(index)) {
                    cursor.owner = Some(RasterTextureRetirementOwner::new(entry));
                }
            }
        }
        Ok(false)
    }

    /// 🩺️ `(candidate, presenting, staged items)` — the three facts a stale-witness refusal is about,
    /// so a fault banner can name what actually moved instead of only that something did.
    pub fn presentation_witnesses(&self) -> (Option<RasterTextureWitness>, Option<RasterTextureWitness>, usize) {
        (self.witnesses.candidate(), self.witnesses.presenting(), self.staged.len)
    }

    /// 🪶 Commits the rasters this operation staged, one bounded step at a time.
    ///
    /// ⚖️ An operation that admitted NO raster owes nothing: an engine surface whose first raster has
    /// not been reserved yet is "not yet", never "stale". Once the retirement is admitted the cursor
    /// owns the witness slots — see [`RasterTextureTable::begin_retirement`].
    pub fn commit_presented_step(&mut self, witness: RasterTextureWitness) -> Result<bool, &'static str> {
        if self.witnesses.begin_retirement(witness)? == RasterWitnessAdmission::Nothing {
            self.residency.commit(witness)?;
            return Ok(true);
        }
        self.begin_retirement(RasterTextureRetirementMode::Commit(witness))?;
        if !self.retirement_step()? {
            return Ok(false);
        }
        self.residency.commit(witness)?;
        Ok(true)
    }

    pub fn abort_presented_step(&mut self, witness: RasterTextureWitness) -> Result<bool, &'static str> {
        if self.witnesses.begin_retirement(witness)? == RasterWitnessAdmission::Nothing && self.staged.is_empty() && self.upload.is_none() {
            self.residency.abort(witness)?;
            return Ok(true);
        }
        self.begin_retirement(RasterTextureRetirementMode::Abort(witness))?;
        if !self.retirement_step()? {
            return Ok(false);
        }
        self.residency.abort(witness)?;
        Ok(true)
    }

    pub fn release_previous_ownership(&mut self) {
        self.residency.release_previous();
    }

    pub fn retire_unowned_step(&mut self) -> Result<bool, &'static str> {
        self.begin_retirement(RasterTextureRetirementMode::Unowned)?;
        self.retirement_step()
    }

    pub fn begin_exact_close(&mut self, key: &str) -> Result<bool, &'static str> {
        let key = RasterTextureKey::new(key)?;
        if self.exact_close == Some(key) {
            return Ok(true);
        }
        if self.exact_close.is_some() {
            return Err("raster exact-close authority was occupied");
        }
        if self.closing || self.retirement.is_some() || self.upload.is_some() || self.upload_close.is_some() || self.reservation.is_some() || self.reservation_retirement.is_some() || !self.witnesses.is_empty() {
            return Ok(false);
        }
        self.residency.remove_key(key);
        self.exact_close = Some(key);
        Ok(true)
    }

    pub fn close_exact_step(&mut self, key: &str) -> Result<bool, &'static str> {
        let key = RasterTextureKey::new(key)?;
        if self.exact_close != Some(key) {
            return Err("raster exact-close token was stale");
        }
        if self.retirement.is_none() {
            let retained = self.live.get(key.as_str()).is_some() || self.staged.get(key.as_str()).is_some();
            if !retained {
                self.exact_close = None;
                return Ok(true);
            }
            self.begin_retirement(RasterTextureRetirementMode::Exact(key))?;
        }
        Ok(self.retirement_step()? && self.live.get(key.as_str()).is_none() && self.staged.get(key.as_str()).is_none())
    }

    pub fn exact_close_terminal_is_empty(&self, key: &str) -> bool {
        let Ok(key) = RasterTextureKey::new(key) else { return false };
        self.exact_close.is_none() && self.live.get(key.as_str()).is_none() && self.staged.get(key.as_str()).is_none() && !self.residency.protects(key)
    }

    pub fn close_upload_step(&mut self) -> RasterTextureCleanupStep {
        if let Some(retirement) = self.reservation_retirement.as_mut() {
            let step = retirement.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster reservation must be terminal-empty");
                self.reservation_retirement = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        if let Some(reservation) = self.reservation.take() {
            self.reservation_retirement = Some(RasterTextureReservationCloseCursor::reservation(reservation));
            return RasterTextureCleanupStep::retained();
        }
        if self.upload_close.is_none() {
            if let Some(upload) = self.upload.take() {
                self.upload_close = Some(RasterTextureUploadCloseCursor::new(upload));
                return RasterTextureCleanupStep::retained();
            }
            return RasterTextureCleanupStep::Complete;
        }
        let cursor = self.upload_close.as_mut().expect("retained raster upload close cursor");
        let step = cursor.step();
        if matches!(step, RasterTextureCleanupStep::Complete) {
            assert!(cursor.terminal_is_empty(), "completed raster upload close must be terminal-empty");
            self.upload_close = None;
            return RasterTextureCleanupStep::retained();
        }
        step
    }

    pub fn close_step(&mut self) -> Result<bool, &'static str> {
        self.closing = true;
        match self.close_upload_step() {
            RasterTextureCleanupStep::Pending { .. } => return Ok(false),
            RasterTextureCleanupStep::Blocked(fault) => return Err(fault),
            RasterTextureCleanupStep::Complete => {}
        }
        if self.retirement.is_some() && !self.retirement_step()? {
            return Ok(false);
        }
        if self.retirement.is_none() && (!self.live.is_empty() || !self.staged.is_empty() || !self.witnesses.is_empty()) {
            self.begin_retirement(RasterTextureRetirementMode::Close)?;
            return Ok(false);
        }
        self.residency.clear();
        Ok(self.terminal_is_empty())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.live.is_empty()
            && self.staged.is_empty()
            && self.upload.is_none()
            && self.upload_close.is_none()
            && self.retirement.is_none()
            && self.reservation.is_none()
            && self.reservation_retirement.is_none()
            && self.witnesses.is_empty()
            && self.residency.is_empty()
            && self.exact_close.is_none()
    }
}
