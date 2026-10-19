//! 🔧️ Render job state and close: the render job, the preparation sections, the draw measure cursor, the job rejection and the job constructors, close and abandonment steps. See [`super`].

use super::*;

//#region ⚙️PreparationJob
/// ⚙️ Bounded worker job that measures and seals one owned render packet.
pub struct PreparedRenderJob {
    pub(super) has_animated_primitives: bool,
    pub input: Option<PreparedRenderInput>,
    pub(super) usage: PreparedRenderUsage,
    pub(super) section: PreparationSection,
    pub(super) draw_cursor: DrawMeasureCursor,
    pub(super) overlay_cursor: DrawMeasureCursor,
    pub(super) pipeline_cursor: DrawMeasureCursor,
    pub(super) pipeline_overlay_cursor: DrawMeasureCursor,
    pub(super) pipeline_overlay: bool,
    pub(super) metadata_cursor: usize,
    pub commands: Option<PreparedRenderCommandPages>,
    pub(super) digest: u64,
    pub(super) fault: Option<&'static str>,
    pub fault_payload: RetainedPayloadBuilder,
    pub(super) receiver: PreparedRenderReceiver,
    pub(super) rejected_upload: Option<PreparedRenderUpload>,
    pub(super) rejected_packet: Option<PreparedRenderPacket>,
    pub(super) raster_backing_closed: bool,
    pub(super) packet_command_staged: bool,
    pub(super) published: bool,
    pub(super) closing: bool,
    pub(super) abandonment_slot: u8,
}

#[derive(Clone, Copy)]
pub(super) enum PreparationSection {
    Draw,
    Overlay,
    Uploads,
    Evictions,
    Damage,
    Clips,
    Directives,
    Validate,
    Snap,
    Order,
    Tessellate,
    Batch,
    Hash,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawMeasureCursor {
    LayerHeader(usize),
    LayerUi { layer: usize, item: usize, overlay: bool },
    LayerVector { layer: usize, item: usize, overlay: bool },
    LayerRaster { layer: usize, raster: usize, overlay: bool },
    LayerRasterKey { layer: usize, raster: usize, byte: usize, overlay: bool },
    PassHeader(usize),
    PassBackdropSnapshot { pass: usize },
    PassSceneClear { pass: usize },
    PassShadowBegin(usize),
    PassShadowInstance { pass: usize, draw: usize, instance: usize },
    PassDraw { pass: usize, draw: usize, translucent: bool },
    PassDrawKey { pass: usize, draw: usize, byte: usize, translucent: bool },
    PassInstance { pass: usize, draw: usize, instance: usize, translucent: bool },
    PassInstanceKey { pass: usize, draw: usize, instance: usize, byte: usize, translucent: bool },
    PassLine { pass: usize, draw: usize },
    PassLineVertex { pass: usize, draw: usize, vertex: usize },
    PassTextured { pass: usize, draw: usize },
    PassTexturedInstance { pass: usize, draw: usize, instance: usize },
    PassTexturedKey { pass: usize, draw: usize, instance: usize, byte: usize },
    PassGrid { pass: usize },
    PassMaterial { pass: usize, draw: usize, translucent: bool },
    PassMaterialMeshKey { pass: usize, draw: usize, byte: usize, translucent: bool },
    PassMaterialTextureKey { pass: usize, draw: usize, byte: usize, translucent: bool },
    PassMaterialInstance { pass: usize, draw: usize, instance: usize, translucent: bool },
    PassMaterialInstanceKey { pass: usize, draw: usize, instance: usize, byte: usize, translucent: bool },
    PassPostprocess { pass: usize },
    Glass(usize),
    Complete,
}

impl Default for DrawMeasureCursor {
    fn default() -> Self {
        Self::LayerHeader(0)
    }
}

/// 🛡️ Exact input returned when the fixed packet mailbox refuses a job.
pub struct PreparedRenderJobRejected {
    pub(super) fault: &'static str,
    pub(super) input: Option<PreparedRenderInput>,
}

impl PreparedRenderJobRejected {
    pub fn fault(&self) -> &'static str {
        self.fault
    }

    pub fn take_rejected(&mut self) -> Option<PreparedRenderInput> {
        self.input.take()
    }

    pub fn next_close_demands(&self,copy:usize)->Result<RetainedCloneGrant,ValueError>{match self.input.as_ref(){Some(input)if !input.terminal_is_empty()=>input.next_close_demands(copy),_=>prepared_owner_demands(self.terminal_is_empty(),Ok(0))}}

    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<CloseStep,ValueError>{
        let Some(input)=self.input.as_mut()else{return Ok(CloseStep::Complete{progress:Default::default()})};
        if !input.terminal_is_empty(){return prepared_close_child(input.close_step(grant))}
        if grant.maximum_items==0{return Ok(CloseStep::Pending{progress:Default::default()})}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original rejected render input header requires depth"))}
        self.input=None;
        Ok(CloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.input.is_none()
    }
}

impl PreparedRenderJob {
    pub(super) fn next_close_demands(&self, copy: usize) -> Result<RetainedCloneGrant, ValueError> {
        if let Some(upload) = self.rejected_upload.as_ref() { if !upload.terminal_is_empty() { return upload.next_close_demands(copy); } }
        else if let Some(packet) = self.rejected_packet.as_ref() { if !packet.retirement_is_empty() { return packet.next_close_demands(copy); } }
        else if self.commands.is_none() {
            if let Some(input) = self.input.as_ref() { if !input.terminal_is_empty() { return input.next_close_demands(copy); } }
            else if !self.receiver.terminal_is_empty() { return self.receiver.next_close_demands(copy); }
        }
        if self.rejected_upload.is_none() && self.rejected_packet.is_none() && self.commands.is_none() && self.input.is_none() && self.receiver.terminal_is_empty() && !self.fault_payload.terminal_is_empty() {
            let demand = self.fault_payload.retirement_demands()?;
            return Ok(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth });
        }
        prepared_owner_demands(self.terminal_is_empty(), self.next_close_release_byte_demand())
    }

    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if let Some(upload) = self.rejected_upload.as_ref() { return if upload.terminal_is_empty() { Ok(0) } else { upload.next_close_release_byte_demand() } }
        if let Some(packet) = self.rejected_packet.as_ref() { return if packet.retirement_is_empty() { Ok(0) } else { packet.next_close_release_byte_demand() } }
        if let Some(commands) = self.commands.as_ref() { return commands.next_close_release_byte_demand() }
        if let Some(input) = self.input.as_ref() { return input.next_close_release_byte_demand() }
        if !self.receiver.terminal_is_empty() { return self.receiver.next_close_release_byte_demand(); }
        self.fault_payload.retirement_demands().map(|demand| demand.release_bytes)
    }
    pub(super) fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        if !self.closing { return Ok(CloseStep::Blocked); }
        if self.terminal_is_empty() { return Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() }); }
        if grant.maximum_items == 0 { return Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original prepared render job close requires depth")); }
        if let Some(upload) = self.rejected_upload.as_mut() {
            if !upload.terminal_is_empty() { return prepared_close_child(upload.close_granted(grant)) }
            self.rejected_upload = None; return prepared_close_progress(0)
        }
        if let Some(packet) = self.rejected_packet.as_mut() {
            if !packet.retirement_is_empty() { return prepared_close_child(packet.close_granted(grant)) }
            self.rejected_packet = None; return prepared_close_progress(0)
        }
        if let Some(commands) = self.commands.as_mut() {
            if !commands.terminal_is_empty() { return prepared_close_child(commands.close_granted(grant)) }
            self.commands = None; return prepared_close_progress(0)
        }
        if let Some(input) = self.input.as_mut() {
            if !input.terminal_is_empty() { return prepared_close_child(input.close_granted(grant)) }
            self.input = None; return prepared_close_progress(0)
        }
        if !self.receiver.terminal_is_empty() { return prepared_close_child(self.receiver.close_granted(grant)) }
        if !self.fault_payload.terminal_is_empty() {
            return self.fault_payload.close_step_granted(grant).map(|step|CloseStep::Pending { progress: step.progress() });
        }
        if self.abandonment_slot != u8::MAX {
            let Some(state) = PREPARED_RENDER_JOB_ABANDONMENT_STATE.get(usize::from(self.abandonment_slot)) else { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original prepared render job abandonment slot is absent")); };
            let current = state.load(Ordering::Acquire);
            if !matches!(current, 1 | 3) || state.compare_exchange(current, if current == 3 { 3 } else { 0 }, Ordering::AcqRel, Ordering::Acquire).is_err() { return Ok(CloseStep::Blocked); }
            self.abandonment_slot = u8::MAX; return prepared_close_progress(0)
        }
        Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() })
    }

    /// 🩺️ The exact reason this job refused, once it has refused — the string [`Self::fault_outcome`]
    /// recorded. The original held fault detail carries no text pages here (a fixed `&'static str`
    /// needs no page), so without this accessor a preparation fault is completely invisible to its
    /// driver, which is exactly how one silent refusal froze the wgpu shell at one presented frame per
    /// boot (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w7b-presenter-one-frame-per-boot.md`).
    pub fn fault(&self) -> Option<&'static str> {
        self.fault
    }

    #[expect(clippy::result_large_err, reason = "Job admission failure retains the exact input and its credits for incremental retirement without allocating.")]
    pub fn try_new(mut input: PreparedRenderInput) -> Result<Self, PreparedRenderJobRejected> {
        let Some(receiver) = PreparedRenderReceiver::try_reserve() else {
            return Err(PreparedRenderJobRejected { fault: "prepared render packet mailbox exhausted", input: Some(input) });
        };
        let Some(slot) = input.permit.as_ref().map(|permit| usize::from(permit.slot)) else {
            return Err(PreparedRenderJobRejected { fault: "prepared render process owner was missing", input: Some(input) });
        };
        let Some(state) = PREPARED_RENDER_JOB_ABANDONMENT_STATE.get(slot) else {
            return Err(PreparedRenderJobRejected { fault: "prepared render abandonment slot was missing", input: Some(input) });
        };
        if state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(PreparedRenderJobRejected { fault: "prepared render abandonment slot was occupied", input: Some(input) });
        }
        let input_slot = usize::from(input.abandonment_slot);
        if input_slot != slot || PREPARED_RENDER_INPUT_ABANDONMENT_STATE[input_slot].compare_exchange(1, 0, Ordering::AcqRel, Ordering::Acquire).is_err() {
            state.store(0, Ordering::Release);
            return Err(PreparedRenderJobRejected { fault: "prepared render input handoff generation was stale", input: Some(input) });
        }
        input.abandonment_slot = u8::MAX;
        Ok(Self::from_admitted(input, receiver, slot as u8))
    }

    fn from_admitted(input: PreparedRenderInput, receiver: PreparedRenderReceiver, abandonment_slot: u8) -> Self {
        Self {
            has_animated_primitives: false,
            input: Some(input),
            usage: PreparedRenderUsage::default(),
            section: PreparationSection::Draw,
            draw_cursor: DrawMeasureCursor::default(),
            overlay_cursor: DrawMeasureCursor::default(),
            pipeline_cursor: DrawMeasureCursor::default(),
            pipeline_overlay_cursor: DrawMeasureCursor::default(),
            pipeline_overlay: false,
            metadata_cursor: 0,
            commands: Some(PreparedRenderCommandPages::default()),
            digest: 0xcbf29ce484222325,
            fault: None,
            fault_payload: RetainedPayloadBuilder::new(JobPayloadStream::Fault),
            receiver,
            rejected_upload: None,
            rejected_packet: None,
            raster_backing_closed: false,
            packet_command_staged: false,
            published: false,
            closing: false,
            abandonment_slot,
        }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn new(input: PreparedRenderInput, _items_per_step: usize) -> Self {
        match Self::try_new(input) {
            Ok(job) => job,
            Err(_) => panic!("test prepared render mailbox must admit one job"),
        }
    }

    pub fn receiver(&self) -> Option<PreparedRenderReceiver> {
        self.receiver.try_clone()
    }

    pub fn take_packet(&self) -> Option<PreparedRenderPacket> {
        self.receiver.take_latest()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.input.is_none() && self.commands.is_none() && self.rejected_upload.is_none() && self.rejected_packet.is_none() && self.receiver.terminal_is_empty() && self.fault_payload.terminal_is_empty() && self.abandonment_slot == u8::MAX
    }

    /// 🧹 Advances one exact job owner recovered after an interrupted worker execution.

    pub fn next_abandoned_close_demands(maximum_copy_bytes: usize) -> Result<RetainedCloneGrant, ValueError> {
        let Some(index) = PREPARED_RENDER_JOB_ABANDONMENT_STATE.iter().position(|state| state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_ok()) else { return Ok(RetainedCloneGrant::default()) };
        let pointer = PREPARED_RENDER_JOB_ABANDONMENT_OWNER[index].load(Ordering::Acquire);
        let result = if pointer.is_null() { Ok(RetainedCloneGrant::default()) } else {
            let job = unsafe { &*pointer };
            if job.terminal_is_empty() { prepared_owner_demands(false, Ok(size_of::<Self>())) }
            else { job.next_close_demands(maximum_copy_bytes) }
        };
        PREPARED_RENDER_JOB_ABANDONMENT_STATE[index].store(2, Ordering::Release);
        result
    }
    pub fn close_abandoned_step(grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let Some(slot) = PREPARED_RENDER_JOB_ABANDONMENT_STATE.iter().position(|state| state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_ok()) else { return Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() }); };
        let pointer = PREPARED_RENDER_JOB_ABANDONMENT_OWNER[slot].swap(std::ptr::null_mut(), Ordering::AcqRel);
        if pointer.is_null() { PREPARED_RENDER_JOB_ABANDONMENT_STATE[slot].store(2, Ordering::Release); return Ok(CloseStep::Blocked); }
        let mut job = unsafe { Box::from_raw(pointer) };
        let step = if job.terminal_is_empty() {
            if grant.maximum_items == 0 || grant.maximum_release_bytes < size_of::<Self>() { Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }) }
            else if grant.maximum_depth == 0 { Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original abandoned prepared job close requires depth")) }
            else {
                drop(job); PREPARED_RENDER_JOB_ABANDONMENT_STATE[slot].store(0, Ordering::Release);
                return prepared_close_progress(size_of::<Self>())
            }
        } else { job.closing = true; prepared_close_child(job.close_granted(grant)) };
        PREPARED_RENDER_JOB_ABANDONMENT_OWNER[slot].store(Box::into_raw(job), Ordering::Release);
        PREPARED_RENDER_JOB_ABANDONMENT_STATE[slot].store(2, Ordering::Release);
        step
    }

    pub(super) fn input(&self) -> Option<&PreparedRenderInput> {
        self.input.as_ref()
    }

}
//#endregion ⚙️PreparationJob
