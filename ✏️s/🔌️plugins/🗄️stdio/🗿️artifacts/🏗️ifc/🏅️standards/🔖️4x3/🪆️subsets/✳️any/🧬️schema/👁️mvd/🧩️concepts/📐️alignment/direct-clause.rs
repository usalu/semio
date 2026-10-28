//! 🛤️ A Direct clause retains local semantics while borrowing one original prepared population per turn.
use super::*;
use super::super::population::{IfcMvdPopulationPin, IfcMvdPopulationRequestPin, IfcMvdPopulationRequestView, IfcMvdPopulationView, IfcMvdPopulationLookup};
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ClauseState {
    authority: Option<semio_framework_job::OriginalSourceBinding>,
    phase: u8,
    target: usize,
    field: u8,
    layout: usize,
    position: usize,
    segment: usize,
    parameter: usize,
    curve: Option<u64>,
    role: u8,
    matches: usize,
    matched: usize,
    unknown: bool,
    owner: Option<Owner>,
    expression: Option<Expression>,
    status: Option<IfcAttributeQueryStatus>,
    source_invalid: bool,
    any_incomplete: bool,
    any_fail: bool,
    any_pass: bool,
    done: bool,
    donated: bool,
    diagnostic_code: Option<&'static str>,
    diagnostic_global: bool,
    diagnostic_pointer: bool,
    diagnostic_item: bool,
    diagnostic_step: u8,
    path_item: usize,
}
semio_framework_value::artifact_retire_leaf!(ClauseState);
#[derive(Default, semio_framework_value::RetireOwned)]
pub(super) struct IfcAlignmentDirectClause {
    report: Option<IfcAlignmentDirectReport>,
    diagnostic: Option<IfcConceptDiagnostic>,
    member: Membership,
    query: IfcAttributeQueryWork,
    number: PrimitiveZero,
    state: ClauseState,
    pin: Option<IfcMvdPopulationPin>,
    request_pin: Option<IfcMvdPopulationRequestPin>,
}
#[cfg(test)]
pub(super) type ClauseMetadata = (ClauseState, Option<IfcMvdPopulationPin>, Option<IfcMvdPopulationRequestPin>);
fn lookup(view: Option<&IfcMvdPopulationView<'_, IfcAlignmentDirectRequest>>, id: u64, cx: &semio_framework_job::StepContext<'_>) -> Result<IdMatch, ValueError> {
    Ok(match view.ok_or_else(|| invalid("mvd.direct.prepared-index"))?.lookup(id, cx)? { IfcMvdPopulationLookup::Missing => IdMatch::Missing, IfcMvdPopulationLookup::Unique(row) => IdMatch::Unique(row), IfcMvdPopulationLookup::Ambiguous => IdMatch::Ambiguous })
}
impl IfcAlignmentDirectClause {
    fn base() -> RetirementDemand {
        RetirementDemand { copy_bytes: 4 * size_of::<ClauseState>() + 4 * size_of::<IfcAlignmentDirectTargetReport>() + 2 * size_of::<IfcAttributeQueryWork>() + 2 * size_of::<PrimitiveZero>() + 2 * size_of::<Option<IfcMvdPopulationPin>>() + 2 * size_of::<Option<IfcMvdPopulationRequestPin>>() + 4 * 256, depth: 1, ..Default::default() }
    }
    fn diagnostic_path(&self) -> &'static [&'static str] {
        if self.state.diagnostic_global || self.state.diagnostic_code.is_some_and(|code| code.starts_with("mvd.direct.target.") || code == "mvd.direct.role.unknown" && self.state.role == 0) { &[] } else { path(self.state.field, self.state.layout) }
    }
    fn diagnostic_list(&self) -> &Vec<IfcConceptDiagnostic> {
        let report = self.report.as_ref().unwrap();
        if self.state.diagnostic_global { &report.diagnostics } else { &report.targets[self.state.target].diagnostics }
    }
    pub(super) fn demands(&self, request: &IfcAlignmentDirectRequest) -> Result<RetirementDemand, ValueError> {
        let mut demand = Self::base();
        let child = match self.state.phase {
            6 if self.state.target < request.targets.len() => {
                let values = &self.report.as_ref().unwrap().targets;
                vector_growth::<IfcAlignmentDirectTargetReport>(values.len(), values.capacity())?
            }
            60 | 61 => {
                let row = &request.targets[self.state.target];
                let length = if self.state.phase == 60 { row.segment.len() } else { row.kind.len() };
                RetirementDemand { copy_bytes: 2 * length, capacity_bytes: length, depth: 1, ..Default::default() }
            }
            8 => RetirementDemand { copy_bytes: self.member.copy_bound(), depth: 1, ..Default::default() },
            10 => self.query.demands(),
            12 => RetirementDemand { copy_bytes: self.number.copy_bound(), depth: 1, ..Default::default() },
            17 => {
                let length = decimal_len(self.state.curve.unwrap());
                RetirementDemand { copy_bytes: 2 * length, capacity_bytes: length, depth: 1, ..Default::default() }
            }
            20 => {
                let step = self.state.diagnostic_step;
                if step == 0 {
                    let list = self.diagnostic_list();
                    vector_growth::<IfcConceptDiagnostic>(list.len(), list.capacity())?
                } else {
                    let length = match step {
                        1 => self.diagnostic_path().len() * size_of::<String>(),
                        2 => self.diagnostic_path().get(self.state.path_item).map_or(0, |value| value.len()),
                        3 => {
                            if self.state.diagnostic_pointer {
                                pointer_len(self.state.owner, self.state.diagnostic_item)
                            } else {
                                0
                            }
                        }
                        _ => 0,
                    };
                    RetirementDemand { copy_bytes: 2 * length + 2 * size_of::<IfcConceptDiagnostic>(), capacity_bytes: length, depth: 1, ..Default::default() }
                }
            }
            _ => RetirementDemand::default(),
        };
        demand.copy_bytes = demand.copy_bytes.checked_add(child.copy_bytes).ok_or_else(|| invalid("mvd.direct.capacity"))?;
        demand.capacity_bytes = child.capacity_bytes;
        demand.release_bytes = child.release_bytes;
        demand.depth = demand.depth.max(child.depth);
        Ok(demand)
    }
    fn fail(&mut self, code: &'static str, status: &'static str, pointer: bool, item: bool) {
        self.report.as_mut().unwrap().targets[self.state.target].status = status;
        self.state.diagnostic_code = Some(code);
        self.state.diagnostic_global = false;
        self.state.diagnostic_pointer = pointer && self.state.owner.is_some();
        self.state.diagnostic_item = item;
        self.state.diagnostic_step = 0;
        self.state.path_item = 0;
        self.state.phase = 20;
    }
    fn field(&mut self, field: u8, position: usize) {
        self.state.field = field;
        self.state.position = position;
        self.state.owner = None;
        self.state.expression = None;
        self.state.status = None;
        self.state.phase = 9;
    }
    fn role(&mut self, role: u8, position: usize) {
        self.state.role = role;
        self.state.position = position;
        let expected = match role {
            0 => "IFCALIGNMENTSEGMENT",
            1..=3 => PARAMETERS[(role - 1) as usize],
            4 => "IFCPRODUCTDEFINITIONSHAPE",
            5 => "IFCSHAPEREPRESENTATION",
            _ => "IFCCURVESEGMENT",
        };
        self.member.start(position, expected);
        self.state.phase = 8;
    }
    pub(super) fn advance_prepared(&mut self, view: IfcMvdPopulationView<'_, IfcAlignmentDirectRequest>, epoch: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "io.cancelled").into()); }
        let pin = view.pin(cx)?;
        if self.request_pin.is_some() || pin.epoch() != epoch || self.pin.is_some_and(|pin| !pin.matches(&view, cx)) || self.state.authority.is_some_and(|old| !old.matches(cx)) || self.state.donated { return Err(invalid("mvd.direct.clause-source").into()); }
        let request = view.request(cx)?;
        let demand = self.demands(request)?;
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) { return Ok(false); }
        if self.state.done { return Ok(true); }
        if self.pin.is_none() { self.pin = Some(pin); }
        self.advance_admitted(Some(&view), request, epoch, cx)
    }
    pub(super) fn advance_unavailable(&mut self, view: IfcMvdPopulationRequestView<'_, IfcAlignmentDirectRequest>, epoch: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "io.cancelled").into()); }
        let pin = view.pin(cx)?;
        if self.pin.is_some() || pin.epoch() != epoch || view.status(cx)? != super::super::population::IfcMvdPopulationStatus::Unavailable || self.request_pin.is_some_and(|pin| !pin.matches(&view, cx)) || self.state.authority.is_some_and(|old| !old.matches(cx)) || self.state.donated { return Err(invalid("mvd.direct.clause-source").into()); }
        let request = view.request(cx)?;
        let demand = self.demands(request)?;
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) { return Ok(false); }
        if self.state.done { return Ok(true); }
        if self.request_pin.is_none() { self.request_pin = Some(pin); }
        self.advance_admitted(None, request, epoch, cx)
    }
    fn advance_admitted(&mut self, view: Option<&IfcMvdPopulationView<'_, IfcAlignmentDirectRequest>>, request: &IfcAlignmentDirectRequest, epoch: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "io.cancelled").into()); }
        if self.state.authority.is_some_and(|old| !old.matches(cx)) || self.state.donated { return Err(invalid("mvd.original-source").into()); }
        if self.state.done { return Ok(true); }
        let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: Self::base().copy_bytes, ..Default::default() };
        match self.state.phase {
            0 => { self.state.authority = Some(semio_framework_job::OriginalSourceBinding::borrow(cx)); self.report = Some(empty_report()); self.state.phase = 6; if view.is_none() { self.source_failure(); } },
            6 => {
                if self.state.target == request.targets.len() {
                    self.state.phase = 16;
                } else {
                    let rows = &mut self.report.as_mut().unwrap().targets;
                    let growth = vector_growth::<IfcAlignmentDirectTargetReport>(rows.len(), rows.capacity())?;
                    if growth.capacity_bytes > 0 {
                        reserve(rows, growth.capacity_bytes)?;
                        progress.copied_bytes += growth.copy_bytes;
                        progress.retained_capacity_bytes = rows.capacity() * size_of::<IfcAlignmentDirectTargetReport>();
                        progress.released_bytes = growth.release_bytes;
                    } else {
                        rows.push(IfcAlignmentDirectTargetReport { segment: String::new(), kind: String::new(), applicable: request.targets[self.state.target].applicable, status: "incomplete", diagnostics: vec![], curve: None });
                        self.state.phase = 60;
                    }
                    self.state.curve = None;
                }
            }
            60 | 61 => {
                let row = &request.targets[self.state.target];
                let value = owned(if self.state.phase == 60 { &row.segment } else { &row.kind })?;
                let length = value.len();
                let capacity = value.capacity();
                let target = &mut self.report.as_mut().unwrap().targets[self.state.target];
                if self.state.phase == 60 {
                    target.segment = value;
                    self.state.phase = 61;
                } else {
                    target.kind = value;
                    self.state.phase = 7;
                }
                progress.copied_bytes += 2 * length;
                progress.retained_capacity_bytes = capacity;
            }
            7 => {
                let input = &request.targets[self.state.target];
                self.state.layout = kind(&input.kind).unwrap();
                if !input.applicable {
                    self.report.as_mut().unwrap().targets[self.state.target].status = "not-applicable";
                    self.state.phase = 15;
                } else if self.state.source_invalid {
                    self.state.phase = 15;
                } else {
                    match lookup(view, positive_id(&input.segment)?, cx)? {
                        IdMatch::Unique(position) => {
                            self.state.segment = position;
                            self.state.field = 0;
                            self.state.owner = None;
                            self.state.matches = 0;
                            self.state.unknown = false;
                            self.role(0, position);
                        }
                        IdMatch::Missing => self.fail("mvd.direct.target.missing", "incomplete", false, false),
                        IdMatch::Ambiguous => self.fail("mvd.direct.target.ambiguous", "incomplete", false, false),
                    }
                }
            }
            8 => {
                let source = view.ok_or_else(|| invalid("mvd.direct.prepared-source"))?.snapshot(cx)?;
                if self.member.advance(source) {
                    let result = self.member.result();
                    match self.state.role {
                        0 => {
                            if result == Some(true) {
                                self.field(0, self.state.segment);
                            } else {
                                self.fail(if result.is_none() { "mvd.direct.role.unknown" } else { "mvd.direct.target.type" }, if result.is_none() { "incomplete" } else { "fail" }, false, false);
                            }
                        }
                        1..=3 => {
                            self.state.unknown |= result.is_none();
                            if result == Some(true) {
                                self.state.matches += 1;
                                self.state.matched = (self.state.role - 1) as usize;
                            }
                            if self.state.role < 3 {
                                self.role(self.state.role + 1, self.state.parameter);
                            } else if self.state.unknown || self.state.matches != 1 {
                                self.state.field = 0;
                                self.fail("mvd.direct.role.unknown", "incomplete", false, false);
                            } else if self.state.matched != self.state.layout {
                                self.state.field = 0;
                                self.fail("mvd.direct.parameter.type", "fail", false, false);
                            } else {
                                self.field(1, self.state.parameter);
                            }
                        }
                        4..=6 => {
                            if result != Some(true) {
                                self.fail(if result.is_none() { "mvd.direct.role.unknown" } else { "mvd.direct.path.unsupported" }, "incomplete", false, false);
                            } else if self.state.role == 4 {
                                self.field(3, self.state.position);
                            } else if self.state.role == 5 {
                                self.field(4, self.state.position);
                            } else {
                                self.state.curve = Some(source.entities[self.state.position].id);
                                self.state.phase = 17;
                            }
                        }
                        _ => return Err(invalid("mvd.direct.role").into()),
                    }
                }
                progress.copied_bytes += self.member.copy_bound();
            }
            9 => {
                self.query = Default::default();
                self.state.phase = 10;
            }
            10 => {
                progress.copied_items = 0;
                cx.consume_retained(progress)?;
                let source = view.ok_or_else(|| invalid("mvd.direct.prepared-source"))?.snapshot(cx)?;
                let name = field_name(self.state.field, self.state.layout);
                let owner = Some(field_owner(self.state.field, self.state.layout));
                if self.query.advance::<Ifc4x3AttributeMetadata, _>(source, self.state.position, name, owner, epoch, cx)? {
                    let result = self.query.completed::<Ifc4x3AttributeMetadata, _>(source, self.state.position, name, owner, epoch, cx)?.ok_or_else(|| invalid("mvd.direct.query"))?;
                    self.state.status = Some(result.status);
                    if let Some(first) = &result.matches[0] {
                        self.state.owner = Some(Owner { entity: first.address.entity, part: first.address.part.checked_sub(1), argument: first.address.index });
                        self.state.expression = Some(Expression(first.type_expr));
                    }
                    self.state.phase = 11;
                }
                return Ok(false);
            }
            11 => {
                let source = view.ok_or_else(|| invalid("mvd.direct.prepared-source"))?.snapshot(cx)?;
                let value = self.state.owner.and_then(|owner| owner.value(source));
                if self.state.status != Some(IfcAttributeQueryStatus::Unique) || value.is_none() {
                    self.fail(
                        match self.state.status {
                            Some(IfcAttributeQueryStatus::Unknown) => "mvd.direct.attribute.unknown",
                            Some(IfcAttributeQueryStatus::Ambiguous) => "mvd.direct.attribute.ambiguous",
                            _ => "mvd.direct.attribute.missing",
                        },
                        "incomplete",
                        true,
                        false,
                    );
                } else {
                    match self.state.field {
                        0 => {
                            let id = match value {
                                Some(IfcValue::Reference(id)) => Some(*id),
                                _ => None,
                            };
                            let resolved = id.map(|id| lookup(view, id, cx)).transpose()?;
                            match resolved {
                                Some(IdMatch::Unique(position)) => {
                                    self.state.parameter = position;
                                    self.state.matches = 0;
                                    self.state.unknown = false;
                                    self.role(1, position);
                                }
                                Some(IdMatch::Missing) => self.fail("mvd.direct.reference.missing", "incomplete", true, false),
                                Some(IdMatch::Ambiguous) => self.fail("mvd.direct.reference.ambiguous", "incomplete", true, false),
                                _ => self.fail("mvd.direct.attribute.shape", "incomplete", true, false),
                            }
                        }
                        1 => {
                            self.number.start(self.state.owner.unwrap(), self.state.expression.unwrap(), 0);
                            self.state.phase = 12;
                        }
                        2..=4 => {
                            let item = self.state.field >= 3;
                            let mut id = None;
                            let mut error = None;
                            let mut error_item = false;
                            if self.state.field == 2 {
                                match value {
                                    Some(IfcValue::Unset) => error = Some("mvd.direct.path.missing"),
                                    Some(IfcValue::Reference(value)) => id = Some(*value),
                                    _ => error = Some("mvd.direct.attribute.shape"),
                                }
                            } else {
                                match value {
                                    Some(IfcValue::Aggregate(items)) => {
                                        if items.len() != 1 {
                                            error = Some(if items.is_empty() { "mvd.direct.path.missing" } else { "mvd.direct.path.ambiguous" });
                                        } else if let IfcValue::Reference(value) = items[0] {
                                            id = Some(value);
                                        } else {
                                            error = Some("mvd.direct.attribute.shape");
                                            error_item = true;
                                        }
                                    }
                                    _ => error = Some("mvd.direct.attribute.shape"),
                                }
                            }
                            if let Some(code) = error {
                                self.fail(code, "incomplete", true, error_item);
                            } else {
                                match lookup(view, id.unwrap(), cx)? {
                                    IdMatch::Unique(position) => self.role(self.state.field + 2, position),
                                    IdMatch::Missing => self.fail("mvd.direct.reference.missing", "incomplete", true, item),
                                    IdMatch::Ambiguous => self.fail("mvd.direct.reference.ambiguous", "incomplete", true, item),
                                }
                            }
                        }
                        _ => {
                            let expression = match value {
                                Some(IfcValue::TypedValue(typed)) if typed.items.len() == 1 && (valid_name_equal(&typed.name, "IFCLENGTHMEASURE") || valid_name_equal(&typed.name, "IFCPARAMETERVALUE")) => {
                                    metadata::defined_type(&typed.name).and_then(|declaration| declaration.type_expr).map(Expression)
                                }
                                _ => None,
                            };
                            if expression.is_none() {
                                self.fail("mvd.direct.curve.shape", "incomplete", true, false);
                            } else {
                                self.state.expression = expression;
                                self.number.start(self.state.owner.unwrap(), expression.unwrap(), 1);
                                self.state.phase = 12;
                            }
                        }
                    }
                }
            }
            12 => {
                let source = view.ok_or_else(|| invalid("mvd.direct.prepared-source"))?.snapshot(cx)?;
                if self.number.advance(source) {
                    let curve = self.state.field == 5;
                    match self.number.result() {
                        None => self.fail(if curve { "mvd.direct.curve.shape" } else { "mvd.direct.business.shape" }, "incomplete", true, false),
                        Some(false) => self.fail(if curve { "mvd.direct.curve.nonzero" } else { "mvd.direct.business.nonzero" }, "fail", true, false),
                        Some(true) => {
                            if curve {
                                self.report.as_mut().unwrap().targets[self.state.target].status = "pass";
                                self.state.phase = 15;
                            } else {
                                self.field(2, self.state.segment);
                            }
                        }
                    }
                }
                progress.copied_bytes += self.number.copy_bound();
            }
            15 => {
                let status = self.report.as_ref().unwrap().targets[self.state.target].status;
                self.state.any_incomplete |= status == "incomplete";
                self.state.any_fail |= status == "fail";
                self.state.any_pass |= status == "pass";
                self.state.target += 1;
                self.state.phase = 6;
            }
            16 => {
                self.report.as_mut().unwrap().status = if self.state.source_invalid || self.state.any_incomplete {
                    "incomplete"
                } else if self.state.any_fail {
                    "fail"
                } else if self.state.any_pass {
                    "pass"
                } else {
                    "not-applicable"
                };
                self.state.done = true;
            }
            17 => {
                let value = decimal(self.state.curve.unwrap())?;
                progress.copied_bytes += 2 * value.len();
                progress.retained_capacity_bytes = value.capacity();
                self.report.as_mut().unwrap().targets[self.state.target].curve = Some(value);
                self.field(5, self.state.position);
            }
            20 => {
                self.diagnostic_step(&mut progress)?;
            }
            _ => return Err(invalid("mvd.direct.state").into()),
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        Ok(self.state.done)
    }
    fn source_failure(&mut self) {
        self.state.source_invalid = true;
        self.state.diagnostic_code = Some("mvd.direct.source");
        self.state.diagnostic_global = true;
        self.state.diagnostic_pointer = false;
        self.state.diagnostic_item = false;
        self.state.diagnostic_step = 0;
        self.state.path_item = 0;
        self.state.phase = 20;
    }
    fn diagnostic_step(&mut self, progress: &mut RetainedCloneProgress) -> Result<(), ValueError> {
        match self.state.diagnostic_step {
            0 => {
                let list = self.diagnostic_list();
                let growth = vector_growth::<IfcConceptDiagnostic>(list.len(), list.capacity())?;
                if growth.capacity_bytes > 0 {
                    let report = self.report.as_mut().unwrap();
                    let list = if self.state.diagnostic_global { &mut report.diagnostics } else { &mut report.targets[self.state.target].diagnostics };
                    reserve(list, growth.capacity_bytes)?;
                    progress.copied_bytes += growth.copy_bytes;
                    progress.retained_capacity_bytes = list.capacity() * size_of::<IfcConceptDiagnostic>();
                    progress.released_bytes = growth.release_bytes;
                } else {
                    let code = self.state.diagnostic_code.unwrap();
                    let (en, de) = labels(code);
                    self.diagnostic = Some(IfcConceptDiagnostic { code, path: vec![], pointer: None, label: IfcConceptLabel { en, de } });
                    self.state.diagnostic_step = 1;
                }
            }
            1 => {
                let count = self.diagnostic_path().len();
                let values = &mut self.diagnostic.as_mut().unwrap().path;
                if count > 0 {
                    values.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "mvd.direct.allocation"))?;
                    progress.retained_capacity_bytes = values.capacity() * size_of::<String>();
                    progress.copied_bytes += 2 * count * size_of::<String>();
                }
                self.state.diagnostic_step = 2;
            }
            2 => {
                let values = self.diagnostic_path();
                if let Some(text) = values.get(self.state.path_item) {
                    let value = owned(text)?;
                    progress.copied_bytes += 2 * value.len();
                    progress.retained_capacity_bytes = value.capacity();
                    self.diagnostic.as_mut().unwrap().path.push(value);
                    self.state.path_item += 1;
                } else {
                    self.state.diagnostic_step = 3;
                }
            }
            3 => {
                if self.state.diagnostic_pointer {
                    let value = pointer(self.state.owner.unwrap(), self.state.diagnostic_item)?;
                    progress.copied_bytes += 2 * value.len();
                    progress.retained_capacity_bytes = value.capacity();
                    self.diagnostic.as_mut().unwrap().pointer = Some(value);
                }
                self.state.diagnostic_step = 4;
            }
            _ => {
                let value = self.diagnostic.take().unwrap();
                let report = self.report.as_mut().unwrap();
                if self.state.diagnostic_global {
                    report.diagnostics.push(value);
                    self.state.phase = 6;
                } else {
                    report.targets[self.state.target].diagnostics.push(value);
                    self.state.phase = 15;
                }
                self.state.diagnostic_code = None;
            }
        }
        Ok(())
    }
    pub(super) fn take_demands() -> RetirementDemand { RetirementDemand { copy_bytes: 2 * size_of::<Option<IfcAlignmentDirectReport>>() + size_of::<bool>(), depth: 1, ..Default::default() } }
    fn take(&mut self, output: &mut Option<IfcAlignmentDirectReport>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "io.cancelled").into());
        }
        if self.state.authority.is_none_or(|old| !old.matches(cx)) {
            return Err(invalid("mvd.original-source").into());
        }
        let demand = Self::take_demands();
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) {
            return Ok(false);
        }
        if !self.state.done || self.state.donated || output.is_some() {
            return Err(invalid("mvd.direct.output").into());
        }
        *output = self.report.take();
        self.state.donated = true;
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(true)
    }
    pub(super) fn take_prepared(&mut self, view: IfcMvdPopulationView<'_, IfcAlignmentDirectRequest>, output: &mut Option<IfcAlignmentDirectReport>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "io.cancelled").into()); }
        if self.pin.is_none_or(|pin| !pin.matches(&view, cx)) { return Err(invalid("mvd.direct.clause-source").into()); }
        self.take(output, cx)
    }
    pub(super) fn take_unavailable(&mut self, view: IfcMvdPopulationRequestView<'_, IfcAlignmentDirectRequest>, output: &mut Option<IfcAlignmentDirectReport>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "io.cancelled").into()); }
        if self.request_pin.is_none_or(|pin| !pin.matches(&view, cx)) || view.status(cx)? != super::super::population::IfcMvdPopulationStatus::Unavailable { return Err(invalid("mvd.direct.clause-source").into()); }
        self.take(output, cx)
    }
    #[cfg(test)]
    pub(super) fn metadata(&self) -> ClauseMetadata {
        (self.state, self.pin, self.request_pin)
    }
}
