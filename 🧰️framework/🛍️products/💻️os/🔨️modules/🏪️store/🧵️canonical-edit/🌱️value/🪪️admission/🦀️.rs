//! 🪪️ Bounded admission of immutable duplicate-free dynamic values to canonical Store encoding.

use crate::DslValue;
use protocol::value::DslValueSource;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::mem::ManuallyDrop;

/// 🔏️ An immutable source whose full object tree has passed bounded canonical admission.
#[must_use]
pub struct ArtifactCanonicalValue<R: DslValueSource> {
    source: R,
}

impl<R: DslValueSource> ArtifactCanonicalValue<R> {
    pub fn into_source(self) -> R {
        self.source
    }
    pub(super) fn value(&self) -> &DslValue {
        self.source.value()
    }
}

/// 📏️ Requested and admitted key-index capacity limits, excluding fixed traversal storage and the source.
#[derive(Clone, Copy, Debug)]
pub struct ArtifactCanonicalValueLimits {
    pub maximum_depth: usize,
    pub maximum_retained_bytes: usize,
    pub maximum_work_items: usize,
}

/// 🎟️ One structural permit and an exact allowance for hashed or compared key bytes.
#[derive(Clone, Copy, Debug)]
pub struct ArtifactCanonicalValueGrant {
    pub maximum_items: usize,
    pub maximum_bytes: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ArtifactCanonicalValueCheckpoint {
    pub processed_items: usize,
    pub processed_bytes: usize,
    pub retained_bytes: usize,
}

#[must_use = "canonical admission progress must be observed"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactCanonicalValueStep {
    Blocked,
    Progress(ArtifactCanonicalValueCheckpoint),
    Complete(ArtifactCanonicalValueCheckpoint),
}

/// 🧾️ Bounded close work and the exact source transfer that completes cancellation.
#[must_use = "bounded close progress and exact source return must be observed"]
#[derive(Debug, PartialEq, Eq)]
pub enum ArtifactCanonicalValueCloseStep<R> {
    Blocked(ArtifactCanonicalValueCheckpoint),
    Progress { structural_items: usize, checkpoint: ArtifactCanonicalValueCheckpoint },
    Returned { source: R, structural_items: usize, checkpoint: ArtifactCanonicalValueCheckpoint },
    Complete(ArtifactCanonicalValueCheckpoint),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Inspect,
    FinishNode,
    InitializeTable,
    HashKey,
    ProbeKey,
    CompareKey,
    ClearTable,
    Complete,
    Closing,
    Closed,
}

#[derive(Clone, Copy)]
struct KeySlot {
    hash: u64,
    index: usize,
}

struct Frame {
    next: usize,
    length: usize,
}

fn reserve_table_with(length: usize, remaining: usize, reserve: impl FnOnce(&mut Vec<Option<KeySlot>>, usize) -> Result<(), std::collections::TryReserveError>) -> Result<Vec<Option<KeySlot>>, &'static str> {
    let bytes = length.checked_mul(size_of::<Option<KeySlot>>()).ok_or("canonical-value.capacity-limit")?;
    if bytes > remaining {
        return Err("canonical-value.capacity-limit");
    }
    let mut table = Vec::new();
    reserve(&mut table, length).map_err(|_| "canonical-value.allocation-failed")?;
    if table.capacity().saturating_mul(size_of::<Option<KeySlot>>()) > remaining {
        return Err("canonical-value.capacity-limit");
    }
    Ok(table)
}

/// 🧭️ Retains one exact source while validating every key with bounded hash and collision work.
#[must_use = "Complete admission or drive cancellation before returning the exact source"]
pub struct ArtifactCanonicalValueAdmission<R: DslValueSource> {
    source: ManuallyDrop<Option<R>>,
    limits: ArtifactCanonicalValueLimits,
    checkpoint: ArtifactCanonicalValueCheckpoint,
    phase: Phase,
    path: Vec<usize>,
    frames: Vec<Frame>,
    table: Vec<Option<KeySlot>>,
    table_length: usize,
    key_index: usize,
    key_offset: usize,
    probe_slot: usize,
    compare_offset: usize,
    key_hash: u64,
    hasher: DefaultHasher,
}

impl<R: DslValueSource> ArtifactCanonicalValueAdmission<R> {
    pub fn new(source: R, limits: ArtifactCanonicalValueLimits) -> Result<Self, (R, &'static str)> {
        if limits.maximum_depth == 0 || limits.maximum_depth > super::ARTIFACT_CANONICAL_JSON_DEPTH || limits.maximum_work_items == 0 {
            return Err((source, "canonical-value.invalid-limits"));
        }
        Ok(Self {
            source: ManuallyDrop::new(Some(source)),
            limits,
            checkpoint: ArtifactCanonicalValueCheckpoint::default(),
            phase: Phase::Inspect,
            path: Vec::with_capacity(limits.maximum_depth),
            frames: Vec::with_capacity(limits.maximum_depth),
            table: Vec::new(),
            table_length: 0,
            key_index: 0,
            key_offset: 0,
            probe_slot: 0,
            compare_offset: 0,
            key_hash: 0,
            hasher: DefaultHasher::new(),
        })
    }

    pub fn checkpoint(&self) -> ArtifactCanonicalValueCheckpoint {
        self.checkpoint
    }

    pub fn advance(&mut self, grant: ArtifactCanonicalValueGrant) -> Result<ArtifactCanonicalValueStep, &'static str> {
        if grant.maximum_items == 0 || grant.maximum_bytes == 0 {
            return Ok(ArtifactCanonicalValueStep::Blocked);
        }
        if self.phase == Phase::Complete {
            return Ok(ArtifactCanonicalValueStep::Complete(self.checkpoint));
        }
        if matches!(self.phase, Phase::Closing | Phase::Closed) {
            return Err("canonical-value.closing");
        }
        if self.checkpoint.processed_items >= self.limits.maximum_work_items {
            self.phase = Phase::Closing;
            return Err("canonical-value.work-limit");
        }
        if let Err(error) = self.advance_item(grant.maximum_bytes.min(256)) {
            self.phase = Phase::Closing;
            return Err(error);
        }
        self.checkpoint.processed_items += 1;
        Ok(if self.phase == Phase::Complete { ArtifactCanonicalValueStep::Complete(self.checkpoint) } else { ArtifactCanonicalValueStep::Progress(self.checkpoint) })
    }

    fn enter_children(&mut self, length: usize) -> Result<(), &'static str> {
        if length == 0 {
            self.phase = Phase::FinishNode;
            return Ok(());
        }
        if self.path.len() + 1 >= self.limits.maximum_depth {
            return Err("canonical-value.depth-limit");
        }
        self.frames.push(Frame { next: 0, length });
        self.path.push(0);
        self.phase = Phase::Inspect;
        Ok(())
    }

    fn advance_item(&mut self, bytes: usize) -> Result<(), &'static str> {
        if self.phase == Phase::FinishNode {
            if let Some(frame) = self.frames.last_mut() {
                frame.next += 1;
                if frame.next < frame.length {
                    *self.path.last_mut().ok_or("canonical-value.invalid-path")? = frame.next;
                    self.phase = Phase::Inspect;
                } else {
                    self.frames.pop();
                    self.path.pop();
                }
            } else {
                self.phase = Phase::Complete;
            }
            return Ok(());
        }
        let root = self.source.as_ref().ok_or("canonical-value.missing-source")?.value();
        let value = super::indexed_value(root, &self.path).map_err(|_| "canonical-value.invalid-path")?;
        match self.phase {
            Phase::Inspect => match value {
                DslValue::Array(values) => self.enter_children(values.len())?,
                DslValue::Object(values) if !values.is_empty() => {
                    let length = values.len().checked_mul(2).and_then(|length| length.checked_add(1)).ok_or("canonical-value.capacity-limit")?;
                    self.table = reserve_table_with(length, self.limits.maximum_retained_bytes, Vec::try_reserve_exact)?;
                    self.checkpoint.retained_bytes = self.table.capacity().saturating_mul(size_of::<Option<KeySlot>>());
                    self.table_length = length;
                    self.key_index = 0;
                    self.key_offset = 0;
                    self.hasher = DefaultHasher::new();
                    self.phase = Phase::InitializeTable;
                }
                DslValue::Number(protocol::value::Number::Float(value)) if !value.is_finite() => return Err("canonical-value.non-finite-number"),
                _ => self.phase = Phase::FinishNode,
            },
            Phase::InitializeTable => {
                self.table.push(None);
                if self.table.len() == self.table_length {
                    self.phase = Phase::HashKey;
                }
            }
            Phase::HashKey => {
                let DslValue::Object(values) = value else {
                    return Err("canonical-value.invalid-path");
                };
                let key = values.get(self.key_index).ok_or("canonical-value.invalid-path")?.0.as_bytes();
                if self.key_offset < key.len() {
                    let end = key.len().min(self.key_offset.saturating_add(bytes));
                    self.hasher.write(&key[self.key_offset..end]);
                    self.checkpoint.processed_bytes += end - self.key_offset;
                    self.key_offset = end;
                } else {
                    self.key_hash = self.hasher.finish();
                    self.probe_slot = (self.key_hash % self.table.len() as u64) as usize;
                    self.phase = Phase::ProbeKey;
                }
            }
            Phase::ProbeKey => {
                if let Some(slot) = self.table[self.probe_slot] {
                    if slot.hash == self.key_hash {
                        self.compare_offset = 0;
                        self.phase = Phase::CompareKey;
                    } else {
                        self.probe_slot = (self.probe_slot + 1) % self.table.len();
                    }
                } else {
                    let DslValue::Object(values) = value else {
                        return Err("canonical-value.invalid-path");
                    };
                    self.table[self.probe_slot] = Some(KeySlot { hash: self.key_hash, index: self.key_index });
                    self.key_index += 1;
                    self.key_offset = 0;
                    self.hasher = DefaultHasher::new();
                    self.phase = if self.key_index == values.len() { Phase::ClearTable } else { Phase::HashKey };
                }
            }
            Phase::CompareKey => {
                let DslValue::Object(values) = value else {
                    return Err("canonical-value.invalid-path");
                };
                let slot = self.table[self.probe_slot].ok_or("canonical-value.invalid-path")?;
                let left = values[self.key_index].0.as_bytes();
                let right = values[slot.index].0.as_bytes();
                let end = left.len().min(self.compare_offset.saturating_add(bytes));
                let equal = left.len() == right.len() && left[self.compare_offset..end] == right[self.compare_offset..end];
                self.checkpoint.processed_bytes += if left.len() == right.len() { end - self.compare_offset } else { 0 };
                self.compare_offset = end;
                if !equal {
                    self.probe_slot = (self.probe_slot + 1) % self.table.len();
                    self.phase = Phase::ProbeKey;
                } else if end == left.len() {
                    return Err("canonical-value.duplicate-key");
                }
            }
            Phase::ClearTable => {
                if self.table.pop().is_none() {
                    self.table = Vec::new();
                    self.checkpoint.retained_bytes = 0;
                    let DslValue::Object(values) = value else {
                        return Err("canonical-value.invalid-path");
                    };
                    self.enter_children(values.len())?;
                }
            }
            _ => return Err("canonical-value.invalid-phase"),
        }
        Ok(())
    }

    pub fn take_value(&mut self) -> Option<ArtifactCanonicalValue<R>> {
        if self.phase != Phase::Complete {
            return None;
        }
        self.source.take().map(|source| ArtifactCanonicalValue { source })
    }

    pub fn cancel(&mut self) {
        if self.phase != Phase::Closed {
            self.phase = Phase::Closing;
        }
    }

    /// ♻️ Drops one numeric slot or traversal frame, then returns the exact source under a structural permit.
    pub fn close_step(&mut self, grant: ArtifactCanonicalValueGrant) -> ArtifactCanonicalValueCloseStep<R> {
        if self.terminal_is_empty() {
            return ArtifactCanonicalValueCloseStep::Complete(self.checkpoint);
        }
        if grant.maximum_items == 0 {
            return ArtifactCanonicalValueCloseStep::Blocked(self.checkpoint);
        }
        self.cancel();
        let progress = |checkpoint| ArtifactCanonicalValueCloseStep::Progress { structural_items: 1, checkpoint };
        if !self.table.is_empty() {
            self.table.pop();
            return progress(self.checkpoint);
        }
        if self.table.capacity() > 0 {
            self.table = Vec::new();
            self.checkpoint.retained_bytes = 0;
            return progress(self.checkpoint);
        }
        if self.frames.pop().is_some() {
            self.path.pop();
            return progress(self.checkpoint);
        }
        let source = self.source.take().expect("canonical admission source must be returned exactly once");
        self.phase = Phase::Closed;
        ArtifactCanonicalValueCloseStep::Returned { source, structural_items: 1, checkpoint: self.checkpoint }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.source.is_none() && self.frames.is_empty() && self.path.is_empty() && self.table.is_empty() && self.table.capacity() == 0 && self.checkpoint.retained_bytes == 0
    }
}

impl<R: DslValueSource> Drop for ArtifactCanonicalValueAdmission<R> {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.terminal_is_empty(), "canonical value admission dropped before returning its source");
        }
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️allocation/🦀️.rs"]
mod allocation_tests;
