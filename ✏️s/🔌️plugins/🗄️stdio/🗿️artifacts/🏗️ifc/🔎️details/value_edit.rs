//! 🪡️ Native argument ownership crosses one child or collection slot per original turn.
use super::*;
use semio_framework_value::retained_clone::{RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;

#[derive(semio_framework_value::RetireOwned)]
struct Parent {
    value: IfcValue,
    index: usize,
}

#[cfg(test)]
#[path = "value_edit_tests.rs"]
mod tests;
#[derive(Clone, Copy)]
enum Target {
    Whole,
    Payload,
    Name,
    Items,
}
#[derive(Clone, Copy)]
enum Rebuild {
    Insert(usize),
    Remove(usize),
    Move(usize, usize),
}
#[derive(Clone, Copy)]
enum Phase {
    Start,
    Stack,
    Walk,
    Input,
    Receive,
    Take,
    Apply,
    Allocate,
    Rebuild,
    Ascend,
    Done,
}

pub(super) struct IfcValueEdit {
    current: Option<IfcValue>,
    parents: Option<Vec<Parent>>,
    input: Option<DslValue>,
    replacement: Option<IfcValue>,
    retired: Option<IfcValue>,
    retired_parent: Option<Parent>,
    rebuilt: Option<Vec<IfcValue>>,
    receiver: IfcValueReceiver,
    retirement: RetainedCloneClose,
    phase: Phase,
    target: Target,
    rebuild: Option<Rebuild>,
    next: usize,
    offset: usize,
    prefix: usize,
    tuple: bool,
    closing: bool,
}

fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
fn items(value: &mut IfcValue) -> Result<&mut Vec<IfcValue>, ValueError> {
    match value {
        IfcValue::Aggregate(items) => Ok(items),
        IfcValue::TypedValue(typed) => Ok(&mut typed.items),
        _ => Err(invalid("IFC item edit requires an aggregate or typed items")),
    }
}
fn item_count(value: &IfcValue) -> Result<usize, ValueError> {
    match value {
        IfcValue::Aggregate(items) => Ok(items.len()),
        IfcValue::TypedValue(typed) => Ok(typed.items.len()),
        _ => Err(invalid("IFC item edit requires an aggregate or typed items")),
    }
}
fn pointer(event: &editing::SnapshotEditEvent) -> Result<&str, ValueError> {
    match event {
        editing::SnapshotEditEvent::SetValue { path, .. } | editing::SnapshotEditEvent::InsertValue { path, .. } | editing::SnapshotEditEvent::RemoveValue { path } | editing::SnapshotEditEvent::MoveValue { path, .. } => Ok(path),
        _ => Err(invalid("IFC native value fields cannot be renamed")),
    }
}
fn segment(path: &str, offset: usize) -> Result<(&str, usize), ValueError> {
    let tail = path.get(offset..).ok_or_else(|| invalid("IFC native pointer offset is invalid"))?;
    let tail = tail.strip_prefix('/').ok_or_else(|| invalid("IFC native pointer requires a field"))?;
    let length = tail.find('/').unwrap_or(tail.len());
    if length == 0 {
        return Err(invalid("IFC native pointer has an empty field"));
    }
    Ok((&tail[..length], offset + length + 1))
}
fn index(text: &str, length: usize, append: bool) -> Result<usize, ValueError> {
    if text == "-" && append {
        return Ok(length);
    }
    if text.len() > 20 || text.is_empty() || text.len() > 1 && text.starts_with('0') || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(invalid("IFC item index must be canonical"));
    }
    text.parse::<usize>().map_err(|_| invalid("IFC item index is outside its domain"))
}

impl Default for IfcValueEdit {
    fn default() -> Self {
        Self {
            current: None,
            parents: None,
            input: None,
            replacement: None,
            retired: None,
            retired_parent: None,
            rebuilt: None,
            receiver: IfcValueReceiver::new(IfcValueReceiveMode::Native),
            retirement: Default::default(),
            phase: Phase::Start,
            target: Target::Whole,
            rebuild: None,
            next: 0,
            offset: 0,
            prefix: 0,
            tuple: false,
            closing: false,
        }
    }
}
impl IfcValueEdit {
    pub(super) fn started(&self) -> bool {
        !matches!(self.phase, Phase::Start)
    }
    pub(super) fn supply_native(&mut self,value:&mut Option<IfcValue>)->Result<(),ValueError>{if self.started()||self.replacement.is_some()||value.is_none(){return Err(invalid("IFC original native input is absent or already supplied"));}self.replacement=value.take();Ok(())}
    fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if !self.retirement.is_empty() {
            return self.retirement.next_owner_demands::<IfcValue>(false, &None, 4096);
        }
        if self.retired.is_some() {
            return self.retirement.next_owner_demands::<IfcValue>(true, &None, 4096);
        }
        self.retirement.next_owner_demands::<Parent>(self.retired_parent.is_some(), &None, 4096)
    }
    fn retirement_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.retirement.is_empty() {
            return self.retirement.step_granted(grant);
        }
        if self.retired.is_some() {
            return self.retirement.begin_granted(&mut self.retired, grant)?.ok_or_else(|| invalid("IFC displaced value owner missing"));
        }
        self.retirement.begin_granted(&mut self.retired_parent, grant)?.ok_or_else(|| invalid("IFC parent owner missing"))
    }
    pub(super) fn demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.retired.is_some() || self.retired_parent.is_some() || !self.retirement.is_empty() {
            return self.retirement_demands();
        }
        if matches!(self.phase, Phase::Receive) {
            return self.receiver.demands();
        }
        let capacity_bytes = match self.phase {
            Phase::Stack => 128 * size_of::<Parent>(),
            Phase::Allocate => self.output_length()?.checked_mul(size_of::<IfcValue>()).ok_or_else(|| invalid("IFC collection capacity overflow"))?,
            _ => 0,
        };
        Ok(RetirementDemand { copy_bytes: 3 * size_of::<IfcValue>() + size_of::<Parent>() + size_of::<DslValue>(), capacity_bytes, depth: 1, ..Default::default() })
    }
    fn output_length(&self) -> Result<usize, ValueError> {
        let length = item_count(self.current.as_ref().ok_or_else(|| invalid("IFC edited collection missing"))?)?;
        match self.rebuild.ok_or_else(|| invalid("IFC collection operation missing"))? {
            Rebuild::Insert(_) => length.checked_add(1).ok_or_else(|| invalid("IFC collection length overflow")),
            Rebuild::Remove(_) => length.checked_sub(1).ok_or_else(|| invalid("IFC collection is empty")),
            Rebuild::Move(_, _) => Ok(length),
        }
    }
    pub(super) fn advance(&mut self, owner: &mut Option<IfcValue>, event: &mut editing::SnapshotEditEvent, prefix: usize, tuple: bool, reference:Option<reference::ReferenceRequest>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC native value edit cancelled").into());
        }
        if self.closing {
            return Err(invalid("IFC native value edit is closing").into());
        }
        if matches!(self.phase, Phase::Done) {
            return Ok(true);
        }
        let demand = self.demands()?;
        let grant = cx.retained_grant();
        if cx.should_yield()
            || grant.maximum_items == 0
            || grant.maximum_copy_bytes < demand.copy_bytes
            || grant.maximum_capacity_bytes < demand.capacity_bytes
            || grant.maximum_release_bytes < demand.release_bytes
            || grant.maximum_depth < demand.depth
        {
            return Ok(false);
        }
        if self.retired.is_some() || self.retired_parent.is_some() || !self.retirement.is_empty() {
            let step = self.retirement_step(grant).map_err(|error| semio_framework_job::OriginalTurnFailure::debit_original(error, cx))?;
            cx.consume_retained(step.progress())?;
            cx.consume_fuel(step.progress().copied_items as u64);
            return Ok(false);
        }
        if matches!(self.phase, Phase::Receive) {
            if self.receiver.advance(cx)? {
                self.phase = Phase::Take;
            }
            return Ok(false);
        }
        if matches!(self.phase, Phase::Take) {
            if self.receiver.take(&mut self.replacement, cx)? {
                self.phase = Phase::Apply;
            }
            return Ok(false);
        }
        let mut progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        match self.phase {
            Phase::Start => {
                let path = pointer(event)?;
                if path.len() > 8192 || path.get(prefix..).is_none_or(|suffix|!suffix.is_empty()&&!suffix.starts_with('/')) {
                    return Err(invalid("IFC native edit leaves its bounded owner pointer").into());
                }
                self.prefix = prefix;
                self.offset = prefix;
                self.tuple = tuple;
                self.current = owner.take();
                if self.current.is_none() {
                    return Err(invalid("IFC native edit owner missing").into());
                }
                self.phase = Phase::Walk;
                progress.copied_bytes = size_of::<IfcValue>();
            }
            Phase::Stack => {
                let parents = self.parents.get_or_insert_with(Vec::new);
                parents.try_reserve_exact(128).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC edited parent stack allocation failed"))?;
                progress.retained_capacity_bytes = parents.capacity() * size_of::<Parent>();
                self.phase = Phase::Walk;
            }
            Phase::Walk => {
                let path = pointer(event)?;
                if let Some(reference)=reference.filter(|reference|reference.owner_end==self.offset){
                    self.target=Target::Whole;
                    if reference.kind==reference::ReferenceKind::Append{self.rebuild=Some(Rebuild::Insert(item_count(self.current.as_ref().unwrap())?));}
                    self.phase=Phase::Input;
                }else if self.offset == path.len() {
                    if !matches!(event, editing::SnapshotEditEvent::SetValue { .. } | editing::SnapshotEditEvent::InsertValue { .. }) {
                        return Err(invalid("IFC root value requires replacement").into());
                    }
                    self.target = if self.tuple && self.offset == self.prefix { Target::Payload } else { Target::Whole };
                    self.phase = Phase::Input;
                } else {
                    let (field, mut offset) = segment(path, self.offset)?;
                    if !(self.tuple && self.offset == self.prefix) {
                        if field != "value" {
                            return Err(invalid("IFC edit does not address a native value").into());
                        }
                        if offset == path.len() {
                            if !matches!(event, editing::SnapshotEditEvent::SetValue { .. }) {
                                return Err(invalid("IFC payload requires replacement").into());
                            }
                            self.target = Target::Payload;
                            self.phase = Phase::Input;
                            cx.consume_retained(progress)?;
                            cx.consume_fuel(1);
                            return Ok(false);
                        }
                    }
                    let (field, offset_after_field) = if self.tuple && self.offset == self.prefix { (field, offset) } else { segment(path, offset)? };
                    offset = offset_after_field;
                    let typed = matches!(self.current, Some(IfcValue::TypedValue(_)));
                    let (item, end) = if typed {
                        if field == "name" && offset == path.len() && matches!(event, editing::SnapshotEditEvent::SetValue { .. }) {
                            self.target = Target::Name;
                            self.phase = Phase::Input;
                            cx.consume_retained(progress)?;
                            cx.consume_fuel(1);
                            return Ok(false);
                        }
                        if field != "items" {
                            return Err(invalid("IFC typed value requires name or items").into());
                        }
                        if offset == path.len() && matches!(event, editing::SnapshotEditEvent::SetValue { .. }) {
                            self.target = Target::Items;
                            self.phase = Phase::Input;
                            cx.consume_retained(progress)?;
                            cx.consume_fuel(1);
                            return Ok(false);
                        }
                        segment(path, offset)?
                    } else {
                        (field, offset)
                    };
                    let length = item_count(self.current.as_ref().unwrap())?;
                    let position = index(item, length, matches!(event, editing::SnapshotEditEvent::InsertValue { .. } | editing::SnapshotEditEvent::MoveValue { .. }))?;
                    if end == path.len() && !matches!(event, editing::SnapshotEditEvent::SetValue { .. }) {
                        self.rebuild = Some(match event {
                            editing::SnapshotEditEvent::InsertValue { .. } if position <= length => Rebuild::Insert(position),
                            editing::SnapshotEditEvent::RemoveValue { .. } if position < length => Rebuild::Remove(position),
                            editing::SnapshotEditEvent::MoveValue { from, .. } => {
                                let parent_end = path.rfind('/').unwrap();
                                let (parent, origin) = from.rsplit_once('/').ok_or_else(|| invalid("IFC move origin is missing"))?;
                                if parent != &path[..parent_end] {
                                    return Err(invalid("IFC move must remain in its owning collection").into());
                                }
                                let start = index(origin, length, false)?;
                                let destination = if item == "-" { length.saturating_sub(1) } else { position };
                                if start >= length || destination >= length {
                                    return Err(invalid("IFC move index is out of bounds").into());
                                }
                                Rebuild::Move(start, destination)
                            }
                            _ => return Err(invalid("IFC item index is out of bounds").into()),
                        });
                        self.phase = if matches!(self.rebuild, Some(Rebuild::Insert(_))) { Phase::Input } else { Phase::Allocate };
                        self.target = Target::Whole;
                    } else {
                        if position >= length {
                            return Err(invalid("IFC nested item is out of bounds").into());
                        }
                        if self.parents.is_none() {
                            self.phase = Phase::Stack;
                        } else {
                            let parents = self.parents.as_mut().unwrap();
                            if parents.len() == 128 {
                                return Err(invalid("IFC edited value exceeds its declared depth").into());
                            }
                            let child = std::mem::replace(&mut items(self.current.as_mut().unwrap())?[position], IfcValue::Unset);
                            let parent = Parent { value: self.current.replace(child).unwrap(), index: position };
                            parents.push(parent);
                            self.offset = end;
                            progress.copied_bytes = 2 * size_of::<IfcValue>() + size_of::<Parent>();
                        }
                    }
                }
            }
            Phase::Input => {
                if self.replacement.is_some(){self.phase=Phase::Apply;progress.copied_bytes=size_of::<Phase>();}
                else if let Some(reference)=reference{
                    self.replacement=Some(reference.value());self.phase=Phase::Apply;progress.copied_bytes=size_of::<IfcValue>();
                }else if self.input.is_none() {
                    let value = match event {
                        editing::SnapshotEditEvent::SetValue { value, .. } | editing::SnapshotEditEvent::InsertValue { value, .. } => value,
                        _ => return Err(invalid("IFC edit input is missing").into()),
                    };
                    self.input = Some(std::mem::replace(value, DslValue::Null));
                    progress.copied_bytes = size_of::<DslValue>();
                } else {
                    let received = match self.target {
                        Target::Whole => self.receiver.receive_owned(&mut self.input, cx)?,
                        Target::Payload => self.receiver.receive_native_payload(&mut self.input, self.current.as_ref().unwrap(), cx)?,
                        Target::Name => self.receiver.receive_native_payload(&mut self.input, &IfcValue::String(String::new()), cx)?,
                        Target::Items => self.receiver.receive_native_payload(&mut self.input, &IfcValue::Aggregate(Vec::new()), cx)?,
                    };
                    if received {
                        self.phase = Phase::Receive;
                    }
                    return Ok(false);
                }
            }
            Phase::Apply => {
                if matches!(self.rebuild, Some(Rebuild::Insert(_))) {
                    self.phase = Phase::Allocate;
                } else {
                    match self.target {
                        Target::Name => {
                            if !matches!(self.replacement, Some(IfcValue::String(_))) || !matches!(self.current, Some(IfcValue::TypedValue(_))) {
                                return Err(invalid("IFC typed name requires its typed owner and text").into());
                            }
                            let Some(IfcValue::String(name)) = self.replacement.take() else { unreachable!() };
                            let Some(IfcValue::TypedValue(typed)) = self.current.as_mut() else { unreachable!() };
                            self.retired = Some(IfcValue::String(std::mem::replace(&mut typed.name, name)));
                        }
                        Target::Items => {
                            if !matches!(self.replacement, Some(IfcValue::Aggregate(_))) || !matches!(self.current, Some(IfcValue::TypedValue(_))) {
                                return Err(invalid("IFC typed items require their typed owner and an array").into());
                            }
                            let Some(IfcValue::Aggregate(next)) = self.replacement.take() else { unreachable!() };
                            let Some(IfcValue::TypedValue(typed)) = self.current.as_mut() else { unreachable!() };
                            self.retired = Some(IfcValue::Aggregate(std::mem::replace(&mut typed.items, next)));
                        }
                        _ => self.retired = self.current.replace(self.replacement.take().ok_or_else(|| invalid("IFC received replacement missing"))?),
                    }
                    self.phase = Phase::Ascend;
                    progress.copied_bytes = 2 * size_of::<IfcValue>();
                }
            }
            Phase::Allocate => {
                let length = self.output_length()?;
                let output = self.rebuilt.get_or_insert_with(Vec::new);
                output.try_reserve_exact(length).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC edited collection allocation failed"))?;
                progress.retained_capacity_bytes = output.capacity() * size_of::<IfcValue>();
                self.next = 0;
                self.phase = Phase::Rebuild;
            }
            Phase::Rebuild => {
                let length = self.output_length()?;
                if self.next < length {
                    let source = match self.rebuild.unwrap() {
                        Rebuild::Insert(at) => {
                            if self.next == at {
                                None
                            } else {
                                Some(self.next - usize::from(self.next > at))
                            }
                        }
                        Rebuild::Remove(at) => Some(self.next + usize::from(self.next >= at)),
                        Rebuild::Move(start, end) => Some(if self.next == end {
                            start
                        } else if start < end && (start..end).contains(&self.next) {
                            self.next + 1
                        } else if start > end && (end + 1..=start).contains(&self.next) {
                            self.next - 1
                        } else {
                            self.next
                        }),
                    };
                    let value = match source {
                        Some(index) => std::mem::replace(&mut items(self.current.as_mut().unwrap())?[index], IfcValue::Unset),
                        None => self.replacement.take().ok_or_else(|| invalid("IFC inserted value missing"))?,
                    };
                    self.rebuilt.as_mut().unwrap().push(value);
                    self.next += 1;
                    progress.copied_bytes = size_of::<IfcValue>();
                } else {
                    let next = self.rebuilt.take().unwrap();
                    self.retired = Some(IfcValue::Aggregate(std::mem::replace(items(self.current.as_mut().unwrap())?, next)));
                    self.phase = Phase::Ascend;
                    progress.copied_bytes = size_of::<IfcValue>();
                }
            }
            Phase::Ascend => {
                if let Some(parent) = self.parents.as_mut().and_then(Vec::pop) {
                    self.retired_parent = Some(parent);
                    let parent = self.retired_parent.as_mut().unwrap();
                    let slot = &mut items(&mut parent.value)?[parent.index];
                    *slot = self.current.take().ok_or_else(|| invalid("IFC edited child missing"))?;
                    self.current = Some(std::mem::replace(&mut parent.value, IfcValue::Unset));
                    progress.copied_bytes = 2 * size_of::<IfcValue>() + size_of::<Parent>();
                } else {
                    if owner.is_some() {
                        return Err(invalid("IFC edited owner destination is occupied").into());
                    }
                    *owner = self.current.take();
                    self.phase = Phase::Done;
                    progress.copied_bytes = size_of::<IfcValue>();
                }
            }
            Phase::Receive | Phase::Take | Phase::Done => unreachable!(),
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        Ok(matches!(self.phase, Phase::Done))
    }
    pub(super) fn begin_close(&mut self) {
        self.closing = true;
        self.receiver.begin_close();
    }
    pub(super) fn terminal_is_empty(&self) -> bool {
        self.current.is_none()
            && self.parents.is_none()
            && self.input.is_none()
            && self.replacement.is_none()
            && self.retired.is_none()
            && self.retired_parent.is_none()
            && self.rebuilt.is_none()
            && self.receiver.terminal_is_empty()
            && self.retirement.is_empty()
    }
    pub(super) fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if !self.receiver.terminal_is_empty() {
            return self.receiver.close_demands();
        }
        if self.retired.is_some() || self.retired_parent.is_some() || !self.retirement.is_empty() {
            return self.retirement_demands();
        }
        if self.current.is_some() || self.replacement.is_some() {
            return self.retirement.next_owner_demands::<IfcValue>(true, &None, 4096);
        }
        if self.input.is_some() {
            return self.retirement.next_owner_demands::<DslValue>(true, &None, 4096);
        }
        if self.rebuilt.is_some() {
            return self.retirement.next_owner_demands::<Vec<IfcValue>>(true, &None, 4096);
        }
        self.retirement.next_owner_demands::<Vec<Parent>>(self.parents.is_some(), &None, 4096)
    }
    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing {
            return Err(invalid("IFC native edit close was not initiated"));
        }
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if !self.receiver.terminal_is_empty() {
            return self.receiver.close_step(grant);
        }
        if self.retired.is_some() || self.retired_parent.is_some() || !self.retirement.is_empty() {
            return self.retirement_step(grant);
        }
        if self.current.is_some() {
            return self.retirement.begin_granted(&mut self.current, grant)?.ok_or_else(|| invalid("IFC current owner missing"));
        }
        if self.replacement.is_some() {
            return self.retirement.begin_granted(&mut self.replacement, grant)?.ok_or_else(|| invalid("IFC replacement owner missing"));
        }
        if self.input.is_some() {
            return self.retirement.begin_granted(&mut self.input, grant)?.ok_or_else(|| invalid("IFC original input owner missing"));
        }
        if self.rebuilt.is_some() {
            return self.retirement.begin_granted(&mut self.rebuilt, grant)?.ok_or_else(|| invalid("IFC rebuilt collection owner missing"));
        }
        self.retirement.begin_granted(&mut self.parents, grant)?.ok_or_else(|| invalid("IFC ancestor owner missing"))
    }
}
