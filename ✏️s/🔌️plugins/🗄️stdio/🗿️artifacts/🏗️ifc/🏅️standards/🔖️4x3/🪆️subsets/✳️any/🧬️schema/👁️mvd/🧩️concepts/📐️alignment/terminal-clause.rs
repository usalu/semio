//! 🚆️ Local ordered business state borrows one prepared source and completed original request index.
use super::*;
use super::super::preparation::{IfcMvdRequestIndexPin, IfcMvdRequestIndexView};
use super::super::population::{IfcMvdPopulationPin, IfcMvdPopulationRequestPin, IfcMvdPopulationRequestView, IfcMvdPopulationView, IfcMvdPopulationLookup, IfcMvdPopulationStatus};
#[derive(Default, Clone, Copy)]
struct Relation {
    first: Option<usize>,
    count: u8,
    unresolved: bool,
}
semio_framework_value::artifact_retire_leaf!(Relation);
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ClauseState {
    authority: Option<semio_framework_job::OriginalSourceBinding>,
    phase: u8,
    target: usize,
    row: usize,
    member: usize,
    layout: usize,
    role: usize,
    matches: usize,
    matched: usize,
    unknown: bool,
    relation_unknown: bool,
    relations_unresolved: bool,
    field: u8,
    query_resume: u8,
    position: usize,
    owner: Option<Owner>,
    expression: Option<Expression>,
    query_status: Option<IfcAttributeQueryStatus>,
    list: Option<Owner>,
    failed: bool,
    incomplete: bool,
    source_invalid: bool,
    any_incomplete: bool,
    any_fail: bool,
    any_pass: bool,
    done: bool,
    donated: bool,
    code: Option<&'static str>,
    global: bool,
    path: u8,
    pointer: Option<Owner>,
    pointer_item: Option<usize>,
    diagnostic_step: u8,
    path_item: usize,
    resume: u8,
}
semio_framework_value::artifact_retire_leaf!(ClauseState);
#[derive(Default, semio_framework_value::RetireOwned)]
pub(super) struct IfcAlignmentTerminalClause {
    report: Option<IfcAlignmentTerminalReport>,
    diagnostic: Option<IfcConceptDiagnostic>,
    relations: Vec<Relation>,
    member: Membership,
    query: IfcAttributeQueryWork,
    number: PrimitiveZero,
    state: ClauseState,
    pin: Option<IfcMvdPopulationPin>,
    request_pin: Option<IfcMvdPopulationRequestPin>,
    request_rows: Option<IfcMvdRequestIndexPin>,
}
#[cfg(test)]
pub(super) type ClauseMetadata = (ClauseState, Option<IfcMvdPopulationPin>, Option<IfcMvdPopulationRequestPin>, Option<IfcMvdRequestIndexPin>);
fn lookup(view: Option<&IfcMvdPopulationView<'_, IfcAlignmentTerminalRequest>>, id: u64, cx: &semio_framework_job::StepContext<'_>) -> Result<IdMatch, ValueError> {
    Ok(match view.ok_or_else(|| invalid("mvd.terminal.prepared-index"))?.lookup(id, cx)? { IfcMvdPopulationLookup::Missing => IdMatch::Missing, IfcMvdPopulationLookup::Unique(row) => IdMatch::Unique(row), IfcMvdPopulationLookup::Ambiguous => IdMatch::Ambiguous })
}
fn request_lookup(view: &IfcMvdRequestIndexView<'_, IfcAlignmentTerminalRequest>, id: u64, cx: &semio_framework_job::StepContext<'_>) -> Result<IdMatch, ValueError> {
    Ok(match view.lookup(id, cx)? { IfcMvdPopulationLookup::Missing => IdMatch::Missing, IfcMvdPopulationLookup::Unique(row) => IdMatch::Unique(row), IfcMvdPopulationLookup::Ambiguous => IdMatch::Ambiguous })
}
impl IfcAlignmentTerminalClause {
    fn base() -> RetirementDemand {
        RetirementDemand {
            copy_bytes: 4 * size_of::<ClauseState>() + 4 * size_of::<IfcAlignmentTerminalTargetReport>() + 2 * size_of::<IfcAttributeQueryWork>() + 2 * size_of::<PrimitiveZero>() + 2 * size_of::<Membership>() + 2 * size_of::<Option<IfcMvdPopulationPin>>() + 2 * size_of::<Option<IfcMvdPopulationRequestPin>>() + 2 * size_of::<Option<IfcMvdRequestIndexPin>>() + 4 * 256,
            depth: 1,
            ..Default::default()
        }
    }
    fn diagnostics(&self) -> &Vec<IfcConceptDiagnostic> {
        let report = self.report.as_ref().unwrap();
        if self.state.global { &report.diagnostics } else { &report.targets[self.state.target].diagnostics }
    }
    pub(super) fn demands(&self, request: &IfcAlignmentTerminalRequest) -> Result<RetirementDemand, ValueError> {
        let mut demand = Self::base();
        let child = match self.state.phase {
            30 if self.relations.len() < request.targets.len() => vector_growth::<Relation>(self.relations.len(), self.relations.capacity())?,
            32 | 8 | 38 | 40 => RetirementDemand { copy_bytes: self.member.copy_bound(), depth: 1, ..Default::default() },
            10 => self.query.demands(),
            42 => RetirementDemand { copy_bytes: self.number.copy_bound(), depth: 1, ..Default::default() },
            6 if self.state.target < request.targets.len() => {
                let rows = &self.report.as_ref().unwrap().targets;
                vector_growth::<IfcAlignmentTerminalTargetReport>(rows.len(), rows.capacity())?
            }
            60 | 61 => {
                let row = &request.targets[self.state.target];
                let length = if self.state.phase == 60 { row.layout.len() } else { row.kind.len() };
                RetirementDemand { copy_bytes: 2 * length, capacity_bytes: length, depth: 1, ..Default::default() }
            }
            20 => {
                if self.state.diagnostic_step == 0 {
                    let list = self.diagnostics();
                    vector_growth::<IfcConceptDiagnostic>(list.len(), list.capacity())?
                } else {
                    let length = match self.state.diagnostic_step {
                        1 => path_count(self.state.path) * size_of::<String>(),
                        2 => {
                            if self.state.path_item < path_count(self.state.path) {
                                path_word_len(self.state.path, self.state.path_item, self.state.layout, self.state.member)
                            } else {
                                0
                            }
                        }
                        3 => pointer_len(self.state.pointer, self.state.pointer_item),
                        _ => 0,
                    };
                    RetirementDemand { copy_bytes: 2 * length + 2 * size_of::<IfcConceptDiagnostic>(), capacity_bytes: length, depth: 1, ..Default::default() }
                }
            }
            _ => RetirementDemand::default(),
        };
        demand.copy_bytes = demand.copy_bytes.checked_add(child.copy_bytes).ok_or_else(|| invalid("mvd.terminal.capacity"))?;
        demand.capacity_bytes = child.capacity_bytes;
        demand.release_bytes = child.release_bytes;
        demand.depth = demand.depth.max(child.depth);
        Ok(demand)
    }
    fn queue(&mut self, code: &'static str, incomplete: bool, path: u8, pointer: Option<Owner>, item: Option<usize>, resume: u8) {
        self.state.incomplete |= incomplete;
        self.state.failed |= !incomplete;
        self.state.code = Some(code);
        self.state.global = false;
        self.state.path = path;
        self.state.pointer = pointer;
        self.state.pointer_item = item;
        self.state.diagnostic_step = 0;
        self.state.path_item = 0;
        self.state.resume = resume;
        self.state.phase = 20;
    }
    fn field(&mut self, field: u8, position: usize, resume: u8) {
        self.state.field = field;
        self.state.position = position;
        self.state.query_resume = resume;
        self.state.owner = None;
        self.state.expression = None;
        self.state.query_status = None;
        self.query = Default::default();
        self.state.phase = 10;
    }
    fn member_start(&mut self, position: usize, expected: &'static str, phase: u8) {
        self.state.position = position;
        self.member.start(position, expected);
        self.state.phase = phase;
    }
    fn bad_field(&mut self, view: Option<&IfcMvdPopulationView<'_, IfcAlignmentTerminalRequest>>, cx: &semio_framework_job::StepContext<'_>, path: u8, resume: u8) -> Result<bool, ValueError> {
        let source = view.ok_or_else(|| invalid("mvd.terminal.prepared-source"))?.snapshot(cx)?;
        if self.state.query_status == Some(IfcAttributeQueryStatus::Unique) && self.state.owner.and_then(|owner| owner.value(source)).is_some() {
            return Ok(false);
        }
        let code = match self.state.query_status {
            Some(IfcAttributeQueryStatus::Unknown) => "mvd.attribute.unknown",
            Some(IfcAttributeQueryStatus::Ambiguous) => "mvd.attribute.ambiguous",
            _ => "mvd.attribute.missing",
        };
        self.queue(code, true, path, self.state.owner, None, resume);
        Ok(true)
    }
    fn list_len(&self, view: Option<&IfcMvdPopulationView<'_, IfcAlignmentTerminalRequest>>, cx: &semio_framework_job::StepContext<'_>) -> Result<usize, ValueError> {
        let source = view.ok_or_else(|| invalid("mvd.terminal.prepared-source"))?.snapshot(cx)?;
        match self.state.list.and_then(|owner| owner.value(source)) {
            Some(IfcValue::Aggregate(items)) => Ok(items.len()),
            _ => Err(invalid("mvd.terminal.list")),
        }
    }
    fn input_index(&self, index: &IfcMvdRequestIndexView<'_, IfcAlignmentTerminalRequest>, cx: &semio_framework_job::StepContext<'_>) -> Result<IfcMvdRequestIndexPin, ValueError> {
        let pin = index.pin(cx)?;
        if self.request_rows.is_some_and(|original| !original.matches(index, cx)) { return Err(invalid("mvd.terminal.request-index")); }
        Ok(pin)
    }
    pub(super) fn advance_prepared(&mut self, view: IfcMvdPopulationView<'_, IfcAlignmentTerminalRequest>, request_index: IfcMvdRequestIndexView<'_, IfcAlignmentTerminalRequest>, epoch: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "io.cancelled").into()); }
        let pin = view.pin(cx)?;
        if !request_index.matches_prepared(&view, cx) { return Err(invalid("mvd.terminal.request-index").into()); }
        if self.request_pin.is_some() || pin.epoch() != epoch || self.pin.is_some_and(|pin| !pin.matches(&view, cx)) || self.state.authority.is_some_and(|old| !old.matches(cx)) || self.state.donated { return Err(invalid("mvd.terminal.clause-source").into()); }
        let request = view.request(cx)?;
        let rows = self.input_index(&request_index, cx)?;
        let demand = self.demands(request)?;
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) { return Ok(false); }
        if self.state.done { return Ok(true); }
        if self.pin.is_none() { self.pin = Some(pin); }
        if self.request_rows.is_none() { self.request_rows = Some(rows); }
        self.advance_admitted(Some(&view), request, &request_index, epoch, cx)
    }
    pub(super) fn advance_unavailable(&mut self, view: IfcMvdPopulationRequestView<'_, IfcAlignmentTerminalRequest>, request_index: IfcMvdRequestIndexView<'_, IfcAlignmentTerminalRequest>, epoch: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "io.cancelled").into()); }
        let pin = view.pin(cx)?;
        if !request_index.matches_unavailable(&view, cx) { return Err(invalid("mvd.terminal.request-index").into()); }
        if self.pin.is_some() || pin.epoch() != epoch || view.status(cx)? != IfcMvdPopulationStatus::Unavailable || self.request_pin.is_some_and(|pin| !pin.matches(&view, cx)) || self.state.authority.is_some_and(|old| !old.matches(cx)) || self.state.donated { return Err(invalid("mvd.terminal.clause-source").into()); }
        let request = view.request(cx)?;
        let rows = self.input_index(&request_index, cx)?;
        let demand = self.demands(request)?;
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) { return Ok(false); }
        if self.state.done { return Ok(true); }
        if self.request_pin.is_none() { self.request_pin = Some(pin); }
        if self.request_rows.is_none() { self.request_rows = Some(rows); }
        self.advance_admitted(None, request, &request_index, epoch, cx)
    }
    fn advance_admitted(&mut self, view: Option<&IfcMvdPopulationView<'_, IfcAlignmentTerminalRequest>>, request: &IfcAlignmentTerminalRequest, request_index: &IfcMvdRequestIndexView<'_, IfcAlignmentTerminalRequest>, epoch: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: Self::base().copy_bytes, ..Default::default() };
        match self.state.phase {
            0 => { self.state.authority = Some(semio_framework_job::OriginalSourceBinding::borrow(cx)); self.report = Some(empty_report()); self.state.phase = 30; if view.is_none() { self.source_failure(); } },
            30 => {
                if self.relations.len() == request.targets.len() {
                    self.state.row = 0;
                    self.state.phase = 31;
                } else {
                    let growth = vector_growth::<Relation>(self.relations.len(), self.relations.capacity())?;
                    if growth.capacity_bytes > 0 {
                        reserve(&mut self.relations, growth.capacity_bytes)?;
                        progress.copied_bytes += growth.copy_bytes;
                        progress.retained_capacity_bytes = self.relations.capacity() * size_of::<Relation>();
                        progress.released_bytes = growth.release_bytes;
                    } else {
                        self.relations.push(Relation::default());
                    }
                }
            }
            31 => {
                let source = view.ok_or_else(|| invalid("mvd.terminal.prepared-source"))?.snapshot(cx)?;
                if self.state.row == source.entities.len() {
                    self.state.target = 0;
                    self.state.phase = 6;
                } else {
                    self.member_start(self.state.row, "IFCRELNESTS", 32);
                }
            }
            32 => {
                let source = view.ok_or_else(|| invalid("mvd.terminal.prepared-source"))?.snapshot(cx)?;
                if self.member.advance(source) {
                    if self.member.matched() {
                        self.state.relation_unknown = self.member.result().is_none();
                        self.field(0, self.state.row, 33);
                    } else {
                        self.state.relations_unresolved |= self.member.result().is_none();
                        self.state.row += 1;
                        self.state.phase = 31;
                    }
                }
                progress.copied_bytes += self.member.copy_bound();
            }
            33 => {
                let source = view.ok_or_else(|| invalid("mvd.terminal.prepared-source"))?.snapshot(cx)?;
                let value = self.state.owner.and_then(|owner| owner.value(source));
                if self.state.query_status != Some(IfcAttributeQueryStatus::Unique) || !matches!(value, Some(IfcValue::Reference(_))) {
                    self.state.relations_unresolved = true;
                } else if let Some(IfcValue::Reference(id)) = value {
                    if let IdMatch::Unique(target) = request_lookup(request_index, *id, cx)? {
                        if request.targets[target].applicable == Some(true) {
                            let slot = &mut self.relations[target];
                            if slot.count == 0 {
                                slot.first = Some(self.state.row);
                            }
                            slot.count = slot.count.saturating_add(1).min(2);
                            slot.unresolved |= self.state.relation_unknown || !matches!(lookup(view, source.entities[self.state.row].id, cx)?,IdMatch::Unique(position)if position==self.state.row);
                        }
                    }
                }
                self.state.row += 1;
                self.state.phase = 31;
            }
            6 => {
                if self.state.target == request.targets.len() {
                    self.state.phase = 16;
                } else {
                    let rows = &mut self.report.as_mut().unwrap().targets;
                    let growth = vector_growth::<IfcAlignmentTerminalTargetReport>(rows.len(), rows.capacity())?;
                    if growth.capacity_bytes > 0 {
                        reserve(rows, growth.capacity_bytes)?;
                        progress.copied_bytes += growth.copy_bytes;
                        progress.retained_capacity_bytes = rows.capacity() * size_of::<IfcAlignmentTerminalTargetReport>();
                        progress.released_bytes = growth.release_bytes;
                    } else {
                        rows.push(IfcAlignmentTerminalTargetReport { layout: String::new(), kind: String::new(), applicable: request.targets[self.state.target].applicable, status: "incomplete", diagnostics: vec![] });
                        self.state.phase = 60;
                    }
                }
            }
            60 | 61 => {
                let row = &request.targets[self.state.target];
                let value = owned(if self.state.phase == 60 { &row.layout } else { &row.kind })?;
                let length = value.len();
                let capacity = value.capacity();
                let output = &mut self.report.as_mut().unwrap().targets[self.state.target];
                if self.state.phase == 60 {
                    output.layout = value;
                    self.state.phase = 61;
                } else {
                    output.kind = value;
                    self.state.phase = 7;
                }
                progress.copied_bytes += 2 * length;
                progress.retained_capacity_bytes = capacity;
            }
            7 => {
                self.state.failed = false;
                self.state.incomplete = false;
                self.state.list = None;
                self.state.layout = kind(&request.targets[self.state.target].kind).unwrap();
                match request.targets[self.state.target].applicable {
                    Some(false) => {
                        self.report.as_mut().unwrap().targets[self.state.target].status = "not-applicable";
                        self.state.phase = 15;
                    }
                    None => self.queue("mvd.applicability", true, 0, None, None, 15),
                    Some(true) => {
                        if self.state.source_invalid {
                            self.state.incomplete = true;
                            self.state.phase = 15;
                        } else {
                            match lookup(view, positive_id(&request.targets[self.state.target].layout)?, cx)? {
                                IdMatch::Unique(position) => {
                                    self.state.role = 0;
                                    self.state.matches = 0;
                                    self.state.unknown = false;
                                    self.member_start(position, LAYOUTS[0], 8);
                                }
                                IdMatch::Missing => self.queue("mvd.target.missing", true, 0, None, None, 15),
                                IdMatch::Ambiguous => self.queue("mvd.target.ambiguous", true, 0, None, None, 15),
                            }
                        }
                    }
                }
            }
            8 => {
                let source = view.ok_or_else(|| invalid("mvd.terminal.prepared-source"))?.snapshot(cx)?;
                if self.member.advance(source) {
                    self.state.unknown |= self.member.result().is_none();
                    if self.member.matched() {
                        self.state.matches += 1;
                        self.state.matched = self.state.role;
                    }
                    if self.state.role < 2 {
                        self.state.role += 1;
                        self.member_start(self.state.position, LAYOUTS[self.state.role], 8);
                    } else {
                        let pointer = Some(Owner { entity: self.state.position, part: None, argument: usize::MAX });
                        if self.state.unknown || self.state.matches > 1 {
                            self.queue("mvd.target.unknown", true, 0, pointer, None, 15);
                        } else if self.state.matches == 0 || self.state.matched != self.state.layout {
                            self.queue("mvd.target.type", false, 0, pointer, None, 15);
                        } else {
                            self.state.phase = 34;
                        }
                    }
                }
                progress.copied_bytes += self.member.copy_bound();
            }
            34 => {
                if self.state.relations_unresolved || self.relations[self.state.target].unresolved {
                    self.queue("mvd.nesting.unresolved", true, 1, None, None, 35);
                } else {
                    self.state.phase = 35;
                }
            }
            35 => {
                let slot = self.relations[self.state.target];
                if slot.count != 1 {
                    self.queue(if slot.count == 0 { "mvd.nesting.missing" } else { "mvd.nesting.ambiguous" }, true, 1, None, None, 15);
                } else {
                    self.field(1, slot.first.unwrap(), 36);
                }
            }
            10 => {
                progress.copied_items = 0;
                cx.consume_retained(progress)?;
                let source = view.ok_or_else(|| invalid("mvd.terminal.prepared-source"))?.snapshot(cx)?;
                let name = field_name(self.state.field, self.state.layout);
                let owner = Some(field_owner(self.state.field, self.state.layout));
                if self.query.advance::<Ifc4x3AttributeMetadata, _>(source, self.state.position, name, owner, epoch, cx)? {
                    let result = self.query.completed::<Ifc4x3AttributeMetadata, _>(source, self.state.position, name, owner, epoch, cx)?.ok_or_else(|| invalid("mvd.terminal.query"))?;
                    self.state.query_status = Some(result.status);
                    if let Some(first) = &result.matches[0] {
                        self.state.owner = Some(Owner { entity: first.address.entity, part: first.address.part.checked_sub(1), argument: first.address.index });
                        self.state.expression = Some(Expression(first.type_expr));
                    }
                    self.state.phase = self.state.query_resume;
                }
                return Ok(false);
            }
            36 => {
                if !self.bad_field(view, cx, 1, 15)? {
                    let source = view.ok_or_else(|| invalid("mvd.terminal.prepared-source"))?.snapshot(cx)?;
                    let value = self.state.owner.and_then(|owner| owner.value(source));
                    if !matches!(value,Some(IfcValue::Aggregate(items))if !items.is_empty()) {
                        self.queue("mvd.attribute.shape", true, 1, self.state.owner, None, 15);
                    } else {
                        self.state.list = self.state.owner;
                        self.state.member = 0;
                        self.state.phase = 37;
                    }
                }
            }
            37 => {
                let source = view.ok_or_else(|| invalid("mvd.terminal.prepared-source"))?.snapshot(cx)?;
                let Some(IfcValue::Aggregate(items)) = self.state.list.and_then(|owner| owner.value(source)) else {
                    return Err(invalid("mvd.terminal.list").into());
                };
                if self.state.member == items.len() {
                    self.state.phase = 15;
                } else {
                    match &items[self.state.member] {
                        IfcValue::Reference(id) => match lookup(view, *id, cx)? {
                            IdMatch::Unique(position) => self.member_start(position, "IFCALIGNMENTSEGMENT", 38),
                            IdMatch::Missing => self.queue("mvd.member.missing", true, 2, self.state.list, Some(self.state.member), 45),
                            IdMatch::Ambiguous => self.queue("mvd.member.ambiguous", true, 2, self.state.list, Some(self.state.member), 45),
                        },
                        _ => self.queue("mvd.member.reference", true, 2, self.state.list, Some(self.state.member), 45),
                    }
                }
            }
            38 => {
                let source = view.ok_or_else(|| invalid("mvd.terminal.prepared-source"))?.snapshot(cx)?;
                if self.member.advance(source) {
                    if self.member.result() != Some(true) {
                        self.queue("mvd.member.type", true, 2, self.state.list, Some(self.state.member), 45);
                    } else {
                        self.field(2, self.state.position, 39);
                    }
                }
                progress.copied_bytes += self.member.copy_bound();
            }
            39 => {
                if !self.bad_field(view, cx, 3, 45)? {
                    let source = view.ok_or_else(|| invalid("mvd.terminal.prepared-source"))?.snapshot(cx)?;
                    match self.state.owner.and_then(|owner| owner.value(source)) {
                        Some(IfcValue::Reference(id)) => match lookup(view, *id, cx)? {
                            IdMatch::Unique(position) => {
                                self.state.role = 0;
                                self.state.matches = 0;
                                self.state.unknown = false;
                                self.member_start(position, PARAMETERS[0], 40);
                            }
                            IdMatch::Missing => self.queue("mvd.member.missing", true, 3, self.state.owner, None, 45),
                            IdMatch::Ambiguous => self.queue("mvd.member.ambiguous", true, 3, self.state.owner, None, 45),
                        },
                        _ => self.queue("mvd.attribute.shape", true, 3, self.state.owner, None, 45),
                    }
                }
            }
            40 => {
                let source = view.ok_or_else(|| invalid("mvd.terminal.prepared-source"))?.snapshot(cx)?;
                if self.member.advance(source) {
                    self.state.unknown |= self.member.result().is_none();
                    if self.member.matched() {
                        self.state.matches += 1;
                        self.state.matched = self.state.role;
                    }
                    if self.state.role < 2 {
                        self.state.role += 1;
                        self.member_start(self.state.position, PARAMETERS[self.state.role], 40);
                    } else if self.state.unknown || self.state.matches != 1 || self.state.matched != self.state.layout {
                        self.queue("mvd.parameter.type", true, 3, self.state.owner, None, 45);
                    } else {
                        self.field(3, self.state.position, 41);
                    }
                }
                progress.copied_bytes += self.member.copy_bound();
            }
            41 => {
                if !self.bad_field(view, cx, 4, 45)? {
                    self.number.start(self.state.owner.unwrap(), self.state.expression.unwrap(), 0);
                    self.state.phase = 42;
                }
            }
            42 => {
                let source = view.ok_or_else(|| invalid("mvd.terminal.prepared-source"))?.snapshot(cx)?;
                if self.number.advance(source) {
                    match self.number.result() {
                        None => self.queue("mvd.length.shape", true, 4, self.state.owner, None, 45),
                        Some(false) if self.state.member + 1 == self.list_len(view, cx)? => self.queue("mvd.terminal.nonzero", false, 4, self.state.owner, None, 45),
                        _ => self.state.phase = 45,
                    }
                }
                progress.copied_bytes += self.number.copy_bound();
            }
            45 => {
                self.state.member += 1;
                self.state.phase = 37;
            }
            15 => {
                let output = &mut self.report.as_mut().unwrap().targets[self.state.target];
                if output.applicable != Some(false) {
                    output.status = if self.state.incomplete {
                        "incomplete"
                    } else if self.state.failed {
                        "fail"
                    } else {
                        "pass"
                    };
                }
                self.state.any_incomplete |= output.status == "incomplete";
                self.state.any_fail |= output.status == "fail";
                self.state.any_pass |= output.status == "pass";
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
            20 => self.diagnostic_step(&mut progress)?,
            _ => return Err(invalid("mvd.terminal.state").into()),
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        Ok(self.state.done)
    }
    fn source_failure(&mut self) {
        self.state.source_invalid = true;
        self.state.code = Some("mvd.source");
        self.state.global = true;
        self.state.path = 0;
        self.state.pointer = None;
        self.state.pointer_item = None;
        self.state.diagnostic_step = 0;
        self.state.path_item = 0;
        self.state.resume = 6;
        self.state.phase = 20;
    }
    fn diagnostic_step(&mut self, progress: &mut RetainedCloneProgress) -> Result<(), ValueError> {
        match self.state.diagnostic_step {
            0 => {
                let list = self.diagnostics();
                let growth = vector_growth::<IfcConceptDiagnostic>(list.len(), list.capacity())?;
                if growth.capacity_bytes > 0 {
                    let report = self.report.as_mut().unwrap();
                    let list = if self.state.global { &mut report.diagnostics } else { &mut report.targets[self.state.target].diagnostics };
                    reserve(list, growth.capacity_bytes)?;
                    progress.copied_bytes += growth.copy_bytes;
                    progress.retained_capacity_bytes = list.capacity() * size_of::<IfcConceptDiagnostic>();
                    progress.released_bytes = growth.release_bytes;
                } else {
                    let code = self.state.code.unwrap();
                    let (en, de) = labels(code);
                    self.diagnostic = Some(IfcConceptDiagnostic { code, path: vec![], pointer: None, label: IfcConceptLabel { en, de } });
                    self.state.diagnostic_step = 1;
                }
            }
            1 => {
                let count = path_count(self.state.path);
                let words = &mut self.diagnostic.as_mut().unwrap().path;
                if count > 0 {
                    words.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "mvd.terminal.allocation"))?;
                    progress.retained_capacity_bytes = words.capacity() * size_of::<String>();
                    progress.copied_bytes += 2 * count * size_of::<String>();
                }
                self.state.diagnostic_step = 2;
            }
            2 => {
                if self.state.path_item == path_count(self.state.path) {
                    self.state.diagnostic_step = 3;
                } else {
                    let value = path_word(self.state.path, self.state.path_item, self.state.layout, self.state.member)?;
                    progress.copied_bytes += 2 * value.len();
                    progress.retained_capacity_bytes = value.capacity();
                    self.diagnostic.as_mut().unwrap().path.push(value);
                    self.state.path_item += 1;
                }
            }
            3 => {
                if let Some(owner) = self.state.pointer {
                    let value = pointer(owner, self.state.pointer_item)?;
                    progress.copied_bytes += 2 * value.len();
                    progress.retained_capacity_bytes = value.capacity();
                    self.diagnostic.as_mut().unwrap().pointer = Some(value);
                }
                self.state.diagnostic_step = 4;
            }
            _ => {
                let value = self.diagnostic.take().unwrap();
                let report = self.report.as_mut().unwrap();
                if self.state.global {
                    report.diagnostics.push(value);
                } else {
                    report.targets[self.state.target].diagnostics.push(value);
                }
                self.state.phase = self.state.resume;
                self.state.code = None;
            }
        }
        Ok(())
    }
    pub(super) fn take_demands() -> RetirementDemand { RetirementDemand { copy_bytes: 2 * size_of::<Option<IfcAlignmentTerminalReport>>() + size_of::<bool>(), depth: 1, ..Default::default() } }
    fn take(&mut self, output: &mut Option<IfcAlignmentTerminalReport>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
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
            return Err(invalid("mvd.terminal.output").into());
        }
        *output = self.report.take();
        self.state.donated = true;
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(true)
    }
    pub(super) fn take_prepared(&mut self, view: IfcMvdPopulationView<'_, IfcAlignmentTerminalRequest>, index: IfcMvdRequestIndexView<'_, IfcAlignmentTerminalRequest>, output: &mut Option<IfcAlignmentTerminalReport>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "io.cancelled").into()); }
        if self.pin.is_none_or(|pin| !pin.matches(&view, cx)) { return Err(invalid("mvd.terminal.clause-source").into()); }
        if !index.matches_prepared(&view, cx) { return Err(invalid("mvd.terminal.request-index").into()); }
        self.input_index(&index, cx)?;
        self.take(output, cx)
    }
    pub(super) fn take_unavailable(&mut self, view: IfcMvdPopulationRequestView<'_, IfcAlignmentTerminalRequest>, index: IfcMvdRequestIndexView<'_, IfcAlignmentTerminalRequest>, output: &mut Option<IfcAlignmentTerminalReport>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "io.cancelled").into()); }
        if self.request_pin.is_none_or(|pin| !pin.matches(&view, cx)) || view.status(cx)? != IfcMvdPopulationStatus::Unavailable { return Err(invalid("mvd.terminal.clause-source").into()); }
        if !index.matches_unavailable(&view, cx) { return Err(invalid("mvd.terminal.request-index").into()); }
        self.input_index(&index, cx)?;
        self.take(output, cx)
    }
    #[cfg(test)]
    pub(super) fn metadata(&self) -> ClauseMetadata {
        (self.state, self.pin, self.request_pin, self.request_rows)
    }
}
