//! 🫴️ Original owned graph values move through a bounded native receiving frontier.
use super::*;
use semio_framework_value::retained_clone::{RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;

#[derive(Clone, Copy)]
pub enum IfcValueReceiveMode {
    Native,
    Decimal,
}

#[derive(semio_framework_value::RetireOwned)]
struct Frame {
    input: Vec<DslValue>,
    values: Vec<IfcValue>,
    name: Option<String>,
    next: usize,
}

#[derive(Clone, Copy)]
enum Phase {
    Init,
    Node,
    Stack,
    Allocate,
    Children,
    Deliver,
    Binary,
    Ready,
    Spent,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Unset,
    Derived,
    Integer,
    Real,
    String,
    Enum,
    Binary,
    Reference,
    Aggregate,
    Typed,
}

pub struct IfcValueReceiver {
    mode: IfcValueReceiveMode,
    payload_kind: Option<Kind>,
    current: Option<DslValue>,
    frames: Option<Vec<Frame>>,
    frame: Option<Frame>,
    output: Option<IfcValue>,
    retired_input: Option<DslValue>,
    retired_frame: Option<Frame>,
    retirement: RetainedCloneClose,
    phase: Phase,
    nodes: usize,
    binary_offset: usize,
    binary_payload: usize,
    closing: bool,
}

fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
fn fits(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_items != 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}
fn decimal(text: &str, signed: bool) -> Result<(), ValueError> {
    let digits = if signed { text.strip_prefix('-').unwrap_or(text) } else { text };
    if text.len() > 20 || digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) || (digits.len() > 1 && digits.starts_with('0')) || text == "-0" {
        return Err(invalid("IFC decimal transport requires a canonical exact integer"));
    }
    Ok(())
}
fn fields(value: &DslValue) -> Result<&Vec<(String, DslValue)>, ValueError> {
    let DslValue::Object(fields) = value else { return Err(invalid("IFC value requires a closed tagged object")) };
    Ok(fields)
}
fn field(fields: &[(String, DslValue)], name: &str) -> Result<usize, ValueError> {
    fields.iter().position(|(key, _)| key == name).ok_or_else(|| invalid("IFC value field is missing"))
}
fn shape(value: &DslValue) -> Result<(Kind, usize), ValueError> {
    let object = fields(value)?;
    if object.len() > 2 {
        return Err(invalid("IFC value has unknown or repeated fields"));
    }
    let tag = field(object, "kind")?;
    let DslValue::String(name) = &object[tag].1 else { return Err(invalid("IFC value kind requires text")) };
    let kind = match name.as_str() {
        "unset" => Kind::Unset,
        "derived" => Kind::Derived,
        "integer" => Kind::Integer,
        "real" => Kind::Real,
        "string" => Kind::String,
        "enum" => Kind::Enum,
        "binary" => Kind::Binary,
        "reference" => Kind::Reference,
        "aggregate" => Kind::Aggregate,
        "typedValue" => Kind::Typed,
        _ => return Err(invalid("Unknown IFC value kind")),
    };
    if matches!(kind, Kind::Unset | Kind::Derived) {
        if object.len() != 1 {
            return Err(invalid("IFC marker has no payload"));
        }
        return Ok((kind, 0));
    }
    if object.len() != 2 {
        return Err(invalid("IFC value requires its payload"));
    }
    Ok((kind, field(object, "value")?))
}

impl IfcValueReceiver {
    pub fn new(mode: IfcValueReceiveMode) -> Self {
        Self {
            mode,
            payload_kind: None,
            current: None,
            frames: None,
            frame: None,
            output: None,
            retired_input: None,
            retired_frame: None,
            retirement: Default::default(),
            phase: Phase::Init,
            nodes: 0,
            binary_offset: 0,
            binary_payload: 0,
            closing: false,
        }
    }
    pub fn receive_native_payload(&mut self, input: &mut Option<DslValue>, exemplar: &IfcValue, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if !matches!(self.mode, IfcValueReceiveMode::Native) {
            return Err(invalid("IFC bare payload requires native receiving mode").into());
        }
        let kind = match exemplar {
            IfcValue::Integer(_) => Kind::Integer,
            IfcValue::Real(_) => Kind::Real,
            IfcValue::String(_) => Kind::String,
            IfcValue::Enum(_) => Kind::Enum,
            IfcValue::Binary(_) => Kind::Binary,
            IfcValue::Reference(_) => Kind::Reference,
            IfcValue::Aggregate(_) => Kind::Aggregate,
            IfcValue::TypedValue(_) => Kind::Typed,
            _ => return Err(invalid("IFC marker has no editable payload").into()),
        };
        if self.current.is_some() {
            if input.is_some() || self.payload_kind != Some(kind) {
                return Err(invalid("IFC receiver cannot rebind its original source or payload shape").into());
            }
            return Ok(true);
        }
        let received = self.receive_owned(input, cx)?;
        if received {
            self.payload_kind = Some(kind);
        }
        Ok(received)
    }
    pub fn receive_owned(&mut self, input: &mut Option<DslValue>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if self.current.is_some() {
            if input.is_some() {
                return Err(invalid("IFC receiver cannot rebind its original source").into());
            }
            return Ok(true);
        }
        if self.nodes != 0 || self.closing {
            return Err(invalid("IFC receiver cannot accept another source").into());
        }
        let bytes = size_of::<DslValue>();
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC value receiving cancelled").into());
        }
        if cx.should_yield() || !fits(cx.retained_grant(), RetirementDemand { copy_bytes: bytes, depth: 1, ..Default::default() }) {
            return Ok(false);
        }
        self.current = input.take();
        if self.current.is_none() {
            return Err(invalid("IFC original value source is missing").into());
        }
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(true)
    }
    pub fn demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.retired_input.is_some() || self.retired_frame.is_some() || !self.retirement.is_empty() {
            return self.retire_demands();
        }
        let copy = match self.phase {
            Phase::Node => size_of::<Frame>() + size_of::<IfcValue>() + 2 * size_of::<DslValue>(),
            Phase::Allocate => size_of::<Frame>(),
            Phase::Children => size_of::<Frame>() + size_of::<IfcValue>() + size_of::<DslValue>(),
            Phase::Deliver | Phase::Binary => size_of::<IfcValue>() + size_of::<DslValue>(),
            _ => 0,
        };
        let capacity = match self.phase {
            Phase::Stack => 129 * size_of::<Frame>(),
            Phase::Allocate => self.frame.as_ref().ok_or_else(|| invalid("IFC collection frame missing"))?.input.len().checked_mul(size_of::<IfcValue>()).ok_or_else(|| invalid("IFC collection capacity overflow"))?,
            _ => 0,
        };
        Ok(RetirementDemand { copy_bytes: copy, capacity_bytes: capacity, depth: 1, ..Default::default() })
    }
    fn retire_demands(&self) -> Result<RetirementDemand, ValueError> {
        if !self.retirement.is_empty() {
            return self.retirement.next_owner_demands::<DslValue>(false, &None, 4096);
        }
        if self.retired_input.is_some() {
            return self.retirement.next_owner_demands::<DslValue>(true, &None, 4096);
        }
        self.retirement.next_owner_demands::<Frame>(self.retired_frame.is_some(), &None, 4096)
    }
    fn retire_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.retirement.is_empty() {
            return self.retirement.step_granted(grant);
        }
        if self.retired_input.is_some() {
            return self.retirement.begin_granted(&mut self.retired_input, grant)?.ok_or_else(|| invalid("IFC envelope retirement omitted its owner"));
        }
        self.retirement.begin_granted(&mut self.retired_frame, grant)?.ok_or_else(|| invalid("IFC collection retirement omitted its owner"))
    }
    fn payload(&self, index: usize) -> Result<&DslValue, ValueError> {
        if self.payload_kind.is_some() {
            return self.current.as_ref().ok_or_else(|| invalid("IFC payload missing"));
        }
        Ok(&fields(self.current.as_ref().ok_or_else(|| invalid("IFC node missing"))?)?[index].1)
    }
    fn payload_mut(&mut self, index: usize) -> &mut DslValue {
        if self.payload_kind.is_some() {
            return self.current.as_mut().unwrap();
        }
        let Some(DslValue::Object(fields)) = self.current.as_mut() else { unreachable!() };
        &mut fields[index].1
    }
    fn finish_scalar(&mut self, value: IfcValue) {
        self.output = Some(value);
        self.retired_input = self.current.take();
        self.payload_kind = None;
        self.phase = Phase::Deliver;
    }
    pub fn advance(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC value receiving cancelled").into());
        }
        if self.closing {
            return Err(invalid("IFC receiver is closing").into());
        }
        if matches!(self.phase, Phase::Ready) {
            return Ok(true);
        }
        if cx.should_yield() || !fits(cx.retained_grant(), self.demands()?) {
            return Ok(false);
        }
        if self.retired_input.is_some() || self.retired_frame.is_some() || !self.retirement.is_empty() {
            let progress = match self.retire_step(cx.retained_grant()) {
                Ok(step) => step.progress(),
                Err(error) => return Err(semio_framework_job::OriginalTurnFailure::debit_original(error, cx)),
            };
            cx.consume_retained(progress)?;
            cx.consume_fuel(progress.copied_items as u64);
            return Ok(false);
        }
        let mut progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        match self.phase {
            Phase::Init => {
                if self.current.is_none() {
                    return Err(invalid("IFC original source has not been received").into());
                }
                self.phase = Phase::Node;
            }
            Phase::Stack => {
                let stack = self.frames.get_or_insert_with(Vec::new);
                stack.try_reserve_exact(129).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC receiving stack allocation failed"))?;
                progress.retained_capacity_bytes = stack.capacity() * size_of::<Frame>();
                self.phase = Phase::Allocate;
            }
            Phase::Node => {
                if self.nodes >= 65536 {
                    return Err(invalid("IFC value exceeds its declared node extent").into());
                }
                let (kind, payload) = if let Some(kind) = self.payload_kind { (kind, 0) } else { shape(self.current.as_ref().ok_or_else(|| invalid("IFC original node missing"))?)? };
                self.nodes += 1;
                let scalar = match kind {
                    Kind::Unset => Some(IfcValue::Unset),
                    Kind::Derived => Some(IfcValue::Derived),
                    Kind::Integer => {
                        let number = match (self.mode, self.payload(payload)?) {
                            (IfcValueReceiveMode::Native, DslValue::Number(number)) => number.as_i64().ok_or_else(|| invalid("IFC INTEGER is outside i64"))?,
                            (IfcValueReceiveMode::Decimal, DslValue::String(text)) => {
                                decimal(text, true)?;
                                text.parse::<i64>().map_err(|_| invalid("IFC INTEGER is outside i64"))?
                            }
                            _ => return Err(invalid("IFC INTEGER transport does not match receiving mode").into()),
                        };
                        Some(IfcValue::Integer(number))
                    }
                    Kind::Reference => {
                        let id = match (self.mode, self.payload(payload)?) {
                            (IfcValueReceiveMode::Native, DslValue::Number(number)) => number.as_u64().ok_or_else(|| invalid("IFC reference is outside u64"))?,
                            (IfcValueReceiveMode::Decimal, DslValue::String(text)) => {
                                decimal(text, false)?;
                                text.parse::<u64>().map_err(|_| invalid("IFC reference is outside u64"))?
                            }
                            _ => return Err(invalid("IFC reference transport does not match receiving mode").into()),
                        };
                        if id == 0 {
                            return Err(invalid("IFC reference must be positive").into());
                        }
                        Some(IfcValue::Reference(id))
                    }
                    Kind::Real => {
                        let number = match (self.mode, self.payload(payload)?) {
                            (IfcValueReceiveMode::Native, DslValue::Number(number)) => number.as_f64(),
                            (IfcValueReceiveMode::Decimal, value) => {
                                let object = fields(value)?;
                                if object.len() != 1 {
                                    return Err(invalid("IFC REAL bits require a closed object").into());
                                }
                                let DslValue::String(bits) = &object[field(object, "bits")?].1 else { return Err(invalid("IFC REAL bits require decimal text").into()) };
                                decimal(bits, false)?;
                                f64::from_bits(bits.parse::<u64>().map_err(|_| invalid("IFC REAL bits are outside u64"))?)
                            }
                            _ => return Err(invalid("IFC REAL transport does not match receiving mode").into()),
                        };
                        if !number.is_finite() {
                            return Err(invalid("IFC REAL must be finite").into());
                        }
                        Some(IfcValue::Real(number))
                    }
                    Kind::String | Kind::Enum => {
                        if !matches!(self.payload(payload)?, DslValue::String(_)) {
                            return Err(invalid("IFC text value requires text").into());
                        }
                        let DslValue::String(text) = std::mem::replace(self.payload_mut(payload), DslValue::Null) else { unreachable!() };
                        Some(if matches!(kind, Kind::String) { IfcValue::String(text) } else { IfcValue::Enum(text) })
                    }
                    Kind::Binary => {
                        let DslValue::String(text) = self.payload(payload)? else { return Err(invalid("IFC BINARY requires text").into()) };
                        let bytes = text.as_bytes();
                        let Some(unused @ b'0'..=b'3') = bytes.first().copied() else { return Err(invalid("Invalid IFC BINARY unused bit count").into()) };
                        if bytes.len() == 1 && unused != b'0' {
                            return Err(invalid("Invalid empty IFC BINARY").into());
                        }
                        if let Some(first) = bytes.get(1).copied() {
                            let digit = match first {
                                b'0'..=b'9' => first - b'0',
                                b'A'..=b'F' => first - b'A' + 10,
                                _ => return Err(invalid("Invalid IFC BINARY digit").into()),
                            };
                            if digit >= 1 << (4 - (unused - b'0')) {
                                return Err(invalid("Invalid IFC BINARY leading bits").into());
                            }
                        }
                        self.binary_offset = 1;
                        self.binary_payload = payload;
                        self.phase = Phase::Binary;
                        None
                    }
                    Kind::Aggregate | Kind::Typed => {
                        if self.frames.as_ref().map_or(0, Vec::len) >= 128 {
                            return Err(invalid("IFC value exceeds its declared depth").into());
                        }
                        let (name, input) = if matches!(kind, Kind::Aggregate) {
                            let DslValue::Array(input) = self.payload_mut(payload) else { return Err(invalid("IFC aggregate requires an array").into()) };
                            (None, std::mem::take(input))
                        } else {
                            let object = fields(self.payload(payload)?)?;
                            if object.len() != 2 {
                                return Err(invalid("IFC typed value has unknown or repeated fields").into());
                            }
                            let name = field(object, "name")?;
                            let items = field(object, "items")?;
                            if !matches!(&object[name].1, DslValue::String(_)) || !matches!(&object[items].1, DslValue::Array(_)) {
                                return Err(invalid("IFC typed value requires name text and item array").into());
                            }
                            let DslValue::Object(object) = self.payload_mut(payload) else { unreachable!() };
                            let DslValue::String(name) = std::mem::replace(&mut object[name].1, DslValue::Null) else { unreachable!() };
                            let DslValue::Array(input) = std::mem::replace(&mut object[items].1, DslValue::Null) else { unreachable!() };
                            (Some(name), input)
                        };
                        self.frame = Some(Frame { name, input, values: Vec::new(), next: 0 });
                        self.retired_input = self.current.take();
                        self.payload_kind = None;
                        self.phase = if self.frames.is_none() { Phase::Stack } else { Phase::Allocate };
                        progress.copied_bytes = size_of::<Frame>() + size_of::<DslValue>();
                        None
                    }
                };
                if let Some(value) = scalar {
                    self.finish_scalar(value);
                    progress.copied_bytes = size_of::<IfcValue>() + size_of::<DslValue>();
                }
            }
            Phase::Allocate => {
                let frame = self.frame.as_mut().ok_or_else(|| invalid("IFC collection frame missing"))?;
                frame.values.try_reserve_exact(frame.input.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC native collection allocation failed"))?;
                progress.retained_capacity_bytes = frame.values.capacity() * size_of::<IfcValue>();
                self.frames.as_mut().unwrap().push(self.frame.take().unwrap());
                self.phase = Phase::Children;
                progress.copied_bytes = size_of::<Frame>();
            }
            Phase::Children => {
                let stack = self.frames.as_mut().unwrap();
                let frame = stack.last_mut().ok_or_else(|| invalid("IFC collection parent missing"))?;
                if frame.next < frame.input.len() {
                    self.current = Some(std::mem::replace(&mut frame.input[frame.next], DslValue::Null));
                    frame.next += 1;
                    self.phase = Phase::Node;
                    progress.copied_bytes = size_of::<DslValue>();
                } else {
                    let mut frame = stack.pop().unwrap();
                    let items = std::mem::take(&mut frame.values);
                    self.output = Some(match frame.name.take() {
                        Some(name) => IfcValue::TypedValue(IfcTypedValue { name, items }),
                        None => IfcValue::Aggregate(items),
                    });
                    self.retired_frame = Some(frame);
                    self.phase = Phase::Deliver;
                    progress.copied_bytes = size_of::<Frame>() + size_of::<IfcValue>();
                }
            }
            Phase::Deliver => {
                if let Some(parent) = self.frames.as_mut().and_then(|frames| frames.last_mut()) {
                    if parent.values.len() == parent.values.capacity() {
                        return Err(invalid("IFC native receiving capacity was not funded").into());
                    }
                    parent.values.push(self.output.take().ok_or_else(|| invalid("IFC child output missing"))?);
                    self.phase = Phase::Children;
                    progress.copied_bytes = size_of::<IfcValue>();
                } else {
                    self.phase = Phase::Ready;
                }
            }
            Phase::Binary => {
                let DslValue::String(text) = self.payload(self.binary_payload)? else { unreachable!() };
                let length = text.len();
                let end = (self.binary_offset + 64).min(length);
                if !text.as_bytes()[self.binary_offset..end].iter().all(|byte| byte.is_ascii_digit() || (b'A'..=b'F').contains(byte)) {
                    return Err(invalid("Invalid IFC BINARY digit").into());
                }
                self.binary_offset = end;
                if end == length {
                    let DslValue::String(text) = std::mem::replace(self.payload_mut(self.binary_payload), DslValue::Null) else { unreachable!() };
                    self.finish_scalar(IfcValue::Binary(text));
                    progress.copied_bytes = size_of::<IfcValue>() + size_of::<DslValue>();
                }
            }
            Phase::Ready => return Ok(true),
            Phase::Spent => return Err(invalid("IFC receiver output was already transferred").into()),
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        Ok(matches!(self.phase, Phase::Ready))
    }
    pub fn take(&mut self, output: &mut Option<IfcValue>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC value transfer cancelled").into());
        }
        if output.is_some() || !matches!(self.phase, Phase::Ready) {
            return Err(invalid("IFC receiving output is not transferable").into());
        }
        let bytes = size_of::<IfcValue>();
        if !fits(cx.retained_grant(), RetirementDemand { copy_bytes: bytes, depth: 1, ..Default::default() }) || cx.should_yield() {
            return Ok(false);
        }
        *output = self.output.take();
        self.phase = Phase::Spent;
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(true)
    }
    pub fn begin_close(&mut self) {
        self.closing = true;
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.current.is_none() && self.frames.is_none() && self.frame.is_none() && self.output.is_none() && self.retired_input.is_none() && self.retired_frame.is_none() && self.retirement.is_empty()
    }
    pub fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.retired_input.is_some() || self.retired_frame.is_some() || !self.retirement.is_empty() {
            return self.retire_demands();
        }
        if self.current.is_some() {
            return self.retirement.next_owner_demands::<DslValue>(true, &None, 4096);
        }
        if self.output.is_some() {
            return self.retirement.next_owner_demands::<IfcValue>(true, &None, 4096);
        }
        if self.frame.is_some() {
            return self.retirement.next_owner_demands::<Frame>(true, &None, 4096);
        }
        self.retirement.next_owner_demands::<Vec<Frame>>(self.frames.is_some(), &None, 4096)
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing {
            return Err(invalid("IFC receiver close was not initiated"));
        }
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if self.retired_input.is_some() || self.retired_frame.is_some() || !self.retirement.is_empty() {
            return self.retire_step(grant);
        }
        if self.current.is_some() {
            return self.retirement.begin_granted(&mut self.current, grant)?.ok_or_else(|| invalid("IFC input retirement omitted its owner"));
        }
        if self.output.is_some() {
            return self.retirement.begin_granted(&mut self.output, grant)?.ok_or_else(|| invalid("IFC output retirement omitted its owner"));
        }
        if self.frame.is_some() {
            return self.retirement.begin_granted(&mut self.frame, grant)?.ok_or_else(|| invalid("IFC frame retirement omitted its owner"));
        }
        self.retirement.begin_granted(&mut self.frames, grant)?.ok_or_else(|| invalid("IFC stack retirement omitted its owner"))
    }
}
