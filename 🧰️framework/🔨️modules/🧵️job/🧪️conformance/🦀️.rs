//! 🧵️ Canonical 🧪️conformance implementation under its own native owner.
use semio_framework_job::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_job_scope::JobScope;
//#region 🔥️TortureJob
/// 🎲️ A tiny, dependency-free xorshift64 step — deterministic given `x`, no allocation, no external
/// RNG crate (this crate stays zero-third-party-dependency, mirroring `semio_framework_trace`'s own
/// leaf-crate mandate). `| 1` on first seeding (see [`TortureJob::new`]) keeps the state off the
/// all-zero fixed point xorshift can never escape.
fn xorshift64(mut x: u64) -> u64 {
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

/// 🎲️ splitmix64 seed expansion — avalanches a caller-supplied `seed` into a well-mixed 64-bit state
/// before [`xorshift64`] ever sees it. Without this, [`TortureJob::new`]'s old plain `seed | 1` let
/// adjacent seeds (e.g. `42`/`43`) collapse onto the identical state (`|1` only ever touches bit 0),
/// which made two DIFFERENT seeds silently replay identical output — exactly the determinism bug this
/// conformance job exists to catch, so it must not carry one itself.
fn splitmix64(seed: u64) -> u64 {
    let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn read_u64_le(bytes: &[u8], cursor: &mut usize) -> u64 {
    let value = u64::from_le_bytes(bytes[*cursor..*cursor + 8].try_into().expect("TortureJob::from_checkpoint: truncated u64 field"));
    *cursor += 8;
    value
}

/// 🔥️ The Phase 2 conformance job (design ticket packet P2a item 7 / exit gate): long-running,
/// continuously preview-producing, checkpointable, cancellable, and deterministic given its seed —
/// every "unit" mixes a xorshift64 draw into an accumulator, cancellation and the fuel/deadline bound
/// are checked every unit, and every [`TortureJob::preview_every_units`]/[`TortureJob::checkpoint_every_units`]
/// units it returns [`StepOutcome::PreviewReady`]/[`StepOutcome::CheckpointReady`] instead of looping
/// further — so a caller sees continuous, real progress, not just a final answer. State is hand-rolled
/// little-endian bytes (design doc Decision 2's "opaque, job-encoded `Vec<u8>`" — this job has no
/// `RecordSpec` to hand `pack`'s schema-typed `encode_record_body` and stays zero-dependency, see
/// `📓️p2a-job-protocol.md`'s deviation note).
pub struct TortureJob {
    total_units: u64,
    completed_units: u64,
    rng_state: u64,
    accumulator: u64,
    checkpoint_every_units: u64,
    preview_every_units: u64,
    units_since_checkpoint: u64,
    units_since_preview: u64,
    pending_kind:Option<JobOutcomeKind>,
    pending_bytes:[u8;64],
    pending_length:usize,
    pending_cursor:usize,
    outcome_delivered:bool,
    terminal_stage:u8,
    payload:RetainedPayloadBuilder,
    terminal_state:RetainedPayloadBuilder,
    terminal_output:RetainedPayloadBuilder,
    scope: JobScope,
    closing: bool,
}

/// 🩺️ How many units [`TortureJob::step`] processes between cheap `should_yield` polls — small enough
/// that overshoot past the 8 ms ceiling within one check interval is negligible (each unit is a
/// handful of integer ops), large enough that the `now_us`/fuel check itself isn't the hot-loop
/// bottleneck.
const TORTURE_YIELD_CHECK_INTERVAL: u64 = 64;

impl TortureJob {
    pub fn new(seed: u64, total_units: u64, checkpoint_every_units: u64, preview_every_units: u64, parent_cancel: &CancelToken) -> TortureJob {
        TortureJob {
            total_units,
            completed_units: 0,
            rng_state: splitmix64(seed) | 1,
            accumulator: 0,
            checkpoint_every_units,
            preview_every_units,
            units_since_checkpoint: 0,
            units_since_preview: 0,
            pending_kind:None,pending_bytes:[0;64],pending_length:0,pending_cursor:0,outcome_delivered:false,terminal_stage:0,
            payload:RetainedPayloadBuilder::new(JobPayloadStream::Preview),terminal_state:RetainedPayloadBuilder::new(JobPayloadStream::CommitState),terminal_output:RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput),
            scope: JobScope::child_of(parent_cancel),
            closing: false,
        }
    }

    pub fn completed_units(&self) -> u64 {
        self.completed_units
    }

    pub fn total_units(&self) -> u64 {
        self.total_units
    }

    fn checkpoint_bytes(&self) -> [u8; 48] {
        let mut state = [0u8; 48];
        for (index, value) in [self.total_units, self.completed_units, self.rng_state, self.accumulator, self.checkpoint_every_units, self.preview_every_units].into_iter().enumerate() {
            state[index * 8..index * 8 + 8].copy_from_slice(&value.to_le_bytes());
        }
        state
    }

    /// 🔁️ Rebuilds a [`TortureJob`] from a [`Checkpoint::state`] produced by [`TortureJob::checkpoint`]
    /// — the resume half of the checkpoint → restore → resume conformance test. `parent_cancel` is
    /// supplied fresh (a restored job gets a NEW scope, same as any resumed operation reattaching to
    /// whatever scope owns it now).
    pub fn from_checkpoint(bytes: &[u8], parent_cancel: &CancelToken) -> TortureJob {
        let mut cursor = 0usize;
        let total_units = read_u64_le(bytes, &mut cursor);
        let completed_units = read_u64_le(bytes, &mut cursor);
        let rng_state = read_u64_le(bytes, &mut cursor);
        let accumulator = read_u64_le(bytes, &mut cursor);
        let checkpoint_every_units = read_u64_le(bytes, &mut cursor);
        let preview_every_units = read_u64_le(bytes, &mut cursor);
        TortureJob {
            total_units,
            completed_units,
            rng_state,
            accumulator,
            checkpoint_every_units,
            preview_every_units,
            units_since_checkpoint: 0,
            units_since_preview: 0,
            pending_kind:None,pending_bytes:[0;64],pending_length:0,pending_cursor:0,outcome_delivered:false,terminal_stage:0,
            payload:RetainedPayloadBuilder::new(JobPayloadStream::Preview),terminal_state:RetainedPayloadBuilder::new(JobPayloadStream::CommitState),terminal_output:RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput),
            scope: JobScope::child_of(parent_cancel),
            closing: false,
        }
    }

    fn encode_preview(&self, sequence: u64) -> [u8; 24] {
        let mut out = [0u8; 24];
        out[..8].copy_from_slice(&sequence.to_le_bytes());
        out[8..16].copy_from_slice(&self.completed_units.to_le_bytes());
        out[16..].copy_from_slice(&self.accumulator.to_le_bytes());
        out
    }

    fn prepare_original_scalar(&mut self,cx:&mut StepContext<'_>,kind:JobOutcomeKind,source:&[u8])->Result<bool,ValueError>{if source.len()>self.pending_bytes.len(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original scalar emission exceeds its inline extent"))}let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_copy_bytes<source.len()||grant.maximum_depth==0{return Ok(false)}self.pending_bytes[..source.len()].copy_from_slice(source);self.pending_length=source.len();self.pending_cursor=0;self.pending_kind=Some(kind);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:source.len(),..Default::default()})?;Ok(true)}
    fn original_scalar_step(payload:&mut RetainedPayloadBuilder,bytes:&[u8],cursor:&mut usize,cx:&mut StepContext<'_>)->Result<bool,ValueError>{if !payload.is_initialized(){payload.advance_initialization(cx)?;return Ok(false)}if *cursor<bytes.len(){payload.append_original(cx,bytes,cursor)?;return Ok(false)}if payload.published().is_none(){payload.seal(cx)?;return Ok(false)}Ok(true)}
    fn original_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.scope.terminal_is_empty(){return self.scope.child_retirement_demands(body)}for original in[&self.payload,&self.terminal_state,&self.terminal_output]{if !original.terminal_is_empty(){let mut demand=original.retirement_demands()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original Torture payload parent depth overflow"))?;return Ok(demand)}}Ok(RetirementDemand{depth:usize::from(self.pending_kind.is_some()||self.pending_length!=0||self.pending_cursor!=0||self.outcome_delivered),..Default::default()})}

    fn output_bytes(&self) -> [u8; 16] {
        let mut output = [0u8; 16];
        output[..8].copy_from_slice(&self.completed_units.to_le_bytes());
        output[8..].copy_from_slice(&self.accumulator.to_le_bytes());
        output
    }
}

impl InteractiveJob for TortureJob {
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.original_retirement_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.original_retirement_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.original_retirement_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.original_retirement_demands(0)?.depth)}
    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{
        if cx.is_cancelled(){return JobOutcomeBorrow::admit_cancelled(cx).map_err(OriginalTurnFailure::from)}
        if self.outcome_delivered{if !self.payload.terminal_is_empty(){self.payload.close_step(cx)?;return Ok(None)}let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}self.pending_kind=None;self.pending_length=0;self.pending_cursor=0;self.outcome_delivered=false;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(None)}
        if let Some(kind)=self.pending_kind{
            if matches!(kind,JobOutcomeKind::Complete){match self.terminal_stage{1=>{if Self::original_scalar_step(&mut self.terminal_state,&self.pending_bytes[..self.pending_length],&mut self.pending_cursor,cx)?{let grant=cx.retained_grant();if grant.maximum_items>0&&grant.maximum_depth>0{self.terminal_stage=2;self.pending_cursor=0;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;}}return Ok(None)},2=>{let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_copy_bytes<16||grant.maximum_depth==0{return Ok(None)}let bytes=self.output_bytes();if self.prepare_original_scalar(cx,kind,&bytes)?{self.terminal_stage=3;}return Ok(None)},3=>{if Self::original_scalar_step(&mut self.terminal_output,&self.pending_bytes[..self.pending_length],&mut self.pending_cursor,cx)?{let grant=cx.retained_grant();if grant.maximum_items>0&&grant.maximum_depth>0{self.terminal_stage=4;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;}}return Ok(None)},_=>return JobOutcomeBorrow::admit_complete(cx,self.terminal_state.published(),self.terminal_output.published()).map_err(OriginalTurnFailure::from)}}
            let stream=if matches!(kind,JobOutcomeKind::PreviewReady){JobPayloadStream::Preview}else{JobPayloadStream::CheckpointState};let selected=self.payload.select_stream(stream,cx.retained_grant())?;cx.consume_retained(selected.progress())?;if matches!(selected,RetainedCloneStep::Progress(_))||selected.progress()!=Default::default(){return Ok(None)}
            if !Self::original_scalar_step(&mut self.payload,&self.pending_bytes[..self.pending_length],&mut self.pending_cursor,cx)?{return Ok(None)}let original=self.payload.published().expect("original sealed Torture payload");let result=match kind{JobOutcomeKind::PreviewReady=>JobOutcomeBorrow::admit_preview(cx,original),JobOutcomeKind::CheckpointReady{applied_progress}=>JobOutcomeBorrow::admit_checkpoint(cx,original,applied_progress),_=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Torture semantic variant changed").into())}?;if result.is_some(){self.outcome_delivered=true;}return Ok(result)
        }
        let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0||cx.should_yield(){return Ok(None)}
        if self.completed_units==self.total_units{if grant.maximum_copy_bytes<48{return Ok(None)}self.scope.assert_completable().map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original Torture scope has a live structured child"))?;let bytes=self.checkpoint_bytes();if self.prepare_original_scalar(cx,JobOutcomeKind::Complete,&bytes)?{self.terminal_stage=1;}return Ok(None)}
        if self.units_since_preview>=self.preview_every_units{if grant.maximum_copy_bytes<24{return Ok(None)}let sequence=cx.next_preview_sequence().map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Torture preview sequence exhausted"))?;let bytes=self.encode_preview(sequence);if self.prepare_original_scalar(cx,JobOutcomeKind::PreviewReady,&bytes)?{self.units_since_preview=0;}return Ok(None)}
        if self.units_since_checkpoint>=self.checkpoint_every_units{if grant.maximum_copy_bytes<48{return Ok(None)}let bytes=self.checkpoint_bytes();if self.prepare_original_scalar(cx,JobOutcomeKind::CheckpointReady{applied_progress:self.completed_units},&bytes)?{self.units_since_checkpoint=0;}return Ok(None)}
        self.rng_state=xorshift64(self.rng_state);let mix=self.rng_state.rotate_left((self.completed_units%61)as u32);self.accumulator=self.accumulator.wrapping_add(mix);self.completed_units+=1;self.units_since_checkpoint+=1;self.units_since_preview+=1;cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(None)
    }
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),JobOutcomeKind::PreviewReady=>descriptor.preview(self.payload.published().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Torture preview custody absent"))?),JobOutcomeKind::CheckpointReady{..}=>descriptor.checkpoint(self.payload.published().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Torture checkpoint custody absent"))?),JobOutcomeKind::Complete=>descriptor.complete(self.terminal_state.published(),self.terminal_output.published()),JobOutcomeKind::Fault=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Torture worker fault requires worker custody"))}}

    fn begin_close(&mut self) {
        self.closing = true;
        self.scope.begin_close();
    }

    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
        if cx.should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}let grant=cx.retained_grant();let demand=self.original_retirement_demands(grant.maximum_copy_bytes)?;
        if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        if !self.scope.terminal_is_empty(){return Ok(match self.scope.pump_child_close(cx)?{InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},step=>step})}
        for original in[&mut self.payload,&mut self.terminal_state,&mut self.terminal_output]{if !original.terminal_is_empty(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};cx.consume_fuel(1);let step=original.close_step_granted(child).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;cx.consume_retained(step.progress())?;return Ok(InteractiveJobCloseStep::Pending{progress:step.progress()})}}
        cx.consume_fuel(1);let progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_retained(progress)?;self.pending_kind=None;self.pending_length=0;self.pending_cursor=0;self.outcome_delivered=false;Ok(InteractiveJobCloseStep::Pending{progress})
    }

    fn terminal_is_empty(&self)->bool{self.closing&&self.scope.terminal_is_empty()&&self.payload.terminal_is_empty()&&self.terminal_state.terminal_is_empty()&&self.terminal_output.terminal_is_empty()&&self.pending_kind.is_none()&&self.pending_length==0&&self.pending_cursor==0&&!self.outcome_delivered}
}
//#endregion 🔥️TortureJob
