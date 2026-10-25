//! 📤️ Upload and retirement owners: the texture upload cursor, the upload pixels, the retirement owner, the upload close cursor, the retirement mode and cursor and the texture table state. See [`super`].

use super::*;

pub(super) struct RasterTextureUploadCursor {
    pub(super) mip_levels:u32,
    pub(super) mip:u32,
    pub(super) mip_row:u32,
    pub(super) admission: Option<RasterTextureAdmission>,
    pub(super) row: u32,
    pub(super) texture: Option<gpu_device::Texture>,
    pub(super) view: Option<gpu_device::TextureView>,
    pub(super) bind_group: Option<gpu_device::BindGroup>,
    pub(super) allocation_claim: Option<RasterTextureStageClaim>,
    pub(super) cpu_release: Option<crate::raster_ownership::SceneRasterReleaseWitness>,
}

#[derive(Clone, Copy)]
pub enum RasterUploadPixels<'a> {
    #[cfg(any(test, feature = "testkit"))]
    Contiguous(&'a [u8]),
    Pages(&'a PreparedRasterPages),
    Scene(&'a crate::raster_ownership::SceneRasterLease),
}

impl RasterUploadPixels<'_> {
    pub(super) fn len(&self) -> usize {
        match self {
            #[cfg(any(test, feature = "testkit"))]
            Self::Contiguous(pixels) => pixels.len(),
            Self::Pages(pixels) => pixels.byte_len(),
            Self::Scene(lease) => lease.byte_len(),
        }
    }

    pub(super) fn dimensions_match(&self, width: u32, height: u32) -> bool {
        match self {
            #[cfg(any(test, feature = "testkit"))]
            Self::Contiguous(_) => true,
            Self::Pages(pixels) => pixels.width() == width && pixels.height() == height,
            Self::Scene(lease) => {
                let descriptor = lease.identity().descriptor();
                descriptor.width == width && descriptor.height == height
            }
        }
    }

    #[cfg_attr(not(any(test, feature = "testkit")), allow(unused_variables))]
    pub(super) fn content_identity(&self, width: u32, height: u32) -> Option<RasterContentIdentity> {
        match self {
            #[cfg(any(test, feature = "testkit"))]
            Self::Contiguous(pixels) => RasterContentIdentity::test_pixels(width, height, pixels),
            Self::Pages(pixels) => Some(pixels.content_identity()),
            Self::Scene(lease) => Some(lease.identity().content()),
        }
    }

    #[cfg_attr(not(any(test, feature = "testkit")), allow(unused_variables))]
    pub(super) fn with_rows<R>(&self, row: u32, start: usize, end: usize, rows: u32, read: impl FnOnce(&[u8], u32) -> R) -> Option<R> {
        match self {
            #[cfg(any(test, feature = "testkit"))]
            Self::Contiguous(pixels) => pixels.get(start..end).map(|page| read(page, rows)),
            Self::Pages(pixels) => pixels.page_for_row(row).map(|(page, page_rows)| read(page, page_rows)),
            Self::Scene(lease) => lease.with_rows(row, lease.transfer_bytes(), read).ok(),
        }
    }

    pub(super) fn transfer_bytes(&self) -> usize {
        match self {
            Self::Scene(lease) => lease.transfer_bytes(),
            #[cfg(any(test, feature = "testkit"))]
            Self::Contiguous(_) => crate::prepared::PREPARED_RASTER_PAGE_BYTES,
            Self::Pages(_) => crate::prepared::PREPARED_RASTER_PAGE_BYTES,
        }
    }

    pub(super) fn release_witness(&self) -> Option<crate::raster_ownership::SceneRasterReleaseWitness> {
        match self {
            Self::Scene(lease) => lease.release_witness(),
            #[cfg(any(test, feature = "testkit"))]
            Self::Contiguous(_) => None,
            Self::Pages(_) => None,
        }
    }

    pub(super) fn release_reused(&self) {
        if let Self::Scene(lease) = self {
            let _ = lease.release_committed();
        }
    }

    pub fn mip_levels(&self,width:u32,height:u32)->u32{if matches!(self,Self::Scene(lease) if matches!(lease.identity().descriptor().profile,crate::raster_ownership::SceneRasterProfile::MeshBaseColorSrgb|crate::raster_ownership::SceneRasterProfile::MeshNumericLinear)){u32::BITS-width.max(height).leading_zeros()}else{1}}

    pub(super) fn texture_format(&self) -> gpu_device::TextureFormat {
        if matches!(self,Self::Scene(lease) if lease.identity().descriptor().profile==crate::raster_ownership::SceneRasterProfile::MeshNumericLinear){gpu_device::TextureFormat::Rgba8Unorm}else{gpu_device::TextureFormat::Rgba8UnormSrgb}
    }
}

pub(super) struct RasterTextureRetirementOwner {
    pub(super) key: Option<RasterTextureKey>,
    pub(super) content_identity: Option<RasterContentIdentity>,
    pub(super) scene_revision: Option<u64>,
    pub(super) preview_generation: Option<u64>,
    pub(super) operation: Option<u64>,
    pub(super) width: Option<u32>,
    pub(super) height: Option<u32>,
    pub(super) bytes: Option<usize>,
    pub(super) bind_group: Option<gpu_device::BindGroup>,
    pub(super) view: Option<gpu_device::TextureView>,
    pub(super) texture: Option<gpu_device::Texture>,
    pub(super) cpu_release: Option<crate::raster_ownership::SceneRasterReleaseWitness>,
    pub(super) gpu_resident: Option<crate::raster_ownership::SceneRasterGpuWitness>,
}

impl RasterTextureRetirementOwner {
    pub(super) fn new(entry: RasterTextureEntry<RasterTexture>) -> Self {
        Self {
            key: Some(entry.key),
            content_identity: Some(entry.content_identity),
            scene_revision: Some(entry.witness.scene_revision),
            preview_generation: Some(entry.witness.preview_generation),
            operation: Some(entry.witness.operation),
            width: Some(entry.value.width),
            height: Some(entry.value.height),
            bytes: Some(entry.bytes),
            bind_group: Some(entry.value.bind_group),
            view: Some(entry.value.view),
            texture: Some(entry.value.texture),
            cpu_release: entry.cpu_release,
            gpu_resident: entry.gpu_resident,
        }
    }

    pub(super) fn step(&mut self) -> RasterTextureCleanupStep {
        if self.bind_group.take().is_some() {
            return RasterTextureCleanupStep::root();
        }
        if self.view.take().is_some() {
            return RasterTextureCleanupStep::root();
        }
        if let Some(texture) = self.texture.take() {
            texture.destroy();
            return RasterTextureCleanupStep::root();
        }
        if let Some(release) = self.cpu_release.take() {
            let _ = release.release();
            return RasterTextureCleanupStep::scalar();
        }
        if self.gpu_resident.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.key.take().is_some() {
            return RasterTextureCleanupStep::root();
        }
        if self.content_identity.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.scene_revision.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.preview_generation.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.operation.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.width.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.height.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.bytes.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        RasterTextureCleanupStep::Complete
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.key.is_none()
            && self.content_identity.is_none()
            && self.scene_revision.is_none()
            && self.preview_generation.is_none()
            && self.operation.is_none()
            && self.width.is_none()
            && self.height.is_none()
            && self.bytes.is_none()
            && self.bind_group.is_none()
            && self.view.is_none()
            && self.texture.is_none()
            && self.cpu_release.is_none()
            && self.gpu_resident.is_none()
    }

    pub(super) fn retained_bytes(&self) -> usize {
        self.bytes.unwrap_or(0)
    }
}

pub(super) struct RasterTextureUploadCloseCursor {
    pub(super) source: Option<RasterTextureUploadCursor>,
    pub(super) admission: Option<RasterTextureAdmission>,
    pub(super) admission_retirement: Option<RasterTextureReservationRetirement>,
    allocation_claim_retirement: Option<RasterTextureStageClaimRetirement>,
    pub(super) owner: Option<RasterTextureRetirementOwner>,
    row_retired: bool,
}

impl RasterTextureUploadCloseCursor {
    pub(super) fn new(source: RasterTextureUploadCursor) -> Self {
        Self { source: Some(source), admission: None, admission_retirement: None, allocation_claim_retirement: None, owner: None, row_retired: false }
    }

    pub(super) fn step(&mut self) -> RasterTextureCleanupStep {
        if let Some(owner) = self.owner.as_mut() {
            let step = owner.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(owner.terminal_is_empty(), "completed raster GPU owner must be terminal-empty");
                self.owner = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        if let Some(retirement) = self.allocation_claim_retirement.as_mut() {
            let step = retirement.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster allocation claim must be terminal-empty");
                self.allocation_claim_retirement = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        if let Some(retirement) = self.admission_retirement.as_mut() {
            let step = retirement.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster admission must be terminal-empty");
                self.admission_retirement = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        if let Some(source) = self.source.as_mut() {
            if self.admission.is_none() {
                if let Some(admission) = source.admission.take() {
                    self.admission = Some(admission);
                    return RasterTextureCleanupStep::retained();
                }
            }
            if let Some(claim) = source.allocation_claim.take() {
                self.allocation_claim_retirement = Some(RasterTextureStageClaimRetirement::new(claim));
                return RasterTextureCleanupStep::retained();
            }
            if source.texture.is_some() || source.view.is_some() || source.bind_group.is_some() {
                let admission = self.admission.as_ref().expect("retained upload close admission");
                self.owner = Some(RasterTextureRetirementOwner {
                    key: Some(admission.key),
                    content_identity: Some(admission.content_identity),
                    scene_revision: Some(admission.witness.scene_revision),
                    preview_generation: Some(admission.witness.preview_generation),
                    operation: Some(admission.witness.operation),
                    width: Some(admission.width),
                    height: Some(admission.height),
                    bytes: Some(admission.bytes),
                    bind_group: source.bind_group.take(),
                    view: source.view.take(),
                    texture: source.texture.take(),
                    cpu_release: source.cpu_release.take(),
                    gpu_resident: None,
                });
                return RasterTextureCleanupStep::retained();
            }
            if !self.row_retired {
                source.row = 0;source.mip_levels=0;source.mip=0;source.mip_row=0;
                self.row_retired = true;
                return RasterTextureCleanupStep::scalar();
            }
            if let Some(release) = source.cpu_release.take() {
                let _ = release.release();
                return RasterTextureCleanupStep::scalar();
            }
            self.source = None;
            return RasterTextureCleanupStep::retained();
        }
        if let Some(admission) = self.admission.take() {
            self.admission_retirement = Some(admission.into_retirement());
            return RasterTextureCleanupStep::retained();
        }
        RasterTextureCleanupStep::Complete
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.source.is_none() && self.admission.is_none() && self.admission_retirement.is_none() && self.allocation_claim_retirement.is_none() && self.owner.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RasterTextureRetirementMode {
    Abort(RasterTextureWitness),
    Commit(RasterTextureWitness),
    Unowned,
    Exact(RasterTextureKey),
    Close,
}

pub(super) struct RasterTextureRetirementCursor {
    pub(super) mode: RasterTextureRetirementMode,
    pub(super) scan: usize,
    pub(super) owner: Option<RasterTextureRetirementOwner>,
}

pub struct RasterTextureTable {
    pub(super) live: FixedRasterTextureRegistry<RasterTexture>,
    pub(super) staged: FixedRasterTextureRegistry<RasterTexture>,
    pub(super) upload: Option<RasterTextureUploadCursor>,
    pub(super) upload_close: Option<RasterTextureUploadCloseCursor>,
    pub(super) retirement: Option<RasterTextureRetirementCursor>,
    pub(super) reservation: Option<RasterTextureReservation>,
    pub(super) reservation_retirement: Option<RasterTextureReservationCloseCursor>,
    pub(super) witnesses: RasterOperationWitnessLedger,
    pub(super) residency: RasterResidencyLedger,
    pub(super) exact_close: Option<RasterTextureKey>,
    pub(super) next_reservation_nonce: u64,
    pub(super) layout: gpu_device::BindGroupLayout,
    pub(super) sampler: gpu_device::Sampler,
    pub(super) mip_layout:gpu_device::BindGroupLayout,
    pub(super) mip_pipelines:[gpu_device::RenderPipeline;2],
    pub(super) closing: bool,
}
