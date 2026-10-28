//! 🧱️ Literal flat IFC records move into retained graph slots one original physical frontier at a time.
use super::{Frame, IfcComplexType, IfcEntity, IfcHeader, IfcSnapshot, IfcTypedValue, IfcValue};
use semio_framework_dsl_record::receiving::owned::{OwnedSlot, funds, metadata, receipt};
use semio_framework_value::{
    RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetireOwned, RetirementDemand, ValueError, ValueRefusalKind,
    retirement::{RetirementCursor, RetirementStep},
};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::mem::size_of;
fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "literal IFC Pack value forest has invalid shape, ownership or references")
}
fn fixed() -> RetirementDemand {
    metadata::<(IfcValue, IfcValue, IfcEntity, usize, usize, usize, u8)>()
}
fn backing<T>(count: usize) -> Result<RetirementDemand, ValueError> {
    Ok(RetirementDemand { copy_bytes: size_of::<Vec<T>>(), capacity_bytes: count.checked_mul(size_of::<T>()).ok_or_else(invalid)?, depth: 1, ..Default::default() })
}
#[derive(RetireOwned)]
struct ValueRow {
    value: Option<IfcValue>,
    depth: u8,
}
fn roots(frame: &Frame, group: usize) -> Result<&[usize], ValueError> {
    match group {
        0 => Ok(&frame.file_description),
        1 => Ok(&frame.file_name),
        2 => Ok(&frame.file_schema),
        _ => Err(invalid()),
    }
}
fn header(snapshot: &mut IfcSnapshot, group: usize) -> Result<&mut Vec<IfcValue>, ValueError> {
    match group {
        0 => Ok(&mut snapshot.header.file_description),
        1 => Ok(&mut snapshot.header.file_name),
        2 => Ok(&mut snapshot.header.file_schema),
        _ => Err(invalid()),
    }
}
fn items(value: &mut IfcValue) -> Result<&mut Vec<IfcValue>, ValueError> {
    match value {
        IfcValue::Aggregate(v) => Ok(v),
        IfcValue::TypedValue(v) => Ok(&mut v.items),
        _ => Err(invalid()),
    }
}
fn take(values: &mut [ValueRow], index: usize) -> Result<IfcValue, ValueError> {
    values.get_mut(index).and_then(|v| v.value.take()).ok_or_else(invalid)
}
pub(super) fn binary_head(text: &str) -> Result<(), ValueError> {
    let bytes = text.as_bytes();
    let Some(&unused) = bytes.first() else {
        return Err(invalid());
    };
    if !(b'0'..=b'3').contains(&unused) || (bytes.len() == 1 && unused != b'0') {
        return Err(invalid());
    }
    if let Some(&first) = bytes.get(1) {
        let digit = match first {
            b'0'..=b'9' => first - b'0',
            b'A'..=b'F' => first - b'A' + 10,
            _ => return Err(invalid()),
        };
        if digit >= (16u8 >> (unused - b'0')) {
            return Err(invalid());
        }
    }
    Ok(())
}
/// 🧳️ Source, partial values, rows and graph output remain in one original receiving owner through failure.
#[derive(Default)]
pub(super) struct GraphReceiving {
    pub(super) frame: OwnedSlot<Frame>,
    values: OwnedSlot<Vec<ValueRow>>,
    pending: OwnedSlot<IfcValue>,
    entity: OwnedSlot<IfcEntity>,
    complex: OwnedSlot<IfcComplexType>,
    output: OwnedSlot<IfcSnapshot>,
    phase: u8,
    node: usize,
    child: usize,
    entity_index: usize,
    complex_index: usize,
    group: usize,
    root: usize,
    binary: usize,
    depth: u8,
    closing: bool,
    closed: bool,
}
impl GraphReceiving {
    #[cfg(test)]
    pub(super) fn is_scanning_binary(&self) -> bool {
        self.phase == 21
    }
    pub(super) fn demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.closing || self.closed {
            return self.close_demands(body);
        }
        let frame = self.frame.value.as_ref().ok_or_else(invalid)?;
        match self.phase {
            0 => backing::<ValueRow>(frame.values.len()),
            3 => backing::<IfcValue>(frame.values.get(self.node).ok_or_else(invalid)?.children.len()),
            6 => backing::<IfcEntity>(frame.entities.len()),
            7 if self.group < 3 => backing::<IfcValue>(roots(frame, self.group)?.len()),
            10 => backing::<IfcValue>(frame.entities.get(self.entity_index).ok_or_else(invalid)?.arguments.len()),
            12 => backing::<IfcComplexType>(frame.entities.get(self.entity_index).ok_or_else(invalid)?.complex.len()),
            14 => backing::<IfcValue>(frame.entities.get(self.entity_index).and_then(|e| e.complex.get(self.complex_index)).ok_or_else(invalid)?.arguments.len()),
            21 => {
                let text = frame.values.get(self.node).and_then(|v| v.text.as_ref()).ok_or_else(invalid)?;
                Ok(RetirementDemand { copy_bytes: (text.len() - self.binary).min(body.max(1)).min(4096) + size_of::<usize>(), depth: 1, ..Default::default() })
            }
            _ => Ok(fixed()),
        }
    }
    pub(super) fn step(&mut self, native: &mut NativeDecodeControl<'_>, body: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        if self.closing || self.closed || self.is_ready() {
            return Ok(Default::default());
        }
        let demand = self.demands(body)?;
        if !funds(grant, demand) {
            return Ok(Default::default());
        }
        native.checkpoint()?;
        if self.phase == 0 {
            native.begin_stage(0)?;
        }
        let mut progress = receipt(demand);
        progress.retained_capacity_bytes = 0;
        let frame = self.frame.value.as_mut().ok_or_else(invalid)?;
        let performed = (|| {
            match self.phase {
                0 => {
                    if frame.values.len() > 2_000_000 || frame.entities.len() > 1_000_000 {
                        return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "literal IFC Pack graph exceeds receiving cardinality admission"));
                    }
                    *self.values.value = Some(native.allocate_vec(frame.values.len())?);
                    progress.retained_capacity_bytes = self.values.value.as_ref().unwrap().capacity() * size_of::<ValueRow>();
                    self.phase = 1;
                }
                1 => {
                    let values = self.values.value.as_mut().unwrap();
                    if values.len() < frame.values.len() {
                        values.push(ValueRow { value: None, depth: 0 });
                    } else {
                        self.node = frame.values.len();
                        self.phase = 2;
                    }
                }
                2 => {
                    if self.node == 0 {
                        self.phase = 6;
                    } else {
                        self.node -= 1;
                        self.child = 0;
                        self.depth = 1;
                        let node = &frame.values[self.node];
                        let payloads = usize::from(node.integer.is_some()) + usize::from(node.real.is_some()) + usize::from(node.text.is_some()) + usize::from(node.reference.is_some()) + usize::from(node.name.is_some());
                        let valid = match node.kind.as_str() {
                            "unset" | "derived" => payloads == 0 && node.children.is_empty(),
                            "integer" => node.integer.is_some() && payloads == 1 && node.children.is_empty(),
                            "real" => node.real.is_some() && payloads == 1 && node.children.is_empty(),
                            "string" | "binary" | "enum" => node.text.is_some() && payloads == 1 && node.children.is_empty(),
                            "reference" => node.reference.is_some() && payloads == 1 && node.children.is_empty(),
                            "aggregate" => payloads == 0,
                            "typed" => node.name.is_some() && payloads == 1,
                            _ => false,
                        };
                        if !valid {
                            return Err(invalid());
                        }
                        if node.kind == "binary" {
                            binary_head(node.text.as_deref().unwrap())?;
                            self.binary = 1;
                            self.phase = 21;
                        } else {
                            self.phase = 22;
                        }
                    }
                }
                21 => {
                    let text = frame.values[self.node].text.as_ref().unwrap();
                    let count = (text.len() - self.binary).min(body.max(1)).min(4096);
                    let end = self.binary + count;
                    if text.as_bytes()[self.binary..end].iter().any(|v| !v.is_ascii_digit() && !(b'A'..=b'F').contains(v)) {
                        return Err(invalid());
                    }
                    self.binary = end;
                    if end == text.len() {
                        self.phase = 22;
                    }
                }
                22 => {
                    let node = &mut frame.values[self.node];
                    *self.pending.value = Some(match node.kind.as_str() {
                        "unset" => IfcValue::Unset,
                        "derived" => IfcValue::Derived,
                        "integer" => IfcValue::Integer(node.integer.unwrap()),
                        "real" => IfcValue::Real(node.real.unwrap()),
                        "string" => IfcValue::String(node.text.take().unwrap()),
                        "binary" => IfcValue::Binary(node.text.take().unwrap()),
                        "enum" => IfcValue::Enum(node.text.take().unwrap()),
                        "reference" => IfcValue::Reference(node.reference.unwrap()),
                        "aggregate" => IfcValue::Aggregate(Vec::new()),
                        "typed" => IfcValue::TypedValue(IfcTypedValue { name: node.name.take().unwrap(), items: Vec::new() }),
                        _ => return Err(invalid()),
                    });
                    self.phase = if matches!(self.pending.value.as_ref().unwrap(), IfcValue::Aggregate(_) | IfcValue::TypedValue(_)) { 3 } else { 5 };
                }
                3 => {
                    let out = items(self.pending.value.as_mut().unwrap())?;
                    *out = native.allocate_vec(frame.values[self.node].children.len())?;
                    progress.retained_capacity_bytes = out.capacity() * size_of::<IfcValue>();
                    self.phase = 4;
                }
                4 => {
                    let children = &frame.values[self.node].children;
                    if let Some(&index) = children.get(self.child) {
                        if index <= self.node {
                            return Err(invalid());
                        }
                        let values = self.values.value.as_mut().unwrap();
                        let child = values.get(index).ok_or_else(invalid)?;
                        let depth = child.depth.checked_add(1).ok_or_else(invalid)?;
                        if depth > 64 {
                            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "literal IFC Pack value depth exceeds receiving admission"));
                        }
                        self.depth = self.depth.max(depth);
                        let output = items(self.pending.value.as_mut().unwrap())?;
                        let value = take(values, index)?;
                        output.push(value);
                        self.child += 1;
                    } else {
                        self.phase = 5;
                    }
                }
                5 => {
                    let row = &mut self.values.value.as_mut().unwrap()[self.node];
                    row.value = self.pending.value.take();
                    row.depth = self.depth;
                    self.phase = 2;
                }
                6 => {
                    *self.output.value = Some(IfcSnapshot { schema: std::mem::take(&mut frame.schema), header: IfcHeader { file_description: Vec::new(), file_name: Vec::new(), file_schema: Vec::new() }, entities: Vec::new() });
                    let out = &mut self.output.value.as_mut().unwrap().entities;
                    *out = native.allocate_vec(frame.entities.len())?;
                    progress.retained_capacity_bytes = out.capacity() * size_of::<IfcEntity>();
                    self.group = 0;
                    self.phase = 7;
                }
                7 => {
                    if self.group == 3 {
                        self.entity_index = 0;
                        self.phase = 9;
                    } else {
                        let out = header(self.output.value.as_mut().unwrap(), self.group)?;
                        *out = native.allocate_vec(roots(frame, self.group)?.len())?;
                        progress.retained_capacity_bytes = out.capacity() * size_of::<IfcValue>();
                        self.root = 0;
                        self.phase = 8;
                    }
                }
                8 => {
                    if let Some(&index) = roots(frame, self.group)?.get(self.root) {
                        let output = header(self.output.value.as_mut().unwrap(), self.group)?;
                        let value = take(self.values.value.as_mut().unwrap(), index)?;
                        output.push(value);
                        self.root += 1;
                    } else {
                        self.group += 1;
                        self.phase = 7;
                    }
                }
                9 => {
                    if self.entity_index == frame.entities.len() {
                        self.root = 0;
                        self.phase = 18;
                    } else {
                        let row = &mut frame.entities[self.entity_index];
                        *self.entity.value = Some(IfcEntity { id: row.id, name: std::mem::take(&mut row.name), args: Vec::new(), complex: Vec::new() });
                        self.root = 0;
                        self.complex_index = 0;
                        self.phase = 10;
                    }
                }
                10 => {
                    let out = &mut self.entity.value.as_mut().unwrap().args;
                    *out = native.allocate_vec(frame.entities[self.entity_index].arguments.len())?;
                    progress.retained_capacity_bytes = out.capacity() * size_of::<IfcValue>();
                    self.phase = 11;
                }
                11 => {
                    if let Some(&index) = frame.entities[self.entity_index].arguments.get(self.root) {
                        let value = take(self.values.value.as_mut().unwrap(), index)?;
                        self.entity.value.as_mut().unwrap().args.push(value);
                        self.root += 1;
                    } else {
                        self.phase = 12;
                    }
                }
                12 => {
                    let out = &mut self.entity.value.as_mut().unwrap().complex;
                    *out = native.allocate_vec(frame.entities[self.entity_index].complex.len())?;
                    progress.retained_capacity_bytes = out.capacity() * size_of::<IfcComplexType>();
                    self.phase = 13;
                }
                13 => {
                    let entity = &mut frame.entities[self.entity_index];
                    if self.complex_index == entity.complex.len() {
                        self.phase = 17;
                    } else {
                        *self.complex.value = Some(IfcComplexType { name: std::mem::take(&mut entity.complex[self.complex_index].name), args: Vec::new() });
                        self.root = 0;
                        self.phase = 14;
                    }
                }
                14 => {
                    let out = &mut self.complex.value.as_mut().unwrap().args;
                    *out = native.allocate_vec(frame.entities[self.entity_index].complex[self.complex_index].arguments.len())?;
                    progress.retained_capacity_bytes = out.capacity() * size_of::<IfcValue>();
                    self.phase = 15;
                }
                15 => {
                    if let Some(&index) = frame.entities[self.entity_index].complex[self.complex_index].arguments.get(self.root) {
                        let value = take(self.values.value.as_mut().unwrap(), index)?;
                        self.complex.value.as_mut().unwrap().args.push(value);
                        self.root += 1;
                    } else {
                        self.phase = 16;
                    }
                }
                16 => {
                    self.entity.value.as_mut().unwrap().complex.push(self.complex.value.take().unwrap());
                    self.complex_index += 1;
                    self.phase = 13;
                }
                17 => {
                    self.output.value.as_mut().unwrap().entities.push(self.entity.value.take().unwrap());
                    self.entity_index += 1;
                    self.phase = 9;
                }
                18 => {
                    let values = self.values.value.as_ref().unwrap();
                    if self.root == values.len() {
                        self.phase = 19;
                    } else {
                        if values[self.root].value.is_some() {
                            return Err(invalid());
                        }
                        self.root += 1;
                    }
                }
                _ => return Err(invalid()),
            }
            Ok(progress)
        })();
        let progress = performed.map_err(|error| error.with_retained_progress(progress))?;
        native.step().map_err(|error| error.with_retained_progress(progress))?;
        Ok(progress)
    }
    pub(super) fn is_ready(&self) -> bool {
        !self.closing && !self.closed && self.phase == 19 && self.output.value.is_some()
    }
    pub(super) fn take_demand(&self) -> RetirementDemand {
        metadata::<IfcSnapshot>()
    }
    pub(super) fn take_into(&mut self, output: &mut Option<IfcSnapshot>, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        if output.is_some() || !self.is_ready() {
            return Err(invalid());
        }
        let demand = self.take_demand();
        if !funds(grant, demand) {
            return Ok(Default::default());
        }
        *output = self.output.value.take();
        self.phase = 20;
        Ok(receipt(demand))
    }
    pub(super) fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.closed {
            return Ok(Default::default());
        }
        if !self.output.terminal_is_empty() {
            return self.output.close_demands(body);
        }
        if !self.pending.terminal_is_empty() {
            return self.pending.close_demands(body);
        }
        if !self.complex.terminal_is_empty() {
            return self.complex.close_demands(body);
        }
        if !self.entity.terminal_is_empty() {
            return self.entity.close_demands(body);
        }
        if !self.values.terminal_is_empty() {
            return self.values.close_demands(body);
        }
        if !self.frame.terminal_is_empty() {
            return self.frame.close_demands(body);
        }
        Ok(metadata::<u8>())
    }
    pub(super) fn close(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if !funds(grant, demand) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        self.closing = true;
        let progress = if !self.output.terminal_is_empty() {
            self.output.close_step(grant)?
        } else if !self.pending.terminal_is_empty() {
            self.pending.close_step(grant)?
        } else if !self.complex.terminal_is_empty() {
            self.complex.close_step(grant)?
        } else if !self.entity.terminal_is_empty() {
            self.entity.close_step(grant)?
        } else if !self.values.terminal_is_empty() {
            self.values.close_step(grant)?
        } else if !self.frame.terminal_is_empty() {
            self.frame.close_step(grant)?
        } else {
            self.closed = true;
            receipt(demand)
        };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }
    pub(super) fn terminal_is_empty(&self) -> bool {
        self.closed && self.output.terminal_is_empty() && self.pending.terminal_is_empty() && self.complex.terminal_is_empty() && self.entity.terminal_is_empty() && self.values.terminal_is_empty() && self.frame.terminal_is_empty()
    }
}
impl RetireOwned for GraphReceiving {
    fn retirement(mut self) -> Box<dyn RetirementCursor> {
        self.closing = true;
        Box::new(self)
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
    fn controlled_retirement_supported() -> bool {
        true
    }
}
impl RetirementCursor for GraphReceiving {
    fn next_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.close_demands(body)
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        match self.close(grant) {
            Ok(step) => RetirementStep::Progress(step.progress()),
            Err(error) => RetirementStep::Failure(error),
        }
    }
    fn terminal_is_empty(&self) -> bool {
        Self::terminal_is_empty(self)
    }
    fn terminal_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
impl Drop for GraphReceiving {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "literal IFC graph receiving requires original funded closure");
    }
}
