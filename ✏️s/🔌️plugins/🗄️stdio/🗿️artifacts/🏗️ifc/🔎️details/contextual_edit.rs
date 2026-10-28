//! 📥️ Small source-choice requests receive the actual native value through original retained copying.
use super::*;
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;
#[derive(Clone, Copy)]
struct Request {
    definition: u64,
    choice: usize,
    list: bool,
}
pub(super) struct EnumerationEdit {
    query: IfcEnumerationQuery,
    cursor: <IfcValue as RetainedClone>::Cursor,
    received: Option<IfcValue>,
    retirement: RetainedCloneClose,
    request: Option<Request>,
    binding: Option<(usize, usize)>,
    phase: u8,
    closing: bool,
}
impl Default for EnumerationEdit {
    fn default() -> Self {
        Self { query: Default::default(), cursor: IfcValue::retained_clone_cursor(), received: None, retirement: Default::default(), request: None, binding: None, phase: 0, closing: false }
    }
}
fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
fn request(event: &editing::SnapshotEditEvent) -> Result<Option<Request>, ValueError> {
    let value = match event {
        editing::SnapshotEditEvent::SetValue { value, .. } | editing::SnapshotEditEvent::InsertValue { value, .. } => value,
        _ => return Ok(None),
    };
    let Some(payload) = value.get("$ifcEnumerationChoice") else {
        return Ok(None);
    };
    let (DslValue::Object(outer), DslValue::Object(fields)) = (value, payload) else {
        return Err(invalid("IFC enumeration request must be a closed source selection"));
    };
    if outer.len() != 1 || fields.len() != 3 {
        return Err(invalid("IFC enumeration request has unexpected fields"));
    }
    let definition = payload.get("definition").and_then(DslValue::as_u64).filter(|id| *id > 0).ok_or_else(|| invalid("IFC enumeration request definition must be an exact positive identity"))?;
    let choice = payload.get("choice").and_then(DslValue::as_str).ok_or_else(|| invalid("IFC enumeration request choice must be canonical text"))?;
    if choice.len() > 20 || choice.is_empty() || choice.len() > 1 && choice.starts_with('0') || !choice.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(invalid("IFC enumeration request choice ordinal is invalid"));
    }
    let choice = choice.parse().map_err(|_| invalid("IFC enumeration request choice ordinal exceeds native extent"))?;
    let Some(DslValue::Bool(list)) = payload.get("list") else {
        return Err(invalid("IFC enumeration request requires its complete item or list mode"));
    };
    Ok(Some(Request { definition, choice, list: *list }))
}
impl EnumerationEdit {
    pub(super) fn complete(&self) -> bool {
        self.phase == 5
    }
    pub(super) fn receives_source_choice(&self) -> bool {
        self.request.is_some()
    }
    pub(super) fn demands(&self, source: Option<&wire::SnapshotSource>, snapshot: &IfcSnapshot, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.phase == 1 {
            let mut demand = self.query.demands();
            demand.copy_bytes += size_of::<u8>();
            return Ok(demand);
        }
        if self.phase == 2 {
            let request = self.request.unwrap();
            let query = &self.query;
            let mut demand = self.cursor.advance_demands(
                source
                    .ok_or_else(|| invalid("IFC enumeration original source was not captured"))?
                    .borrow()
                    .project(request.choice, |snapshot| &query.result(snapshot, 0).expect("bound original enumeration").allowed.expect("admitted source enumeration")[request.choice]),
                body.saturating_sub(1).min(4096),
            )?;
            demand.copy_bytes += size_of::<u8>();
            return Ok(demand);
        }
        let _ = snapshot;
        Ok(RetirementDemand {
            copy_bytes: 4 * size_of::<IfcValue>() + size_of::<Request>() + size_of::<Option<(usize, usize)>>(),
            capacity_bytes: if self.phase == 4 && self.request.is_some_and(|request| request.list) { size_of::<IfcValue>() } else { 0 },
            depth: 1,
            ..Default::default()
        })
    }
    pub(super) fn advance<C: IfcDetailsProfile>(
        &mut self,
        event: &editing::SnapshotEditEvent,
        snapshot: &IfcSnapshot,
        address: &pointer::PathAdmission,
        source: Option<&wire::SnapshotSource>,
        value: &mut value_edit::IfcValueEdit,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC original enumeration receiving cancelled").into());
        }
        let binding = (event as *const _ as usize, snapshot as *const _ as usize);
        if self.binding.is_some_and(|original| original != binding) {
            return Err(invalid("IFC enumeration receiving original owner changed").into());
        }
        if self.closing {
            return Err(invalid("IFC enumeration receiving is closing").into());
        }
        let demand = self.demands(source, snapshot, cx.retained_grant().maximum_copy_bytes)?;
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) {
            return Ok(false);
        }
        if self.complete() {
            return Ok(true);
        }
        if self.phase == 0 {
            let received = request(event)?;
            if let Some(request) = received {
                let native = address.native_address(event).ok_or_else(|| invalid("IFC enumeration choice requires its actual native argument"))?;
                let count = address.field_count();
                if request.list {
                    if count != native.field || !matches!(event, editing::SnapshotEditEvent::SetValue { .. }) {
                        return Err(invalid("IFC enumeration singleton list must replace its whole attribute").into());
                    }
                } else if count != native.field + 2
                    || !address.field_is(
                        match event {
                            editing::SnapshotEditEvent::SetValue { path, .. } | editing::SnapshotEditEvent::InsertValue { path, .. } => path,
                            _ => unreachable!(),
                        },
                        native.field,
                        "value",
                    )
                {
                    return Err(invalid("IFC enumeration choice must address a complete list item").into());
                }
                self.phase = 1;
            } else {
                self.phase = 5;
            }
            self.binding = Some(binding);
            self.request = received;
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(self.complete());
        }
        let request = self.request.unwrap();
        let native = address.native_address(event).ok_or_else(|| invalid("IFC enumeration receiving source address is absent"))?;
        if self.phase == 1 {
            if self.query.advance_address::<C>(snapshot, contextual::IfcEnumerationAddress { entity: native.position, part: native.part, argument: Some(native.argument) }, 0, cx)? {
                let view = self.query.result(snapshot, 0)?;
                if view.enumeration != Some(request.definition) || view.allowed.and_then(|values| values.get(request.choice)).is_none() {
                    return Err(invalid("IFC enumeration choice is unavailable under its original source").into());
                }
                self.phase = 2;
                cx.consume_retained(RetainedCloneProgress { copied_bytes: size_of::<u8>(), ..Default::default() })?;
            }
            return Ok(false);
        }
        if self.phase == 2 {
            let query = &self.query;
            let step = self.cursor.advance(
                source.unwrap().borrow().project(request.choice, |snapshot| &query.result(snapshot, 0).expect("bound original enumeration").allowed.expect("admitted source enumeration")[request.choice]),
                cx.retained_grant().maximum_copy_bytes.saturating_sub(1).min(4096),
                RetainedCloneGrant { maximum_copy_bytes: cx.retained_grant().maximum_copy_bytes.saturating_sub(1), ..cx.retained_grant() },
            )?;
            cx.consume_retained(step.progress())?;
            cx.consume_fuel(step.progress().copied_items as u64);
            if matches!(step, RetainedCloneStep::Complete(_)) {
                self.phase = 3;
                cx.consume_retained(RetainedCloneProgress { copied_bytes: size_of::<u8>(), ..Default::default() })?;
            }
            return Ok(false);
        }
        if self.phase == 3 {
            self.received = self.cursor.take();
            self.phase = 4;
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(false);
        }
        if self.phase == 4 && request.list {
            let mut values = Vec::new();
            values.try_reserve_exact(1).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC enumeration singleton list allocation failed"))?;
            values.push(self.received.take().ok_or_else(|| invalid("IFC enumeration source value is absent"))?);
            self.received = Some(IfcValue::Aggregate(values));
            self.phase = 6;
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: size_of::<IfcValue>(), ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(false);
        }
        value.supply_native(&mut self.received)?;
        self.phase = 5;
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(true)
    }
    pub(super) fn begin_close(&mut self) {
        self.closing = true;
        self.query.begin_close();
        self.cursor.begin_close();
    }
    pub(super) fn terminal_is_empty(&self) -> bool {
        self.binding.is_none() && self.query.terminal_is_empty() && self.cursor.terminal_is_empty() && self.received.is_none() && self.retirement.is_empty()
    }
    pub(super) fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.cursor.terminal_is_empty() {
            return self.cursor.close_demands(body);
        }
        if !self.retirement.is_empty() || self.received.is_some() {
            return self.retirement.next_owner_demands::<IfcValue>(self.received.is_some(), &None, body);
        }
        if !self.query.terminal_is_empty() {
            return Ok(self.query.close_demands());
        }
        Ok(RetirementDemand { copy_bytes: size_of::<Option<(usize, usize)>>(), depth: 1, ..Default::default() })
    }
    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing {
            return Err(invalid("IFC enumeration receiving close was not started"));
        }
        let demand = self.close_demands(4096)?;
        if !retained_clone_grant_admits(grant, demand) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if !self.cursor.terminal_is_empty() {
            return self.cursor.close_step(grant);
        }
        if !self.retirement.is_empty() {
            return self.retirement.step_granted(grant);
        }
        if self.received.is_some() {
            return self.retirement.begin_granted(&mut self.received, grant)?.ok_or_else(|| invalid("IFC enumeration received owner is absent"));
        }
        if !self.query.terminal_is_empty() {
            return self.query.close_step(grant);
        }
        self.binding = None;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }
}
