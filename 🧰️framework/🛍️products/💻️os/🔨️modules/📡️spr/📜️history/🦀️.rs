//! 🗂️ History stream verification, whole-file encoding and incremental hydration.
pub use semio_framework_os_history_schema::*;
pub use semio_framework_os_history_codec::*;
pub use semio_framework_os_history_record_decode::*;
#[path="🛬️decode/🗂️file/📚️frames/🦀️.rs"]
mod retained_history_source_frames;
pub use retained_history_source_frames::{RetainedHistorySourceFrames,RetainedHistorySourceFrame,RetainedHistorySourceFramesStep};
#[path="🛬️decode/🗂️file/🦀️.rs"]
mod retained_history_source;
pub use retained_history_source::{RetainedHistorySourceDecode,RetainedHistorySourcePartial,RetainedHistorySourceStep};
#[path="🛬️decode/🗂️file/🗜️deflate/🦀️.rs"]
mod retained_history_deflate;
pub use retained_history_deflate::{RetainedHistoryDeflateDecode,RetainedHistoryDeflatePayload,RetainedHistoryDeflateStep};

use semio_framework_os_pack::{ByteReader, ByteWriter, CodecId};
use semio_framework_pack_source::{PackSink};
use protocol::format::{Blake3Hasher, FrameCursor, RecoveryMode, ReverseFrameCursor, SprWriter, VerificationLevel, WriteOptions, HEADER_SIZE};
use protocol::ProtocolError;
use protocol::wire::{DictBuilder, DictReader, ProtocolLimits, RecordHasher};
use std::collections::{HashMap, HashSet};
use protocol::codec::{HistoryReadLimits, OriginalWireSourceIdentity, RetainedWireByteSource};
use semio_framework_value::native_decoding::allocation::NativeForwardedDecodeReceipt;
use semio_framework_value::native_decoding::{NativeDecodeAllocation, NativeForwardedDecodeControl};
use semio_framework_value::retained_clone::{admit_retained_clone_close, admit_retained_clone_progress};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::{admit_owned_retirement, owned_retirement_birth_bytes, ErasedSnapshotRetirement, NativeDecodeControl, NativeDecodeRetirementRecipient, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::{size_of_val, ManuallyDrop};


// Whole-file compile: HistoryLog <-> .spr bytes, using protocol::format::SprWriter/FrameCursor.
//
// 🎯️ Design choice: dictionaries are flushed INCREMENTALLY (a REC_STR_DICT delta record right
// before whichever record first needed the new entries), not in a separate pre-pass — this keeps
// encode_history a single forward pass while still satisfying "dict records come before the
// records that reference them"; determinism (and therefore canonical-stability) falls out of the
// fact that DictBuilder's first-use interning order is itself deterministic.

#[derive(Clone, Debug)]
pub struct EncodeOptions {
    pub canonical: bool,
    pub write_backwards_section: bool,
    pub limits: ProtocolLimits,
}

impl Default for EncodeOptions {
    fn default() -> Self {
        Self { canonical: true, write_backwards_section: false, limits: ProtocolLimits::default() }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DecodeOptions {
    pub verification: VerificationLevel,
    pub limits: ProtocolLimits,
}

/// 🚦️ Caller authority of one actual receiving turn: the decoder asks it at every checkpoint and never owns it.
pub trait RetainedHistoryTurn {
    fn should_yield(&self) -> bool;
    fn is_cancelled(&self) -> bool;
}

/// 🛤️ A turn that never yields and is never cancelled, for callers that bound their work through the grant alone.
#[derive(Clone, Copy, Debug, Default)]
pub struct RetainedHistoryUninterrupted;

impl RetainedHistoryTurn for RetainedHistoryUninterrupted {
    fn should_yield(&self) -> bool {
        false
    }
    fn is_cancelled(&self) -> bool {
        false
    }
}

/// 📥️ Original retained History decoder: one cumulative native ledger and one retirement recipient carry the borrowed-range receiving frontier across caller turns, so every typed receiver is quoted by `demands` before it moves.
pub struct RetainedHistoryDecode {
    receipt: Option<NativeForwardedDecodeReceipt>,
    recipient: NativeDecodeRetirementRecipient,
    decoder: ManuallyDrop<Option<RetainedHistorySourceDecode>>,
    partial: ManuallyDrop<Option<RetainedHistorySourcePartial>>,
    fault: ManuallyDrop<Option<RetainedHistoryOpMetaFault>>,
    inner_fault: ManuallyDrop<Option<RetainedHistoryOpMetaFault>>,
    active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    demand: RetirementDemand,
    issued: Option<(RetainedCloneGrant, RetainedCloneProgress)>,
    maximum: usize,
    range: (usize, usize),
    limits: HistoryReadLimits,
    persisted_document: bool,
    admitted: bool,
    closing: bool,
    closed: bool,
}

fn invariant(reason: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, reason)
}

fn permits(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_depth >= demand.depth && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes
}

fn box_demands(owner: &dyn ErasedSnapshotRetirement, body: usize) -> Result<RetirementDemand, ValueError> {
    if owner.terminal_is_empty() {
        return Ok(RetirementDemand { release_bytes: size_of_val(owner), depth: 1, ..Default::default() });
    }
    let mut demand = owner.next_demand(body)?;
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "original History retirement depth overflows"))?;
    Ok(demand)
}

fn birth_demands<T: RetireOwned>(pending: &Option<T>) -> Result<RetirementDemand, ValueError> {
    if pending.is_none() {
        return Ok(Default::default());
    }
    if !T::controlled_retirement_supported() {
        return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original History owner has no controlled retirement authority"));
    }
    Ok(RetirementDemand { capacity_bytes: owned_retirement_birth_bytes::<T>(), depth: 2, ..Default::default() })
}

fn admit_owned<T: RetireOwned>(pending: &mut Option<T>, active: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    if active.is_some() {
        return Err(invariant("original History retirement admission requires an empty child slot"));
    }
    let demand = birth_demands(pending)?;
    let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
    let original = pending.take().ok_or_else(|| invariant("original History retirement admission lost its owner"))?;
    match admit_owned_retirement(original, child) {
        Ok((owner, progress)) => {
            *active = Some(owner);
            admit_retained_clone_progress(child, progress, "original History child birth")?;
            if progress.retained_capacity_bytes != demand.capacity_bytes {
                return Err(invariant("original History child birth receipt differs from its quote"));
            }
            Ok(RetainedCloneStep::Progress(progress))
        }
        Err((error, original)) => {
            *pending = Some(original);
            Err(error)
        }
    }
}

fn close_box(slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    let Some(owner) = slot.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())) };
    if owner.terminal_is_empty() {
        let extent = size_of_val(owner.as_ref());
        drop(slot.take());
        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: extent, ..Default::default() }));
    }
    let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
    let step = owner.close_step(child)?;
    admit_retained_clone_close(child, step, owner.terminal_is_empty(), "original History retirement child")?;
    Ok(RetainedCloneStep::Progress(step.progress()))
}

fn recipient_demands(recipient: &NativeDecodeRetirementRecipient, body: usize) -> Result<RetirementDemand, ValueError> {
    Ok(RetirementDemand { copy_bytes: recipient.next_copy_byte_demand()?, capacity_bytes: recipient.next_capacity_byte_demand(body)?, release_bytes: recipient.next_release_byte_demand()?, depth: recipient.next_depth_demand()? })
}

fn receive(issued: Option<(RetainedCloneGrant, RetainedCloneProgress)>, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
    let Some((original, progress)) = issued else { return Ok(Default::default()) };
    if original == grant {
        Ok(progress)
    } else {
        Err(invariant("original History cursor changed its supplied grant").with_retained_progress(progress))
    }
}

fn receive_failure(issued: Option<(RetainedCloneGrant, RetainedCloneProgress)>, grant: RetainedCloneGrant, primary: ValueError) -> ValueError {
    let progress = issued.map_or_else(|| primary.retained_progress(), |(_, progress)| progress);
    let error = if issued.is_some_and(|(original, _)| original != grant) { invariant("original History failure changed its supplied grant") } else { primary };
    error.with_retained_progress(progress)
}

fn with_control<T>(
    receipt: &mut Option<NativeForwardedDecodeReceipt>,
    recipient: &mut NativeDecodeRetirementRecipient,
    maximum: usize,
    turn: &dyn RetainedHistoryTurn,
    ceiling: usize,
    closing: bool,
    run: impl FnOnce(&mut NativeDecodeControl<'_>) -> T,
) -> Result<T, ValueError> {
    let mut progress = |_| !turn.should_yield() && (closing || !turn.is_cancelled());
    let mut reserved = 0usize;
    let mut allocate = |quote: NativeDecodeAllocation| -> Result<(), ValueError> {
        reserved = reserved.checked_add(quote.bytes).filter(|bytes| *bytes <= ceiling).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "original History decode reservation exceeds its actual turn"))?;
        Ok(())
    };
    let mut control = if let Some(original) = receipt.take() {
        match NativeForwardedDecodeControl::rebind(original, &mut progress, &mut allocate, recipient) {
            Ok(control) => control,
            Err((error, original)) => {
                *receipt = Some(original);
                return Err(error);
            }
        }
    } else {
        let mut control = NativeDecodeControl::new_forwarded(maximum, &mut progress, &mut allocate);
        control.install_retirement_recipient(recipient)?;
        control
    };
    let result = control.scoped_stage(|original| Ok::<_, std::convert::Infallible>(run(original))).unwrap();
    *receipt = Some(control.detach().unwrap_or_else(|_| panic!("original History decode continuation violated canonical control invariants")));
    Ok(result)
}

impl RetainedHistoryDecode {
    pub fn new(range: (usize, usize), limits: HistoryReadLimits, persisted_document: bool, maximum_allocation_bytes: usize) -> Result<Self, ValueError> {
        limits.validate()?;
        if maximum_allocation_bytes == 0 || range.0 > range.1 {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "original History receiving bounds are invalid"));
        }
        Ok(Self {
            receipt: None,
            recipient: NativeDecodeRetirementRecipient::new(),
            decoder: ManuallyDrop::new(None),
            partial: ManuallyDrop::new(None),
            fault: ManuallyDrop::new(None),
            inner_fault: ManuallyDrop::new(None),
            active: ManuallyDrop::new(None),
            demand: RetirementDemand { depth: 2, ..Default::default() },
            issued: None,
            maximum: maximum_allocation_bytes,
            range,
            limits,
            persisted_document,
            admitted: false,
            closing: false,
            closed: false,
        })
    }

    pub fn source_identity(&self) -> Option<OriginalWireSourceIdentity> {
        self.decoder.as_ref().map(RetainedHistorySourceDecode::source_identity)
    }

    pub fn consumed_end(&self) -> usize {
        self.decoder.as_ref().map_or(self.range.1, RetainedHistorySourceDecode::consumed_end)
    }

    pub fn fault(&self) -> Option<&RetainedHistoryOpMetaFault> {
        self.fault.as_ref().or(self.inner_fault.as_ref()).or_else(|| self.decoder.as_ref().and_then(RetainedHistorySourceDecode::fault))
    }

    pub fn take_receipt(&mut self) -> Option<(RetainedCloneGrant, RetainedCloneProgress)> {
        self.issued.take()
    }

    pub fn demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() {
            return box_demands(active.as_ref(), body);
        }
        if self.fault.is_some() {
            return birth_demands(&self.fault);
        }
        if self.inner_fault.is_some() {
            return birth_demands(&self.inner_fault);
        }
        if !self.closing && !self.recipient.terminal_is_empty() {
            return recipient_demands(&self.recipient, body);
        }
        if self.decoder.is_some() {
            return Ok(RetirementDemand { depth: self.demand.depth.max(2), ..self.demand });
        }
        if self.partial.is_some() {
            return birth_demands(&self.partial);
        }
        if !self.recipient.terminal_is_empty() {
            return recipient_demands(&self.recipient, body);
        }
        Ok(RetirementDemand { depth: if self.closed { 0 } else if !self.admitted && !self.closing { 2 } else { 1 }, ..Default::default() })
    }

    pub fn step<S: RetainedWireByteSource + ?Sized>(&mut self, source: &S, turn: &dyn RetainedHistoryTurn, grant: RetainedCloneGrant, history: &mut Option<HistoryLog>) -> Result<RetainedHistorySourceStep, ValueError> {
        if self.closed {
            return Ok(RetainedHistorySourceStep::Closed);
        }
        if self.closing || self.fault.is_some() {
            return Ok(RetainedHistorySourceStep::Fault);
        }
        if turn.should_yield() || turn.is_cancelled() || grant.maximum_items == 0 {
            return Ok(RetainedHistorySourceStep::Blocked);
        }
        if !permits(grant, self.demands(grant.maximum_copy_bytes)?) {
            return Ok(RetainedHistorySourceStep::Blocked);
        }
        let decoder = &mut self.decoder;
        let admitted = &mut self.admitted;
        let quote = &mut self.demand;
        let (range, limits, persisted_document) = (self.range, self.limits, self.persisted_document);
        let (result, issued) = with_control(&mut self.receipt, &mut self.recipient, self.maximum, turn, grant.maximum_capacity_bytes, false, |control| {
            if control.has_retirement_owner() {
                let raw = control.close_retirement_recipient(grant);
                let progress = match &raw {
                    Ok(step) => step.progress(),
                    Err(error) => error.retained_progress(),
                };
                return (raw.map(|_| RetainedHistorySourceStep::Progress).map_err(RetainedHistoryOpMetaFault::from), Some((grant, progress)));
            }
            let result = if !*admitted {
                match RetainedHistorySourceDecode::admit(source, range, limits, persisted_document, control, grant) {
                    Ok(Some(owner)) => {
                        **decoder = Some(owner);
                        *admitted = true;
                        Ok(RetainedHistorySourceStep::Progress)
                    }
                    Ok(None) => Ok(RetainedHistorySourceStep::Blocked),
                    Err(error) => Err(error),
                }
            } else if let Some(owner) = decoder.as_mut() {
                if owner.terminal_is_empty() {
                    decoder.take();
                    Ok(RetainedHistorySourceStep::Closed)
                } else if owner.is_ready() {
                    owner.take_ready_into(history, grant).map(|ready| if ready { RetainedHistorySourceStep::Ready } else { RetainedHistorySourceStep::Blocked })
                } else {
                    owner.step(source, control, grant)
                }
            } else {
                Ok(RetainedHistorySourceStep::Ready)
            };
            let issued = decoder.as_mut().and_then(|owner| owner.take_receipt());
            if let Some(owner) = decoder.as_ref() {
                if let Ok(next) = owner.demands(source, control, grant.maximum_copy_bytes) {
                    *quote = next;
                }
            }
            (result, issued)
        })?;
        let step = match result {
            Ok(step) => step,
            Err(RetainedHistoryOpMetaFault::Value(primary)) => {
                self.closing = true;
                return Err(receive_failure(issued, grant, primary));
            }
            Err(error) => {
                *self.fault = Some(error);
                self.closing = true;
                RetainedHistorySourceStep::Fault
            }
        };
        let progress = receive(issued, grant)?;
        self.issued = issued;
        if self.fault.is_some() {
            return Ok(RetainedHistorySourceStep::Fault);
        }
        if self.admitted && self.decoder.is_none() && self.recipient.terminal_is_empty() && issued.is_none() {
            self.receipt.take();
            self.closed = true;
            self.issued = Some((grant, RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            return Ok(RetainedHistorySourceStep::Ready);
        }
        Ok(if progress.copied_items > 0 { RetainedHistorySourceStep::Progress } else { step })
    }

    pub fn close_step<S: RetainedWireByteSource + ?Sized>(&mut self, source: &S, turn: &dyn RetainedHistoryTurn, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        self.closing = true;
        if turn.should_yield() || grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if !permits(grant, demand) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if self.active.is_some() || self.fault.is_some() || self.inner_fault.is_some() || self.decoder.is_none() && self.partial.is_some() {
            let active = &mut self.active;
            let fault = &mut self.fault;
            let inner_fault = &mut self.inner_fault;
            let partial = &mut self.partial;
            let raw = with_control(&mut self.receipt, &mut self.recipient, self.maximum, turn, grant.maximum_capacity_bytes, true, |control| {
                control.charge(demand.capacity_bytes)?;
                if active.is_some() {
                    close_box(active, grant)
                } else if fault.is_some() {
                    admit_owned(fault, active, grant)
                } else if inner_fault.is_some() {
                    admit_owned(inner_fault, active, grant)
                } else {
                    admit_owned(partial, active, grant)
                }
            })?;
            return raw;
        }
        if self.decoder.is_some() {
            let decoder = &mut self.decoder;
            let partial = &mut self.partial;
            let fault = &mut self.inner_fault;
            let quote = &mut self.demand;
            let (result, issued) = with_control(&mut self.receipt, &mut self.recipient, self.maximum, turn, grant.maximum_capacity_bytes, true, |control| {
                let owner = decoder.as_mut().unwrap();
                owner.cancel();
                let result = if owner.terminal_is_empty() {
                    Ok(RetainedHistorySourceStep::Closed)
                } else if owner.fault().is_some() {
                    owner.take_fault_into(fault, grant).map(|_| RetainedHistorySourceStep::Progress)
                } else if owner.partial_is_receivable() {
                    owner.take_partial_into(partial, grant).map(|_| RetainedHistorySourceStep::Progress)
                } else {
                    owner.close_step(source, control, grant)
                };
                let issued = owner.take_receipt();
                if let Ok(next) = owner.demands(source, control, grant.maximum_copy_bytes) {
                    *quote = next;
                }
                (result, issued)
            })?;
            match result {
                Err(RetainedHistoryOpMetaFault::Value(primary)) => return Err(receive_failure(issued, grant, primary)),
                Err(error) => *self.fault = Some(error),
                Ok(_) => {}
            }
            let progress = receive(issued, grant)?;
            if self.decoder.as_ref().is_some_and(RetainedHistorySourceDecode::terminal_is_empty) {
                self.decoder.take();
                if issued.is_none() {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                }
            }
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if !self.recipient.terminal_is_empty() {
            let raw = with_control(&mut self.receipt, &mut self.recipient, self.maximum, turn, grant.maximum_capacity_bytes, true, |control| control.close_retirement_recipient(grant))?;
            return raw;
        }
        self.receipt.take();
        self.closed = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closed && self.decoder.is_none() && self.partial.is_none() && self.fault.is_none() && self.inner_fault.is_none() && self.active.is_none() && self.recipient.terminal_is_empty() && self.receipt.is_none()
    }
}

impl Drop for RetainedHistoryDecode {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "retained history decoder reached Drop before every native receiving owner was closed");
        if self.terminal_is_empty() {
            unsafe {
                ManuallyDrop::drop(&mut self.decoder);
                ManuallyDrop::drop(&mut self.partial);
                ManuallyDrop::drop(&mut self.fault);
                ManuallyDrop::drop(&mut self.inner_fault);
                ManuallyDrop::drop(&mut self.active);
            }
        }
    }
}

async fn flush_dict_delta<S: PackSink>(writer: &mut SprWriter<S>, dict: &DictBuilder, base: &mut u32) -> Result<(), ProtocolError> where ProtocolError: From<S::Error> {
    let len = dict.len();
    if len > *base {
        let entries = dict.entries_since(*base);
        let mut payload = ByteWriter::new();
        payload.write_u8(1);
        payload.write_varint_u64(*base as u64);
        payload.write_varint_u64(entries.len() as u64);
        for entry in entries {
            payload.write_varint_u64(entry.len() as u64);
            payload.write_bytes(entry.as_bytes());
        }
        writer.write_record(protocol::REC_STR_DICT, true, &payload.into_bytes(), CodecId(0)).await?;
        *base = len;
    }
    Ok(())
}

/// 🎞️ `(commit_seq, chain_hash)` out of a `REC_COMMIT` frame's payload — thin wrapper over
/// `protocol::format::parse_commit_payload` (public since that crate's own follow-up review pass)
/// so `VerificationLevel::Full`'s chain recompute doesn't need this crate's own byte-offset copy.
async fn parse_commit_fields(payload: &[u8]) -> Result<(u64, [u8; 32]), ProtocolError> {
    let commit = protocol::format::parse_commit_payload(payload)?;
    Ok((commit.commit_seq, commit.chain_hash))
}

pub async fn encode_history(log: &HistoryLog, options: &EncodeOptions) -> Result<Vec<u8>, ProtocolError> {
    if log.edits.len() as u64 > options.limits.max_record_count {
        return Err(ProtocolError::LimitExceeded("edit count exceeds ProtocolLimits::max_record_count"));
    }
    let write_options = WriteOptions { required_flags: protocol::REQUIRED_HASH_CHAIN, optional_flags: if options.canonical { protocol::OPTIONAL_CANONICAL } else { 0 } };
    let mut writer = SprWriter::begin(Vec::<u8>::new(), &write_options).await?;
    let mut dict = DictBuilder::new();
    let mut dict_base = 0u32;

    let doc_payload = encode_doc(&log.doc_id, &log.schema, &mut dict).await;
    flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
    writer.write_record(protocol::REC_DOC, true, &doc_payload, CodecId(0)).await?;

    // 🎯️ Built incrementally (an edit's own id is inserted only AFTER it is encoded), matching
    // `HistoryAppender::append_edit`'s streaming semantics and the decoder's causal resolution
    // (`EditIter`/`prescan_full` only ever know edits already decoded). A one-shot, whole-list
    // `ordinals` map would let an edit's own `mutation_meta[i].mutation_id` — legitimately equal
    // to `edit.id` for a single-op edit — resolve to a self-referencing ordinal at encode time,
    // which the decoder can never resolve (it hasn't registered the current edit's id yet).
    let mut ordinals: HashMap<&str, u64> = HashMap::new();
    for (index, edit) in log.edits.iter().enumerate() {
        // 🎯️ `write_backwards_section` is a batch-level policy switch: even when `edit.inverse`
        // is populated (e.g. by a live store that always computes it), a caller can opt out of
        // persisting it here. `HistoryAppender::append_edit` has no such switch — its streaming,
        // one-edit-at-a-time API gives the caller direct per-edit control via the data itself.
        let payload = if options.write_backwards_section {
            encode_edit(edit, &mut dict, |id| ordinals.get(id).copied()).await?
        } else {
            let stripped = HistoryEdit { inverse: Vec::new(), ..edit.clone() };
            encode_edit(&stripped, &mut dict, |id| ordinals.get(id).copied()).await?
        };
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(protocol::REC_EDIT, true, &payload, CodecId(0)).await?;
        ordinals.insert(edit.id.as_str(), index as u64);
    }
    for transition in &log.transitions {
        let payload = encode_transition(transition, &mut dict).await?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(REC_TRANSITION, true, &payload, CodecId(0)).await?;
    }

    if let Some(composition) = &log.composition {
        let composition_payload = encode_composition(composition, &mut dict).await?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(REC_COMPOSITION, false, &composition_payload, CodecId(0)).await?;
    }

    if !log.conflicts.is_empty() {
        let conflicts_payload = encode_conflicts(&log.conflicts, &mut dict, |id| ordinals.get(id).copied()).await?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(REC_CONFLICT, false, &conflicts_payload, CodecId(0)).await?;
    }
    if log.viewer_line.is_some() || log.viewer_checkpoint.is_some() {
        let viewer_payload = encode_viewer(log.viewer_line.as_deref(), log.viewer_checkpoint.as_deref(), &mut dict).await?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(REC_VIEWER, false, &viewer_payload, CodecId(0)).await?;
    }

    writer.commit().await?;
    Ok(writer.into_sink().await)
}

async fn decode_history_from(trusted: &[u8], options: &DecodeOptions) -> Result<HistoryLog, ProtocolError> {
    let mut dict = DictReader::new();
    let mut edit_ids: Vec<String> = Vec::new();
    let mut log = HistoryLog::default();
    let mut cursor = FrameCursor::new(trusted, HEADER_SIZE as u64).await;
    let hasher = Blake3Hasher;
    let full = options.verification == VerificationLevel::Full;
    let mut running_chain = if full { hasher.hash(&trusted[..HEADER_SIZE]) } else { [0u8; 32] };
    let mut pending_digests: Vec<[u8; 32]> = Vec::new();
    let mut saw_conflicts = false;

    while let Some(frame) = cursor.next_frame().await? {
        if full && frame.kind != protocol::REC_COMMIT {
            let frame_bytes = &trusted[frame.offset as usize..(frame.offset + frame.frame_len().await) as usize];
            pending_digests.push(hasher.hash(frame_bytes));
        }
        match frame.kind {
            protocol::REC_STR_DICT => apply_dict_record(&mut dict, frame.payload().await).await?,
            protocol::REC_ACTOR_DICT => {} // v1 never splits an actor dictionary — see 🔖️Payloads note
            protocol::REC_DOC => {
                let (doc_id, schema) = decode_doc(frame.payload().await, &dict).await?;
                log.doc_id = doc_id;
                log.schema = schema;
            }
            protocol::REC_EDIT => {
                let edit_ids_ref = &edit_ids;
                let edit = decode_edit(frame.payload().await, &dict, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await?;
                edit_ids.push(edit.id.clone());
                log.edits.push(edit);
            }
            REC_TRANSITION => log.transitions.push(decode_transition(frame.payload().await, &dict).await?),
            REC_COMPOSITION => log.composition = Some(decode_composition(frame.payload().await, &dict).await?),
            REC_CONFLICT => {
                if saw_conflicts {
                    return Err(ProtocolError::Malformed { what: "history", offset: frame.offset, detail: "duplicate conflict record".to_string() });
                }
                let edit_ids_ref = &edit_ids;
                log.conflicts = decode_conflicts(frame.payload().await, &dict, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await?;
                saw_conflicts = true;
            }
            REC_VIEWER => {
                let (viewer_line, viewer_checkpoint) = decode_viewer(frame.payload().await, &dict).await?;
                log.viewer_line = viewer_line;
                log.viewer_checkpoint = viewer_checkpoint;
            }
            protocol::REC_COMMIT if full => {
                let (commit_seq, chain_hash) = parse_commit_fields(frame.payload().await).await?;
                let mut concat = running_chain.to_vec();
                for digest in &pending_digests {
                    concat.extend_from_slice(digest);
                }
                let recomputed = hasher.hash(&concat);
                if recomputed != chain_hash {
                    return Err(ProtocolError::ChainMismatch { commit_seq });
                }
                running_chain = recomputed;
                pending_digests.clear();
            }
            // Every other kind (REC_PROJECTION, REC_INDEX, REC_FRONTIER, extension range, ...) is
            // foreign to this crate's semantic layer — skip regardless of the frame's critical bit;
            // this reader only enforces criticality for its own known kind set handled above.
            _ => {}
        }
    }
    Ok(log)
}

pub async fn decode_history(bytes: &[u8], options: &DecodeOptions) -> Result<HistoryLog, ProtocolError> {
    HistoryReader::open(bytes, options).await?.log().await
}
//#endregion 🔖️Codec

//#region 🔖️Append
// Streaming append API over protocol::format::SprWriter — the hot path. One edit -> one REC_EDIT
// frame, O(new edit) allocation.
pub struct HistoryAppender<S: PackSink> {
    writer: SprWriter<S>,
    dict: DictBuilder,
    dict_base: u32,
    edit_ordinals: HashMap<String, u64>,
    next_edit_ordinal: u64,
}

impl<S: PackSink> HistoryAppender<S> where ProtocolError: From<S::Error> {
    pub async fn begin(sink: S, doc_id: &str, schema: &str, options: &WriteOptions) -> Result<Self, ProtocolError> {
        let mut writer = SprWriter::begin(sink, options).await?;
        let mut dict = DictBuilder::new();
        let mut dict_base = 0u32;
        let payload = encode_doc(doc_id, schema, &mut dict).await;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(protocol::REC_DOC, true, &payload, CodecId(0)).await?;
        Ok(Self { writer, dict, dict_base, edit_ordinals: HashMap::new(), next_edit_ordinal: 0 })
    }

    pub async fn append_edit(&mut self, edit: &HistoryEdit) -> Result<u64, ProtocolError> {
        let ordinals = &self.edit_ordinals;
        let payload = encode_edit(edit, &mut self.dict, |id| ordinals.get(id).copied()).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        let offset = self.writer.write_record(protocol::REC_EDIT, true, &payload, CodecId(0)).await?;
        self.edit_ordinals.insert(edit.id.clone(), self.next_edit_ordinal);
        self.next_edit_ordinal += 1;
        Ok(offset)
    }

    /// 🔀️ Appends one `REC_TRANSITION` frame (critical); returns its offset.
    pub async fn append_transition(&mut self, transition: &HistoryTransitionRecord) -> Result<u64, ProtocolError> {
        let payload = encode_transition(transition, &mut self.dict).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        self.writer.write_record(REC_TRANSITION, true, &payload, CodecId(0)).await
    }

    /// 🧩️ Appends the composition overlay record (skippable extension); returns its offset.
    pub async fn append_composition(&mut self, composition: &HistoryComposition) -> Result<u64, ProtocolError> {
        let payload = encode_composition(composition, &mut self.dict).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        self.writer.write_record(REC_COMPOSITION, false, &payload, CodecId(0)).await
    }

    /// ⚔️ Appends the log's one conflict record, resolving `Degraded` edit references against
    /// the edits appended so far; a file carries at most one such record.
    pub async fn append_conflicts(&mut self, conflicts: &[HistoryConflict]) -> Result<u64, ProtocolError> {
        let ordinals = &self.edit_ordinals;
        let payload = encode_conflicts(conflicts, &mut self.dict, |id| ordinals.get(id).copied()).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        self.writer.write_record(REC_CONFLICT, false, &payload, CodecId(0)).await
    }

    /// 👁 Appends this replica's head. Both fields absent means the canonical trunk tip, and the record is omitted.
    pub async fn append_viewer(&mut self, line: Option<&str>, checkpoint: Option<&str>) -> Result<u64, ProtocolError> {
        let payload = encode_viewer(line, checkpoint, &mut self.dict).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        self.writer.write_record(REC_VIEWER, false, &payload, CodecId(0)).await
    }

    pub async fn commit(&mut self) -> Result<u64, ProtocolError> {
        self.writer.commit().await
    }

    pub async fn into_sink(self) -> S {
        self.writer.into_sink().await
    }
}
//#endregion 🔖️Append

//#region 🔖️Scan
// Read-side over a byte buffer, via protocol_format cursors. `open` establishes a trusted byte
// range via `protocol::format::recover` (RecoveryMode::LastCommit) once; every subsequent
// operation stays within that range, so a torn tail can never surface a partially-written record.
pub struct HistoryReader<'a> {
    trusted: &'a [u8],
    options: DecodeOptions,
}

impl<'a> HistoryReader<'a> {
    pub async fn open(bytes: &'a [u8], options: &DecodeOptions) -> Result<Self, ProtocolError> {
        let recovery = protocol::format::recover(&bytes, &options.limits, RecoveryMode::LastCommit).await?;
        let trusted = &bytes[..recovery.bytes_recovered as usize];
        Ok(Self { trusted, options: options.clone() })
    }

    pub async fn log(&self) -> Result<HistoryLog, ProtocolError> {
        decode_history_from(self.trusted, &self.options).await
    }

    pub async fn edits(&self) -> EditIter<'a> {
        EditIter { cursor: FrameCursor::new(self.trusted, HEADER_SIZE as u64).await, dict: DictReader::new(), edit_ids: Vec::new() }
    }

    pub async fn edits_rev(&self, limit: usize) -> RevEditIter<'a> {
        match prescan_full(self.trusted).await {
            Ok((dict, edit_ids)) => RevEditIter { state: Ok(RevEditIterReady { cursor: ReverseFrameCursor::at_end(&self.trusted[HEADER_SIZE..]).await, dict, edit_ids, remaining: limit }) },
            Err(e) => RevEditIter { state: Err(Some(e)) },
        }
    }
}

pub struct EditIter<'a> {
    cursor: FrameCursor<'a>,
    dict: DictReader,
    edit_ids: Vec<String>,
}

impl<'a> Iterator for EditIter<'a> {
    type Item = Result<HistoryEdit, ProtocolError>;

    // 🚫️async: E1 — `Iterator::next` is an externally-declared trait method, so it must stay a
    // plain sync `fn`. Its callees (`next_frame`/`payload`/`apply_dict_record`/`decode_edit`) are
    // pure in-memory byte-buffer parsing with no real suspension point, so they are resolved via
    // the crate's one sanctioned E5 bridge (`io::resolve_ready`) rather than awaited.
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match ::semio_framework_async::poll::resolve_ready(self.cursor.next_frame()) {
                Ok(Some(frame)) => match frame.kind {
                    protocol::REC_STR_DICT => {
                        let payload = ::semio_framework_async::poll::resolve_ready(frame.payload());
                        if let Err(e) = ::semio_framework_async::poll::resolve_ready(apply_dict_record(&mut self.dict, payload)) {
                            return Some(Err(e));
                        }
                    }
                    protocol::REC_EDIT => {
                        let edit_ids_ref = &self.edit_ids;
                        let dict_ref = &self.dict;
                        let payload = ::semio_framework_async::poll::resolve_ready(frame.payload());
                        let result = ::semio_framework_async::poll::resolve_ready(decode_edit(payload, dict_ref, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))));
                        match result {
                            Ok(edit) => {
                                self.edit_ids.push(edit.id.clone());
                                return Some(Ok(edit));
                            }
                            Err(e) => return Some(Err(e)),
                        }
                    }
                    _ => continue,
                },
                Ok(None) => return None,
                Err(e) => return Some(Err(e)),
            }
        }
    }
}

struct RevEditIterReady<'a> {
    cursor: ReverseFrameCursor<'a>,
    dict: DictReader,
    edit_ids: Vec<String>,
    remaining: usize,
}

/// 🚧️ `edits_rev` cannot return `Result` per the frozen contract signature, so a prescan
/// failure (needed to build the full dict + edit-id table up front — see `prescan_full`) is
/// deferred: the first `next()` call yields it, every call after that yields `None`.
pub struct RevEditIter<'a> {
    state: Result<RevEditIterReady<'a>, Option<ProtocolError>>,
}

impl<'a> Iterator for RevEditIter<'a> {
    type Item = Result<HistoryEdit, ProtocolError>;

    // 🚫️async: E1 — same rationale as `EditIter::next` above: `Iterator::next` must stay sync, its
    // callees are pure in-memory parsing, resolved via the crate's one `io::resolve_ready` bridge.
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            Err(pending) => pending.take().map(Err),
            Ok(ready) => {
                if ready.remaining == 0 {
                    return None;
                }
                loop {
                    match ::semio_framework_async::poll::resolve_ready(ready.cursor.prev_frame()) {
                        Ok(Some(frame)) => {
                            if frame.kind == protocol::REC_EDIT {
                                let edit_ids_ref = &ready.edit_ids;
                                let dict_ref = &ready.dict;
                                let payload = ::semio_framework_async::poll::resolve_ready(frame.payload());
                                let result = ::semio_framework_async::poll::resolve_ready(decode_edit(payload, dict_ref, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))));
                                ready.remaining -= 1;
                                return Some(result);
                            }
                        }
                        Ok(None) => return None,
                        Err(e) => {
                            ready.remaining = 0;
                            return Some(Err(e));
                        }
                    }
                }
            }
        }
    }
}

/// 🔎️ Builds the FULL dictionary and the FULL forward-ordered edit-id table in one forward
/// pass. Safe to reuse for decoding any earlier record: dict indices and edit ordinals are both
/// append-only and stable once assigned, so the final state is a superset valid at every offset.
async fn prescan_full(trusted: &[u8]) -> Result<(DictReader, Vec<String>), ProtocolError> {
    let mut dict = DictReader::new();
    let mut edit_ids = Vec::new();
    let mut cursor = FrameCursor::new(trusted, HEADER_SIZE as u64).await;
    while let Some(frame) = cursor.next_frame().await? {
        match frame.kind {
            protocol::REC_STR_DICT => apply_dict_record(&mut dict, frame.payload().await).await?,
            protocol::REC_EDIT => {
                let edit_ids_ref = &edit_ids;
                let dict_ref = &dict;
                let edit = decode_edit(frame.payload().await, dict_ref, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await?;
                edit_ids.push(edit.id);
            }
            _ => {}
        }
    }
    Ok((dict, edit_ids))
}
//#endregion 🔖️Scan

//#region 🔖️Frontier
#[derive(Clone, Debug, PartialEq)]
pub struct AlternativeHead {
    pub alternative_id: String,
    pub checkpoint_id: String,
    pub head_edit_ordinal: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FrontierSummary {
    pub document_id: String,
    pub head_edit_ordinal: u64,
    pub head_edit_id: String,
    pub alternatives: Vec<AlternativeHead>,
    pub last_commit_seq: u64,
    pub chain_hash: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FrontierComparison {
    Equal,
    Ahead,
    Behind,
    Diverged { common_edit_count: u64 },
}

/// 🧭️ Compares two frontiers by head edit ordinal, then head edit id/chain hash for the
/// equal-ordinal case. `Diverged::common_edit_count` is a conservative estimate (the shared
/// ordinal itself) — `FrontierSummary` alone carries no shared-ancestry data to verify a true
/// common ancestor; documented "your choice" per the contract.
pub async fn frontier_delta(local: &FrontierSummary, remote: &FrontierSummary) -> FrontierComparison {
    if local.head_edit_ordinal == remote.head_edit_ordinal && local.head_edit_id == remote.head_edit_id && local.chain_hash == remote.chain_hash {
        return FrontierComparison::Equal;
    }
    if local.head_edit_ordinal > remote.head_edit_ordinal {
        return FrontierComparison::Ahead;
    }
    if local.head_edit_ordinal < remote.head_edit_ordinal {
        return FrontierComparison::Behind;
    }
    FrontierComparison::Diverged { common_edit_count: local.head_edit_ordinal }
}
//#endregion 🔖️Frontier

//#region 🔖️Index
// Advisory REC_INDEX payload sections. Rebuildable from scan; never authoritative. Byte layout is
// this crate's own choice (the contract only fixes the section-kind bytes): format u8, then zero
// or more `(section_kind: u8, count: varint, count x entry)` sections back-to-back.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

#[cfg(test)]
use semio_framework_os_history_testing_retirement as test_retirement;
