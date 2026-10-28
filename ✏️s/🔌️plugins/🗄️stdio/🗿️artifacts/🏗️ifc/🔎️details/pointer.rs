//! 🧭️ Borrowed RFC6901 ranges and native ordinals advance under the original request's grants.
use super::*;
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonTree as Tree};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;

fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Range {
    start: usize,
    end: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Mode {
    Existing,
    Insert,
    Member,
    Syntax,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
enum Phase {
    #[default]
    Bind,
    Scan,
    Node,
    Index,
    Key,
    Done,
}
#[derive(Debug)]
pub(super) struct Pointer {
    binding: Option<(usize, usize, usize)>,
    ranges: [Range; 128],
    ordinals: [usize; 128],
    count: usize,
    depth: usize,
    offset: usize,
    start: usize,
    escaped: bool,
    candidate: usize,
    key_chunk: usize,
    key_offset: usize,
    number: usize,
    digits: usize,
    first_zero: bool,
    phase: Phase,
    accepted: bool,
}
impl Default for Pointer {
    fn default() -> Self {
        Self {
            binding: None,
            ranges: [Range::default(); 128],
            ordinals: [0; 128],
            count: 0,
            depth: 0,
            offset: 0,
            start: 0,
            escaped: false,
            candidate: 0,
            key_chunk: 0,
            key_offset: 0,
            number: 0,
            digits: 0,
            first_zero: false,
            phase: Phase::Bind,
            accepted: false,
        }
    }
}
fn decoded(bytes: &[u8], range: Range, offset: &mut usize) -> Option<u8> {
    if *offset == range.end {
        return None;
    }
    let byte = bytes[*offset];
    *offset += 1;
    if byte != b'~' {
        return Some(byte);
    }
    let escaped = bytes[*offset];
    *offset += 1;
    Some(if escaped == b'0' { b'~' } else { b'/' })
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(super) struct NativeAddress{pub position:usize,pub part:Option<usize>,pub argument:usize,pub prefix:usize,pub field:usize}
fn event_pointer(event:&editing::SnapshotEditEvent)->Option<&str>{match event{editing::SnapshotEditEvent::SetValue{path,..}|editing::SnapshotEditEvent::InsertValue{path,..}|editing::SnapshotEditEvent::RemoveValue{path}|editing::SnapshotEditEvent::MoveValue{path,..}|editing::SnapshotEditEvent::RenameKey{path,..}=>Some(path),_=>None}}
impl Pointer {
    fn ordinal(&self,path:&str,index:usize)->Option<usize>{
        let range=self.ranges.get(index).filter(|_|index<self.count)?;let text=path.get(range.start..range.end)?;
        if text.is_empty()||text.len()>20||text.len()>1&&text.starts_with('0')||!text.bytes().all(|byte|byte.is_ascii_digit()){return None;}text.parse().ok()
    }
    pub(super) fn demands(&self) -> RetirementDemand {
        let copy_bytes = match self.phase {
            Phase::Bind => size_of::<Option<(usize, usize, usize)>>() + 2 * size_of::<usize>() + size_of::<Phase>(),
            Phase::Scan => size_of::<Range>() + 3 * size_of::<usize>() + size_of::<bool>() + size_of::<Phase>(),
            Phase::Node => 6 * size_of::<usize>() + 2 * size_of::<bool>() + size_of::<Phase>(),
            Phase::Index | Phase::Key => 6 * size_of::<usize>() + 2 * size_of::<bool>() + size_of::<Phase>(),
            Phase::Done => 0,
        };
        RetirementDemand { copy_bytes, depth: 1, ..Default::default() }
    }
    fn finish(&mut self, accepted: bool) {
        self.accepted = accepted;
        self.phase = Phase::Done;
    }
    fn node<'a>(&self, root: &'a dyn Tree) -> Result<&'a dyn Tree, ValueError> {
        let mut node = root;
        for ordinal in &self.ordinals[..self.depth] {
            node = node.canonical_tree_child(*ordinal)?;
        }
        Ok(node)
    }
    fn equals(&self, path: &str, index: usize, text: &str) -> bool {
        let Some(range) = self.ranges.get(index).filter(|_| index < self.count).copied() else {
            return false;
        };
        if range.end - range.start > text.len().saturating_mul(2) {
            return false;
        }
        let mut offset = range.start;
        for byte in text.bytes() {
            if decoded(path.as_bytes(), range, &mut offset) != Some(byte) {
                return false;
            }
        }
        offset == range.end
    }
    pub(super) fn terminal_is_empty(&self) -> bool {
        self.binding.is_none()
    }
    pub(super) fn clear(&mut self) {
        self.binding = None;
    }
    fn verify(&self, root: &dyn Tree, path: &str) -> Result<(), ValueError> {
        let binding = (root as *const dyn Tree as *const () as usize, path.as_ptr() as usize, path.len());
        if self.binding.is_some_and(|original| original != binding) {
            return Err(invalid("IFC original pointer or snapshot owner changed"));
        }
        Ok(())
    }
    pub(super) fn advance(&mut self, root: &dyn Tree, path: &str, mode: Mode, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<bool>, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC original pointer admission cancelled").into());
        }
        let binding = (root as *const dyn Tree as *const () as usize, path.as_ptr() as usize, path.len());
        self.verify(root, path)?;
        let demand = self.demands();
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) {
            return Ok(None);
        }
        match self.phase {
            Phase::Bind => {
                self.binding = Some(binding);
                if path.len() > editing::SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES || !path.is_empty() && !path.starts_with('/') {
                    self.finish(false);
                } else if path.is_empty() {
                    self.finish(matches!(mode, Mode::Existing | Mode::Syntax));
                } else {
                    self.start = 1;
                    self.offset = 1;
                    self.phase = Phase::Scan;
                }
            }
            Phase::Scan => {
                for _ in 0..64 {
                    if self.offset == path.len() {
                        if self.escaped || self.count == 128 {
                            self.finish(false);
                            break;
                        }
                        self.ranges[self.count] = Range { start: self.start, end: self.offset };
                        self.count += 1;
                        if mode == Mode::Syntax {
                            self.finish(true);
                        } else {
                            self.phase = Phase::Node;
                        }
                        break;
                    }
                    let byte = path.as_bytes()[self.offset];
                    if self.escaped {
                        if !matches!(byte, b'0' | b'1') {
                            self.finish(false);
                            break;
                        }
                        self.escaped = false;
                    } else if byte == b'~' {
                        self.escaped = true;
                    } else if byte == b'/' {
                        if self.count == 128 {
                            self.finish(false);
                            break;
                        }
                        self.ranges[self.count] = Range { start: self.start, end: self.offset };
                        self.count += 1;
                        self.start = self.offset + 1;
                        self.offset += 1;
                        break;
                    }
                    self.offset += 1;
                }
            }
            Phase::Node => {
                if self.depth == self.count {
                    self.finish(true);
                } else {
                    let node = self.node(root)?.canonical_tree_node()?;
                    let last = self.depth + 1 == self.count;
                    match node {
                        Node::Array(_) => {
                            if last && mode == Mode::Member {
                                self.finish(false);
                            } else {
                                self.offset = self.ranges[self.depth].start;
                                self.number = 0;
                                self.digits = 0;
                                self.first_zero = false;
                                self.phase = Phase::Index;
                            }
                        }
                        Node::Object(_) => {
                            if last && mode == Mode::Insert {
                                self.finish(true);
                            } else {
                                self.candidate = 0;
                                self.offset = self.ranges[self.depth].start;
                                self.key_chunk = 0;
                                self.key_offset = 0;
                                self.phase = Phase::Key;
                            }
                        }
                        _ => self.finish(false),
                    }
                }
            }
            Phase::Index => {
                let range = self.ranges[self.depth];
                let Node::Array(length) = self.node(root)?.canonical_tree_node()? else {
                    return Err(invalid("IFC pointer array frontier changed").into());
                };
                let append = mode == Mode::Insert && self.depth + 1 == self.count;
                if append && self.equals(path, self.depth, "-") {
                    self.finish(true);
                } else {
                    for _ in 0..64 {
                        let Some(byte) = decoded(path.as_bytes(), range, &mut self.offset) else {
                            if self.digits == 0 || self.number > length || self.number == length && !append {
                                self.finish(false);
                            } else if append {
                                self.finish(true);
                            } else {
                                self.ordinals[self.depth] = self.number;
                                self.depth += 1;
                                self.phase = Phase::Node;
                            }
                            break;
                        };
                        if !byte.is_ascii_digit() || self.digits > 0 && self.first_zero {
                            self.finish(false);
                            break;
                        }
                        let Some(number) = self.number.checked_mul(10).and_then(|number| number.checked_add(usize::from(byte - b'0'))) else {
                            self.finish(false);
                            break;
                        };
                        if self.digits == 0 {
                            self.first_zero = byte == b'0';
                        }
                        self.digits += 1;
                        self.number = number;
                    }
                }
            }
            Phase::Key => {
                let node = self.node(root)?;
                let Node::Object(length) = node.canonical_tree_node()? else {
                    return Err(invalid("IFC pointer object frontier changed").into());
                };
                if self.candidate == length {
                    self.finish(false);
                } else {
                    let key = node.canonical_tree_key(self.candidate)?;
                    let range = self.ranges[self.depth];
                    for _ in 0..64 {
                        let left = decoded(path.as_bytes(), range, &mut self.offset);
                        let right = key.next_byte(&mut self.key_chunk, &mut self.key_offset).map_err(invalid)?;
                        if left != right {
                            self.candidate += 1;
                            self.offset = range.start;
                            self.key_chunk = 0;
                            self.key_offset = 0;
                            break;
                        }
                        if left.is_none() {
                            self.ordinals[self.depth] = self.candidate;
                            self.depth += 1;
                            self.phase = Phase::Node;
                            break;
                        }
                    }
                }
            }
            Phase::Done => return Ok(Some(self.accepted)),
        }
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok((self.phase == Phase::Done).then_some(self.accepted))
    }
}

#[derive(Default)]
pub(super) struct PathAdmission {
    path: Pointer,
    from: Pointer,
    event: Option<usize>,
    snapshot: Option<usize>,
    stage: u8,
    comparison: usize,
    left: usize,
    right: usize,
    done: Option<bool>,
    closing: bool,
}
impl PathAdmission {
    pub(super) fn position(&self,event:&editing::SnapshotEditEvent)->Option<usize>{let path=event_pointer(event)?;if !self.path.equals(path,0,"entities"){return None;}if self.path.count==2&&matches!(event,editing::SnapshotEditEvent::SetValue{..})||self.path.count>=3&&["args","complex","name"].iter().any(|field|self.path.equals(path,2,field)){self.path.ordinal(path,1)}else{None}}
    pub(super) fn header(&self,event:&editing::SnapshotEditEvent)->Option<(usize,usize)>{let path=event_pointer(event)?;if !self.path.equals(path,0,"header"){return None;}["fileDescription","fileName","fileSchema"].iter().position(|field|self.path.equals(path,1,field)).map(|column|(column,self.path.ranges[1].end))}
    pub(super) fn native_address(&self,event:&editing::SnapshotEditEvent)->Option<NativeAddress>{
        let path=event_pointer(event)?;let position=self.position(event)?;
        let(part,field)=if self.path.equals(path,2,"args"){(None,3)}else if self.path.equals(path,2,"complex")&&self.path.equals(path,4,"args"){(Some(self.path.ordinal(path,3)?),5)}else{return None;};
        Some(NativeAddress{position,part,argument:self.path.ordinal(path,field)?,prefix:self.path.ranges[field].end,field:field+1})
    }
    pub(super) fn field_count(&self)->usize{self.path.count}
    pub(super) fn from_field_count(&self)->usize{self.from.count}
    pub(super) fn from_field_is(&self,path:&str,index:usize,key:&str)->bool{self.from.equals(path,index,key)}
    pub(super) fn from_field_index(&self,path:&str,index:usize)->Option<usize>{self.from.ordinal(path,index)}
    pub(super) fn field_is(&self,path:&str,index:usize,key:&str)->bool{self.path.equals(path,index,key)}
    pub(super) fn field_index(&self,path:&str,index:usize)->Option<usize>{self.path.ordinal(path,index)}
    pub(super) fn field_end(&self,index:usize)->Option<usize>{self.path.ranges.get(index).filter(|_|index<self.path.count).map(|range|range.end)}
    pub(super) fn demands(&self) -> RetirementDemand {
        let mut demand = match self.stage {
            0 => self.path.demands(),
            1 => self.from.demands(),
            _ => RetirementDemand { depth: 1, ..Default::default() },
        };
        demand.copy_bytes += 2 * size_of::<Option<usize>>() + 5 * size_of::<usize>() + size_of::<Option<bool>>();
        demand
    }
    fn virtual_address(&self, event: &editing::SnapshotEditEvent, snapshot: &IfcSnapshot, path: &str) -> Result<bool, ValueError> {
        if matches!(event, editing::SnapshotEditEvent::InsertValue { .. })
            && self.path.count == 4
            && self.path.depth == 2
            && self.path.equals(path, 0, "entities")
            && self.path.equals(path, 2, "complex")
            && (self.path.equals(path, 3, "0") || self.path.equals(path, 3, "-"))
        {
            return Ok(snapshot.entities.get(self.path.ordinals[1]).is_some_and(|entity| entity.complex.is_empty()));
        }
        if !matches!(event, editing::SnapshotEditEvent::SetValue { .. }) || self.path.depth + 1 != self.path.count {
            return Ok(false);
        }
        let reference = self.path.equals(path, self.path.depth, "reference");
        let append = self.path.equals(path, self.path.depth, "appendReference");
        if !reference && !append {
            return Ok(false);
        }
        let node = self.path.node(snapshot)?;
        let Node::Object(length) = node.canonical_tree_node()? else {
            return Ok(false);
        };
        if length == 0 {
            return Ok(false);
        }
        let mut chunk = 0;
        let mut offset = 0;
        let key = node.canonical_tree_key(0)?;
        for byte in b"kind" {
            if key.next_byte(&mut chunk, &mut offset).map_err(invalid)? != Some(*byte) {
                return Ok(false);
            }
        }
        if key.next_byte(&mut chunk, &mut offset).map_err(invalid)?.is_some() {
            return Ok(false);
        }
        Ok(match node.canonical_tree_child(0)?.canonical_tree_node()? {
            Node::String(kind) => reference || append && kind == "aggregate",
            _ => false,
        })
    }
    pub(super) fn advance(
        &mut self,
        event: &editing::SnapshotEditEvent,
        snapshot: &IfcSnapshot,
        semantic: impl FnOnce(&editing::SnapshotEditEvent, &IfcSnapshot) -> Option<bool>,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<Option<bool>, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC original address admission cancelled").into());
        }
        if self.closing {
            return Err(invalid("IFC original address admission closing").into());
        }
        let identity = event as *const editing::SnapshotEditEvent as usize;
        if self.event.is_some_and(|original| original != identity) {
            return Err(invalid("IFC original address event owner changed").into());
        }
        let source = snapshot as *const IfcSnapshot as usize;
        if self.snapshot.is_some_and(|original| original != source) {
            return Err(invalid("IFC original address snapshot owner changed").into());
        }
        match event {
            editing::SnapshotEditEvent::SetValue { path, .. } | editing::SnapshotEditEvent::InsertValue { path, .. } | editing::SnapshotEditEvent::RemoveValue { path } | editing::SnapshotEditEvent::RenameKey { path, .. } => {
                self.path.verify(snapshot, path)?
            }
            editing::SnapshotEditEvent::MoveValue { path, from } => {
                self.path.verify(snapshot, path)?;
                self.from.verify(snapshot, from)?;
            }
            _ => {}
        }
        let demand = self.demands();
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) {
            return Ok(None);
        }
        if let Some(done) = self.done {
            return Ok(Some(done));
        }
        self.event = Some(identity);
        self.snapshot = Some(source);
        let path = match event {
            editing::SnapshotEditEvent::SetValue { path, .. }
            | editing::SnapshotEditEvent::InsertValue { path, .. }
            | editing::SnapshotEditEvent::RemoveValue { path }
            | editing::SnapshotEditEvent::MoveValue { path, .. }
            | editing::SnapshotEditEvent::RenameKey { path, .. } => path,
            editing::SnapshotEditEvent::ReplaceSource { source } => {
                self.done = Some(source.len() <= editing::SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES);
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
                cx.consume_fuel(1);
                return Ok(self.done);
            }
        };
        let child_copy = match self.stage {
            0 => self.path.demands().copy_bytes,
            1 => self.from.demands().copy_bytes,
            _ => 0,
        };
        cx.consume_retained(RetainedCloneProgress { copied_bytes: demand.copy_bytes - child_copy, ..Default::default() })?;
        match self.stage {
            0 => {
                if matches!(event, editing::SnapshotEditEvent::SetValue { .. }) && path.len() > 8192 {
                    self.done = Some(false);
                    cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                    cx.consume_fuel(1);
                } else {
                    let mode = match event {
                        editing::SnapshotEditEvent::InsertValue { .. } => Mode::Insert,
                        editing::SnapshotEditEvent::RenameKey { .. } => Mode::Member,
                        editing::SnapshotEditEvent::MoveValue { .. } => Mode::Syntax,
                        _ => Mode::Existing,
                    };
                    if let Some(accepted) = self.path.advance(snapshot, path, mode, cx)? {
                        let semantic = if path.starts_with("/ifc-") { semantic(event, snapshot) } else { None };
                        let accepted = semantic.unwrap_or(accepted || self.virtual_address(event, snapshot, path)?);
                        self.done = match event {
                            editing::SnapshotEditEvent::MoveValue { .. } if accepted && self.path.count > 0 => {
                                self.stage = 1;
                                None
                            }
                            editing::SnapshotEditEvent::RemoveValue { .. } => Some(accepted && self.path.count > 0),
                            editing::SnapshotEditEvent::RenameKey { key, .. } => Some(accepted && key.len() <= editing::SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES),
                            _ => Some(accepted),
                        };
                    }
                }
            }
            1 => {
                let editing::SnapshotEditEvent::MoveValue { from, .. } = event else { unreachable!() };
                if let Some(accepted) = self.from.advance(snapshot, from, Mode::Existing, cx)? {
                    if !accepted || self.from.count == 0 {
                        self.done = Some(false);
                    } else if self.path.count <= self.from.count {
                        self.done = Some(true);
                    } else {
                        self.stage = 2;
                        self.left = self.from.ranges[0].start;
                        self.right = self.path.ranges[0].start;
                    }
                }
            }
            _ => {
                let editing::SnapshotEditEvent::MoveValue { from, .. } = event else { unreachable!() };
                for _ in 0..64 {
                    let left = decoded(from.as_bytes(), self.from.ranges[self.comparison], &mut self.left);
                    let right = decoded(path.as_bytes(), self.path.ranges[self.comparison], &mut self.right);
                    if left != right {
                        self.done = Some(true);
                        break;
                    }
                    if left.is_none() {
                        self.comparison += 1;
                        if self.comparison == self.from.count {
                            self.done = Some(false);
                            break;
                        }
                        self.left = self.from.ranges[self.comparison].start;
                        self.right = self.path.ranges[self.comparison].start;
                    }
                }
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                cx.consume_fuel(1);
            }
        }
        Ok(self.done)
    }
    pub(super) fn begin_close(&mut self) {
        self.closing = true;
    }
    pub(super) fn terminal_is_empty(&self) -> bool {
        self.event.is_none() && self.snapshot.is_none() && self.path.terminal_is_empty() && self.from.terminal_is_empty()
    }
    pub(super) fn close_demands(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: 2 * size_of::<Option<(usize, usize, usize)>>() + 2 * size_of::<Option<usize>>(), depth: 1, ..Default::default() }
    }
    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing {
            return Err(invalid("IFC original address close not started"));
        }
        let demand = self.close_demands();
        if !retained_clone_grant_admits(grant, demand) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        self.clear();
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }
    pub(super) fn clear(&mut self) {
        self.path.clear();
        self.from.clear();
        self.event = None;
        self.snapshot = None;
    }
}
#[cfg(test)]
#[path = "pointer_tests.rs"]
mod tests;
