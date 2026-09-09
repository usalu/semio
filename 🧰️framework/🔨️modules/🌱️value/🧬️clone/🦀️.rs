//! 🧬️ Source-owning bounded materialization of framework dynamic values.

use super::DslValue;

pub const DSL_VALUE_CLONE_MAXIMUM_DEPTH: usize = 64;
pub const DSL_VALUE_CLONE_CHUNK_BYTES: usize = 256;

/// 🪪️ An immutable value selected from one retained owner, with constant-time root access.
pub trait DslValueCloneSource {
    fn value(&self) -> &DslValue;
}

impl DslValueCloneSource for std::sync::Arc<DslValue> {
    fn value(&self) -> &DslValue { self.as_ref() }
}

/// 📏️ Payload capacity excludes the fixed, at-most-64-frame cursor and the borrowed source.
#[derive(Clone, Copy, Debug)]
pub struct DslValueCloneLimits {
    pub maximum_depth: usize,
    pub maximum_retained_bytes: usize,
}

/// 📍️ Monotonic work counters and currently owned payload allocation capacity.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DslValueCloneCheckpoint {
    pub completed_items: usize,
    pub copied_bytes: usize,
    pub retained_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DslValueCloneStep {
    Progress(DslValueCloneCheckpoint),
    Complete(DslValueCloneCheckpoint),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Active, Complete, Closing, Closed }

struct Frame {
    value: Option<DslValue>,
    key: Option<String>,
}

/// 🧬️ Incrementally clones one frozen value and retires partial output before returning its source.
/// [`DslValueCloneCursor::cancel`] and [`DslValueCloneCursor::close_step`] must drain abandoned work.
pub struct DslValueCloneCursor<R: DslValueCloneSource> {
    source: Option<R>,
    frames: Vec<Frame>,
    completed: Option<DslValue>,
    output: Option<DslValue>,
    limits: DslValueCloneLimits,
    checkpoint: DslValueCloneCheckpoint,
    phase: Phase,
}

fn reserve_vec<T>(length: usize, remaining: usize) -> Result<Vec<T>, &'static str> {
    let bytes = length.checked_mul(size_of::<T>()).ok_or("retained-byte-limit")?;
    if bytes > remaining { return Err("retained-byte-limit"); }
    let mut result = Vec::new();
    result.try_reserve_exact(length).map_err(|_| "allocation-failed")?;
    if result.capacity().saturating_mul(size_of::<T>()) > remaining { return Err("retained-byte-limit"); }
    Ok(result)
}

fn reserve_string(length: usize, remaining: usize) -> Result<String, &'static str> {
    if length > remaining { return Err("retained-byte-limit"); }
    let mut result = String::new();
    result.try_reserve_exact(length).map_err(|_| "allocation-failed")?;
    if result.capacity() > remaining { return Err("retained-byte-limit"); }
    Ok(result)
}

fn own_capacity(value: &DslValue) -> usize {
    match value {
        DslValue::String(value) => value.capacity(),
        DslValue::Array(value) => value.capacity() * size_of::<DslValue>(),
        DslValue::Object(value) => value.capacity() * size_of::<(String, DslValue)>(),
        _ => 0,
    }
}

fn copy_chunk(source: &str, target: &mut String) -> usize {
    let start = target.len();
    let mut end = source.len().min(start.saturating_add(DSL_VALUE_CLONE_CHUNK_BYTES));
    while !source.is_char_boundary(end) { end -= 1; }
    target.push_str(&source[start..end]);
    end - start
}

fn source_at<'a>(root: &'a DslValue, frames: &[Frame]) -> Result<&'a DslValue, &'static str> {
    let mut source = root;
    for frame in frames {
        source = match (&frame.value, source) {
            (Some(DslValue::Array(target)), DslValue::Array(values)) => values.get(target.len()),
            (Some(DslValue::Object(target)), DslValue::Object(values)) => values.get(target.len()).map(|(_, value)| value),
            _ => None,
        }.ok_or("invalid-source-path")?;
    }
    Ok(source)
}

impl<R: DslValueCloneSource> DslValueCloneCursor<R> {
    pub fn new(source: R, limits: DslValueCloneLimits) -> Result<Self, (R, &'static str)> {
        if limits.maximum_depth == 0 || limits.maximum_depth > DSL_VALUE_CLONE_MAXIMUM_DEPTH { return Err((source, "invalid-depth-limit")); }
        let mut frames = Vec::with_capacity(limits.maximum_depth);
        frames.push(Frame { value: None, key: None });
        Ok(Self { source: Some(source), frames, completed: None, output: None, limits, checkpoint: DslValueCloneCheckpoint::default(), phase: Phase::Active })
    }

    pub fn checkpoint(&self) -> DslValueCloneCheckpoint { self.checkpoint }

    pub fn advance(&mut self) -> Result<DslValueCloneStep, &'static str> {
        if self.phase == Phase::Complete { return Ok(DslValueCloneStep::Complete(self.checkpoint)); }
        if self.phase != Phase::Active { return Err("closing"); }
        if let Err(error) = self.advance_item() { self.phase = Phase::Closing; return Err(error); }
        self.checkpoint.completed_items += 1;
        Ok(if self.phase == Phase::Complete { DslValueCloneStep::Complete(self.checkpoint) } else { DslValueCloneStep::Progress(self.checkpoint) })
    }

    fn advance_item(&mut self) -> Result<(), &'static str> {
        if self.completed.is_some() {
            if let Some(frame) = self.frames.last_mut() {
                match frame.value.as_mut() {
                    Some(DslValue::Array(values)) => values.push(self.completed.take().ok_or("invalid-source-path")?),
                    Some(DslValue::Object(values)) => {
                        let key = frame.key.take().ok_or("invalid-source-path")?;
                        values.push((key, self.completed.take().ok_or("invalid-source-path")?));
                    }
                    _ => return Err("invalid-source-path"),
                }
            } else {
                self.output = self.completed.take();
                self.phase = Phase::Complete;
            }
            return Ok(());
        }
        let depth = self.frames.len();
        let source = source_at(self.source.as_ref().ok_or("invalid-source-path")?.value(), &self.frames[..depth - 1])?;
        let remaining = self.limits.maximum_retained_bytes - self.checkpoint.retained_bytes;
        let frame = self.frames.last_mut().ok_or("invalid-source-path")?;
        if frame.value.is_none() {
            let value = match source {
                DslValue::Null => DslValue::Null,
                DslValue::Bool(value) => DslValue::Bool(*value),
                DslValue::Number(value) => DslValue::Number(*value),
                DslValue::String(value) => DslValue::String(reserve_string(value.len(), remaining)?),
                DslValue::Array(value) => DslValue::Array(reserve_vec(value.len(), remaining)?),
                DslValue::Object(value) => DslValue::Object(reserve_vec(value.len(), remaining)?),
            };
            self.checkpoint.retained_bytes += own_capacity(&value);
            if matches!(value, DslValue::Null | DslValue::Bool(_) | DslValue::Number(_)) {
                self.frames.pop();
                self.completed = Some(value);
            } else { frame.value = Some(value); }
            return Ok(());
        }
        match (frame.value.as_mut().ok_or("invalid-source-path")?, source) {
            (DslValue::String(target), DslValue::String(source)) if target.len() < source.len() => {
                self.checkpoint.copied_bytes += copy_chunk(source, target);
                return Ok(());
            }
            (DslValue::Array(target), DslValue::Array(source)) if target.len() < source.len() => {}
            (DslValue::Object(target), DslValue::Object(source)) if target.len() < source.len() => {
                let source_key = &source[target.len()].0;
                if frame.key.is_none() {
                    let key = reserve_string(source_key.len(), remaining)?;
                    self.checkpoint.retained_bytes += key.capacity();
                    frame.key = Some(key);
                    return Ok(());
                }
                let key = frame.key.as_mut().ok_or("invalid-source-path")?;
                if key.len() < source_key.len() {
                    self.checkpoint.copied_bytes += copy_chunk(source_key, key);
                    return Ok(());
                }
            }
            (DslValue::String(_), DslValue::String(_)) | (DslValue::Array(_), DslValue::Array(_)) | (DslValue::Object(_), DslValue::Object(_)) => {
                self.completed = self.frames.pop().and_then(|frame| frame.value);
                return Ok(());
            }
            _ => return Err("invalid-source-path"),
        }
        if depth == self.limits.maximum_depth { return Err("depth-limit"); }
        self.frames.push(Frame { value: None, key: None });
        Ok(())
    }

    pub fn take_value(&mut self) -> Option<DslValue> {
        if self.phase != Phase::Complete { return None; }
        let value = self.output.take();
        if value.is_some() { self.checkpoint.retained_bytes = 0; }
        value
    }

    pub fn take_source(&mut self) -> Option<R> {
        if (self.phase == Phase::Complete && self.output.is_none()) || self.phase == Phase::Closed { self.source.take() } else { None }
    }

    pub fn cancel(&mut self) { if self.phase != Phase::Closed { self.phase = Phase::Closing; } }

    /// ♻️ Removes one value edge, string allocation, or empty frame; never recursively drops a tree.
    pub fn close_step(&mut self) -> bool {
        self.cancel();
        if let Some(value) = self.output.take().or_else(|| self.completed.take()) {
            self.frames.push(Frame { value: Some(value), key: None });
            return false;
        }
        let Some(frame) = self.frames.last_mut() else { self.phase = Phase::Closed; return true; };
        if let Some(key) = frame.key.take() {
            self.checkpoint.retained_bytes -= key.capacity();
            return false;
        }
        let child = match frame.value.as_mut() {
            Some(DslValue::Array(values)) => values.pop(),
            Some(DslValue::Object(values)) => values.pop().map(|(key, value)| { self.checkpoint.retained_bytes -= key.capacity(); value }),
            _ => None,
        };
        if let Some(value) = child {
            self.frames.push(Frame { value: Some(value), key: None });
        } else if let Some(frame) = self.frames.pop() {
            self.checkpoint.retained_bytes -= frame.value.as_ref().map_or(0, own_capacity);
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.source.is_none() && self.frames.is_empty() && self.completed.is_none() && self.output.is_none() && self.checkpoint.retained_bytes == 0
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
