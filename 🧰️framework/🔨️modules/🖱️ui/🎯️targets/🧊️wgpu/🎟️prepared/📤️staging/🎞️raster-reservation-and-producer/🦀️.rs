//! 🎞️ Raster reservation and producer: the raster rejection, the reservation and claim and the paged raster producer with its step. See [`super`].

use super::*;

//#region 🧩️PagedRasterProducer
#[derive(Debug)]
pub struct PreparedRasterRejected {
    pub(super) fault: &'static str,
    /// 🖼️ Whether the refusal is the SOURCE's own ([`Self::is_content_refusal`]).
    content: bool,
    pub(super) key: String,
    pub(super) source: Vec<u8>,
    retained_source: Vec<u8>,
    pub(super) credit: Option<PreparedRasterCredit>,
    source_released: bool,
    retained_source_released: bool,
    pub(super) key_released: bool,
}

impl PreparedRasterRejected {
    pub fn fault(&self) -> &'static str {
        self.fault
    }

    /// 🛑️ Whether this refusal is the source's own — undecodable or oversized, a typed per-image outcome its host retires
    /// silently — rather than a raster-authority fault.
    pub fn is_content_refusal(&self) -> bool {
        self.content
    }

    /// 🪵️ This refusal classified as the source's own ([`Self::is_content_refusal`]) — what a host does with a refusal its
    /// measured source caused.
    pub fn into_content_refusal(mut self) -> Self {
        self.content = true;
        self
    }

    pub fn next_close_demands(&self,_copy:usize)->Result<RetainedCloneGrant,ValueError>{
        let release=if !self.source.is_empty(){0}else if !self.source_released{prepared_vec_bytes(&self.source)?}else if !self.retained_source.is_empty(){0}else if !self.retained_source_released{prepared_vec_bytes(&self.retained_source)?}else if !self.key.is_empty(){0}else if !self.key_released{self.key.capacity()}else{0};
        prepared_owner_demands(self.terminal_is_empty(),Ok(release))
    }

    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<CloseStep,ValueError>{
        let demand=self.next_close_demands(grant.maximum_copy_bytes)?;
        if let Some(step)=prepared_close_gate(grant,self.terminal_is_empty(),Ok(demand.maximum_release_bytes)){return step}
        if !self.source.is_empty(){self.source.truncate(self.source.len().saturating_sub(PREPARED_RASTER_PAGE_BYTES));}
        else if !self.source_released{self.source=Vec::new();self.source_released=true;}
        else if !self.retained_source.is_empty(){self.retained_source.truncate(self.retained_source.len().saturating_sub(PREPARED_RASTER_PAGE_BYTES));}
        else if !self.retained_source_released{self.retained_source=Vec::new();self.retained_source_released=true;}
        else if !self.key.is_empty(){self.key.pop();}
        else if !self.key_released{self.key=String::new();self.key_released=true;}
        else if let Some(credit)=self.credit.as_ref(){let mut ledger=match PREPARED_RASTER_LEDGER.try_lock(){Ok(ledger)=>ledger,Err(std::sync::TryLockError::WouldBlock)=>return Ok(CloseStep::Blocked),Err(std::sync::TryLockError::Poisoned(_))=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original raster credit ledger poisoned"))};if !ledger.release(credit){return Ok(CloseStep::Blocked)}self.credit=None;}
        prepared_close_progress(demand.maximum_release_bytes)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.source.is_empty()
            && self.source.capacity() == 0
            && self.source_released
            && self.retained_source.is_empty()
            && self.retained_source.capacity() == 0
            && self.retained_source_released
            && self.key.is_empty()
            && self.key.capacity() == 0
            && self.key_released
            && self.credit.is_none()
    }
}

#[derive(Debug)]
pub struct PreparedRasterReservation {
    pub(super) key: String,
    pub(super) credit: Option<PreparedRasterCredit>,
    pub(super) claim: Option<PreparedRasterClaim>,
    pub(super) source_bytes: usize,
    pub(super) source_peak_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PreparedRasterClaim {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) byte_len: usize,
    pub(super) rows_per_page: usize,
    pub(super) page_capacity: usize,
}

impl PreparedRasterReservation {
    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Rejected preparation returns its exact source, key, and reservation credit without allocating beyond admission."))]
    pub fn try_reserve(key: String) -> Result<Self, PreparedRasterRejected> {
        Self::try_reserve_source(key, 0)
    }

    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Rejected preparation returns its exact source, key, and reservation credit without allocating beyond admission."))]
    pub fn try_reserve_source(key: String, source_bytes: usize) -> Result<Self, PreparedRasterRejected> {
        let reject = |fault, key| PreparedRasterRejected { fault, content: false, key, source: Vec::new(), retained_source: Vec::new(), credit: None, source_released: false, retained_source_released: false, key_released: false };
        if key.len() > PREPARED_RASTER_KEY_BYTES {
            return Err(reject("raster producer exceeded fixed key credits", key));
        }
        let Some(key_bytes) = key.capacity().checked_mul(2) else { return Err(reject("raster producer key credits overflowed", key)) };
        let Some(source_peak_bytes) = source_bytes.checked_mul(2) else { return Err(reject("raster producer source credits overflowed", key)) };
        let Some(bytes) = source_peak_bytes.checked_add(key_bytes) else { return Err(reject("raster producer source credits overflowed", key)) };
        if source_bytes > PREPARED_RASTER_ITEM_BYTES {
            return Err(reject("raster producer source exceeded fixed credits", key).into_content_refusal());
        }
        let credit = PREPARED_RASTER_LEDGER.lock().ok().and_then(|mut ledger| ledger.reserve(1, bytes));
        let Some(credit) = credit else { return Err(reject("raster producer process credits exhausted", key)) };
        Ok(Self { key, credit: Some(credit), claim: None, source_bytes, source_peak_bytes })
    }

    pub fn reject(self, fault: &'static str, source: Vec<u8>) -> PreparedRasterRejected {
        self.reject_with_retained(fault, source, Vec::new())
    }

    pub fn reject_with_retained(mut self, fault: &'static str, source: Vec<u8>, retained_source: Vec<u8>) -> PreparedRasterRejected {
        PreparedRasterRejected { fault, content: false, key: std::mem::take(&mut self.key), source, retained_source, credit: self.credit.take(), source_released: false, retained_source_released: false, key_released: false }
    }

    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Rejected preparation returns its exact source, key, and reservation credit without allocating beyond admission."))]
    pub fn claim(self, width: u32, height: u32) -> Result<Self, PreparedRasterRejected> {
        self.claim_with_retained(width, height, Vec::new()).map(|(reservation, _)| reservation)
    }

    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Rejected preparation returns its exact source, key, and reservation credit without allocating beyond admission."))]
    pub fn claim_with_retained(mut self, width: u32, height: u32, retained_source: Vec<u8>) -> Result<(Self, Vec<u8>), PreparedRasterRejected> {
        let reject = |reservation: Self, fault, retained_source| reservation.reject_with_retained(fault, Vec::new(), retained_source);
        if retained_source.capacity() > self.source_bytes {
            return Err(reject(self, "raster retained source exceeded its pre-admitted workspace", retained_source));
        }
        let Some(row_bytes) = usize::try_from(width).ok().and_then(|value| value.checked_mul(4)) else { return Err(reject(self, "raster row byte credits overflowed", retained_source)) };
        let Some(byte_len) = row_bytes.checked_mul(usize::try_from(height).unwrap_or(usize::MAX)) else { return Err(reject(self, "raster byte credits overflowed", retained_source)) };
        if width == 0 || height == 0 || row_bytes > PREPARED_RASTER_PAGE_BYTES || byte_len > PREPARED_RASTER_ITEM_BYTES {
            return Err(reject(self, "raster producer exceeded fixed item or byte credits", retained_source));
        }
        let rows_per_page = (PREPARED_RASTER_PAGE_BYTES / row_bytes).max(1);
        let page_capacity = usize::try_from(height).unwrap_or(usize::MAX).div_ceil(rows_per_page);
        let Some(items) = page_capacity.checked_add(8) else { return Err(reject(self, "raster producer item credits overflowed", retained_source)) };
        let Some(key_bytes) = self.key.capacity().checked_mul(2) else { return Err(reject(self, "raster producer key credits overflowed", retained_source)) };
        let Some(page_slot_bytes) = page_capacity.checked_mul(size_of::<PreparedRasterPage>()) else { return Err(reject(self, "raster producer page slot credits overflowed", retained_source)) };
        let Some(bytes) = self.source_peak_bytes.checked_add(byte_len).and_then(|value| value.checked_add(key_bytes)).and_then(|value| value.checked_add(page_slot_bytes)) else {
            return Err(reject(self, "raster producer aggregate bytes overflowed", retained_source));
        };
        let resized = self.credit.as_mut().is_some_and(|credit| PREPARED_RASTER_LEDGER.lock().is_ok_and(|mut ledger| ledger.resize(credit, items, bytes)));
        if !resized {
            return Err(reject(self, "raster producer exact credit resize failed", retained_source));
        }
        self.claim = Some(PreparedRasterClaim { width, height, byte_len, rows_per_page, page_capacity });
        Ok((self, retained_source))
    }

    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Rejected preparation returns its exact source, key, and reservation credit without allocating beyond admission."))]
    pub fn finalize(mut self, source: Vec<u8>, retained_source: Vec<u8>, width: u32, height: u32) -> Result<(PreparedRasterProducer, String), PreparedRasterRejected> {
        let reject = |reservation: Self, fault, source, retained_source| reservation.reject_with_retained(fault, source, retained_source);
        let Some(claim) = self.claim else { return Err(reject(self, "raster producer was not claimed before materialization", source, retained_source)) };
        if claim.width != width || claim.height != height || source.len() != claim.byte_len || source.capacity() > claim.byte_len || retained_source.capacity() > self.source_bytes {
            return Err(reject(self, "raster materialization did not match its exact claim", source, retained_source));
        }
        let Some(content_identity) = RasterContentIdentity::pixels(width, height, claim.byte_len) else { return Err(reject(self, "raster content identity byte credits overflowed", source, retained_source)) };
        let Some(credit) = self.credit.take() else { return Err(reject(self, "raster reservation lost its exact credit", source, retained_source)) };
        let source_generation = PreparedRasterGeneration { slot: credit.slot, epoch: credit.epoch };
        let pages = PreparedRasterPages {
            slots: Vec::with_capacity(claim.page_capacity),
            backing: Vec::new(),
            page_capacity: claim.page_capacity,
            rows_per_page: claim.rows_per_page as u32,
            width,
            height,
            byte_len: claim.byte_len,
            content_identity,
            source_generation,
            frame_generation: 0,
            credit: Some(credit),
            backing_released: false,
            key_released: false,
            close_phase: 0,
        };
        let published_key = self.key.clone();
        let retained_source_released = retained_source.is_empty();
        Ok((PreparedRasterProducer { key: self.key, source, retained_source, pages: Some(pages), frame_generation: None, source_released: false, retained_source_released, closing: false }, published_key))
    }
}

#[derive(Debug)]
pub struct PreparedRasterProducer {
    pub(super) key: String,
    pub(super) source: Vec<u8>,
    retained_source: Vec<u8>,
    pub(super) pages: Option<PreparedRasterPages>,
    pub(super) frame_generation: Option<u64>,
    source_released: bool,
    retained_source_released: bool,
    pub(super) closing: bool,
}

pub enum PreparedRasterProducerStep {
    Pending,
    Complete(PreparedRenderUpload),
    Fault(&'static str),
}

impl PreparedRasterProducer {
    #[cfg(feature = "testkit")]
    pub fn original_source(&self) -> &[u8] { &self.source }

    #[cfg(feature = "testkit")]
    pub fn original_retained_source(&self) -> &[u8] { &self.retained_source }

    #[cfg(feature = "testkit")]
    pub fn original_page_count(&self) -> usize { self.pages.as_ref().map_or(0, |pages| pages.slots.len()) }

    pub fn next_close_demands(&self,_copy:usize)->Result<RetainedCloneGrant,ValueError>{prepared_owner_demands(self.terminal_is_empty(),self.next_close_release_byte_demand())}

    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if let Some(pages) = self.pages.as_ref() {
            if !pages.slots.is_empty() || !pages.backing.is_empty() { return Ok(0); }
            if !pages.backing_released { return prepared_vec_bytes(&pages.backing); }
        }
        if !self.source.is_empty() { return Ok(0); }
        if !self.source_released { return prepared_vec_bytes(&self.source); }
        if !self.retained_source.is_empty() { return Ok(0); }
        if !self.retained_source_released { return prepared_vec_bytes(&self.retained_source); }
        if !self.key.is_empty() { return Ok(0); }
        if self.key.capacity() != 0 || self.pages.as_ref().is_some_and(|pages| !pages.key_released) { return Ok(self.key.capacity()); }
        if let Some(pages) = self.pages.as_ref() { if pages.close_phase == 0 { return prepared_vec_bytes(&pages.slots); } }
        Ok(0)
    }

    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let demand = self.next_close_release_byte_demand();
        if let Some(step) = prepared_close_gate(grant, self.terminal_is_empty(), demand.clone()) { return step; }
        if let Some(pages) = self.pages.as_mut() {
            if pages.retire_page_step() || !pages.retire_backing_step() { return prepared_close_progress(demand.unwrap_or(0)); }
        }
        if !self.source.is_empty() { self.source.truncate(self.source.len().saturating_sub(PREPARED_RASTER_PAGE_BYTES)); }
        else if !self.source_released { self.source = Vec::new(); self.source_released = true; }
        else if !self.retained_source.is_empty() { self.retained_source.truncate(self.retained_source.len().saturating_sub(PREPARED_RASTER_PAGE_BYTES)); }
        else if !self.retained_source_released { self.retained_source = Vec::new(); self.retained_source_released = true; }
        else if !self.key.is_empty() || self.key.capacity() != 0 || self.pages.as_ref().is_some_and(|pages| !pages.key_released) {
            prepared_close_text(&mut self.key);
            if self.key.capacity() == 0 { if let Some(pages) = self.pages.as_mut() { pages.key_released = true; } }
        } else if let Some(pages) = self.pages.as_mut() {
            let phase = pages.close_phase;
            let credit = pages.credit.is_some();
            pages.retire_metadata_step();
            if phase == 10 && credit && pages.credit.is_some() { return Ok(CloseStep::Blocked); }
        }
        prepared_close_progress(demand.unwrap_or(0))
    }

    pub fn source_generation(&self) -> PreparedRasterGeneration {
        self.pages.as_ref().map_or_else(PreparedRasterGeneration::default, PreparedRasterPages::source_generation)
    }

    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Rejected preparation returns its exact source, key, and reservation credit without allocating beyond admission."))]
    pub fn try_admit(key: String, source: Vec<u8>, width: u32, height: u32) -> Result<(Self, String), PreparedRasterRejected> {
        match PreparedRasterReservation::try_reserve(key) {
            Ok(reservation) => match reservation.claim(width, height) {
                Ok(reservation) => reservation.finalize(source, Vec::new(), width, height),
                Err(mut rejected) => {
                    rejected.source = source;
                    Err(rejected)
                }
            },
            Err(mut rejected) => {
                rejected.source = source;
                Err(rejected)
            }
        }
    }

    pub fn bind_frame_generation(&mut self, generation: u64) -> bool {
        if generation == 0 || self.closing {
            return false;
        }
        match self.frame_generation {
            Some(current) => current == generation,
            None => {
                self.frame_generation = Some(generation);
                let Some(pages) = self.pages.as_mut() else { return false };
                pages.frame_generation = generation;
                true
            }
        }
    }

    pub fn step(&mut self, expected_generation: u64) -> PreparedRasterProducerStep {
        if self.closing {
            return PreparedRasterProducerStep::Fault("raster producer is closing");
        }
        if self.frame_generation != Some(expected_generation) {
            return PreparedRasterProducerStep::Fault("raster producer generation is stale");
        }
        if self.source.is_empty() {
            if !self.source_released {
                self.source = Vec::new();
                self.source_released = true;
                return PreparedRasterProducerStep::Pending;
            }
            if !self.retained_source.is_empty() {
                self.retained_source.truncate(self.retained_source.len().saturating_sub(PREPARED_RASTER_PAGE_BYTES));
                return PreparedRasterProducerStep::Pending;
            }
            if !self.retained_source_released {
                self.retained_source = Vec::new();
                self.retained_source_released = true;
                return PreparedRasterProducerStep::Pending;
            }
            let Some(pages) = self.pages.take() else { return PreparedRasterProducerStep::Fault("raster producer lost its admitted page owner") };
            let key = std::mem::take(&mut self.key);
            return PreparedRasterProducerStep::Complete(PreparedRenderUpload::RasterPages { key, pixels: pages });
        }
        let Some(pages) = self.pages.as_mut() else { return PreparedRasterProducerStep::Fault("raster producer lost its admitted page owner") };
        if pages.slots.len() == pages.page_capacity {
            pages.backing = std::mem::take(&mut self.source);
            self.source_released = true;
            return PreparedRasterProducerStep::Pending;
        }
        let Some(row_bytes) = usize::try_from(pages.width).ok().and_then(|width| width.checked_mul(4)) else { return PreparedRasterProducerStep::Fault("raster row byte count exhausted") };
        let Some(page_bytes) = usize::try_from(pages.rows_per_page).ok().and_then(|rows| rows.checked_mul(row_bytes)) else {
            return PreparedRasterProducerStep::Fault("raster page byte count exhausted");
        };
        let Some(logical) = pages.slots.len().checked_add(1).and_then(|next| pages.page_capacity.checked_sub(next)) else {
            return PreparedRasterProducerStep::Fault("raster page cursor exceeded its fixed capacity");
        };
        let Some(start) = logical.checked_mul(page_bytes) else { return PreparedRasterProducerStep::Fault("raster page offset exhausted") };
        let Some(end) = start.checked_add(page_bytes).map(|end| end.min(self.source.len())) else { return PreparedRasterProducerStep::Fault("raster page end exhausted") };
        let Ok(start_row) = u32::try_from(start / row_bytes) else { return PreparedRasterProducerStep::Fault("raster start row exhausted") };
        let Ok(rows) = u32::try_from((end - start) / row_bytes) else { return PreparedRasterProducerStep::Fault("raster page rows exhausted") };
        pages.content_identity.mix_bytes(start, &self.source[start..end]);
        pages.slots.push(PreparedRasterPage { start_row, rows });
        PreparedRasterProducerStep::Pending
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        self.close_granted(grant)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.key.is_empty()
            && self.key.capacity() == 0
            && self.source.is_empty()
            && self.source.capacity() == 0
            && self.source_released
            && self.retained_source.is_empty()
            && self.retained_source.capacity() == 0
            && self.retained_source_released
            && self.pages.as_ref().is_none_or(PreparedRasterPages::terminal_is_empty)
    }
}
//#endregion 🧩️PagedRasterProducer
