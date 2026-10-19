//! 🏗️ Texture table admission: the table constructor, the retained counts, the candidate ownership, the admission step, the engine texture reservation and the stage claims. See [`super`].

use super::*;

impl RasterTextureTable {
    pub fn new(device: &gpu_device::Device, layout: &gpu_device::BindGroupLayout) -> Self {
        let sampler = device.create_sampler(&gpu_device::SamplerDescriptor { label: Some("raster_sampler"), mag_filter: gpu_device::FilterMode::Linear, min_filter: gpu_device::FilterMode::Linear, ..Default::default() });
        let shader=device.create_shader_module(gpu_device::ShaderModuleDescriptor{label:Some("raster_mip_shader"),source:gpu_device::ShaderSource::Wgsl(crate::shaders::RASTER_MIP_SHADER.into())});
        let mip_layout=device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor{label:Some("raster_mip_layout"),entries:&[gpu_device::BindGroupLayoutEntry{binding:0,visibility:gpu_device::ShaderStages::FRAGMENT,ty:gpu_device::BindingType::Texture{sample_type:gpu_device::TextureSampleType::Float{filterable:true},view_dimension:gpu_device::TextureViewDimension::D2,multisampled:false},count:None},gpu_device::BindGroupLayoutEntry{binding:1,visibility:gpu_device::ShaderStages::FRAGMENT,ty:gpu_device::BindingType::Sampler(gpu_device::SamplerBindingType::Filtering),count:None}]});
        let mip_pipeline_layout=device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor{label:Some("raster_mip_pipeline_layout"),bind_group_layouts:&[Some(&mip_layout)],immediate_size:0});
        let mip_pipelines=[gpu_device::TextureFormat::Rgba8Unorm,gpu_device::TextureFormat::Rgba8UnormSrgb].map(|format|device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor{label:Some("raster_mip_pipeline"),layout:Some(&mip_pipeline_layout),vertex:gpu_device::VertexState{module:&shader,entry_point:Some("vs_main"),buffers:&[],compilation_options:Default::default()},fragment:Some(gpu_device::FragmentState{module:&shader,entry_point:Some("fs_main"),targets:&[Some(gpu_device::ColorTargetState{format,blend:None,write_mask:gpu_device::ColorWrites::ALL})],compilation_options:Default::default()}),primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None}));
        Self {
            live: FixedRasterTextureRegistry::default(),
            staged: FixedRasterTextureRegistry::default(),
            upload: None,
            upload_close: None,
            retirement: None,
            reservation: None,
            reservation_retirement: None,
            witnesses: RasterOperationWitnessLedger::default(),
            residency: RasterResidencyLedger::default(),
            exact_close: None,
            next_reservation_nonce: 1,
            layout: layout.clone(),
            sampler,mip_layout,mip_pipelines,
            closing: false,
        }
    }

    pub(super) fn retained_bytes(&self) -> usize {
        self.live.bytes
            + self.staged.bytes
            + self.reservation.map_or(0, |reservation| reservation.bytes)
            + self.reservation_retirement.as_ref().map_or(0, RasterTextureReservationCloseCursor::retained_bytes)
            + self.retirement.as_ref().and_then(|cursor| cursor.owner.as_ref()).map_or(0, RasterTextureRetirementOwner::retained_bytes)
    }

    pub(super) fn retained_items(&self) -> usize {
        self.live.len
            + self.staged.len
            + usize::from(self.reservation.is_some())
            + self.reservation_retirement.as_ref().map_or(0, RasterTextureReservationCloseCursor::retained_items)
            + usize::from(self.retirement.as_ref().is_some_and(|cursor| cursor.owner.is_some()))
    }

    fn validate_freshness(&self, key: RasterTextureKey, witness: RasterTextureWitness) -> Result<(), &'static str> {
        if let Ok(Ok(index)) = self.live.locate(key) {
            let current = self.live.slots[index].as_ref().expect("located live raster");
            if raster_witness_is_stale(current.witness, witness) {
                return Err("raster operation witness was stale or duplicated");
            }
        }
        Ok(())
    }

    pub fn begin_candidate_ownership(&mut self, witness: RasterTextureWitness) -> Result<(), &'static str> {
        if self.closing || self.retirement.is_some() {
            return Err("raster texture table is closing or retiring");
        }
        self.residency.begin_candidate(witness)
    }

    pub fn publish_candidate_ownership(&mut self, witness: RasterTextureWitness, key: &str) -> Result<(), &'static str> {
        self.residency.publish_candidate_key(witness, key)
    }

    pub fn seal_candidate_ownership(&mut self, witness: RasterTextureWitness) -> Result<(), &'static str> {
        self.residency.seal_candidate(witness)
    }

    pub fn residency_counts(&self) -> (usize, usize, usize) {
        self.residency.counts()
    }

    pub fn prepare_admission_step(&mut self, key: &str, width: u32, height: u32,mip_levels:u32, content_identity: RasterContentIdentity, witness: RasterTextureWitness) -> Result<bool, &'static str> {
        let base_bytes = raster_texture_bytes(width,height).ok_or("raster texture byte credits overflowed")?;
        let bytes=raster_mip_bytes(width,height,mip_levels).ok_or("raster mip byte credits overflowed")?;
        if !content_identity.dimensions_match(width, height, base_bytes) {
            return Err("raster content identity dimensions were stale");
        }
        if !self.residency.candidate_is_sealed(witness) {
            return Err("raster keep-set candidate was not sealed before admission");
        }
        let key = RasterTextureKey::new(key)?;
        self.validate_freshness(key, witness)?;
        if raster_content_is_reusable(&self.live, &self.staged, key, content_identity, witness)? {
            return Ok(true);
        }
        if self.upload.is_some() || self.reservation.is_some() {
            return Ok(true);
        }
        if self.retirement.is_some() {
            if !self.retire_unowned_step()? {
                return Ok(false);
            }
            let admitted = raster_admission_fits(self.retained_items(), self.retained_bytes(), bytes);
            return admitted.then_some(true).ok_or("raster texture credits are owned by retained frames");
        }
        let admitted = raster_admission_fits(self.retained_items(), self.retained_bytes(), bytes);
        if admitted {
            return Ok(true);
        }
        let retained_items = self.retained_items();
        let retained_bytes = self.retained_bytes();
        if !self.retire_unowned_step()? {
            return Ok(false);
        }
        if self.retained_items() < retained_items || self.retained_bytes() < retained_bytes {
            return Ok(false);
        }
        Err("raster texture credits are owned by retained frames")
    }

    pub fn reserve_engine_texture(&mut self, key: &str, width: u32, height: u32,mip_levels:u32, content_identity: RasterContentIdentity, candidate: RasterTextureWitness, expected: RasterTextureWitness) -> Result<RasterTextureAdmission, &'static str> {
        if let Some(retirement) = self.reservation_retirement.as_mut() {
            if matches!(retirement.step(), RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster reservation must be terminal-empty");
                self.reservation_retirement = None;
            }
            return Err("raster cancelled reservation retirement is pending");
        }
        if candidate != expected {
            return Err("raster operation authority was stale before admission");
        }
        if self.closing || self.upload.is_some() || self.upload_close.is_some() || self.retirement.is_some() || self.reservation.is_some() {
            return Err("raster texture table is occupied");
        }
        if self.witnesses.candidate_occupied_by(candidate) {
            return Err("raster candidate generation was occupied");
        }
        let key = RasterTextureKey::new(key)?;
        let base_bytes = raster_texture_bytes(width,height).ok_or("raster texture byte credits overflowed")?;
        let bytes=raster_mip_bytes(width,height,mip_levels).ok_or("raster mip byte credits overflowed")?;
        if !content_identity.dimensions_match(width, height, base_bytes) {
            return Err("raster content identity dimensions were stale");
        }
        if width == 0 || height == 0 || bytes > RASTER_TEXTURE_ITEM_BYTE_CAPACITY {
            return Err("raster texture exceeded fixed item byte credits");
        }
        if self.retained_items() >= RASTER_TEXTURE_TABLE_CAPACITY {
            return Err("raster texture table item credits exhausted");
        }
        if self.retained_bytes().checked_add(bytes).is_none_or(|total| total > RASTER_TEXTURE_TABLE_BYTE_CAPACITY) {
            return Err("raster texture table byte credits exhausted");
        }
        self.validate_freshness(key, candidate)?;
        if self.staged.get(key.as_str()).is_some() {
            return Err("raster staged generation was duplicated");
        }
        let _ = self.live.locate(key)?;
        let staged_index = match self.staged.locate(key)? {
            Ok(index) | Err(index) => index,
        };
        let nonce = self.next_reservation_nonce;
        self.next_reservation_nonce = self.next_reservation_nonce.checked_add(1).ok_or("raster reservation generation exhausted")?;
        let reservation = RasterTextureReservation { key, witness: candidate, content_identity, width, height, bytes, staged_index, nonce };
        self.reservation = Some(reservation);
        self.witnesses.admit_candidate(candidate);
        Ok(RasterTextureAdmission { key, witness: candidate, content_identity, width, height, bytes, staged_index, nonce })
    }

    pub fn content_is_reusable(&self, key: &str, identity: RasterContentIdentity, candidate: RasterTextureWitness, expected: RasterTextureWitness) -> Result<bool, &'static str> {
        if candidate != expected {
            return Err("raster operation authority was stale before content reuse");
        }
        let key = RasterTextureKey::new(key)?;
        self.validate_freshness(key, candidate)?;
        raster_content_is_reusable(&self.live, &self.staged, key, identity, candidate)
    }

    #[expect(clippy::result_large_err, reason = "Cancellation must return its exact original admission on a full close slot.")]
    pub fn cancel_engine_texture_admission(&mut self, admission: RasterTextureAdmission) -> Result<(), RasterTextureCancelFault> {
        if self.reservation.as_ref().is_some_and(|reservation| reservation.matches(&admission)) {
            if self.reservation_retirement.is_some() {
                return Err(RasterTextureCancelFault::Returned { fault: "raster reservation retirement capacity exhausted", admission });
            }
            let reservation = self.reservation.take().expect("matching raster reservation");
            self.reservation_retirement = Some(RasterTextureReservationCloseCursor::cancelled(reservation, admission));
            return Ok(());
        }
        if self.reservation_retirement.is_some() {
            return Err(RasterTextureCancelFault::Returned { fault: "raster rejected admission retirement capacity exhausted", admission });
        }
        self.reservation_retirement = Some(RasterTextureReservationCloseCursor::rejected(admission));
        Err(RasterTextureCancelFault::Retained("raster admission was stale and retained for bounded retirement"))
    }

    fn claim_stage_before_gpu_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<RasterTextureStageClaim, &'static str> {
        let staged_occupied = self.staged.slots.get(admission.staged_index).is_none_or(Option::is_some);
        claim_raster_stage_tuple(self.reservation, self.witnesses.candidate(), staged_occupied, admission, expected)
    }

    pub(super) fn claim_texture_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<RasterTextureStageClaim, &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected)
    }

    pub(super) fn claim_view_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<RasterTextureStageClaim, &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected)
    }

    pub(super) fn claim_bind_group_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<RasterTextureStageClaim, &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected)
    }

    pub fn validate_engine_renderer_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected).map(|_| ())
    }

    pub fn validate_engine_target_texture_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected).map(|_| ())
    }

    pub fn validate_engine_target_view_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected).map(|_| ())
    }

    pub fn retain_engine_allocation_fault(&mut self, admission: RasterTextureAdmission, texture: Option<gpu_device::Texture>, view: Option<gpu_device::TextureView>) {
        assert!(self.upload.is_none() && self.upload_close.is_none(), "matching raster allocation fault must own its reserved close slot");
        self.upload_close = Some(RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor {mip_levels:1,mip:1,mip_row:0, admission: Some(admission), row: 0, texture, view, bind_group: None, allocation_claim: None, cpu_release: None }));
    }

    #[expect(clippy::result_large_err, reason = "GPU admission returns the exact resource and reservation owner without allocating during refusal.")]
    pub(super) fn stage_claimed_texture(
        &mut self,
        admission: RasterTextureAdmission,
        value: RasterTexture,
        claim: RasterTextureStageClaim,
        cpu_release: Option<crate::raster_ownership::SceneRasterReleaseWitness>,
    ) -> Result<(), (&'static str, RasterTextureAdmission, RasterTexture, Option<crate::raster_ownership::SceneRasterReleaseWitness>)> {
        if self.reservation != Some(claim.reservation) || !claim.reservation.matches(&admission) || claim.candidate != admission.witness || claim.staged_index != admission.staged_index || claim.staged_nonce != admission.nonce {
            return Err(("raster allocation claim changed before publication", admission, value, cpu_release));
        }
        if self.witnesses.candidate() != Some(claim.candidate) {
            return Err(("raster candidate witness changed after GPU allocation", admission, value, cpu_release));
        }
        if self.staged.slots[claim.staged_index].is_some() {
            return Err(("raster staged slot changed after GPU allocation", admission, value, cpu_release));
        }
        let entry = RasterTextureEntry { key: admission.key, witness: admission.witness, content_identity: admission.content_identity, bytes: admission.bytes, cpu_release, gpu_resident: None, value };
        if let Err(entry) = self.staged.insert_vacant(claim.staged_index, entry) {
            return Err(("raster staged slot changed after preflight", admission, entry.value, entry.cpu_release));
        }
        let _completed_reservation = self.reservation.take().expect("published raster reservation");
        Ok(())
    }

}
