//! 📮️ Render input and mailbox: the prepared render input and its rejection, the mailbox slots and the receiver. See [`super`].

use super::*;

/// 🧰️ Owned inputs consumed by the resumable preparation job.
pub struct PreparedRenderInput {
    pub scene_revision: u64,
    pub preview_generation: u64,
    pub damage: PreparedRenderScissors,
    pub clips: PreparedRenderScissors,
    pub directives: PreparedRenderDirectives,
    pub uploads: PreparedRenderUploads,
    pub raster_producers: PreparedRasterProducers,
    pub evictions: PreparedRenderEvictions,
    pub draw: DrawList,
    pub overlay: Option<DrawList>,
    pub time_seconds: f32,
    pub limits: PreparedRenderLimits,
    pub(super) permit: Option<PreparedRenderProcessPermit>,
    pub abandonment_slot: u8,
}

/// 🛡️ Exact input owner returned when the process/page reservation refuses admission.
pub struct PreparedRenderInputRejected {
    pub(super) fault: &'static str,
    pub(super) draw: Option<DrawList>,
    pub(super) overlay: Option<DrawList>,
    pub(super) permit: Option<PreparedRenderProcessPermit>,
}

impl PreparedRenderInputRejected {
    pub fn fault(&self) -> &'static str {
        self.fault
    }

    pub fn take_draw(&mut self) -> Option<DrawList> {
        self.draw.take()
    }

    pub fn take_overlay(&mut self) -> Option<DrawList> {
        self.overlay.take()
    }

    pub fn next_close_demands(&self,_copy:usize)->Result<RetainedCloneGrant,ValueError>{
        let release=if let Some(draw)=self.draw.as_ref(){draw.next_retirement_release_byte_demand()?}else if let Some(draw)=self.overlay.as_ref(){draw.next_retirement_release_byte_demand()?}else{0};
        prepared_owner_demands(self.terminal_is_empty(),Ok(release))
    }

    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<CloseStep,ValueError>{
        let release=self.next_close_demands(grant.maximum_copy_bytes)?.maximum_release_bytes;
        if let Some(step)=prepared_close_gate(grant,self.terminal_is_empty(),Ok(release)){return step}
        if let Some(draw)=self.draw.as_mut(){if !draw.retirement_is_empty(){return prepared_close_child(draw.retire_granted(grant))}self.draw=None;return prepared_close_progress(0)}
        if let Some(draw)=self.overlay.as_mut(){if !draw.retirement_is_empty(){return prepared_close_child(draw.retire_granted(grant))}self.overlay=None;return prepared_close_progress(0)}
        if let Some(permit)=self.permit.as_mut(){let phase=permit.release_phase;if permit.release_step(){self.permit=None}else if permit.release_phase==phase{return Ok(CloseStep::Blocked)}return prepared_close_progress(0)}
        Ok(CloseStep::Complete{progress:Default::default()})
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.draw.is_none() && self.overlay.is_none() && self.permit.is_none()
    }
}

impl PreparedRenderInput {
    pub fn next_close_demands(&self, copy: usize) -> Result<RetainedCloneGrant, ValueError> {
        if let Some(upload) = self.uploads.get(self.uploads.len().saturating_sub(1)) { if !upload.terminal_is_empty() { return upload.next_close_demands(copy); } }
        let mut demand = prepared_owner_demands(self.terminal_is_empty(), self.next_close_release_byte_demand())?;
        demand.maximum_copy_bytes = self.next_close_copy_byte_demand()?;
        Ok(demand)
    }

    pub(super) fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { self.uploads.get(self.uploads.len().saturating_sub(1)).map_or(Ok(0), PreparedRenderUpload::next_close_copy_byte_demand) }

    pub(super) fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if let Some(upload) = self.uploads.get(self.uploads.len().saturating_sub(1)) { return upload.next_close_release_byte_demand(); }
        if let Some(producer) = self.raster_producers.get(self.raster_producers.len().saturating_sub(1)) { return producer.next_close_release_byte_demand(); }
        if let Some(PreparedRenderEviction::Mesh { key }) = self.evictions.get(self.evictions.len().saturating_sub(1)) { return Ok(if key.is_empty() { key.capacity() } else { 0 }); }
        if !self.draw.retirement_is_empty() { return self.draw.next_retirement_release_byte_demand(); }
        if let Some(overlay) = self.overlay.as_ref() { return overlay.next_retirement_release_byte_demand(); }
        
        if !self.damage.is_empty() || !self.clips.is_empty() || !self.directives.is_empty() { return Ok(0); }
        if !self.uploads.terminal_is_empty() { return self.uploads.next_close_release_byte_demand(); }
        if !self.raster_producers.terminal_is_empty() { return self.raster_producers.next_close_release_byte_demand(); }
        if !self.evictions.terminal_is_empty() { return self.evictions.next_close_release_byte_demand(); }
        if !self.damage.terminal_is_empty() { return self.damage.next_close_release_byte_demand(); }
        if !self.clips.terminal_is_empty() { return self.clips.next_close_release_byte_demand(); }
        if !self.directives.terminal_is_empty() { return self.directives.next_close_release_byte_demand(); }
        Ok(0)
    }

    pub(super) fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let demand = self.next_close_release_byte_demand();
        if let Some(step) = prepared_close_gate(grant, self.terminal_is_empty(), demand) { return step; }
        if let Some(upload) = self.uploads.last_mut() {
            if !upload.terminal_is_empty() { return prepared_close_child(upload.close_granted(grant)); }
            self.uploads.pop(); return prepared_close_progress(0);
        }
        if let Some(producer) = self.raster_producers.last_mut() {
            producer.begin_close();
            if !producer.terminal_is_empty() { return prepared_close_child(producer.close_granted(grant)); }
            self.raster_producers.pop(); return prepared_close_progress(0);
        }
        if let Some(PreparedRenderEviction::Mesh { key }) = self.evictions.last_mut() {
            if !key.is_empty() || key.capacity() != 0 { return prepared_close_progress(prepared_close_text(key)); }
            self.evictions.pop(); return prepared_close_progress(0);
        }
        if !self.draw.retirement_is_empty() { return prepared_close_child(self.draw.retire_granted(grant)); }
        if let Some(overlay) = self.overlay.as_mut() {
            if !overlay.retirement_is_empty() { return prepared_close_child(overlay.retire_granted(grant)); }
            self.overlay = None; return prepared_close_progress(0);
        }
        
        if self.damage.pop().is_some() || self.clips.pop().is_some() || self.directives.pop().is_some() { return prepared_close_progress(0); }
        if !self.uploads.terminal_is_empty() { return prepared_close_child(self.uploads.close_backing_granted(grant)); }
        if !self.raster_producers.terminal_is_empty() { return prepared_close_child(self.raster_producers.close_backing_granted(grant)); }
        if !self.evictions.terminal_is_empty() { return prepared_close_child(self.evictions.close_backing_granted(grant)); }
        if !self.damage.terminal_is_empty() { return prepared_close_child(self.damage.close_backing_granted(grant)); }
        if !self.clips.terminal_is_empty() { return prepared_close_child(self.clips.close_backing_granted(grant)); }
        if !self.directives.terminal_is_empty() { return prepared_close_child(self.directives.close_backing_granted(grant)); }
        if let Some(permit) = self.permit.as_mut() {
            let phase = permit.release_phase;
            if permit.release_step() { self.permit = None; }
            else if permit.release_phase == phase { return Ok(CloseStep::Blocked); }
            return prepared_close_progress(0);
        }
        if self.abandonment_slot != u8::MAX {
            let Some(state) = PREPARED_RENDER_INPUT_ABANDONMENT_STATE.get(usize::from(self.abandonment_slot)) else { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original prepared input abandonment slot is absent")); };
            let current = state.load(Ordering::Acquire);
            if !matches!(current, 1 | 3) || state.compare_exchange(current, if current == 3 { 3 } else { 0 }, Ordering::AcqRel, Ordering::Acquire).is_err() { return Ok(CloseStep::Blocked); }
            self.abandonment_slot = u8::MAX; return prepared_close_progress(0);
        }
        prepared_close_progress(0)
    }

    #[expect(clippy::result_large_err, reason = "Admission failure retains both exact draw owners for incremental retirement; boxing would allocate on refusal.")]
    pub fn try_new(scene_revision: u64, preview_generation: u64, draw: DrawList, overlay: Option<DrawList>, time_seconds: f32) -> Result<Self, PreparedRenderInputRejected> {
        let limits = PreparedRenderLimits::default();
        let (draw_items, draw_bytes) = draw.prepared_output_usage();
        let (overlay_items, overlay_bytes) = overlay.as_ref().map_or((0, 0), DrawList::prepared_output_usage);
        let Some(items) = draw_items.checked_add(overlay_items) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render item claim overflowed", draw: Some(draw), overlay, permit: None });
        };
        let Some(bytes) = draw_bytes.checked_add(overlay_bytes).and_then(|value| value.checked_add(limits.max_upload_bytes)) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render byte claim overflowed", draw: Some(draw), overlay, permit: None });
        };
        let Some(maximum_bytes) = limits.max_draw_bytes.checked_add(limits.max_upload_bytes) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render byte limit overflowed", draw: Some(draw), overlay, permit: None });
        };
        if scene_revision == 0 || scene_revision == u64::MAX || preview_generation == 0 || preview_generation == u64::MAX || items > limits.max_draw_items || bytes > maximum_bytes {
            return Err(PreparedRenderInputRejected { fault: "prepared render source exceeded fixed revision, item, or byte credits", draw: Some(draw), overlay, permit: None });
        }
        let Some(pages) = prepared_render_units(bytes).and_then(|pages| pages.checked_add(PREPARED_RENDER_COMMAND_PAGES)) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render page claim overflowed", draw: Some(draw), overlay, permit: None });
        };
        let metadata = size_of::<Option<PreparedRenderUpload>>()
            .checked_mul(PREPARED_RENDER_METADATA_ITEMS)
            .and_then(|value| value.checked_add(size_of::<Option<PreparedRenderEviction>>().checked_mul(PREPARED_RENDER_METADATA_ITEMS)?))
            .and_then(|value| value.checked_add(size_of::<Option<ScissorRect>>().checked_mul(PREPARED_RENDER_METADATA_ITEMS.checked_mul(2)?)?))
            .and_then(|value| value.checked_add(size_of::<Option<RenderDirective>>().checked_mul(PREPARED_RENDER_METADATA_ITEMS)?))
            .and_then(|value| value.checked_add(size_of::<Option<PreparedRasterProducer>>().checked_mul(PREPARED_RENDER_METADATA_ITEMS)?))
            .and_then(|value| value.checked_add(size_of::<PreparedRenderCommandDirectory>().checked_mul(PREPARED_RENDER_COMMAND_DIRECTORIES)?))
            .and_then(|value| value.checked_add(size_of::<PreparedRenderCommandPage>().checked_mul(PREPARED_RENDER_COMMAND_PAGES)?));
        let Some(backing_bytes) = metadata.and_then(|metadata| bytes.checked_add(metadata)) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render backing claim overflowed", draw: Some(draw), overlay, permit: None });
        };
        let Some(permit) = PreparedRenderProcessPermit::try_reserve(pages, backing_bytes) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render process permits exhausted", draw: Some(draw), overlay, permit: None });
        };
        let slot = usize::from(permit.slot);
        if PREPARED_RENDER_INPUT_ABANDONMENT_STATE[slot].compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(PreparedRenderInputRejected { fault: "prepared render input abandonment slot was occupied", draw: Some(draw), overlay, permit: Some(permit) });
        }
        let mut directives = PreparedRenderDirectives::default();
        if directives.try_push(RenderDirective::PreservePreviousOnFailure).is_err() {
            PREPARED_RENDER_INPUT_ABANDONMENT_STATE[slot].store(0, Ordering::Release);
            return Err(PreparedRenderInputRejected { fault: "prepared render directive admission failed", draw: Some(draw), overlay, permit: Some(permit) });
        }
        Ok(Self {
            scene_revision,
            preview_generation,
            damage: PreparedRenderScissors::default(),
            clips: PreparedRenderScissors::default(),
            directives,
            uploads: PreparedRenderUploads::default(),
            raster_producers: PreparedRasterProducers::default(),
            evictions: PreparedRenderEvictions::default(),
            draw,
            overlay,
            time_seconds,
            limits,
            permit: Some(permit),
            abandonment_slot: slot as u8,
        })
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn new(scene_revision: u64, preview_generation: u64, draw: DrawList, overlay: Option<DrawList>, time_seconds: f32) -> Self {
        match Self::try_new(scene_revision, preview_generation, draw, overlay, time_seconds) {
            Ok(input) => input,
            Err(rejected) => {
                let ledger = PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire);
                panic!(
                    "test prepared input must fit fixed process permits: {} (held items {}/{PREPARED_RENDER_PROCESS_SLOTS} pages {}/{PREPARED_RENDER_PROCESS_PAGES} backing-units {})",
                    rejected.fault,
                    (ledger >> PREPARED_RENDER_ITEM_SHIFT) & PREPARED_RENDER_ITEM_MASK,
                    (ledger >> PREPARED_RENDER_PAGE_SHIFT) & PREPARED_RENDER_PAGE_MASK,
                    (ledger >> PREPARED_RENDER_BACKING_SHIFT) & PREPARED_RENDER_BACKING_MASK
                )
            }
        }
    }

    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Rejected preparation returns its exact source, key, and reservation credit without allocating beyond admission."))]
    pub fn try_push_upload(&mut self, upload: PreparedRenderUpload) -> Result<(), PreparedRenderUpload> {
        if self.raster_producers.len().checked_add(self.uploads.len()).is_none_or(|items| items >= self.limits.max_upload_items) {
            return Err(upload);
        }
        self.uploads.try_push(upload)
    }

    #[expect(clippy::result_large_err, reason = "The full producer is returned unchanged when fixed slots are exhausted; refusal must not allocate.")]
    pub fn try_push_raster_producer(&mut self, producer: PreparedRasterProducer) -> Result<(), PreparedRasterProducer> {
        if self.raster_producers.len().checked_add(self.uploads.len()).is_none_or(|items| items >= self.limits.max_upload_items) {
            return Err(producer);
        }
        self.raster_producers.try_push(producer)
    }

    pub fn try_push_eviction(&mut self, eviction: PreparedRenderEviction) -> Result<(), PreparedRenderEviction> {
        self.evictions.try_push(eviction)
    }

    /// 🧱 Admits and transfers the final retained draw owners before worker submission.
    #[expect(clippy::result_large_err, reason = "A rejected bind returns the exact draw and overlay for incremental retirement without another allocation.")]
    pub fn try_bind_draw(&mut self, draw: DrawList, overlay: Option<DrawList>) -> Result<(), PreparedRenderInputRejected> {
        if !self.draw.retirement_is_empty() || self.overlay.is_some() {
            return Err(PreparedRenderInputRejected { fault: "prepared render draw owner was already bound", draw: Some(draw), overlay, permit: None });
        }
        let (draw_items, draw_bytes) = draw.prepared_output_usage();
        let (overlay_items, overlay_bytes) = overlay.as_ref().map_or((0, 0), DrawList::prepared_output_usage);
        let Some(items) = draw_items.checked_add(overlay_items) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render draw item claim overflowed", draw: Some(draw), overlay, permit: None });
        };
        let Some(bytes) = draw_bytes.checked_add(overlay_bytes) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render draw byte claim overflowed", draw: Some(draw), overlay, permit: None });
        };
        if items > self.limits.max_draw_items || bytes > self.limits.max_draw_bytes {
            return Err(PreparedRenderInputRejected { fault: "prepared render draw owner exceeded fixed credits", draw: Some(draw), overlay, permit: None });
        }
        let Some(pages) = prepared_render_units(bytes) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render draw page claim overflowed", draw: Some(draw), overlay, permit: None });
        };
        if !self.permit.as_mut().is_some_and(|permit| permit.try_grow(pages, bytes)) {
            return Err(PreparedRenderInputRejected { fault: "prepared render draw process credits were refused", draw: Some(draw), overlay, permit: None });
        }
        self.draw = draw;
        self.overlay = overlay;
        Ok(())
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        self.close_granted(grant)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.uploads.terminal_is_empty()
            && self.raster_producers.terminal_is_empty()
            && self.evictions.terminal_is_empty()
            && self.damage.terminal_is_empty()
            && self.clips.terminal_is_empty()
            && self.directives.terminal_is_empty()
            && self.draw.retirement_is_empty()
            && self.overlay.is_none()
            && self.permit.is_none()
            && self.abandonment_slot == u8::MAX
    }

    /// 🧹 Advances one exact pre-submission input owner recovered after interruption.
    pub fn next_abandoned_close_demands(copy: usize) -> Result<RetainedCloneGrant, ValueError> {
        prepared_abandoned_demands(PREPARED_RENDER_INPUT_ABANDONMENT_STATE.iter().zip(PREPARED_RENDER_INPUT_ABANDONMENT_OWNER.iter()), 2, copy, Self::next_close_demands, Self::terminal_is_empty)
    }

    pub fn close_abandoned_step(grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        prepared_abandoned_close(PREPARED_RENDER_INPUT_ABANDONMENT_STATE.iter().zip(PREPARED_RENDER_INPUT_ABANDONMENT_OWNER.iter()), 2, grant, Self::terminal_is_empty, Self::close_granted)
    }
}

impl Drop for PreparedRenderInput {
    fn drop(&mut self) {
        if self.terminal_is_empty() || self.abandonment_slot == u8::MAX {
            return;
        }
        let slot = usize::from(self.abandonment_slot);
        let Some(state) = PREPARED_RENDER_INPUT_ABANDONMENT_STATE.get(slot) else { return };
        if state.load(Ordering::Acquire) != 1 {
            return;
        }
        let input = Box::new(Self {
            scene_revision: std::mem::take(&mut self.scene_revision),
            preview_generation: std::mem::take(&mut self.preview_generation),
            damage: std::mem::take(&mut self.damage),
            clips: std::mem::take(&mut self.clips),
            directives: std::mem::take(&mut self.directives),
            uploads: std::mem::take(&mut self.uploads),
            raster_producers: std::mem::take(&mut self.raster_producers),
            evictions: std::mem::take(&mut self.evictions),
            draw: std::mem::replace(&mut self.draw, DrawList::empty()),
            overlay: self.overlay.take(),
            time_seconds: std::mem::take(&mut self.time_seconds),
            limits: std::mem::take(&mut self.limits),
            permit: self.permit.take(),
            abandonment_slot: self.abandonment_slot,
        });
        self.abandonment_slot = u8::MAX;
        PREPARED_RENDER_INPUT_ABANDONMENT_OWNER[slot].store(Box::into_raw(input), Ordering::Release);
        state.store(2, Ordering::Release);
    }
}

//#region ⚙️PreparationJob
const PREPARED_RENDER_MAILBOX_SLOTS: usize = 64;

pub(super) struct PreparedRenderMailboxSlot {
    pub(super) state: AtomicU8,
    pub(super) generation: AtomicU64,
    references: AtomicUsize,
    pub packet: AtomicPtr<PreparedRenderPacket>,
}

impl PreparedRenderMailboxSlot {
    pub(super) const fn new() -> Self {
        Self { state: AtomicU8::new(0), generation: AtomicU64::new(0), references: AtomicUsize::new(0), packet: AtomicPtr::new(std::ptr::null_mut()) }
    }
}

pub static PREPARED_RENDER_MAILBOX: [PreparedRenderMailboxSlot; PREPARED_RENDER_MAILBOX_SLOTS] = [const { PreparedRenderMailboxSlot::new() }; PREPARED_RENDER_MAILBOX_SLOTS];

/// 📬️ Generation-qualified nonblocking capacity-one packet handoff.
pub struct PreparedRenderReceiver {
    pub(super) slot: u8,
    pub(super) generation: u64,
    owned: bool,
}

impl PreparedRenderReceiver {
    pub(super) fn next_close_demands(&self, copy: usize) -> Result<RetainedCloneGrant, ValueError> {
        if !self.owned { return Ok(RetainedCloneGrant::default()); }
        let Some(slot) = PREPARED_RENDER_MAILBOX.get(usize::from(self.slot)) else { return prepared_owner_demands(false, Ok(0)); };
        if slot.generation.load(Ordering::Acquire) != self.generation || matches!(slot.state.load(Ordering::Acquire), 0 | 1) { return prepared_owner_demands(false, Ok(0)); }
        if slot.state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_err() { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "prepared mailbox ownership is checked out")); }
        let pointer = slot.packet.load(Ordering::Acquire);
        let demand = if pointer.is_null() { prepared_owner_demands(false, Ok(0)) } else {
            let packet = unsafe { &*pointer };
            if packet.retirement_is_empty() { prepared_owner_demands(false, Ok(size_of::<PreparedRenderPacket>())) } else { packet.next_close_demands(copy) }
        };
        slot.state.store(2, Ordering::Release);
        demand
    }

    pub(super) fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.owned { return Ok(0); }
        let Some(slot) = PREPARED_RENDER_MAILBOX.get(usize::from(self.slot)) else { return Ok(0); };
        if slot.generation.load(Ordering::Acquire) != self.generation || matches!(slot.state.load(Ordering::Acquire), 0 | 1) { return Ok(0); }
        if slot.state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_err() { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "prepared mailbox ownership is checked out")); }
        let pointer = slot.packet.load(Ordering::Acquire);
        let demand = if pointer.is_null() { Ok(0) } else {
            let packet = unsafe { &*pointer };
            if packet.retirement_is_empty() { Ok(size_of::<PreparedRenderPacket>()) } else { packet.next_close_release_byte_demand() }
        };
        slot.state.store(2, Ordering::Release);
        demand
    }

    pub(super) fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        if !self.owned { return Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() }); }
        if grant.maximum_items == 0 { return Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original prepared mailbox close requires depth")); }
        let Some(slot) = PREPARED_RENDER_MAILBOX.get(usize::from(self.slot)) else { self.owned = false; return prepared_close_progress(0); };
        if slot.generation.load(Ordering::Acquire) != self.generation { self.owned = false; return prepared_close_progress(0); }
        if slot.state.load(Ordering::Acquire) == 1 && slot.packet.load(Ordering::Acquire).is_null() {
            let references = slot.references.fetch_sub(1, Ordering::AcqRel);
            if references == 1 { slot.state.store(0, Ordering::Release); }
            self.owned = false;
            return prepared_close_progress(0);
        }
        if slot.state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_err() { return Ok(CloseStep::Blocked); }
        let pointer = slot.packet.load(Ordering::Acquire);
        if pointer.is_null() { slot.state.store(1, Ordering::Release); return prepared_close_progress(0); }
        let packet = unsafe { &mut *pointer };
        let step = if packet.retirement_is_empty() {
            if grant.maximum_release_bytes < size_of::<PreparedRenderPacket>() { Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }) }
            else {
                slot.packet.store(std::ptr::null_mut(), Ordering::Release);
                unsafe { drop(Box::from_raw(pointer)); }
                slot.state.store(1, Ordering::Release);
                return prepared_close_progress(size_of::<PreparedRenderPacket>());
            }
        } else { prepared_close_child(packet.close_granted(grant)) };
        slot.state.store(2, Ordering::Release);
        step
    }

    pub(super) fn unowned() -> Self {
        Self { slot: u8::MAX, generation: 0, owned: false }
    }

    pub(super) fn try_reserve() -> Option<Self> {
        let slot = PREPARED_RENDER_MAILBOX.iter().position(|slot| slot.state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_ok())?;
        let current = PREPARED_RENDER_MAILBOX[slot].generation.load(Ordering::Acquire);
        let Some(generation) = current.checked_add(1) else {
            PREPARED_RENDER_MAILBOX[slot].state.store(0, Ordering::Release);
            return None;
        };
        if PREPARED_RENDER_MAILBOX[slot].generation.compare_exchange(current, generation, Ordering::AcqRel, Ordering::Acquire).is_err() {
            PREPARED_RENDER_MAILBOX[slot].state.store(0, Ordering::Release);
            return None;
        }
        PREPARED_RENDER_MAILBOX[slot].references.store(1, Ordering::Release);
        Some(Self { slot: slot as u8, generation, owned: true })
    }

    pub fn try_clone(&self) -> Option<Self> {
        let slot = PREPARED_RENDER_MAILBOX.get(usize::from(self.slot))?;
        if !self.owned || slot.generation.load(Ordering::Acquire) != self.generation || slot.state.load(Ordering::Acquire) == 0 {
            return None;
        }
        let references = slot.references.load(Ordering::Acquire);
        let next = references.checked_add(1)?;
        slot.references.compare_exchange(references, next, Ordering::AcqRel, Ordering::Acquire).ok()?;
        Some(Self { slot: self.slot, generation: self.generation, owned: true })
    }

    pub fn take_latest(&self) -> Option<PreparedRenderPacket> {
        let slot = PREPARED_RENDER_MAILBOX.get(usize::from(self.slot))?;
        if slot.generation.load(Ordering::Acquire) != self.generation || slot.state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        let pointer = slot.packet.swap(std::ptr::null_mut(), Ordering::AcqRel);
        slot.state.store(1, Ordering::Release);
        if pointer.is_null() {
            return None;
        }
        Some(*unsafe { Box::from_raw(pointer) })
    }

    #[expect(clippy::result_large_err, reason = "A stale or occupied mailbox returns the exact admitted packet instead of allocating an error owner.")]
    pub(super) fn publish(&self, packet: PreparedRenderPacket) -> Result<(), PreparedRenderPacket> {
        let Some(slot) = PREPARED_RENDER_MAILBOX.get(usize::from(self.slot)) else { return Err(packet) };
        if slot.generation.load(Ordering::Acquire) != self.generation || slot.state.load(Ordering::Acquire) != 1 {
            return Err(packet);
        }
        let pointer = Box::into_raw(Box::new(packet));
        if slot.packet.compare_exchange(std::ptr::null_mut(), pointer, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(*unsafe { Box::from_raw(pointer) });
        }
        slot.state.store(2, Ordering::Release);
        Ok(())
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        !self.owned
    }

    /// ⏭️ Advances one owner from one mailbox abandoned by its final handle.

    pub fn next_abandoned_close_demands(copy: usize) -> Result<RetainedCloneGrant, ValueError> {
        prepared_abandoned_demands(PREPARED_RENDER_MAILBOX.iter().map(|slot| (&slot.state, &slot.packet)), 4, copy, PreparedRenderPacket::next_close_demands, PreparedRenderPacket::retirement_is_empty)
    }
    pub fn close_abandoned_step(grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        prepared_abandoned_close(PREPARED_RENDER_MAILBOX.iter().map(|slot| (&slot.state, &slot.packet)), 4, grant, PreparedRenderPacket::retirement_is_empty, PreparedRenderPacket::close_granted)
    }

}

impl Drop for PreparedRenderReceiver {
    fn drop(&mut self) {
        if !self.owned {
            return;
        }
        let Some(slot) = PREPARED_RENDER_MAILBOX.get(usize::from(self.slot)) else { return };
        if slot.generation.load(Ordering::Acquire) != self.generation {
            self.owned = false;
            return;
        }
        let references = slot.references.fetch_sub(1, Ordering::AcqRel);
        if references == 1 {
            if slot.packet.load(Ordering::Acquire).is_null() {
                slot.state.store(0, Ordering::Release);
            } else {
                slot.state.store(4, Ordering::Release);
            }
        }
        self.owned = false;
    }
}
//#endregion ⚙️PreparationJob
