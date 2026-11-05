//! 🖼️ Granted decoded-image copying preserves profiles, metadata and additional frames without copying replacement pixels.
use super::*;
use semio_s_artifact_stdio_semio_image::schema::snapshot::{SemioImageFrame,SemioImageMetadataEntry,SemioImageSnapshot};

#[derive(semio_framework_value::RetireOwned)]
struct ImageOwned { image: Option<SemioImageSnapshot> }

/// 🧵️ A complete image clone or a metadata-preserving replacement of its first frame.
pub struct RasterImageCopyWork {
    owned: std::mem::ManuallyDrop<Option<ImageOwned>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    binding: Option<usize>,
    phase: u8,
    frame: usize,
    metadata: usize,
    replace_first: bool,
    closing: bool,
}

fn copy_error(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue,message) }

impl Default for RasterImageCopyWork { fn default() -> Self { Self::new() } }

impl RasterImageCopyWork {
    pub fn new() -> Self { Self { owned: std::mem::ManuallyDrop::new(Some(ImageOwned { image: None })), retirement: std::mem::ManuallyDrop::new(None), binding: None, phase: 0, frame: 0, metadata: 0, replace_first: false, closing: false } }

    fn image(&self) -> Result<&SemioImageSnapshot,ValueError> { self.owned.as_ref().and_then(|owned| owned.image.as_ref()).ok_or_else(|| copy_error("image copy candidate is absent")) }

    pub fn demands(&self, source: &SemioImageSnapshot, body: usize) -> Result<semio_framework_value::RetirementDemand,ValueError> {
        if self.closing { return Err(copy_error("image copy is closing")); }
        if self.binding.is_some_and(|binding| binding != source as *const SemioImageSnapshot as usize) { return Err(copy_error("image copy source changed")); }
        let mut demand = semio_framework_value::RetirementDemand { depth: 1, ..Default::default() };
        match self.phase {
            0 => demand.copy_bytes = size_of::<SemioImageSnapshot>(),
            1 => { demand.copy_bytes = source.schema.len(); demand.capacity_bytes = source.schema.len(); }
            2 => demand.capacity_bytes = source.frames.len().checked_mul(size_of::<SemioImageFrame>()).ok_or_else(|| copy_error("image frame reservation overflows"))?,
            3 if self.frame < source.frames.len() => demand.copy_bytes = size_of::<SemioImageFrame>(),
            4 => demand.capacity_bytes = source.frames[self.frame].rgba8.len(),
            5 => demand.copy_bytes = (source.frames[self.frame].rgba8.len() - self.image()?.frames[self.frame].rgba8.len()).min(body.min(32768)),
            6 => demand.capacity_bytes = source.icc.as_ref().map_or(0,Vec::len),
            7 => demand.copy_bytes = (source.icc.as_ref().map_or(0,Vec::len) - self.image()?.icc.as_ref().map_or(0,Vec::len)).min(body.min(32768)),
            8 => demand.capacity_bytes = source.metadata.len().checked_mul(size_of::<SemioImageMetadataEntry>()).ok_or_else(|| copy_error("image metadata reservation overflows"))?,
            9 if self.metadata < source.metadata.len() => demand.copy_bytes = size_of::<SemioImageMetadataEntry>(),
            10 => { demand.copy_bytes = source.metadata[self.metadata].key.len(); demand.capacity_bytes = demand.copy_bytes; }
            11 => { demand.copy_bytes = source.metadata[self.metadata].value.len(); demand.capacity_bytes = demand.copy_bytes; }
            13 => demand.copy_bytes = size_of::<Vec<u8>>(),
            _ => {}
        }
        if matches!(self.phase,1|10|11) && demand.copy_bytes > RASTER_OWNED_FIELD_BYTES { return Err(copy_error("image copy text exceeds its field budget")); }
        Ok(demand)
    }

    pub fn advance(&mut self, source: &SemioImageSnapshot, replacement: &mut Option<semio_framework_pixels::RasterImage>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep,ValueError> {
        let demand = self.demands(source,grant.maximum_copy_bytes)?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || raster_short(grant,demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let mut receipt = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() };
        if self.phase == 0 {
            self.binding = Some(source as *const SemioImageSnapshot as usize);
            self.replace_first = replacement.is_some();
            if self.replace_first && source.frames.is_empty() { return Err(copy_error("replacement image metadata has no first frame")); }
            if replacement.as_ref().is_some_and(|image| image.width != source.width || image.height != source.height) { return Err(copy_error("replacement image extent differs from its metadata")); }
            self.owned.as_mut().ok_or_else(|| copy_error("image copy owner disappeared"))?.image = Some(SemioImageSnapshot { schema: String::new(), width: source.width, height: source.height, colorspace: source.colorspace, bit_depth: source.bit_depth, frames: Vec::new(), icc: None, metadata: Vec::new() });
            self.phase = 1;
            return Ok(RetainedCloneStep::Progress(receipt));
        }
        let image = self.owned.as_mut().and_then(|owned| owned.image.as_mut()).ok_or_else(|| copy_error("image copy candidate disappeared"))?;
        match self.phase {
            1 => { image.schema = raster_exact_string_from_parts(&[source.schema.as_bytes()]).map_err(copy_error)?; receipt.retained_capacity_bytes = image.schema.capacity(); self.phase = 2; }
            2 => { image.frames.try_reserve_exact(source.frames.len()).map_err(|_| copy_error("image frame reservation failed"))?; receipt.retained_capacity_bytes = image.frames.capacity() * size_of::<SemioImageFrame>(); self.phase = 3; }
            3 => {
                if self.frame == source.frames.len() { self.phase = 6; }
                else { image.frames.push(SemioImageFrame { delay_ms: source.frames[self.frame].delay_ms, rgba8: Vec::new() }); self.phase = if self.frame == 0 && self.replace_first { 13 } else { 4 }; }
            }
            4 => { image.frames[self.frame].rgba8.try_reserve_exact(source.frames[self.frame].rgba8.len()).map_err(|_| copy_error("image sample reservation failed"))?; receipt.retained_capacity_bytes = image.frames[self.frame].rgba8.capacity(); self.phase = 5; }
            5 => {
                let target = &mut image.frames[self.frame].rgba8;
                let start = target.len();
                target.extend_from_slice(&source.frames[self.frame].rgba8[start..start+demand.copy_bytes]);
                if target.len() == source.frames[self.frame].rgba8.len() { self.frame += 1; self.phase = 3; }
            }
            6 => {
                if let Some(icc) = &source.icc { let mut bytes = Vec::new(); bytes.try_reserve_exact(icc.len()).map_err(|_| copy_error("image profile reservation failed"))?; receipt.retained_capacity_bytes = bytes.capacity(); image.icc = Some(bytes); self.phase = 7; }
                else { self.phase = 8; }
            }
            7 => {
                let target = image.icc.as_mut().ok_or_else(|| copy_error("image profile owner disappeared"))?;
                let start = target.len();
                target.extend_from_slice(&source.icc.as_ref().ok_or_else(|| copy_error("image source profile disappeared"))?[start..start+demand.copy_bytes]);
                if target.len() == source.icc.as_ref().unwrap().len() { self.phase = 8; }
            }
            8 => { image.metadata.try_reserve_exact(source.metadata.len()).map_err(|_| copy_error("image metadata reservation failed"))?; receipt.retained_capacity_bytes = image.metadata.capacity() * size_of::<SemioImageMetadataEntry>(); self.phase = 9; }
            9 => {
                if self.metadata == source.metadata.len() { self.phase = 12; }
                else { image.metadata.push(SemioImageMetadataEntry { key: String::new(), value: String::new() }); self.phase = 10; }
            }
            10 => { image.metadata[self.metadata].key = raster_exact_string_from_parts(&[source.metadata[self.metadata].key.as_bytes()]).map_err(copy_error)?; receipt.retained_capacity_bytes = image.metadata[self.metadata].key.capacity(); self.phase = 11; }
            11 => { image.metadata[self.metadata].value = raster_exact_string_from_parts(&[source.metadata[self.metadata].value.as_bytes()]).map_err(copy_error)?; receipt.retained_capacity_bytes = image.metadata[self.metadata].value.capacity(); self.metadata += 1; self.phase = 9; }
            12 => return Ok(RetainedCloneStep::Complete(Default::default())),
            13 => { image.frames[0].rgba8 = replacement.take().ok_or_else(|| copy_error("replacement frame owner disappeared"))?.pixels; self.frame = 1; self.phase = 3; }
            _ => return Err(copy_error("image copy phase is invalid")),
        }
        if !receipt.fits(grant) { return Err(copy_error("image copy allocator exceeded its admitted reservation").with_retained_progress(receipt)); }
        Ok(if self.phase == 12 { RetainedCloneStep::Complete(receipt) } else { RetainedCloneStep::Progress(receipt) })
    }

    pub fn take(&mut self) -> Option<SemioImageSnapshot> { if self.closing || self.phase != 12 { return None; } self.owned.as_mut()?.image.take() }

    pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand,ValueError> {
        if let Some(active) = self.retirement.as_ref() { return store::os_store::snapshot_read::artifact_retirement_box_demands(active,body); }
        if self.owned.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.owned); }
        Ok(Default::default())
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> semio_framework_value::RetainedWorkOutcome<RetainedCloneStep> {
        self.closing = true;
        if self.retirement.is_some() { store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement,grant) } else { store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.owned,&mut self.retirement,grant) }
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.owned.is_none() && self.retirement.is_none() }
}

impl Drop for RasterImageCopyWork { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(),"image copy must hand off or retire every owner before Drop"); } }
