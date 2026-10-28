//! 📨️ Caller-authored required references cross the original owned decimal receiver.
use super::*;
use semio_framework_value::retained_clone::{RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;
pub(super) const INPUT: usize = 1usize << (usize::BITS - 1);
pub(super) fn input_ordinal(value: usize) -> Option<usize> {
    (value != usize::MAX && value & INPUT != 0).then_some(value & !INPUT)
}
pub(super) fn request(value: &DslValue) -> Option<(&str, Option<&[DslValue]>)> {
    let DslValue::Object(fields) = value else { return None };
    match fields.as_slice() {
        [(key, DslValue::String(name))] if key == "$ifcType" => Some((name, None)),
        [(a, DslValue::String(name)), (b, DslValue::Array(rows))] if a == "$ifcType" && b == "$ifcInputs" => Some((name, Some(rows))),
        [(a, DslValue::Array(rows)), (b, DslValue::String(name))] if a == "$ifcInputs" && b == "$ifcType" => Some((name, Some(rows))),
        _ => None,
    }
}
pub(super) fn request_name_mut(value: &mut DslValue) -> Option<&mut String> {
    request(value)?;
    let DslValue::Object(fields) = value else { return None };
    let (_, DslValue::String(name)) = fields.iter_mut().find(|(key, _)| key == "$ifcType")? else { return None };
    Some(name)
}
pub(super) fn rows(event: &editing::SnapshotEditEvent) -> Option<&[DslValue]> {
    let editing::SnapshotEditEvent::SetValue { value, .. } = event else { return None };
    request(value)?.1
}
fn payload_mut(event: &mut editing::SnapshotEditEvent, index: usize) -> Option<&mut DslValue> {
    let editing::SnapshotEditEvent::SetValue { value: DslValue::Object(fields), .. } = event else { return None };
    let (_, DslValue::Array(rows)) = fields.iter_mut().find(|(key, _)| key == "$ifcInputs")? else { return None };
    let DslValue::Object(fields) = rows.get_mut(index)? else { return None };
    fields.iter_mut().find(|(key, _)| key == "value").map(|(_, value)| value)
}
fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
pub(super) fn typed_constructor(value:&DslValue)->Option<(&str,&[DslValue])>{let DslValue::Object(fields)=value else{return None};if fields.len()!=2{return None;}let name=fields.iter().find(|(key,_)|key=="name")?.1.as_str()?;let DslValue::Array(items)=&fields.iter().find(|(key,_)|key=="items")?.1 else{return None};if items.len()>1{return None;}if let Some(child)=items.first(){let DslValue::Object(fields)=child else{return None};if fields.len()!=2{return None;}let kind=fields.iter().find(|(key,_)|key=="kind")?.1.as_str()?;let value=&fields.iter().find(|(key,_)|key=="value")?.1;if !match(kind,value){("string"|"integer"|"enum"|"binary",DslValue::String(_))|("aggregate",DslValue::Array(_))=>true,("real",DslValue::Object(bits))=>matches!(bits.as_slice(),[(key,DslValue::String(_))]if key=="bits"),_=>false}{return None;}}Some((name,items))}
pub(super) fn set_members(value:&DslValue)->Option<&[DslValue]>{let(_,items)=typed_constructor(value)?;let child=items.first()?;if child.get("kind")?.as_str()!=Some("aggregate"){return None;}let DslValue::Array(values)=child.get("value")? else{return None};Some(values)}
pub(super) fn reference_text(value:&DslValue)->Option<&str>{let DslValue::Object(fields)=value else{return None};if fields.len()!=2||fields.iter().find(|(key,_)|key=="kind")?.1.as_str()!=Some("reference"){return None;}fields.iter().find(|(key,_)|key=="value")?.1.as_str()}
pub(super) fn real_word(value:&DslValue)->Option<&str>{let DslValue::Object(fields)=value else{return None};if fields.len()!=2||fields.iter().find(|(key,_)|key=="kind")?.1.as_str()!=Some("real"){return None;}let DslValue::Object(bits)=&fields.iter().find(|(key,_)|key=="value")?.1 else{return None};match bits.as_slice(){[(key,DslValue::String(word))]if key=="bits"=>Some(word),_=>None}}
pub(super) fn integer_word(value:&DslValue)->Option<&str>{let DslValue::Object(fields)=value else{return None};if fields.len()!=2||fields.iter().find(|(key,_)|key=="kind")?.1.as_str()!=Some("integer"){return None;}fields.iter().find(|(key,_)|key=="value")?.1.as_str()}
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Input {
    pub(super) index: usize,
    pub(super) value: Option<IfcValue>,
}
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub(super) struct State {
    phase: u8,
    next: usize,
    index: usize,
    previous: Option<usize>,
    member: usize,
    offset: usize,
    closing: bool,
    array:bool,
    integer:bool,
    typed:bool,
}
pub(super) struct Inputs {
    state: State,
    values: Option<Vec<Input>>,
    raw: Option<DslValue>,
    value: Option<IfcValue>,
    receiver: IfcValueReceiver,
    retirement: RetainedCloneClose,
    kind: super::type_input_kind::Cursor,
    member_kind: super::type_input_kind::Cursor,
    leaves: super::type_input_select::Cursor,
    leaf: Option<&'static str>,
}
impl Default for Inputs {
    fn default() -> Self {
        Self { state: Default::default(), values: None, raw: None, value: None, receiver: IfcValueReceiver::new(IfcValueReceiveMode::Decimal), retirement: Default::default(), kind: Default::default(), member_kind:Default::default(),leaves:Default::default(),leaf:None }
    }
}
impl Inputs {
    fn base() -> usize {
        4 * size_of::<State>() + 2 * size_of::<Input>() + 2 * size_of::<DslValue>() + 2 * size_of::<IfcValue>() + size_of::<IfcValueReceiver>() + 2*size_of::<super::type_input_kind::Cursor>() + 2*size_of::<super::type_input_select::Cursor>() + 256
    }
    pub(super) fn values(&self) -> Option<&[Input]> {
        self.values.as_deref()
    }
    pub(super) fn argument(&self, ordinal: usize) -> Option<&IfcValue> {
        self.values.as_ref()?.get(ordinal)?.value.as_ref()
    }
    pub(super) fn take_argument(&mut self, ordinal: usize) -> Option<IfcValue> {
        self.values.as_mut()?.get_mut(ordinal)?.value.take()
    }
    pub(super) fn demands(&self, event: &editing::SnapshotEditEvent, body: usize) -> Result<RetirementDemand, ValueError> {
        let mut demand = RetirementDemand { copy_bytes: Self::base(), depth: 1, ..Default::default() };
        if self.state.phase == 0 {
            demand.capacity_bytes = rows(event).ok_or_else(|| invalid("IFC type inputs missing"))?.len().checked_mul(size_of::<Input>()).ok_or_else(|| invalid("IFC type input capacity overflow"))?;
        }
        let child = match self.state.phase {
            2 => Some(RetirementDemand { copy_bytes: size_of::<DslValue>(), depth: 1, ..Default::default() }),
            3 => Some(self.receiver.demands()?),
            4 => Some(RetirementDemand { copy_bytes: size_of::<IfcValue>(), depth: 1, ..Default::default() }),
            5 if !self.receiver.terminal_is_empty() => Some(self.receiver.close_demands()?),
            _ => None,
        };
        if let Some(child) = child {
            demand.copy_bytes = demand.copy_bytes.checked_add(child.copy_bytes).ok_or_else(|| invalid("IFC type input copy overflow"))?;
            demand.capacity_bytes = child.capacity_bytes;
            demand.release_bytes = child.release_bytes;
            demand.depth = demand.depth.max(child.depth);
        }
        let _ = body;
        Ok(demand)
    }
    pub(super) fn advance<C: IfcDetailsProfile>(&mut self, event: &mut editing::SnapshotEditEvent, name: &str, count: usize, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if self.state.closing {
            return Err(invalid("IFC type input receiver closed").into());
        }
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC type input receiving cancelled").into());
        }
        let demand = self.demands(event, cx.retained_grant().maximum_copy_bytes)?;
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) {
            return Ok(false);
        }
        if self.state.phase == 7 {
            return Ok(true);
        }
        cx.consume_retained(RetainedCloneProgress { copied_bytes: Self::base(), ..Default::default() })?;
        match self.state.phase {
            2 => {
                if self.receiver.receive_owned(&mut self.raw, cx)? {
                    self.state.phase = 3;
                }
                return Ok(false);
            }
            3 => {
                if self.receiver.advance(cx)? {
                    self.state.phase = 4;
                }
                return Ok(false);
            }
            4 => {
                if self.receiver.take(&mut self.value, cx)? {
                    self.receiver.begin_close();
                    self.state.phase = 5;
                }
                return Ok(false);
            }
            5 if !self.receiver.terminal_is_empty() => {
                let grant = RetainedCloneGrant { maximum_copy_bytes: cx.retained_grant().maximum_copy_bytes.min(4096), ..cx.retained_grant() };
                let step = self.receiver.close_step(grant).map_err(|error| semio_framework_job::OriginalTurnFailure::debit_original(error, cx))?;
                cx.consume_retained(step.progress())?;
                cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                return Ok(false);
            }
            0 => {
                let length = rows(event).ok_or_else(|| invalid("IFC type inputs missing"))?.len();
                if length > count || count >= INPUT {
                    return Err(invalid("IFC type input count exceeds target").into());
                }
                let values = self.values.get_or_insert_with(Vec::new);
                values.try_reserve_exact(length).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC type input backing allocation failed"))?;
                cx.consume_retained(RetainedCloneProgress { retained_capacity_bytes: values.capacity() * size_of::<Input>(), ..Default::default() })?;
                self.state.phase = 1;
            }
            1 => {
                let rows = rows(event).ok_or_else(|| invalid("IFC type inputs changed"))?;
                let Some(row) = rows.get(self.state.next) else {
                    self.state.phase = 7;
                    cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                    cx.consume_fuel(1);
                    return Ok(true);
                };
                let DslValue::Object(fields) = row else {
                    return Err(invalid("IFC type input requires closed row").into());
                };
                if fields.len() != 2 {
                    return Err(invalid("IFC type input has excess fields").into());
                }
                let index = fields.iter().find(|(key, _)| key == "index").and_then(|(_, value)| value.as_str()).ok_or_else(|| invalid("IFC type input index missing"))?;
                if index.len() > 20 || index.is_empty() || !index.bytes().all(|byte| byte.is_ascii_digit()) || index.len() > 1 && index.starts_with('0') {
                    return Err(invalid("IFC type input index requires canonical decimal").into());
                }
                let index = index.parse::<usize>().map_err(|_| invalid("IFC type input index outside usize"))?;
                if index >= count || self.state.previous.is_some_and(|previous| previous >= index) {
                    return Err(invalid("IFC type input indices must be ascending unique target slots").into());
                }
                let attribute = C::normalization_attribute(name, index, true).map_err(|_| invalid("IFC type input declaration missing"))?.ok_or_else(|| invalid("IFC type input attribute missing"))?;
                if attribute.optional || attribute.derived {
                    return Err(invalid("IFC type input requires an actual required target slot").into());
                }
                self.state.index = index;
                self.kind.clear();
                self.state.phase = 8;
            }
            8 => {
                let attribute = C::normalization_attribute(name, self.state.index, true).map_err(|_| invalid("IFC type input declaration missing"))?.ok_or_else(|| invalid("IFC type input attribute missing"))?;
                if let Some(kind) = self.kind.advance::<C>(attribute.type_expr)? {
                    let DslValue::Object(fields) = rows(event).and_then(|rows| rows.get(self.state.next)).ok_or_else(|| invalid("IFC input row disappeared"))? else { return Err(invalid("IFC type input requires closed row").into()) };
                    let DslValue::Object(value) = fields.iter().find(|(key, _)| key == "value").map(|(_, value)| value).ok_or_else(|| invalid("IFC type input value missing"))? else {
                        return Err(invalid("IFC type input requires tagged payload").into());
                    };
                    let actual = value.iter().find(|(key, _)| key == "kind").and_then(|(_, value)| value.as_str());
                    let accepted = if kind == super::type_input_kind::Kind::Number { matches!(actual, Some("integer" | "real")) } else { actual == Some(kind.tag()) || kind==super::type_input_kind::Kind::Select&&self.kind.reference_member().is_some()&&actual==Some("reference") };
                    let payload = value.iter().find(|(key, _)| key == "value").map(|(_, value)| value);
                    let shape = match (actual, payload) {
                        (Some("aggregate"),Some(DslValue::Array(items)))=>items.len()<semio_framework_plugin::action_staging::STAGED_FORM_BODY_LIMITS.content,
                        (Some("typedValue"),Some(value))=>typed_constructor(value).is_some_and(|(_,items)|items.len()==1),
                        (Some("real"), Some(DslValue::Object(words))) => matches!(words.as_slice(), [(key,DslValue::String(_))] if key == "bits"),
                        (_, Some(DslValue::String(_))) => actual != Some("real"),
                        _ => false,
                    };
                    if value.len() != 2 || !accepted || !shape {
                        return Err(invalid("IFC type input requires exact declared command payload").into());
                    }
                    self.state.member = 0;
                    self.state.offset = 0;
                    self.member_kind.clear();
                    self.leaves.clear();
                    self.leaf=None;
                    self.state.typed=kind==super::type_input_kind::Kind::Select&&actual==Some("typedValue");
                    self.state.phase = if kind == super::type_input_kind::Kind::Enum { 9 } else if kind == super::type_input_kind::Kind::Binary { 11 } else if kind==super::type_input_kind::Kind::List{12}else if kind==super::type_input_kind::Kind::Select&&actual==Some("typedValue"){13}else{10};
                }
            }
            9 => {
                let payload = rows(event).and_then(|rows| rows.get(self.state.next)).and_then(|row| row.get("value")).and_then(|value| value.get("value")).and_then(DslValue::as_str).ok_or_else(|| invalid("IFC enumeration original text missing"))?;
                let domain = self.kind.domain().ok_or_else(|| invalid("IFC enumeration declaration missing"))?;
                let member = super::type_input_kind::Cursor::member::<C>(domain, self.state.member).ok_or_else(|| invalid("IFC required input is not an exact declared enumeration member"))?;
                if payload.len() != member.len() {
                    self.state.member += 1;
                    self.state.offset = 0;
                } else if self.state.offset == member.len() {
                    self.state.phase = 10;
                } else {
                    let end = member.len().min(self.state.offset + 64);
                    if payload.as_bytes()[self.state.offset..end] == member.as_bytes()[self.state.offset..end] {
                        self.state.offset = end;
                    } else {
                        self.state.member += 1;
                        self.state.offset = 0;
                    }
                }
            }
            10 => {
                self.raw = Some(std::mem::replace(payload_mut(event, self.state.next).ok_or_else(|| invalid("IFC type input donor missing"))?, DslValue::Null));
                self.state.phase = 2;
            }
            13=>{
                let value=rows(event).and_then(|rows|rows.get(self.state.next)).and_then(|row|row.get("value")).and_then(|value|value.get("value")).ok_or_else(||invalid("IFC SELECT original constructor missing"))?;
                let(name,_)=typed_constructor(value).ok_or_else(||invalid("IFC SELECT requires closed constructor"))?;
                if let Some(member)=self.leaf{if name.len()!=member.len(){self.leaf=None;self.state.offset=0;}else if self.state.offset==member.len(){self.state.phase=14;}else{let end=(self.state.offset+64).min(member.len());if name.as_bytes()[self.state.offset..end]==member.as_bytes()[self.state.offset..end]{self.state.offset=end;}else{self.leaf=None;self.state.offset=0;}}}else{match self.leaves.advance::<C>(self.kind.domain().ok_or_else(||invalid("IFC SELECT declaration missing"))?)?{super::type_input_select::Step::Pending=>{},super::type_input_select::Step::Complete{..}=>return Err(invalid("IFC SELECT tag is not a declared leaf").into()),super::type_input_select::Step::Leaf{name:member,..}=>self.leaf=Some(member)}}
            }
            14=>{
                let value=rows(event).and_then(|rows|rows.get(self.state.next)).and_then(|row|row.get("value")).and_then(|value|value.get("value")).ok_or_else(||invalid("IFC SELECT original constructor missing"))?;let(name,items)=typed_constructor(value).ok_or_else(||invalid("IFC SELECT requires closed constructor"))?;
                let expression=C::defined_type_expression(name).ok_or_else(||invalid("IFC SELECT member alias unavailable"))?;if let Some(kind)=self.member_kind.advance_selected::<C>(expression)?{if !matches!(kind,super::type_input_kind::Kind::String|super::type_input_kind::Kind::Integer|super::type_input_kind::Kind::Real|super::type_input_kind::Kind::Number|super::type_input_kind::Kind::Enum|super::type_input_kind::Kind::Binary|super::type_input_kind::Kind::List|super::type_input_kind::Kind::Set|super::type_input_kind::Kind::Array)||if kind==super::type_input_kind::Kind::Number{!matches!(items.first().and_then(|value|value.get("kind")).and_then(DslValue::as_str),Some("integer"|"real"))}else{items.first().and_then(|value|value.get("kind")).and_then(DslValue::as_str)!=Some(kind.tag())}{return Err(invalid("IFC SELECT child does not match declared member").into());}self.state.member=0;self.state.array=kind==super::type_input_kind::Kind::Array;self.state.integer=kind==super::type_input_kind::Kind::List;self.state.phase=if matches!(kind,super::type_input_kind::Kind::Set|super::type_input_kind::Kind::Array|super::type_input_kind::Kind::List){15}else if kind==super::type_input_kind::Kind::Binary{11}else{10};}
            }
            15=>{
                let constructor=rows(event).and_then(|rows|rows.get(self.state.next)).and_then(|row|row.get("value")).and_then(|value|value.get("value")).ok_or_else(||invalid("IFC SET original constructor missing"))?;let values=set_members(constructor).ok_or_else(||invalid("IFC SET requires closed aggregate"))?;if let Some(value)=values.get(self.state.member){if self.state.integer{integer_word(value).ok_or_else(||invalid("IFC LIST member requires closed INTEGER"))?;}else if self.state.array{real_word(value).ok_or_else(||invalid("IFC ARRAY member requires closed REAL"))?;}else{reference_text(value).ok_or_else(||invalid("IFC SET member requires closed reference"))?;}self.state.member+=1;}else{self.state.phase=10;}
            }
            11 => {
                let payload=rows(event).and_then(|rows|rows.get(self.state.next)).and_then(|row|row.get("value")).and_then(|value|value.get("value")).ok_or_else(||invalid("IFC BINARY original payload missing"))?;let text=if self.state.typed{typed_constructor(payload).and_then(|(_,items)|items.first()).and_then(|child|child.get("value")).and_then(DslValue::as_str)}else{payload.as_str()}.ok_or_else(||invalid("IFC BINARY original text missing"))?;
                if text.is_empty() { return Err(invalid("IFC BINARY encoding missing").into()); }
                let end = text.len().min(self.state.offset + 64);
                for (index, byte) in text.as_bytes()[self.state.offset..end].iter().copied().enumerate() {
                    if self.state.offset + index == 0 && !matches!(byte,b'0'..=b'3') || self.state.offset + index > 0 && !matches!(byte,b'0'..=b'9'|b'A'..=b'F') { return Err(invalid("IFC BINARY requires exact canonical hexadecimal").into()); }
                }
                self.state.offset = end;
                if end == text.len() {
                    if text.len() == 1 && text != "0" || text.len() > 1 && u8::from_str_radix(&text[1..2],16).unwrap_or(255) >= 1u8 << (4-(text.as_bytes()[0]-b'0')) { return Err(invalid("IFC BINARY padding outside declared bit count").into()); }
                    self.state.phase = 10;
                }
            }
            12=>{
                let DslValue::Array(items)=rows(event).and_then(|rows|rows.get(self.state.next)).and_then(|row|row.get("value")).and_then(|value|value.get("value")).ok_or_else(||invalid("IFC LIST original members missing"))? else{return Err(invalid("IFC LIST requires closed members").into());};
                if let Some(item)=items.get(self.state.member){let DslValue::Object(fields)=item else{return Err(invalid("IFC LIST member requires typed real").into());};let kind=fields.iter().find(|(key,_)|key=="kind").and_then(|(_,value)|value.as_str());let bits=fields.iter().find(|(key,_)|key=="value").map(|(_,value)|value);if fields.len()!=2||kind!=Some("real")||!matches!(bits,Some(DslValue::Object(words))if matches!(words.as_slice(),[(key,DslValue::String(_))]if key=="bits")){return Err(invalid("IFC LIST member requires closed real word").into());}self.state.member+=1;}else{self.state.phase=10;}
            }
            5 => {
                self.receiver = IfcValueReceiver::new(IfcValueReceiveMode::Decimal);
                self.state.phase = 6;
            }
            6 => {
                self.values.as_mut().ok_or_else(|| invalid("IFC type input owner missing"))?.push(Input { index: self.state.index, value: self.value.take() });
                self.state.previous = Some(self.state.index);
                self.state.next += 1;
                self.state.phase = 1;
            }
            _ => return Err(invalid("IFC type input phase invalid").into()),
        }
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(false)
    }
    pub(super) fn begin_close(&mut self) {
        self.state.closing = true;
        self.receiver.begin_close();
    }
    pub(super) fn terminal_is_empty(&self) -> bool {
        self.values.is_none() && self.raw.is_none() && self.value.is_none() && self.receiver.terminal_is_empty() && self.retirement.is_empty()
    }
    pub(super) fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.receiver.terminal_is_empty() {
            return self.receiver.close_demands();
        }
        if !self.retirement.is_empty() {
            return self.retirement.next_owner_demands::<DslValue>(false, &None, body);
        }
        if self.raw.is_some() {
            return self.retirement.next_owner_demands::<DslValue>(true, &None, body);
        }
        if self.value.is_some() {
            return self.retirement.next_owner_demands::<IfcValue>(true, &None, body);
        }
        self.retirement.next_owner_demands::<Vec<Input>>(self.values.is_some(), &None, body)
    }
    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.state.closing {
            return Err(invalid("IFC type input close not started"));
        }
        if !self.receiver.terminal_is_empty() {
            return self.receiver.close_step(grant);
        }
        if !self.retirement.is_empty() {
            return self.retirement.step_granted(grant);
        }
        if self.raw.is_some() {
            return self.retirement.begin_granted(&mut self.raw, grant)?.ok_or_else(|| invalid("IFC type input raw retirement missing"));
        }
        if self.value.is_some() {
            return self.retirement.begin_granted(&mut self.value, grant)?.ok_or_else(|| invalid("IFC type input value retirement missing"));
        }
        if self.values.is_some() {
            return self.retirement.begin_granted(&mut self.values, grant)?.ok_or_else(|| invalid("IFC type input roster retirement missing"));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    #[cfg(test)]
    pub(super) fn metadata(&self) -> (State, usize, usize, bool, bool) {
        (self.state, self.values.as_ref().map_or(0, Vec::len), self.values.as_ref().map_or(0, Vec::capacity), self.raw.is_some(), self.value.is_some())
    }
}
