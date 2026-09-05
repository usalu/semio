//! 🗂️ One verified input owns dictionary ranges, semantic identity and every rejection until close.
//! No typed snapshot/history/store or public member is constructed by this private handoff.

#[path = "🛂️identity/🦀️.rs"]
mod identity;
#[path = "📇️index/🦀️.rs"]
mod index;
#[path = "🧾️record/🦀️.rs"]
mod record;

use super::{diagnostic, ErasedSnapshotRetirement, MemberOpenDiagnostic, MemberOpenPhase, MemberOpenProgress, MemberOpenRequest, SnapshotRetirementStep, VerifiedMemberHistoryInput};
use crate::os_spr::format::retained::{record::RetainedSprRecordObservation, RetainedSprLimits, RetainedSprVerification};
use crate::os_spr::history::identity::id::{HistoryIdDiagnostic, RetainedHistoryIdV1};
use identity::SemanticRecord;
use index::{DictionaryIndexClose, DictionaryIndexError, DictionaryRange, RetainedDictionaryIndex};
use record::{DictionaryDeltaError, DictionaryDeltaEvent, RetainedDictionaryDelta};
use semio_framework_job::StepContext;
use std::mem::ManuallyDrop;

#[derive(Clone, Copy)]
pub(crate) struct MemberHistoryDictionaryLimits {
    pub dictionary_entries: usize,
    pub dictionary_bytes: u64,
    pub pin_groups: u64,
    pub pins: u64,
    pub records: u64,
}

impl Default for MemberHistoryDictionaryLimits {
    fn default() -> Self {
        Self { dictionary_entries: 8192, dictionary_bytes: 1048576, pin_groups: 1024, pins: 8192, records: 8192 }
    }
}

pub(crate) enum MemberHistoryDictionaryStep {
    Pending(MemberOpenProgress),
    Ready,
    Rejected(MemberOpenDiagnostic),
}
pub(crate) struct MemberHistoryDictionaryAdmissionError {
    pub diagnostic: MemberOpenDiagnostic,
    pub input: VerifiedMemberHistoryInput,
}

struct DictionaryOwners {
    input: ManuallyDrop<Option<VerifiedMemberHistoryInput>>,
    index: ManuallyDrop<Option<RetainedDictionaryIndex>>,
}

impl DictionaryOwners {
    fn request(&self) -> Result<&MemberOpenRequest, MemberOpenDiagnostic> {
        let input = self.input.as_ref().ok_or(MemberOpenDiagnostic::Stale)?;
        if let Some(error) = input.diagnostic {
            return Err(error);
        }
        input.request.as_ref().ok_or(MemberOpenDiagnostic::Stale)
    }
    fn check(&self, cx: &StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
        self.request()?.check_step_authority(cx)
    }
    fn terminal_is_empty(&self) -> bool {
        self.input.is_none() && self.index.is_none()
    }
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.terminal_is_empty() {
            return Ok(SnapshotRetirementStep::Complete);
        }
        if items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(index) = self.index.as_mut() {
            return Ok(match index.close_step(items, bytes) {
                DictionaryIndexClose::Complete if index.terminal_is_empty() => {
                    self.index.take();
                    SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }
                }
                DictionaryIndexClose::Complete => return Err("dictionary index returned false terminal".into()),
                DictionaryIndexClose::Pending { released_items, released_bytes } => SnapshotRetirementStep::Pending { released_items, released_bytes },
            });
        }
        let input = self.input.as_mut().ok_or("dictionary input owner is absent")?;
        match input.close_step(items, bytes)? {
            SnapshotRetirementStep::Complete if input.terminal_is_empty() => {
                self.input.take();
                Ok(SnapshotRetirementStep::Complete)
            }
            SnapshotRetirementStep::Complete => Err("dictionary input returned false terminal".into()),
            SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > items || released_bytes > bytes => Err("dictionary input exceeded retirement grant".into()),
            step => Ok(step),
        }
    }
}

impl Drop for DictionaryOwners {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "dictionary owners require bounded retirement");
    }
}

struct PendingByte {
    offset: u64,
    value: u8,
    framed: bool,
}
struct DictionaryLookup {
    range: DictionaryRange,
    copied: u64,
}

pub(crate) struct MemberHistoryDictionaryOwner {
    owners: ManuallyDrop<Option<DictionaryOwners>>,
    scanner: Option<RetainedSprVerification>,
    record: Option<RetainedSprRecordObservation>,
    delta: Option<RetainedDictionaryDelta>,
    semantic: Option<SemanticRecord>,
    id: Option<RetainedHistoryIdV1>,
    lookup: Option<DictionaryLookup>,
    lookup_byte: Option<u8>,
    pending: Option<PendingByte>,
    schema: &'static str,
    limits: MemberHistoryDictionaryLimits,
    end: u64,
    groups: u64,
    pins: u64,
    document_seen: bool,
    document_matches: bool,
    composition_matches: bool,
    active_seen: bool,
    cursor_seen: bool,
    composition_seen: bool,
    initial_history_exact: bool,
    initial_payload_bytes: u8,
    payload_done: bool,
    record_matches: bool,
    id_retiring: bool,
    ready: bool,
    closing: bool,
    error: Option<MemberOpenDiagnostic>,
    transition: &'static str,
}

impl MemberHistoryDictionaryOwner {
    pub(super) fn begin(input: VerifiedMemberHistoryInput, schema: &'static str, limits: MemberHistoryDictionaryLimits, cx: &StepContext<'_>) -> Result<Self, MemberHistoryDictionaryAdmissionError> {
        let valid_limits = limits.dictionary_entries <= 8192 && limits.dictionary_bytes <= 1048576 && limits.pin_groups <= 1024 && limits.pins <= 8192 && limits.records > 0 && limits.records <= 8192;
        let admitted = input.diagnostic.map_or(Ok(()), Err).and_then(|_| input.request.as_ref().ok_or(MemberOpenDiagnostic::Stale)?.check_step_authority(cx));
        let error = admitted.err().or_else(|| (!valid_limits).then_some(MemberOpenDiagnostic::Capacity)).or_else(|| (schema.is_empty() || schema.len() > 256 || schema.chars().any(char::is_control)).then_some(MemberOpenDiagnostic::Identity));
        if let Some(diagnostic) = error {
            return Err(MemberHistoryDictionaryAdmissionError { diagnostic, input });
        }
        let end = input.verified_end();
        let scanner = match RetainedSprVerification::new(end, RetainedSprLimits { records: limits.records, ..RetainedSprLimits::default() }) {
            Ok(scanner) => scanner,
            Err(error) => return Err(MemberHistoryDictionaryAdmissionError { diagnostic: diagnostic(error), input }),
        };
        let index = match RetainedDictionaryIndex::new(end, limits.dictionary_entries, limits.dictionary_bytes) {
            Ok(index) => index,
            Err(error) => return Err(MemberHistoryDictionaryAdmissionError { diagnostic: index_error(error), input }),
        };
        Ok(Self {
            owners: ManuallyDrop::new(Some(DictionaryOwners { input: ManuallyDrop::new(Some(input)), index: ManuallyDrop::new(Some(index)) })),
            scanner: Some(scanner),
            record: None,
            delta: None,
            semantic: None,
            id: None,
            lookup: None,
            lookup_byte: None,
            pending: None,
            schema,
            limits,
            end,
            groups: 0,
            pins: 0,
            document_seen: false,
            document_matches: false,
            composition_matches: false,
            active_seen: false,
            cursor_seen: false,
            composition_seen: false,
            initial_history_exact: true,
            initial_payload_bytes: 0,
            payload_done: false,
            record_matches: false,
            id_retiring: false,
            ready: false,
            closing: false,
            error: None,
            transition: "begin",
        })
    }

    fn reject(&mut self, error: MemberOpenDiagnostic) -> MemberOpenDiagnostic {
        if let Some(index) = self.owners.as_mut().and_then(|owners| owners.index.as_mut()) {
            index.reject_record();
        }
        *self.error.get_or_insert(error)
    }
    fn check(&mut self, cx: &StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
        if self.closing {
            return Err(MemberOpenDiagnostic::Cancelled);
        }
        if let Some(error) = self.error {
            return Err(error);
        }
        let result = self.owners.as_ref().ok_or(MemberOpenDiagnostic::Stale).and_then(|owners| owners.check(cx));
        result.map_err(|error| self.reject(error))
    }
    fn progress(&self) -> MemberHistoryDictionaryStep {
        MemberHistoryDictionaryStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::History, completed: self.scanner.as_ref().map_or(self.end, RetainedSprVerification::consumed), total: self.end })
    }

    pub(crate) fn step(&mut self, cx: &mut StepContext<'_>) -> MemberHistoryDictionaryStep {
        if let Err(error) = self.check(cx) {
            return MemberHistoryDictionaryStep::Rejected(error);
        }
        if self.ready {
            return MemberHistoryDictionaryStep::Ready;
        }
        cx.set_stage("member-open.history.dictionary");
        while !cx.should_yield() {
            if let Err(error) = self.check(cx) {
                return MemberHistoryDictionaryStep::Rejected(error);
            }
            if let Err(error) = self.unit(cx) {
                return MemberHistoryDictionaryStep::Rejected(self.reject(error));
            }
            if let Err(error) = self.check(cx) {
                return MemberHistoryDictionaryStep::Rejected(error);
            }
            if self.ready {
                return MemberHistoryDictionaryStep::Ready;
            }
        }
        self.progress()
    }

    fn unit(&mut self, cx: &mut StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
        let owners = self.owners.as_mut().ok_or(MemberOpenDiagnostic::Stale)?;
        if self.delta.as_ref().is_some_and(RetainedDictionaryDelta::has_event) {
            cx.consume_fuel(1);
            let event = self.delta.as_mut().unwrap().take_event().map_err(delta_error)?.ok_or(MemberOpenDiagnostic::Malformed)?;
            let index = owners.index.as_mut().ok_or(MemberOpenDiagnostic::Stale)?;
            match event {
                DictionaryDeltaEvent::Begin { base, count } => {
                    index.begin_delta(base, count).map_err(index_error)?;
                    self.transition = "delta-begin";
                }
                DictionaryDeltaEvent::Entry { offset, length } => {
                    index.append(DictionaryRange { offset, length }).map_err(index_error)?;
                    self.transition = "entry";
                }
            }
            return Ok(());
        }
        if self.id_retiring {
            cx.consume_fuel(1);
            self.transition = "id-retire";
            if self.id.as_mut().ok_or(MemberOpenDiagnostic::Stale)?.close_bytes(1) == 0 {
                self.id.take();
                self.id_retiring = false;
                self.lookup = None;
            }
            return Ok(());
        }
        if let Some(id) = self.id.as_mut() {
            if id.is_complete() {
                cx.consume_fuel(1);
                self.transition = "id-complete";
                self.semantic.as_mut().ok_or(MemberOpenDiagnostic::Malformed)?.accept_id(id.finish().map_err(id_error)?, owners.request()?, self.schema)?;
                self.id_retiring = true;
                return Ok(());
            }
            if let Some(index) = id.lookup() {
                cx.consume_fuel(1);
                self.transition = "id-lookup";
                let range = owners.index.as_ref().ok_or(MemberOpenDiagnostic::Stale)?.lookup(index as usize).map_err(index_error)?;
                let length = usize::try_from(range.length).map_err(|_| MemberOpenDiagnostic::Capacity)?;
                id.begin_dictionary(index, length, &mut 1).map_err(id_error)?;
                self.lookup = Some(DictionaryLookup { range, copied: 0 });
                return Ok(());
            }
            if id.dictionary_pending() {
                if let Some(mut byte) = self.lookup_byte.take() {
                    cx.consume_fuel(1);
                    self.transition = "id-feed";
                    let result = id.push_dictionary(byte, &mut 1).map_err(id_error);
                    byte = 0;
                    let _ = byte;
                    result?;
                } else {
                    let lookup = self.lookup.as_mut().ok_or(MemberOpenDiagnostic::Malformed)?;
                    if lookup.copied >= lookup.range.length {
                        return Err(MemberOpenDiagnostic::Malformed);
                    }
                    let mut byte = [0; 1];
                    let offset = usize::try_from(lookup.range.offset + lookup.copied).map_err(|_| MemberOpenDiagnostic::Capacity)?;
                    let copied = owners.input.as_mut().ok_or(MemberOpenDiagnostic::Stale)?.copy_verified_history_chunk(offset, &mut byte, cx)?;
                    if copied == 1 {
                        self.lookup_byte = Some(byte[0]);
                        lookup.copied += 1;
                        self.transition = "id-copy";
                    }
                    byte.fill(0);
                }
                return Ok(());
            }
        }
        if let Some(pending) = self.pending.as_mut() {
            if !pending.framed {
                cx.consume_fuel(1);
                self.transition = "framing";
                self.scanner.as_mut().ok_or(MemberOpenDiagnostic::Stale)?.push(&[pending.value], &mut 1).map_err(diagnostic)?;
                pending.framed = true;
                return Ok(());
            }
            if self.record.is_none() {
                if let Some(record) = self.scanner.as_ref().ok_or(MemberOpenDiagnostic::Stale)?.observe_record_header().map_err(diagnostic)? {
                    cx.consume_fuel(1);
                    self.transition = "record";
                    if let Some(mut consumed) = self.pending.take() {
                        consumed.value = 0;
                    }
                    if (matches!(record.kind(), 1 | 3 | 8) && record.flags() != 2) || (matches!(record.kind(), 64 | 65) && record.flags() != 0) {
                        return Err(MemberOpenDiagnostic::Malformed);
                    }
                    if record.kind() == 1 && self.document_seen {
                        return Err(MemberOpenDiagnostic::Malformed);
                    }
                    if record.kind() == 3 {
                        self.delta = Some(RetainedDictionaryDelta::new(record.payload_start(), record.payload_end()).map_err(delta_error)?);
                    }
                    self.initial_history_exact &= matches!(record.kind(), 1 | 3 | 8 | 64 | 65 | crate::os_spr::REC_COMMIT);
                    self.initial_history_exact &= match record.kind() {
                        8 => !self.active_seen,
                        64 => !self.cursor_seen,
                        65 => !self.composition_seen,
                        _ => true,
                    };
                    self.initial_payload_bytes = 0;
                    if matches!(record.kind(), 1 | 65) {
                        self.semantic = Some(SemanticRecord::new(record.kind()));
                    }
                    self.record = Some(record);
                    self.payload_done = false;
                    return Ok(());
                }
            }
            let payload = self.record.as_ref().is_some_and(|record| pending.offset >= record.payload_start() && pending.offset < record.payload_end());
            if payload && self.semantic.as_ref().is_some_and(SemanticRecord::needs_id) && self.id.is_none() {
                cx.consume_fuel(1);
                self.id = Some(RetainedHistoryIdV1::new());
                self.transition = "id-begin";
                return Ok(());
            }
            cx.consume_fuel(1);
            self.transition = if payload { "payload" } else { "framing-release" };
            let mut pending = self.pending.take().unwrap();
            let result = if payload {
                if matches!(self.record.as_ref().map(RetainedSprRecordObservation::kind), Some(8) | Some(64)) {
                    let expected: &[u8] = if self.record.as_ref().unwrap().kind() == 8 { &[1, 0] } else { &[1, 0, 0, 0] };
                    self.initial_history_exact &= expected.get(self.initial_payload_bytes as usize) == Some(&pending.value);
                    self.initial_payload_bytes = self.initial_payload_bytes.saturating_add(1);
                }
                if let Some(delta) = self.delta.as_mut() {
                    delta.push(pending.value, &mut 1).map(|_| ()).map_err(delta_error)
                } else if let Some(id) = self.id.as_mut() {
                    id.push_wire(pending.value, &mut 1).map(|_| ()).map_err(id_error)
                } else if let Some(semantic) = self.semantic.as_mut() {
                    semantic.push(pending.value, owners.request()?, self.limits, &mut self.groups, &mut self.pins)
                } else {
                    Ok(())
                }
            } else {
                Ok(())
            };
            pending.value = 0;
            result?;
            return Ok(());
        }
        let consumed = self.scanner.as_ref().ok_or(MemberOpenDiagnostic::Stale)?.consumed();
        if let Some(record) = self.record.as_ref() {
            if consumed == record.payload_end() && !self.payload_done {
                cx.consume_fuel(1);
                self.transition = "payload-end";
                if self.id.is_some() {
                    return Err(MemberOpenDiagnostic::Malformed);
                }
                if let Some(delta) = self.delta.as_mut() {
                    delta.finish().map_err(delta_error)?;
                    delta.close_bytes(0);
                    self.delta.take();
                }
                if let Some(semantic) = self.semantic.as_ref() {
                    self.record_matches = semantic.finish()?;
                }
                self.initial_history_exact &= match self.record.as_ref().map(RetainedSprRecordObservation::kind) {
                    Some(8) => self.initial_payload_bytes == 2,
                    Some(64) => self.initial_payload_bytes == 4,
                    _ => true,
                };
                self.semantic = None;
                self.payload_done = true;
                return Ok(());
            }
            if consumed == record.frame_end() {
                cx.consume_fuel(1);
                self.transition = "record-end";
                if !self.payload_done {
                    return Err(MemberOpenDiagnostic::Malformed);
                }
                match record.kind() {
                    3 => {
                        owners.index.as_mut().ok_or(MemberOpenDiagnostic::Stale)?.publish_delta().map_err(index_error)?;
                        self.transition = "delta";
                    }
                    1 => {
                        self.document_seen = true;
                        self.document_matches = self.record_matches;
                        self.transition = "document";
                    }
                    65 => {
                        self.composition_matches = self.record_matches;
                        self.composition_seen = true;
                        self.transition = "composition";
                    }
                    8 => self.active_seen = true,
                    64 => self.cursor_seen = true,
                    _ => {}
                }
                self.record = None;
                return Ok(());
            }
        }
        if consumed == self.end {
            cx.consume_fuel(1);
            self.transition = "ready";
            let span = self.scanner.as_mut().unwrap().finish().map_err(diagnostic)?;
            let original = owners.input.as_ref().and_then(|input| input.span.as_ref()).ok_or(MemberOpenDiagnostic::Stale)?;
            if span.end() != self.end || span.tail() != 0 || span.sequence() != original.sequence() || span.chain() != original.chain() {
                return Err(MemberOpenDiagnostic::Malformed);
            }
            if !self.document_seen || !self.document_matches || !self.composition_matches {
                return Err(MemberOpenDiagnostic::Identity);
            }
            self.ready = true;
            return Ok(());
        }
        let mut byte = [0; 1];
        let copied = owners.input.as_mut().ok_or(MemberOpenDiagnostic::Stale)?.copy_verified_history_chunk(consumed as usize, &mut byte, cx)?;
        if copied == 1 {
            self.pending = Some(PendingByte { offset: consumed, value: byte[0], framed: false });
            self.transition = "copy";
        }
        byte.fill(0);
        Ok(())
    }

    pub(crate) fn take_ready(&mut self, cx: &mut StepContext<'_>) -> Result<Option<VerifiedMemberHistoryDictionary>, MemberOpenDiagnostic> {
        self.check(cx)?;
        if !self.ready || cx.should_yield() {
            return Ok(None);
        }
        cx.consume_fuel(1);
        self.check(cx)?;
        if cx.deadline_exceeded() {
            return Ok(None);
        }
        self.scanner = None;
        self.ready = false;
        let initial_history_exact = self.initial_history_exact && self.active_seen && self.cursor_seen && self.composition_seen;
        Ok(Some(VerifiedMemberHistoryDictionary { owners: ManuallyDrop::new(self.owners.take()), schema: self.schema, initial_history_exact, closing: false, diagnostic: None }))
    }
}

impl ErasedSnapshotRetirement for MemberHistoryDictionaryOwner {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.terminal_is_empty() {
            return Ok(SnapshotRetirementStep::Complete);
        }
        self.closing = true;
        if items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.pending.is_some() || self.lookup_byte.is_some() {
            if bytes == 0 {
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            if let Some(mut pending) = self.pending.take() {
                pending.value = 0;
            } else if let Some(byte) = self.lookup_byte.as_mut() {
                *byte = 0;
                self.lookup_byte.take();
            }
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 1 });
        }
        if let Some(delta) = self.delta.as_mut() {
            let released_bytes = delta.close_bytes(bytes);
            let released_items = usize::from(delta.terminal_is_empty());
            if released_items == 1 {
                self.delta.take();
            }
            return Ok(SnapshotRetirementStep::Pending { released_items, released_bytes });
        }
        if let Some(id) = self.id.as_mut() {
            if bytes == 0 {
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let released_bytes = id.close_bytes(bytes);
            let released_items = usize::from(released_bytes < bytes);
            if released_items == 1 {
                self.id.take();
            }
            return Ok(SnapshotRetirementStep::Pending { released_items, released_bytes });
        }
        self.scanner = None;
        self.record = None;
        self.semantic = None;
        self.lookup = None;
        self.id_retiring = false;
        self.ready = false;
        if let Some(owners) = self.owners.as_mut() {
            let result = owners.close_step(items, bytes)?;
            if matches!(result, SnapshotRetirementStep::Complete) && owners.terminal_is_empty() {
                self.owners.take();
            }
            return Ok(result);
        }
        Ok(SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool {
        self.owners.is_none() && self.pending.is_none() && self.lookup_byte.is_none() && self.delta.is_none() && self.id.is_none() && self.scanner.is_none() && self.record.is_none() && self.semantic.is_none() && self.lookup.is_none()
    }
}

impl Drop for MemberHistoryDictionaryOwner {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "dictionary operation requires retained close or handoff");
    }
}

pub(crate) struct VerifiedMemberHistoryDictionary {
    owners: ManuallyDrop<Option<DictionaryOwners>>,
    schema: &'static str,
    initial_history_exact: bool,
    closing: bool,
    diagnostic: Option<MemberOpenDiagnostic>,
}

impl VerifiedMemberHistoryDictionary {
    pub(super) fn check_step_authority(&mut self, cx: &StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
        if let Some(error) = self.diagnostic {
            return Err(error);
        }
        let checked = if self.closing { Err(MemberOpenDiagnostic::Cancelled) } else { self.owners.as_ref().ok_or(MemberOpenDiagnostic::Stale).and_then(|owners| owners.check(cx)) };
        if let Err(error) = checked {
            self.diagnostic = Some(error);
            if let Some(index) = self.owners.as_mut().and_then(|owners| owners.index.as_mut()) {
                index.reject_record();
            }
        }
        checked
    }

    pub(super) fn initial_history_is_exact(&self) -> bool {
        self.initial_history_exact
    }

    pub(super) fn clone_initial_identity(&mut self, cx: &StepContext<'_>) -> Result<(crate::os_io::ArtifactRef, Option<crate::os_store::OwnerRef>, &'static str), MemberOpenDiagnostic> {
        self.check_step_authority(cx)?;
        let request = self.owners.as_ref().ok_or(MemberOpenDiagnostic::Stale)?.request()?;
        Ok((request.admitted_expected()?.clone(), request.owner().cloned(), self.schema))
    }
}

impl ErasedSnapshotRetirement for VerifiedMemberHistoryDictionary {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
        self.closing = true;
        let Some(owners) = self.owners.as_mut() else {
            return Ok(SnapshotRetirementStep::Complete);
        };
        let result = owners.close_step(items, bytes)?;
        if matches!(result, SnapshotRetirementStep::Complete) && owners.terminal_is_empty() {
            self.owners.take();
        }
        Ok(result)
    }
    fn terminal_is_empty(&self) -> bool {
        self.owners.is_none()
    }
}

impl Drop for VerifiedMemberHistoryDictionary {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "semantic input witness requires bounded retirement");
    }
}

fn index_error(error: DictionaryIndexError) -> MemberOpenDiagnostic {
    match error {
        DictionaryIndexError::Capacity => MemberOpenDiagnostic::Capacity,
        DictionaryIndexError::State => MemberOpenDiagnostic::Stale,
        DictionaryIndexError::Malformed => MemberOpenDiagnostic::Malformed,
    }
}
fn delta_error(error: DictionaryDeltaError) -> MemberOpenDiagnostic {
    match error {
        DictionaryDeltaError::Capacity => MemberOpenDiagnostic::Capacity,
        DictionaryDeltaError::State => MemberOpenDiagnostic::Stale,
        DictionaryDeltaError::Cancelled => MemberOpenDiagnostic::Cancelled,
        DictionaryDeltaError::Malformed => MemberOpenDiagnostic::Malformed,
    }
}
fn id_error(error: HistoryIdDiagnostic) -> MemberOpenDiagnostic {
    match error {
        HistoryIdDiagnostic::Capacity => MemberOpenDiagnostic::Capacity,
        HistoryIdDiagnostic::State => MemberOpenDiagnostic::Stale,
        HistoryIdDiagnostic::Cancelled => MemberOpenDiagnostic::Cancelled,
        HistoryIdDiagnostic::Identity => MemberOpenDiagnostic::Identity,
        HistoryIdDiagnostic::Malformed => MemberOpenDiagnostic::Malformed,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
