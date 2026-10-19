//! 🗒️ Atlas pages and raster pages: the `PreparedAtlasPages` staging with its drop and the `PreparedRasterPages` staging. See [`super`].

use super::*;

//#region 🧩️PagedRasterProducer
impl PreparedAtlasPages {
    pub fn original_owner_progress(&self) -> RetainedCloneProgress { self.observed }

    pub fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.owner.as_ref().map_or(0, |owner| owner.original_close_progress().copied_bytes)) }

    pub(super) fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.owner.as_ref().map_or(0, |owner| owner.original_close_progress().released_bytes)) }

    pub fn close_original_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let Some(owner) = self.owner.as_mut() else { return Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() }); };
        let grant = atlas_cap(grant, owner.authority.retirement);
        let progress = owner.original_close_progress();
        if let Some(step) = atlas_gate(grant, progress) { return step; }
        if !owner.terminal_is_empty() || owner.handoff_pending {
            let step = owner.close_original_step(grant)?;
            atlas_add(&mut self.observed, step.progress());
            return Ok(step);
        }
        let slot = usize::from(self.abandonment_slot);
        if PREPARED_ATLAS_ABANDONMENT_STATE[slot].load(Ordering::Acquire) != 1 { return atlas_refused(ValueRefusalKind::InvariantViolated); }
        drop(self.owner.take());
        PREPARED_ATLAS_ABANDONMENT_STATE[slot].store(0, Ordering::Release);
        self.abandonment_slot = u8::MAX;
        atlas_add(&mut self.observed, progress);
        Ok(CloseStep::Complete { progress })
    }

    pub(super) fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> { self.close_original_step(grant) }

    pub fn try_new(width: u32, height: u32, channels: u8, byte_len: usize, authority: PreparedAtlasAuthority) -> Result<(Self, RetainedCloneProgress), ValueError> {
        let fault = |message| ValueError::literal(ValueRefusalKind::OwnershipLimit, message);
        let row_bytes = usize::try_from(width).ok().and_then(|width| width.checked_mul(usize::from(channels))).ok_or_else(|| fault("atlas row bytes exhausted"))?;
        let expected = row_bytes.checked_mul(usize::try_from(height).map_err(|_| fault("atlas height exhausted"))?).ok_or_else(|| fault("atlas byte length exhausted"))?;
        if expected != byte_len || row_bytes == 0 || row_bytes > PREPARED_ATLAS_PAGE_BYTES { return Err(fault("atlas dimensions do not fit fixed page credits")); }
        let rows_per_page = u32::try_from(PREPARED_ATLAS_PAGE_BYTES / row_bytes).map_err(|_| fault("atlas rows per page exhausted"))?;
        let rows_per_page_usize = usize::try_from(rows_per_page).map_err(|_| fault("atlas rows per page exhausted"))?;
        let pages = usize::try_from(height).ok().and_then(|height| height.checked_add(rows_per_page_usize.checked_sub(1)?)).map(|rows| rows / rows_per_page_usize).ok_or_else(|| fault("atlas page count exhausted"))?;
        if pages > PREPARED_ATLAS_PAGE_CAPACITY || byte_len > PREPARED_ATLAS_PROCESS_PAYLOAD_BYTES { return Err(fault("atlas page or byte credits exceeded")); }
        let slot_backing = size_of::<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>();
        let page_backing = pages.checked_mul(PREPARED_ATLAS_PAGE_BYTES).ok_or_else(|| fault("atlas page backing credits exhausted"))?;
        let backing_bytes = slot_backing.checked_add(page_backing).and_then(|bytes| bytes.checked_add(size_of::<PreparedAtlasAbandonment>())).ok_or_else(|| fault("atlas aggregate backing credits exhausted"))?;
        let progress = atlas_progress(size_of::<PreparedAtlasAbandonment>() + size_of::<Self>() + size_of::<u64>() * 2 + size_of::<u8>(), size_of::<PreparedAtlasAbandonment>(), 0);
        if !progress.fits(authority.normal) || authority.normal.maximum_depth == 0 { return Err(fault("atlas original header authority exhausted")); }
        if authority.retirement.maximum_items == 0 || authority.retirement.maximum_copy_bytes < size_of::<*mut PreparedAtlasAbandonment>() + size_of::<u8>() + size_of::<bool>() * 3 || authority.retirement.maximum_depth == 0 { return Err(fault("atlas abandonment handoff authority exhausted")); }
        let Some(abandonment_slot) = PREPARED_ATLAS_ABANDONMENT_STATE.iter().position(|state| state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_ok()) else { return Err(fault("atlas process permit credits exhausted")); };
        let Some(permit) = PreparedAtlasPermit::try_reserve(pages, byte_len, backing_bytes) else {
            PREPARED_ATLAS_ABANDONMENT_STATE[abandonment_slot].store(0, Ordering::Release);
            return Err(fault("atlas process permit credits exhausted"));
        };
        let owner = Box::new(PreparedAtlasAbandonment { slots: None, len: 0, permit: Some(permit), authority, handoff_pending: false, closing: false });
        Ok((Self { owner: Some(owner), width, height, channels, byte_len, rows_per_page, abandonment_slot: abandonment_slot as u8, observed: progress }, progress))
    }

    pub fn admit_original_slots(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        let Some(owner) = self.owner.as_mut() else { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "atlas original header was released")); };
        if owner.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "atlas original owner is retiring")); }
        if owner.slots.is_some() { return Ok(RetainedCloneProgress::default()); }
        let bytes = size_of::<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>();
        let progress = atlas_progress(bytes + size_of::<Option<Box<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>>>(), bytes, 0);
        let grant = atlas_cap(grant, owner.authority.normal);
        if !progress.fits(grant) || grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "atlas original slot authority exhausted")); }
        let mut slots = Box::<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>::new_uninit();
        let pointer = slots.as_mut_ptr().cast::<Option<PreparedAtlasPage>>();
        for index in 0..PREPARED_ATLAS_PAGE_CAPACITY { unsafe { pointer.add(index).write(None) }; }
        owner.slots = Some(unsafe { slots.assume_init() });
        atlas_add(&mut self.observed, progress);
        Ok(progress)
    }

    pub fn slots_are_admitted(&self) -> bool { self.owner.as_ref().is_some_and(|owner| owner.slots.is_some()) }

    pub fn push_page(&mut self, source: &[u8], start_row: u32, grant: RetainedCloneGrant) -> Result<(bool, RetainedCloneProgress), ValueError> {
        let fault = |message| ValueError::literal(ValueRefusalKind::OwnershipLimit, message);
        let next_row = self.next_row();
        let owner = self.owner.as_mut().ok_or_else(|| fault("atlas original header was released"))?;
        if owner.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "atlas original owner is retiring")); }
        if owner.len == PREPARED_ATLAS_PAGE_CAPACITY || start_row >= self.height || start_row != next_row { return Err(fault("atlas page item credits exceeded")); }
        let row_bytes = usize::try_from(self.width).ok().and_then(|width| width.checked_mul(usize::from(self.channels))).ok_or_else(|| fault("atlas row bytes exhausted"))?;
        let rows = (self.height - start_row).min(self.rows_per_page);
        let bytes = usize::try_from(rows).ok().and_then(|rows| rows.checked_mul(row_bytes)).ok_or_else(|| fault("atlas page byte count exhausted"))?;
        let start = usize::try_from(start_row).ok().and_then(|row| row.checked_mul(row_bytes)).ok_or_else(|| fault("atlas page offset exhausted"))?;
        let slice = source.get(start..start.checked_add(bytes).ok_or_else(|| fault("atlas page end exhausted"))?).ok_or_else(|| fault("atlas page source was truncated"))?;
        let progress = atlas_progress(PREPARED_ATLAS_PAGE_BYTES + bytes + size_of::<Option<PreparedAtlasPage>>() + size_of::<usize>(), PREPARED_ATLAS_PAGE_BYTES, 0);
        let grant = atlas_cap(grant, owner.authority.normal);
        if !progress.fits(grant) || grant.maximum_depth == 0 { return Err(fault("atlas original page authority exhausted")); }
        let slots = owner.slots.as_mut().ok_or_else(|| fault("atlas slot backing was not admitted"))?;
        if slots[owner.len].is_some() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "atlas original page slot was occupied")); }
        let mut page = Box::<[u8; PREPARED_ATLAS_PAGE_BYTES]>::new_uninit();
        unsafe { page.as_mut_ptr().cast::<u8>().write_bytes(0, PREPARED_ATLAS_PAGE_BYTES) };
        let mut page = unsafe { page.assume_init() };
        page[..bytes].copy_from_slice(slice);
        slots[owner.len] = Some(PreparedAtlasPage { bytes: page, len: u16::try_from(bytes).map_err(|_| fault("atlas page length exhausted"))?, start_row, rows });
        owner.len += 1;
        atlas_add(&mut self.observed, progress);
        Ok((start_row.checked_add(rows) == Some(self.height), progress))
    }

    pub fn page(&self, index: usize) -> Option<(&[u8], u32, u32)> {
        let page = self.owner.as_ref()?.slots.as_ref()?.get(index)?.as_ref()?;
        Some((&page.bytes[..usize::from(page.len)], page.start_row, page.rows))
    }

    pub fn len(&self) -> usize { self.owner.as_ref().map_or(0, |owner| owner.len) }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
    pub fn next_row(&self) -> u32 { self.len().checked_sub(1).and_then(|index| self.owner.as_ref()?.slots.as_ref()?.get(index)?.as_ref()).and_then(|page| page.start_row.checked_add(page.rows)).unwrap_or(0) }
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn byte_len(&self) -> usize { self.byte_len }
    pub fn terminal_is_empty(&self) -> bool { self.owner.is_none() && self.abandonment_slot == u8::MAX }

    pub fn next_abandoned_close_demands(_copy: usize) -> Result<RetainedCloneGrant, ValueError> {
        for (state, owner) in PREPARED_ATLAS_ABANDONMENT_STATE.iter().zip(PREPARED_ATLAS_ABANDONMENT_OWNER.iter()) {
            if state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_err() { continue; }
            let pointer = owner.load(Ordering::Acquire);
            let result = if pointer.is_null() { Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "atlas original abandonment pointer was absent")) } else {
                let progress = unsafe { &*pointer }.original_close_progress();
                Ok(RetainedCloneGrant { maximum_items: progress.copied_items, maximum_copy_bytes: progress.copied_bytes, maximum_capacity_bytes: progress.retained_capacity_bytes, maximum_release_bytes: progress.released_bytes, maximum_depth: 1 })
            };
            state.store(2, Ordering::Release);
            return result;
        }
        Ok(RetainedCloneGrant::default())
    }

    pub fn close_abandoned_step(grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        for (state, owner) in PREPARED_ATLAS_ABANDONMENT_STATE.iter().zip(PREPARED_ATLAS_ABANDONMENT_OWNER.iter()) {
            if state.load(Ordering::Acquire) != 2 { continue; }
            if grant.maximum_items == 0 { return Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }); }
            if state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_err() { return Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }); }
            let pointer = owner.load(Ordering::Acquire);
            if pointer.is_null() { state.store(2, Ordering::Release); return atlas_refused(ValueRefusalKind::InvariantViolated); }
            let value = unsafe { &mut *pointer };
            let grant = atlas_cap(grant, value.authority.retirement);
            let progress = value.original_close_progress();
            if let Some(step) = atlas_gate(grant, progress) { state.store(2, Ordering::Release); return step; }
            if value.terminal_is_empty() && !value.handoff_pending {
                owner.store(std::ptr::null_mut(), Ordering::Release);
                drop(unsafe { Box::from_raw(pointer) });
                state.store(0, Ordering::Release);
                return Ok(CloseStep::Pending { progress });
            }
            let step = value.close_original_step(grant);
            state.store(2, Ordering::Release);
            return step;
        }
        if PREPARED_ATLAS_ABANDONMENT_STATE.iter().any(|state| state.load(Ordering::Acquire) == 3) { return Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }); }
        Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() })
    }
}

impl Drop for PreparedAtlasPages {
    fn drop(&mut self) {
        let Some(mut owner) = self.owner.take() else { return };
        let slot = usize::from(self.abandonment_slot);
        owner.handoff_pending = true;
        PREPARED_ATLAS_ABANDONMENT_OWNER[slot].store(Box::into_raw(owner), Ordering::Release);
        PREPARED_ATLAS_ABANDONMENT_STATE[slot].store(2, Ordering::Release);
        self.abandonment_slot = u8::MAX;
    }
}

impl PreparedRasterPages {
    pub(super) fn next_close_release_byte_demand(&self, key: &String) -> Result<usize, ValueError> {
        if !self.slots.is_empty() || !self.backing.is_empty() { return Ok(0); }
        if !self.backing_released { return prepared_vec_bytes(&self.backing); }
        if !key.is_empty() { return Ok(0); }
        if !self.key_released { return Ok(key.capacity()); }
        if self.close_phase == 0 { return prepared_vec_bytes(&self.slots); }
        Ok(0)
    }

    pub(super) fn close_with_key_granted(&mut self, key: &mut String, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let demand = self.next_close_release_byte_demand(key);
        if let Some(step) = prepared_close_gate(grant, self.terminal_is_empty() && key.capacity() == 0, demand.clone()) { return step; }
        let phase = self.close_phase;
        let credit = self.credit.is_some();
        self.retire_with_key_step(key);
        if phase == 10 && credit && self.credit.is_some() { return Ok(CloseStep::Blocked); }
        prepared_close_progress(demand.unwrap_or(0))
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn byte_len(&self) -> usize {
        self.byte_len
    }

    pub fn content_identity(&self) -> RasterContentIdentity {
        self.content_identity
    }

    pub fn source_generation(&self) -> PreparedRasterGeneration {
        self.source_generation
    }

    pub fn frame_generation(&self) -> u64 {
        self.frame_generation
    }

    pub fn page_for_row(&self, row: u32) -> Option<(&[u8], u32)> {
        if row >= self.height || self.rows_per_page == 0 || self.slots.len() != self.page_capacity {
            return None;
        }
        let logical = usize::try_from(row / self.rows_per_page).ok()?;
        let physical = self.page_capacity.checked_sub(logical.checked_add(1)?)?;
        let page = self.slots.get(physical)?;
        let row_bytes = usize::try_from(self.width).ok()?.checked_mul(4)?;
        let start = usize::try_from(page.start_row).ok()?.checked_mul(row_bytes)?;
        let len = usize::try_from(page.rows).ok()?.checked_mul(row_bytes)?;
        let bytes = self.backing.get(start..start.checked_add(len)?)?;
        (page.start_row == row).then_some((bytes, page.rows))
    }

    pub(super) fn retire_page_step(&mut self) -> bool {
        self.slots.pop().is_some()
    }

    pub(super) fn retire_backing_step(&mut self) -> bool {
        if !self.backing.is_empty() {
            self.backing.truncate(self.backing.len().saturating_sub(PREPARED_RASTER_PAGE_BYTES));
            return false;
        }
        if !self.backing_released {
            self.backing = Vec::new();
            self.backing_released = true;
            return false;
        }
        true
    }

    pub(super) fn retire_metadata_step(&mut self) -> bool {
        match self.close_phase {
            0 => self.slots = Vec::new(),
            1 => self.page_capacity = 0,
            2 => self.rows_per_page = 0,
            3 => self.width = 0,
            4 => self.height = 0,
            5 => self.byte_len = 0,
            6 => self.content_identity = RasterContentIdentity::pixels(0, 0, 0).expect("empty raster identity"),
            7 => self.source_generation = PreparedRasterGeneration::default(),
            8 => self.frame_generation = 0,
            9 => self.backing_released = true,
            10 => {
                let Some(credit) = self.credit.as_ref() else {
                    self.close_phase = 11;
                    return true;
                };
                let Ok(mut ledger) = PREPARED_RASTER_LEDGER.lock() else { return false };
                if !ledger.release(credit) {
                    return false;
                }
                self.credit = None;
            }
            _ => return true,
        }
        self.close_phase += 1;
        false
    }

    pub(super) fn retire_with_key_step(&mut self, key: &mut String) -> bool {
        if self.retire_page_step() {
            return false;
        }
        if !self.retire_backing_step() {
            return false;
        }
        if key.pop().is_some() {
            return false;
        }
        if !self.key_released {
            *key = String::new();
            self.key_released = true;
            return false;
        }
        self.retire_metadata_step()
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.close_phase >= 10 && self.slots.is_empty() && self.slots.capacity() == 0 && self.backing.is_empty() && self.backing.capacity() == 0 && self.backing_released && self.key_released && self.credit.is_none()
    }

    #[cfg(any(test, feature = "testkit"))]
    pub(super) fn page_pointer(&self, logical: usize) -> Option<*const u8> {
        self.page_for_row(u32::try_from(logical).ok()?.checked_mul(self.rows_per_page)?).map(|(bytes, _)| bytes.as_ptr())
    }
}
//#endregion 🧩️PagedRasterProducer
