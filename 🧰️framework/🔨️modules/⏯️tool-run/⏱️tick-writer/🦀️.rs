//! ⏱️ Tool run tick: the tick record, its limit errors and the bounded tick writer.

use super::*;

//#region 🔖️Tick
/// 📽️ One job report: provisional ops, trace pages, steps and optional progress.
#[derive(semio_framework_value::RetireOwned)]
#[derive(Clone, Debug, PartialEq)]
pub struct ToolRunTick {
    pub identity: ToolRunIdentity,
    pub sequence: u64,
    pub progress: Option<ToolRunProgress>,
    pub steps: Vec<ToolRunStep>,
    pub trace: Vec<ToolRunTracePage>,
    pub append_ops: Vec<Vec<u8>>,
    pub append_entities: Vec<u64>,
    pub retract_to: Option<u32>,
    /// 📨️ Opaque plugin bytes the run's windows read back through `ToolRunView::payload` — the latest intermediate
    /// result of the run (a live readout, a partial solution); the newest tick carrying one wins.
    pub payload: Option<Vec<u8>>,
    pub payload_update:Option<ToolRunPayloadUpdate>,
}

impl ToolRunTick {
    /// 🧳️ Pack record body; rejects ticks over `TOOL_RUN_TICK_BYTES_MAX`.
    pub fn encode(&self) -> Result<Vec<u8>, ToolRunCodecError> {
        validate_tick_payload(self.payload.as_deref(),self.payload_update)?;
        let mut record = RecordValue::default();
        record.fields.insert(1, FieldValue::Record(identity_record(&self.identity)));
        record.fields.insert(2, FieldValue::UInt(self.sequence));
        if let Some(progress) = &self.progress {
            record.fields.insert(3, FieldValue::Record(progress_record(progress)?));
        }
        if !self.steps.is_empty() {
            record.fields.insert(4, FieldValue::List(self.steps.iter().map(|step| step_record(step).map(FieldValue::Record)).collect::<Result<_, _>>()?));
        }
        if !self.trace.is_empty() {
            record.fields.insert(5, FieldValue::List(self.trace.iter().map(|page| page.encode().map(FieldValue::Bytes64)).collect::<Result<_, _>>()?));
        }
        if !self.append_ops.is_empty() {
            record.fields.insert(6, FieldValue::List(self.append_ops.iter().map(|op| FieldValue::Bytes64(op.clone())).collect()));
        }
        if !self.append_entities.is_empty() {
            record.fields.insert(7, FieldValue::Bytes64(self.append_entities.iter().flat_map(|entity| entity.to_le_bytes()).collect()));
        }
        if let Some(retract_to) = self.retract_to {
            record.fields.insert(8, FieldValue::UInt(u64::from(retract_to)));
        }
        if let Some(payload) = &self.payload {
            record.fields.insert(9, FieldValue::Bytes64(payload.clone()));
        }
        if let Some(update)=self.payload_update{record.fields.insert(10,FieldValue::Record(payload_update_record(update)));}
        let bytes = encode_body(tick_spec(), &record)?;
        if bytes.len() > TOOL_RUN_TICK_BYTES_MAX {
            return Err(ToolRunCodecError::Limit("tick bytes"));
        }
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, ToolRunCodecError> {
        if bytes.len() > TOOL_RUN_TICK_BYTES_MAX {
            return Err(ToolRunCodecError::Limit("tick bytes"));
        }
        let mut record = decode_body(tick_spec(), bytes)?;
        let progress = match record.get(3) {
            None | Some(FieldValue::Absent) => None,
            Some(FieldValue::Record(progress)) => Some(progress_from_record(progress)?),
            Some(_) => return Err(ToolRunCodecError::Malformed("progress")),
        };
        let steps = list(&record, 4)?.iter().map(|item| step_from_record(item_record(item)?)).collect::<Result<_, _>>()?;
        let trace = list(&record, 5)?.iter().map(|item| ToolRunTracePage::decode(item_bytes(item)?)).collect::<Result<_, _>>()?;
        let append_ops = list(&record, 6)?.iter().map(|item| item_bytes(item).map(<[u8]>::to_vec)).collect::<Result<_, _>>()?;
        let append_entities = u64_column(bytes_field(&record, 7)?, "appendEntities")?;
        let retract_to = optional_uint(&record, 8)?.map(|value| narrow(value, "retractTo")).transpose()?;
        let payload_update=match record.get(10){None|Some(FieldValue::Absent)=>None,Some(FieldValue::Record(update))=>Some(payload_update_from_record(update)?),Some(_)=>return Err(ToolRunCodecError::Malformed("payload update"))};
        let payload = match record.fields.remove(&9) {
            None | Some(FieldValue::Absent) => None,
            Some(FieldValue::Bytes64(payload)) => Some(payload),
            Some(_) => return Err(ToolRunCodecError::Malformed("payload")),
        };
        validate_tick_payload(payload.as_deref(),payload_update)?;
        Ok(Self { identity: identity_from_field(&record, 1)?, sequence: uint(&record, 2)?, progress, steps, trace, append_ops, append_entities, retract_to, payload,payload_update })
    }
}

/// 🛑️ A writer call that would exceed a contract limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolRunLimitError {
    ProvisionalOps,
    StepArgs,
}

impl std::fmt::Display for ToolRunLimitError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::ProvisionalOps => "toolRun.limit.provisionalOps",
            Self::StepArgs => "toolRun.limit.stepArgs",
        })
    }
}

impl std::error::Error for ToolRunLimitError {}

/// ✍️ Domain-neutral tick builder for run jobs (§3.7): bytes only, pages split at 4 096 ops.
#[derive(Clone, Debug, semio_framework_value::RetireOwned)]
pub struct ToolRunTickWriter {
    #[cfg(feature = "testkit")] #[doc(hidden)] pub identity: ToolRunIdentity,
    #[cfg(not(feature = "testkit"))] pub(crate) identity: ToolRunIdentity,
    #[cfg(feature = "testkit")] #[doc(hidden)] pub next_sequence: u64,
    #[cfg(not(feature = "testkit"))] pub(crate) next_sequence: u64,
    #[cfg(feature = "testkit")] #[doc(hidden)] pub next_page: u32,
    #[cfg(not(feature = "testkit"))] pub(crate) next_page: u32,
    #[cfg(feature = "testkit")] #[doc(hidden)] pub next_step: u64,
    #[cfg(not(feature = "testkit"))] pub(crate) next_step: u64,
    #[cfg(feature = "testkit")] #[doc(hidden)] pub provisional_base: u32,
    #[cfg(not(feature = "testkit"))] pub(crate) provisional_base: u32,
    #[cfg(feature = "testkit")] #[doc(hidden)] pub progress: Option<ToolRunProgress>,
    #[cfg(not(feature = "testkit"))] pub(crate) progress: Option<ToolRunProgress>,
    #[cfg(feature = "testkit")] #[doc(hidden)] pub steps: Vec<ToolRunStep>,
    #[cfg(not(feature = "testkit"))] pub(crate) steps: Vec<ToolRunStep>,
    #[cfg(feature = "testkit")] #[doc(hidden)] pub pages: Vec<ToolRunTracePage>,
    #[cfg(not(feature = "testkit"))] pub(crate) pages: Vec<ToolRunTracePage>,
    #[cfg(feature = "testkit")] #[doc(hidden)] pub page_ops: Vec<ToolRunTraceOp>,
    #[cfg(not(feature = "testkit"))] pub(crate) page_ops: Vec<ToolRunTraceOp>,
    #[cfg(feature = "testkit")] #[doc(hidden)] pub append_ops: Vec<Vec<u8>>,
    #[cfg(not(feature = "testkit"))] pub(crate) append_ops: Vec<Vec<u8>>,
    #[cfg(feature = "testkit")] #[doc(hidden)] pub append_entities: Vec<u64>,
    #[cfg(not(feature = "testkit"))] pub(crate) append_entities: Vec<u64>,
    #[cfg(feature = "testkit")] #[doc(hidden)] pub entity_marks: Vec<u32>,
    #[cfg(not(feature = "testkit"))] pub(crate) entity_marks: Vec<u32>,
    #[cfg(feature = "testkit")] #[doc(hidden)] pub retract_to: Option<u32>,
    #[cfg(not(feature = "testkit"))] pub(crate) retract_to: Option<u32>,
    #[cfg(feature = "testkit")] #[doc(hidden)] pub payload: Option<Vec<u8>>,
    #[cfg(not(feature = "testkit"))] pub(crate) payload: Option<Vec<u8>>,
    #[cfg(feature = "testkit")] #[doc(hidden)] pub payload_update: Option<ToolRunPayloadUpdate>,
    #[cfg(not(feature = "testkit"))] pub(crate) payload_update: Option<ToolRunPayloadUpdate>,
    original_payload_source: Option<(usize,usize)>,
    pending_bytes: usize,
}

fn admit_original_column<T>(column:&mut Vec<T>,grant:semio_framework_value::RetainedCloneGrant)->Result<Option<semio_framework_value::RetainedCloneProgress>,semio_framework_value::ValueError>{
 use semio_framework_value::{RetainedCloneProgress as P,ValueError,ValueRefusalKind};
 if column.len()<column.capacity(){return Ok(None)}let overflow=||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tick column extent overflow");let capacity=column.len().checked_add(1).and_then(|n|n.checked_mul(size_of::<T>())).ok_or_else(overflow)?;let released=column.capacity().checked_mul(size_of::<T>()).ok_or_else(overflow)?;if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<capacity||grant.maximum_release_bytes<released{return Ok(Some(P::default()))}column.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original tick column native allocation refused"))?;Ok(Some(P{copied_items:1,retained_capacity_bytes:capacity,released_bytes:released,..Default::default()}))
}

impl ToolRunTickWriter {
    /// 🔭️ Borrows the complete original writer rows for the tick wire cursor without cloning or moving them.
    pub fn next_sequence(&self)->u64{self.next_sequence}
    pub fn next_page(&self)->u32{self.next_page}
    pub fn original_progress(&self)->&Option<ToolRunProgress>{&self.progress}
    pub fn steps(&self)->&[ToolRunStep]{&self.steps}
    pub fn pages(&self)->&[ToolRunTracePage]{&self.pages}
    pub fn page_ops(&self)->&[ToolRunTraceOp]{&self.page_ops}
    pub fn append_ops(&self)->&[Vec<u8>]{&self.append_ops}
    pub fn append_entities(&self)->&[u64]{&self.append_entities}
    pub fn original_retract_to(&self)->Option<u32>{self.retract_to}
    pub fn original_payload(&self)->Option<&[u8]>{self.payload.as_deref()}
    pub fn payload_update(&self)->Option<ToolRunPayloadUpdate>{self.payload_update}
    pub fn new(identity: ToolRunIdentity) -> Self {
        Self::with_provisional_base(identity, 0)
    }

    /// ⏩️ A writer continuing a run whose provisional list already holds `provisional_len` ops — a job
    /// resumed from its checkpoint or a revalidate job — so `retract_to` below that length emits
    /// `retractTo` and `append_op` honours the provisional cap from there.
    pub fn with_provisional_base(identity: ToolRunIdentity, provisional_len: u32) -> Self {
        Self {
            identity,
            next_sequence: 0,
            next_page: 0,
            next_step: 0,
            provisional_base: provisional_len,
            progress: None,
            steps: Vec::new(),
            pages: Vec::new(),
            page_ops: Vec::new(),
            append_ops: Vec::new(),
            append_entities: Vec::new(),
            entity_marks: Vec::new(),
            retract_to: None,
            payload: None,
            payload_update:None,
            original_payload_source:None,
            pending_bytes: 0,
        }
    }

    pub fn identity(&self) -> ToolRunIdentity {
        self.identity
    }

    /// ⛓️ The original monotonic wire sequence also seeds a ranged publication identity.
    pub fn original_sequence(&self)->u64{self.next_sequence}

    /// ⏭️ After original wire acknowledgement, admits only an empty continuation and leaves all original columns owned.
    pub fn original_successor_demands() -> semio_framework_value::RetirementDemand {
        semio_framework_value::RetirementDemand { copy_bytes: 2 * size_of::<Self>() + size_of::<Option<Self>>() + 4 * size_of::<usize>(), depth: 1, ..Default::default() }
    }
    pub fn admit_original_successor(&self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<Option<(Self,semio_framework_value::retained_clone::RetainedCloneProgress)>,semio_framework_value::ValueError>{
        let demand=Self::original_successor_demands();
        if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes{return Ok(None)}
        let invalid=||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original tick continuation metadata overflow");
        let next_sequence=self.next_sequence.checked_add(1).ok_or_else(invalid)?;
        let next_page=self.next_page.checked_add(u32::from(!self.page_ops.is_empty())).ok_or_else(invalid)?;
        let provisional_base=self.provisional_base.checked_add(u32::try_from(self.append_ops.len()).map_err(|_|invalid())?).ok_or_else(invalid)?;
        let mut successor=Self::with_provisional_base(self.identity,provisional_base);successor.next_sequence=next_sequence;successor.next_page=next_page;successor.next_step=self.next_step;
        Ok(Some((successor,semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))
    }


    /// 🪢️ Stamps later ticks with a new identity (generation change).
    pub fn rebind(&mut self, identity: ToolRunIdentity) {
        self.identity = identity;
        for page in &mut self.pages {
            page.identity = identity;
        }
    }

    /// 🪙️ Provisional op count after the pending tick is applied.
    pub fn provisional_len(&self) -> u32 {
        self.provisional_base + self.append_ops.len() as u32
    }

    pub fn upsert(&mut self, key: u64, verdict: ToolRunVerdict, reason: u16, subject: ToolRunTraceSubject) {
        self.trace_op(ToolRunTraceOp::Upsert { key, verdict, reason, subject });
    }

    pub fn retire(&mut self, key: u64) {
        self.trace_op(ToolRunTraceOp::Retire { key });
    }

    pub fn clear_trace(&mut self) {
        self.trace_op(ToolRunTraceOp::Clear);
    }

    /// 🗒️ Appends a step with the writer's next step sequence.
    pub fn step(&mut self, kind: ToolRunStepKind, stage: u16, reason: u16, subject: Option<u64>, args: &[ToolRunStepArg]) -> Result<(), ToolRunLimitError> {
        if args.len() > TOOL_RUN_STEP_ARGS_MAX {
            return Err(ToolRunLimitError::StepArgs);
        }
        self.steps.push(ToolRunStep { sequence: self.next_step, kind, stage, reason, subject, repeat: 1, args: args.to_vec() });
        self.next_step += 1;
        self.pending_bytes += 40 + 9 * args.len();
        Ok(())
    }

    /// 🧬️ Appends one `OpBinary`-encoded provisional op; refuses past `TOOL_RUN_PROVISIONAL_OPS_MAX`.
    pub fn append_op(&mut self, op: Vec<u8>) -> Result<(), ToolRunLimitError> {
        if self.provisional_len() >= TOOL_RUN_PROVISIONAL_OPS_MAX {
            return Err(ToolRunLimitError::ProvisionalOps);
        }
        self.pending_bytes += op.len() + 6;
        self.append_ops.push(op);
        Ok(())
    }

    /// 📥️ Admits exact original wire backing; native column growth precedes a separate metadata handoff.
    pub fn admit_original_op(&mut self,original:&mut Option<Vec<u8>>,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{
        use semio_framework_value::{RetainedCloneStep as S,RetainedCloneProgress as P,ValueError,ValueRefusalKind};
        let Some(source)=original.as_ref()else{return Ok(S::Complete(P::default()))};
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(S::Progress(P::default()))}
        if self.provisional_len()>=TOOL_RUN_PROVISIONAL_OPS_MAX{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tick provisional column is full"))}
        let pending=self.pending_bytes.checked_add(source.len()).and_then(|bytes|bytes.checked_add(6)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tick byte extent overflow"))?;
        if self.append_ops.len()==self.append_ops.capacity(){let old=self.append_ops.capacity()*size_of::<Vec<u8>>();let capacity=(self.append_ops.len()+1)*size_of::<Vec<u8>>();if grant.maximum_capacity_bytes<capacity||grant.maximum_release_bytes<old{return Ok(S::Progress(P::default()))}self.append_ops.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tick column backing refused"))?;return Ok(S::Progress(P{copied_items:1,retained_capacity_bytes:capacity,released_bytes:old,..Default::default()}))}
        self.append_ops.push(original.take().unwrap());self.pending_bytes=pending;Ok(S::Complete(P{copied_items:1,..Default::default()}))
    }

    /// 🧲️ Preserves the same original entity until each native column has its own paid backing.
    pub fn admit_original_entity(&mut self,original:&mut Option<u64>,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{
     use semio_framework_value::{RetainedCloneStep as S,RetainedCloneProgress as P,ValueError,ValueRefusalKind};if original.is_none(){return Ok(S::Complete(P::default()))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(S::Progress(P::default()))}if let Some(progress)=admit_original_column(&mut self.entity_marks,grant)?{return Ok(S::Progress(progress))}if let Some(progress)=admit_original_column(&mut self.append_entities,grant)?{return Ok(S::Progress(progress))}let pending=self.pending_bytes.checked_add(8).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original entity pending extent overflow"))?;self.entity_marks.push(self.provisional_len());self.append_entities.push(original.take().unwrap());self.pending_bytes=pending;Ok(S::Complete(P{copied_items:1,..Default::default()}))
    }

    /// 📈️ Admits original trace page storage before moving its original operation metadata.
    pub fn admit_original_trace(&mut self,original:&mut Option<ToolRunTraceOp>,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{
     use semio_framework_value::{RetainedCloneStep as S,RetainedCloneProgress as P,ValueError,ValueRefusalKind};let Some(source)=original.as_ref()else{return Ok(S::Complete(P::default()))};if grant.maximum_items==0||grant.maximum_depth==0{return Ok(S::Progress(P::default()))}let overflow=||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original trace pending extent overflow");if self.page_ops.len()==TOOL_RUN_TRACE_PAGE_OPS_MAX{if let Some(progress)=admit_original_column(&mut self.pages,grant)?{return Ok(S::Progress(progress))}let next=self.next_page.checked_add(1).ok_or_else(overflow)?;let pending=self.pending_bytes.checked_add(TOOL_RUN_TRACE_PAGE_OVERHEAD_BYTES).ok_or_else(overflow)?;self.pages.push(ToolRunTracePage{identity:self.identity,page:self.next_page,ops:std::mem::take(&mut self.page_ops)});self.next_page=next;self.pending_bytes=pending;return Ok(S::Progress(P{copied_items:1,..Default::default()}))}if let Some(progress)=admit_original_column(&mut self.page_ops,grant)?{return Ok(S::Progress(progress))}let pending=self.pending_bytes.checked_add(source.wire_bytes()).ok_or_else(overflow)?;self.page_ops.push(original.take().unwrap());self.pending_bytes=pending;Ok(S::Complete(P{copied_items:1,..Default::default()}))
    }

    /// 🔢️ Admits one original progress counter backing and then its separate metadata binding.
    pub fn admit_original_progress_counter(progress:&mut ToolRunProgress,original:&mut Option<ToolRunCounter>,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{
     use semio_framework_value::{RetainedCloneStep as S,RetainedCloneProgress as P,ValueError,ValueRefusalKind};if original.is_none(){return Ok(S::Complete(P::default()))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(S::Progress(P::default()))}if progress.counters.len()>=TOOL_RUN_COUNTERS_MAX{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original progress counters full"))}if let Some(progress)=admit_original_column(&mut progress.counters,grant)?{return Ok(S::Progress(progress))}progress.counters.push(original.take().unwrap());Ok(S::Complete(P{copied_items:1,..Default::default()}))
    }

    /// 🧮️ Transfers the same original progress carrier only into an unoccupied admitted binding.
    pub fn admit_original_progress(&mut self,original:&mut Option<ToolRunProgress>,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{
     use semio_framework_value::{RetainedCloneStep as S,RetainedCloneProgress as P,ValueError,ValueRefusalKind};if original.is_none(){return Ok(S::Complete(P::default()))}if self.progress.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original progress binding requires previous tick acknowledgement"))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(S::Progress(P::default()))}self.progress=original.take();Ok(S::Complete(P{copied_items:1,..Default::default()}))
    }

    /// 📬️ Births exact native payload backing before incrementally copying the original caller's octets.
    pub fn admit_original_payload(&mut self,source:&[u8],cursor:&mut usize,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{
     use semio_framework_value::{RetainedCloneStep as S,RetainedCloneProgress as P,ValueError,ValueRefusalKind};if *cursor>source.len(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original payload cursor exceeds source"))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(S::Progress(P::default()))}let identity=(source.as_ptr()as usize,source.len());if self.original_payload_source.is_some_and(|original|original!=identity){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original payload requires same borrowed source pointer and extent"))}if self.original_payload_source.is_none(){if self.payload.is_some()||*cursor!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original payload source cannot replace pending owner"))}let copy=size_of::<Option<(usize,usize)>>()+size_of::<Option<ToolRunPayloadUpdate>>();if grant.maximum_copy_bytes<copy{return Ok(S::Progress(P::default()))}self.original_payload_source=Some(identity);self.payload_update=Some(ToolRunPayloadUpdate::Replace);return Ok(S::Progress(P{copied_items:1,copied_bytes:copy,..Default::default()}))}if self.payload.is_none(){if *cursor!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original payload backing cannot resume substituted source"))}if grant.maximum_capacity_bytes<source.len(){return Ok(S::Progress(P::default()))}let pending=self.pending_bytes.checked_add(source.len()).and_then(|n|n.checked_add(6)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original payload pending extent overflow"))?;let mut target=Vec::new();target.try_reserve_exact(source.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original payload native allocation refused"))?;self.payload=Some(target);self.pending_bytes=pending;return Ok(S::Progress(P{copied_items:1,retained_capacity_bytes:source.len(),..Default::default()}))}let target=self.payload.as_mut().unwrap();if target.len()!=*cursor||target.capacity()<source.len(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original payload source does not match retained native extent"))}if *cursor==source.len(){return Ok(S::Complete(P::default()))}let count=grant.maximum_copy_bytes.min(source.len()-*cursor);if count==0{return Ok(S::Progress(P::default()))}target.extend_from_slice(&source[*cursor..*cursor+count]);*cursor+=count;let progress=P{copied_items:1,copied_bytes:count,..Default::default()};Ok(if *cursor==source.len(){S::Complete(progress)}else{S::Progress(progress)})
    }

    /// 🏷️ Tags `entity` with the provisional length reached so far; a later `retract_to` below that length
    /// drops it again.
    pub fn append_entity(&mut self, entity: u64) {
        self.pending_bytes += 8;
        self.entity_marks.push(self.provisional_len());
        self.append_entities.push(entity);
    }

    /// 🪚️ Releases one actual unpublished wire backing or adjusts original retraction metadata per turn.
    pub fn admit_original_retraction(&mut self,len:u32,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{
     use semio_framework_value::{RetainedCloneStep as S,RetainedCloneProgress as P,ValueError,ValueRefusalKind};if grant.maximum_items==0||grant.maximum_depth==0{return Ok(S::Progress(P::default()))}let kept=if len>=self.provisional_base{(len-self.provisional_base)as usize}else{0};if self.append_ops.len()>kept{let source=self.append_ops.last().unwrap();let released=source.capacity();if grant.maximum_release_bytes<released{return Ok(S::Progress(P::default()))}let pending=self.pending_bytes.checked_sub(source.len()+6).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original retraction byte count lacks wire owner"))?;drop(self.append_ops.pop());self.pending_bytes=pending;return Ok(S::Progress(P{copied_items:1,released_bytes:released,..Default::default()}))}if self.entity_marks.last().is_some_and(|mark|*mark>len){if self.entity_marks.len()!=self.append_entities.len(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original entity marks disagree with owner column"))}let pending=self.pending_bytes.checked_sub(8).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original retraction byte count lacks entity"))?;self.entity_marks.pop();self.append_entities.pop();self.pending_bytes=pending;return Ok(S::Progress(P{copied_items:1,..Default::default()}))}if len<self.provisional_base{self.retract_to=Some(self.retract_to.map_or(len,|previous|previous.min(len)));self.provisional_base=len;return Ok(S::Complete(P{copied_items:1,..Default::default()}))}Ok(S::Complete(P::default()))
    }

    /// ✂️ Truncates the provisional list to `len` ops; pending appends beyond it are dropped, and so is
    /// every pending entity appended once the list was longer than `len`.
    pub fn retract_to(&mut self, len: u32) {
        if len >= self.provisional_base {
            self.append_ops.truncate((len - self.provisional_base) as usize);
        } else {
            self.append_ops.clear();
            self.retract_to = Some(self.retract_to.map_or(len, |retract| retract.min(len)));
            self.provisional_base = len;
        }
        let kept = self.entity_marks.iter().take_while(|mark| **mark <= len).count();
        self.pending_bytes = self.pending_bytes.saturating_sub((self.append_entities.len() - kept) * 8);
        self.append_entities.truncate(kept);
        self.entity_marks.truncate(kept);
    }

    pub fn progress(&mut self, progress: ToolRunProgress) {
        self.progress = Some(progress);
    }

    /// 📊️ Borrows the original progress owner for separately admitted native counter construction.
    pub fn original_progress_mut(&mut self)->Option<&mut ToolRunProgress>{self.progress.as_mut()}

    /// 🎒️ Sets the pending tick's plugin payload; a later call before `finish` replaces it.
    pub fn payload(&mut self, payload: Vec<u8>) {
        self.pending_bytes = self.pending_bytes.saturating_sub(self.payload.as_ref().map_or(0, |previous| previous.len() + 6)) + payload.len() + 6;
        self.payload = Some(payload);
        self.payload_update=Some(ToolRunPayloadUpdate::Replace);
    }

    /// 📖️ Borrows pending wire bytes while the writer keeps their original allocation.
    pub fn payload_bytes(&self) -> Option<&[u8]> {
        self.payload.as_deref()
    }

    pub fn is_empty(&self) -> bool {
        self.progress.is_none() && self.steps.is_empty() && self.pages.is_empty() && self.page_ops.is_empty() && self.append_ops.is_empty() && self.append_entities.is_empty() && self.retract_to.is_none() && self.payload.is_none()
    }

    /// 🌡️ Estimated encoded bytes of the pending tick.
    pub fn pending_bytes(&self) -> usize {
        self.pending_bytes
    }

    /// 🚰️ True once the pending tick should be finished to stay under the tick byte cap.
    pub fn should_flush(&self) -> bool {
        self.pending_bytes >= TOOL_RUN_TICK_BYTES_MAX / 2
    }

    /// 🎬️ Takes the pending tick with the next sequence, or `None` when nothing is pending.
    pub fn finish(&mut self) -> Option<ToolRunTick> {
        self.seal_page();
        if self.is_empty() {
            return None;
        }
        let tick = ToolRunTick {
            identity: self.identity,
            sequence: self.next_sequence,
            progress: self.progress.take(),
            steps: std::mem::take(&mut self.steps),
            trace: std::mem::take(&mut self.pages),
            append_ops: std::mem::take(&mut self.append_ops),
            append_entities: std::mem::take(&mut self.append_entities),
            retract_to: self.retract_to.take(),
            payload: self.payload.take(),
            payload_update:self.payload_update.take(),
        };
        self.original_payload_source=None;
        self.entity_marks.clear();
        self.provisional_base += tick.append_ops.len() as u32;
        self.next_sequence += 1;
        self.pending_bytes = 0;
        Some(tick)
    }

    fn trace_op(&mut self, op: ToolRunTraceOp) {
        self.pending_bytes += op.wire_bytes();
        self.page_ops.push(op);
        if self.page_ops.len() == TOOL_RUN_TRACE_PAGE_OPS_MAX {
            self.seal_page();
        }
    }

    fn seal_page(&mut self) {
        if self.page_ops.is_empty() {
            return;
        }
        self.pending_bytes += TOOL_RUN_TRACE_PAGE_OVERHEAD_BYTES;
        self.pages.push(ToolRunTracePage { identity: self.identity, page: self.next_page, ops: std::mem::take(&mut self.page_ops) });
        self.next_page += 1;
    }
}
//#endregion 🔖️Tick
