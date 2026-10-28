//! 🎟️ Retained event admission reads original nodes and native JSON bytes without encoded-body allocation.
use super::*;
use semio_framework_plugin::plugin_app_close_prelude::store;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}

#[derive(Clone, Copy)]
enum Phase {
    Inspect,
    Key,
    Child,
    Finish,
}
struct ValueAdmission {
    path: [usize; 128],
    next: [usize; 129],
    objects: [bool; 129],
    depth: usize,
    nodes: usize,
    other: usize,
    offset: usize,
    phase: Phase,
    done: bool,
}
impl Default for ValueAdmission {
    fn default() -> Self {
        Self { path: [0; 128], next: [0; 129], objects: [false; 129], depth: 0, nodes: 0, other: 0, offset: 0, phase: Phase::Inspect, done: false }
    }
}
impl ValueAdmission {
    fn node<'a>(&self, root: &'a DslValue) -> Result<&'a DslValue, ValueError> {
        let mut value = root;
        for index in &self.path[..self.depth] {
            value = match value {
                DslValue::Array(values) => values.get(*index),
                DslValue::Object(values) => values.get(*index).map(|(_, value)| value),
                _ => None,
            }
            .ok_or_else(|| invalid("IFC event admission path missing"))?;
        }
        Ok(value)
    }
    fn advance(&mut self, root: Option<&DslValue>) -> Result<(), ValueError> {
        let Some(root) = root else {
            self.done = true;
            return Ok(());
        };
        match self.phase {
            Phase::Inspect => {
                if self.nodes == editing::SNAPSHOT_EDIT_MAXIMUM_VALUE_NODES {
                    return Err(invalid("IFC event exceeds its declared value node extent"));
                }
                self.nodes += 1;
                let value = self.node(root)?;
                self.objects[self.depth] = matches!(value, DslValue::Object(_));
                self.next[self.depth] = 0;
                match value {
                    DslValue::Number(semio_framework_value::Number::Float(value)) if !value.is_finite() => return Err(invalid("IFC event has a non-finite value")),
                    DslValue::Array(_) => self.phase = Phase::Child,
                    DslValue::Object(_) => {
                        self.other = 0;
                        self.offset = 0;
                        self.phase = Phase::Key;
                    }
                    _ => self.phase = Phase::Finish,
                }
            }
            Phase::Key => {
                let DslValue::Object(entries) = self.node(root)? else {
                    return Err(invalid("IFC event object frontier missing"));
                };
                let next = self.next[self.depth];
                if next == entries.len() {
                    self.phase = Phase::Finish;
                } else if self.other == next {
                    self.phase = Phase::Child;
                } else {
                    let left = entries[next].0.as_bytes();
                    let right = entries[self.other].0.as_bytes();
                    if left.len() != right.len() {
                        self.other += 1;
                        self.offset = 0;
                    } else {
                        let end = (self.offset + 64).min(left.len());
                        if left[self.offset..end] != right[self.offset..end] {
                            self.other += 1;
                            self.offset = 0;
                        } else if end == left.len() {
                            return Err(invalid("IFC event contains a duplicate object key"));
                        } else {
                            self.offset = end;
                        }
                    }
                }
            }
            Phase::Child => {
                let length = match self.node(root)? {
                    DslValue::Array(values) => values.len(),
                    DslValue::Object(values) => values.len(),
                    _ => return Err(invalid("IFC event collection frontier missing")),
                };
                if self.next[self.depth] == length {
                    self.phase = Phase::Finish;
                } else {
                    if self.depth == 128 {
                        return Err(invalid("IFC event exceeds its declared value depth"));
                    }
                    self.path[self.depth] = self.next[self.depth];
                    self.depth += 1;
                    self.phase = Phase::Inspect;
                }
            }
            Phase::Finish => {
                if self.depth == 0 {
                    self.done = true;
                } else {
                    self.depth -= 1;
                    self.next[self.depth] += 1;
                    self.other = 0;
                    self.offset = 0;
                    self.phase = if self.objects[self.depth] { Phase::Key } else { Phase::Child };
                }
            }
        }
        Ok(())
    }
}
pub(super) struct EventAdmission {
    values: ValueAdmission,
    codec: store::ArtifactPreparedOperationCursor,
    chunk: [u8; 64],
    pub bytes: usize,
    done: bool,
    closing: bool,
}
impl Default for EventAdmission {
    fn default() -> Self {
        Self { values: Default::default(), codec: Default::default(), chunk: [0; 64], bytes: 0, done: false, closing: false }
    }
}
impl EventAdmission {
    pub(super) fn complete(&self)->bool{self.done}
    pub fn demands(&self) -> Result<RetirementDemand, ValueError> {
        if !self.values.done {
            return Ok(RetirementDemand { depth: 1, ..Default::default() });
        }
        if self.codec.complete && !self.codec.closed {
            return self.codec.retirement_demands();
        }
        if self.codec.closed {
            return Ok(RetirementDemand { depth: 1, ..Default::default() });
        }
        self.codec.canonical_json_work_demands()
    }
    pub fn advance(&mut self, event: &editing::SnapshotEditEvent, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC original event admission cancelled").into());
        }
        if self.closing {
            return Err(invalid("IFC event admission is closing").into());
        }
        if self.done {
            return Ok(true);
        }
        let demand = self.demands()?;
        let grant = cx.retained_grant();
        if cx.should_yield()
            || grant.maximum_items == 0
            || demand.copy_bytes > grant.maximum_copy_bytes
            || demand.capacity_bytes > grant.maximum_capacity_bytes
            || demand.release_bytes > grant.maximum_release_bytes
            || demand.depth > grant.maximum_depth
        {
            return Ok(false);
        }
        if !self.values.done {
            let value = match event {
                editing::SnapshotEditEvent::SetValue { value, .. } | editing::SnapshotEditEvent::InsertValue { value, .. } => Some(value),
                _ => None,
            };
            self.values.advance(value)?;
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(false);
        }
        let step = if self.codec.complete { self.codec.close(grant) } else { self.codec.advance(store::ArtifactPreparedOperationSource::CanonicalJson { header: b"", body: store::ArtifactCanonicalTreeSource::view(event) }, &mut self.chunk, grant) };
        let step = match step {
            Ok(step) => step,
            Err(error) => {
                cx.consume_retained(error.reason.retained_progress())?;
                return Err(error.reason.into());
            }
        };
        self.bytes = self.bytes.checked_add(step.written_bytes).ok_or_else(|| invalid("IFC event wire size overflow"))?;
        let items = step.processed_items;
        cx.consume_retained(RetainedCloneProgress { copied_items: items, copied_bytes: step.copied_bytes, retained_capacity_bytes: step.retained_capacity_bytes, released_bytes: step.released_bytes })?;
        cx.consume_fuel(items.max(1) as u64);
        if self.bytes > editing::SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES {
            return Err(invalid("IFC event exceeds its declared raw byte extent").into());
        }
        if self.codec.closed {
            self.done = true;
        }
        Ok(self.done)
    }
    pub fn begin_close(&mut self) {
        self.closing = true;
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.codec.closed
    }
    pub fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        self.codec.retirement_demands()
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let step = self.codec.close(grant).map_err(|error| error.reason)?;
        let progress = RetainedCloneProgress { copied_items: step.processed_items, copied_bytes: step.copied_bytes, retained_capacity_bytes: step.retained_capacity_bytes, released_bytes: step.released_bytes };
        Ok(if step.complete { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }
}
#[cfg(test)]
#[path = "admission_tests.rs"]
mod tests;
