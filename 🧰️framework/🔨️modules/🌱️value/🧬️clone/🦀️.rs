//! 🧬️ Source-owning bounded materialization of framework dynamic values.

use std::mem::{size_of, ManuallyDrop};

use super::{DslValue, DslValueSource};

pub const DSL_VALUE_CLONE_MAXIMUM_DEPTH: usize = 64;
pub const DSL_VALUE_CLONE_CHUNK_BYTES: usize = 256;

/// 📏️ Requested and admitted payload capacity excludes the fixed, at-most-64-frame cursor and source.
#[derive(Clone, Copy, Debug)]
pub struct DslValueCloneLimits {
    pub maximum_depth: usize,
    pub maximum_retained_bytes: usize,
}

/// 🎟️ Per-call permits for structural work and copied UTF-8 bytes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DslValueCloneGrant {
    pub maximum_items: usize,
    pub maximum_bytes: usize,
}

/// 🧾️ Work consumed by one advance or close call.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DslValueCloneReceipt {
    pub structural_items: usize,
    pub copied_bytes: usize,
}

/// 📍️ Monotonic clone work and currently retained payload allocation capacity.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DslValueCloneCheckpoint {
    pub completed_items: usize,
    pub copied_bytes: usize,
    pub retained_bytes: usize,
}

#[must_use = "clone progress and grant consumption must be observed"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DslValueCloneStep {
    Blocked(DslValueCloneCheckpoint),
    Progress { receipt: DslValueCloneReceipt, checkpoint: DslValueCloneCheckpoint },
    Complete { receipt: DslValueCloneReceipt, checkpoint: DslValueCloneCheckpoint },
}

#[must_use = "bounded close progress and exact source return must be observed"]
#[derive(Debug, PartialEq, Eq)]
pub enum DslValueCloneCloseStep<R> {
    Blocked(DslValueCloneCheckpoint),
    Progress { receipt: DslValueCloneReceipt, checkpoint: DslValueCloneCheckpoint },
    Returned { source: R, receipt: DslValueCloneReceipt, checkpoint: DslValueCloneCheckpoint },
    Complete(DslValueCloneCheckpoint),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Active,
    Complete,
    Closing,
    Closed,
}

#[derive(Default)]
struct TextCopy {
    bytes: [u8; 4],
    length: usize,
}

struct Frame {
    value: Option<DslValue>,
    key: Option<String>,
    text: TextCopy,
}

struct DslValueCloneState<R: DslValueSource> {
    source: Option<R>,
    frames: Vec<Frame>,
    completed: Option<DslValue>,
    output: Option<DslValue>,
    limits: DslValueCloneLimits,
    checkpoint: DslValueCloneCheckpoint,
    phase: Phase,
}

/// 🧬️ Incrementally clones one frozen value and returns its source only through bounded close.
#[must_use = "value clone cursors must transfer output and explicitly close or cancel and explicitly close"]
pub struct DslValueCloneCursor<R: DslValueSource> {
    state: ManuallyDrop<DslValueCloneState<R>>,
}

enum AdvanceWork {
    Blocked,
    Progress(DslValueCloneReceipt),
}

fn reserve_vec<T>(length: usize, remaining: usize) -> Result<Vec<T>, &'static str> {
    reserve_vec_with(length, remaining, Vec::try_reserve_exact)
}

fn reserve_vec_with<T>(length: usize, remaining: usize, reserve: impl FnOnce(&mut Vec<T>, usize) -> Result<(), std::collections::TryReserveError>) -> Result<Vec<T>, &'static str> {
    let bytes = length.checked_mul(size_of::<T>()).ok_or("retained-byte-limit")?;
    if bytes > remaining {
        return Err("retained-byte-limit");
    }
    let mut result = Vec::new();
    reserve(&mut result, length).map_err(|_| "allocation-failed")?;
    if result.capacity().saturating_mul(size_of::<T>()) > remaining {
        return Err("retained-byte-limit");
    }
    Ok(result)
}

fn reserve_string(length: usize, remaining: usize) -> Result<String, &'static str> {
    reserve_vec(length, remaining).map(|value| String::from_utf8(value).expect("empty reserved string has valid UTF-8"))
}

fn own_capacity(value: &DslValue) -> usize {
    match value {
        DslValue::String(value) => value.capacity(),
        DslValue::Array(value) => value.capacity() * size_of::<DslValue>(),
        DslValue::Object(value) => value.capacity() * size_of::<(String, DslValue)>(),
        _ => 0,
    }
}

fn copy_chunk(source: &str, target: &mut String, pending: &mut TextCopy, maximum_bytes: usize) -> Result<usize, &'static str> {
    let maximum = maximum_bytes.min(DSL_VALUE_CLONE_CHUNK_BYTES);
    let mut copied = 0;
    while copied < maximum && target.len() + pending.length < source.len() {
        let character = source[target.len()..].chars().next().ok_or("invalid-source-path")?;
        let width = character.len_utf8();
        let available = (maximum - copied).min(width - pending.length);
        let start = target.len() + pending.length;
        let end = start + available;
        pending.bytes[pending.length..pending.length + available].copy_from_slice(&source.as_bytes()[start..end]);
        pending.length += available;
        copied += available;
        if pending.length == width {
            target.push_str(std::str::from_utf8(&pending.bytes[..width]).map_err(|_| "invalid-source-path")?);
            pending.length = 0;
        }
    }
    Ok(copied)
}

fn source_at<'a>(root: &'a DslValue, frames: &[Frame]) -> Result<&'a DslValue, &'static str> {
    let mut source = root;
    for frame in frames {
        source = match (&frame.value, source) {
            (Some(DslValue::Array(target)), DslValue::Array(values)) => values.get(target.len()),
            (Some(DslValue::Object(target)), DslValue::Object(values)) => values.get(target.len()).map(|(_, value)| value),
            _ => None,
        }
        .ok_or("invalid-source-path")?;
    }
    Ok(source)
}

impl<R: DslValueSource> DslValueCloneCursor<R> {
    pub fn new(source: R, limits: DslValueCloneLimits) -> Result<Self, (R, &'static str)> {
        if limits.maximum_depth == 0 || limits.maximum_depth > DSL_VALUE_CLONE_MAXIMUM_DEPTH {
            return Err((source, "invalid-depth-limit"));
        }
        let mut frames = Vec::with_capacity(limits.maximum_depth);
        frames.push(Frame { value: None, key: None, text: TextCopy::default() });
        Ok(Self {
            state: ManuallyDrop::new(DslValueCloneState {
                source: Some(source),
                frames,
                completed: None,
                output: None,
                limits,
                checkpoint: DslValueCloneCheckpoint::default(),
                phase: Phase::Active,
            }),
        })
    }

    pub fn checkpoint(&self) -> DslValueCloneCheckpoint {
        self.state.checkpoint
    }

    pub fn advance(&mut self, grant: DslValueCloneGrant) -> Result<DslValueCloneStep, &'static str> {
        if self.state.phase == Phase::Complete {
            return Ok(DslValueCloneStep::Complete { receipt: DslValueCloneReceipt::default(), checkpoint: self.state.checkpoint });
        }
        if self.state.phase != Phase::Active {
            return Err("closing");
        }
        let receipt = match self.advance_item(grant) {
            Ok(AdvanceWork::Blocked) => return Ok(DslValueCloneStep::Blocked(self.state.checkpoint)),
            Ok(AdvanceWork::Progress(receipt)) => receipt,
            Err(error) => {
                self.state.phase = Phase::Closing;
                return Err(error);
            }
        };
        self.state.checkpoint.completed_items += receipt.structural_items;
        self.state.checkpoint.copied_bytes += receipt.copied_bytes;
        Ok(if self.state.phase == Phase::Complete {
            DslValueCloneStep::Complete { receipt, checkpoint: self.state.checkpoint }
        } else {
            DslValueCloneStep::Progress { receipt, checkpoint: self.state.checkpoint }
        })
    }

    fn advance_item(&mut self, grant: DslValueCloneGrant) -> Result<AdvanceWork, &'static str> {
        let state = &mut *self.state;
        if state.completed.is_some() {
            if grant.maximum_items == 0 {
                return Ok(AdvanceWork::Blocked);
            }
            if let Some(frame) = state.frames.last_mut() {
                match frame.value.as_mut() {
                    Some(DslValue::Array(values)) => values.push(state.completed.take().ok_or("invalid-source-path")?),
                    Some(DslValue::Object(values)) => {
                        let key = frame.key.take().ok_or("invalid-source-path")?;
                        values.push((key, state.completed.take().ok_or("invalid-source-path")?));
                    }
                    _ => return Err("invalid-source-path"),
                }
            } else {
                state.output = state.completed.take();
                state.phase = Phase::Complete;
            }
            return Ok(AdvanceWork::Progress(DslValueCloneReceipt { structural_items: 1, copied_bytes: 0 }));
        }
        let depth = state.frames.len();
        let source = source_at(state.source.as_ref().ok_or("invalid-source-path")?.value(), &state.frames[..depth - 1])?;
        let remaining = state.limits.maximum_retained_bytes.checked_sub(state.checkpoint.retained_bytes).ok_or("retained-byte-limit")?;
        let frame = state.frames.last_mut().ok_or("invalid-source-path")?;
        if frame.value.is_none() {
            if grant.maximum_items == 0 {
                return Ok(AdvanceWork::Blocked);
            }
            let value = match source {
                DslValue::Null => DslValue::Null,
                DslValue::Bool(value) => DslValue::Bool(*value),
                DslValue::Number(value) => DslValue::Number(*value),
                DslValue::String(value) => DslValue::String(reserve_string(value.len(), remaining)?),
                DslValue::Array(value) => DslValue::Array(reserve_vec(value.len(), remaining)?),
                DslValue::Object(value) => DslValue::Object(reserve_vec(value.len(), remaining)?),
            };
            state.checkpoint.retained_bytes += own_capacity(&value);
            if matches!(value, DslValue::Null | DslValue::Bool(_) | DslValue::Number(_)) {
                state.frames.pop();
                state.completed = Some(value);
            } else {
                frame.value = Some(value);
            }
            return Ok(AdvanceWork::Progress(DslValueCloneReceipt { structural_items: 1, copied_bytes: 0 }));
        }
        match (frame.value.as_mut().ok_or("invalid-source-path")?, source) {
            (DslValue::String(target), DslValue::String(source)) if target.len() + frame.text.length < source.len() => {
                if grant.maximum_bytes == 0 {
                    return Ok(AdvanceWork::Blocked);
                }
                let copied_bytes = copy_chunk(source, target, &mut frame.text, grant.maximum_bytes)?;
                return Ok(AdvanceWork::Progress(DslValueCloneReceipt { structural_items: 0, copied_bytes }));
            }
            (DslValue::Array(target), DslValue::Array(source)) if target.len() < source.len() => {}
            (DslValue::Object(target), DslValue::Object(source)) if target.len() < source.len() => {
                let source_key = &source[target.len()].0;
                if frame.key.is_none() {
                    if grant.maximum_items == 0 {
                        return Ok(AdvanceWork::Blocked);
                    }
                    let key = reserve_string(source_key.len(), remaining)?;
                    state.checkpoint.retained_bytes += key.capacity();
                    frame.key = Some(key);
                    return Ok(AdvanceWork::Progress(DslValueCloneReceipt { structural_items: 1, copied_bytes: 0 }));
                }
                let key = frame.key.as_mut().ok_or("invalid-source-path")?;
                if key.len() + frame.text.length < source_key.len() {
                    if grant.maximum_bytes == 0 {
                        return Ok(AdvanceWork::Blocked);
                    }
                    let copied_bytes = copy_chunk(source_key, key, &mut frame.text, grant.maximum_bytes)?;
                    return Ok(AdvanceWork::Progress(DslValueCloneReceipt { structural_items: 0, copied_bytes }));
                }
            }
            (DslValue::String(_), DslValue::String(_)) | (DslValue::Array(_), DslValue::Array(_)) | (DslValue::Object(_), DslValue::Object(_)) => {
                if grant.maximum_items == 0 {
                    return Ok(AdvanceWork::Blocked);
                }
                state.completed = state.frames.pop().and_then(|frame| frame.value);
                return Ok(AdvanceWork::Progress(DslValueCloneReceipt { structural_items: 1, copied_bytes: 0 }));
            }
            _ => return Err("invalid-source-path"),
        }
        if grant.maximum_items == 0 {
            return Ok(AdvanceWork::Blocked);
        }
        if depth == state.limits.maximum_depth {
            return Err("depth-limit");
        }
        state.frames.push(Frame { value: None, key: None, text: TextCopy::default() });
        Ok(AdvanceWork::Progress(DslValueCloneReceipt { structural_items: 1, copied_bytes: 0 }))
    }

    pub fn take_value(&mut self) -> Option<DslValue> {
        if self.state.phase != Phase::Complete {
            return None;
        }
        let value = self.state.output.take();
        if value.is_some() {
            self.state.checkpoint.retained_bytes = 0;
            self.state.phase = Phase::Closing;
        }
        value
    }

    pub fn cancel(&mut self) {
        if self.state.phase != Phase::Closed {
            self.state.phase = Phase::Closing;
        }
    }

    /// ♻️ Retires or transfers one structural owner; payload capacity is separate from copied-byte work.
    pub fn close_step(&mut self, grant: DslValueCloneGrant) -> DslValueCloneCloseStep<R> {
        if self.state.phase == Phase::Closed {
            return DslValueCloneCloseStep::Complete(self.state.checkpoint);
        }
        if grant.maximum_items == 0 {
            return DslValueCloneCloseStep::Blocked(self.state.checkpoint);
        }
        self.cancel();
        let receipt = DslValueCloneReceipt { structural_items: 1, copied_bytes: 0 };
        if let Some(value) = self.state.output.take().or_else(|| self.state.completed.take()) {
            self.state.frames.push(Frame { value: Some(value), key: None, text: TextCopy::default() });
            return DslValueCloneCloseStep::Progress { receipt, checkpoint: self.state.checkpoint };
        }
        if let Some(frame) = self.state.frames.last_mut() {
            if let Some(key) = frame.key.take() {
                self.state.checkpoint.retained_bytes -= key.capacity();
                return DslValueCloneCloseStep::Progress { receipt, checkpoint: self.state.checkpoint };
            }
            let child = match frame.value.as_mut() {
                Some(DslValue::Array(values)) => values.pop(),
                Some(DslValue::Object(values)) => values.pop().map(|(key, value)| {
                    frame.key = Some(key);
                    value
                }),
                _ => None,
            };
            if let Some(value) = child {
                self.state.frames.push(Frame { value: Some(value), key: None, text: TextCopy::default() });
            } else if let Some(frame) = self.state.frames.pop() {
                self.state.checkpoint.retained_bytes -= frame.value.as_ref().map_or(0, own_capacity);
            }
            return DslValueCloneCloseStep::Progress { receipt, checkpoint: self.state.checkpoint };
        }
        let source = self.state.source.take().expect("clone close source must be returned exactly once");
        self.state.phase = Phase::Closed;
        DslValueCloneCloseStep::Returned { source, receipt, checkpoint: self.state.checkpoint }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.phase == Phase::Closed
            && self.state.source.is_none()
            && self.state.frames.is_empty()
            && self.state.completed.is_none()
            && self.state.output.is_none()
            && self.state.checkpoint.retained_bytes == 0
    }
}

impl<R: DslValueSource> Drop for DslValueCloneCursor<R> {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            debug_assert!(std::thread::panicking(), "DslValueCloneCursor must reach terminal-empty before drop");
            return;
        }
        unsafe { ManuallyDrop::drop(&mut self.state) };
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
