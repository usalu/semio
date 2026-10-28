//! 🧱️ Local Body clauses borrow authenticated request, population and request-index views for one funded turn.
use super::*;
use super::helpers::*;
use super::super::population::{IfcMvdPopulationLookup, IfcMvdPopulationPin, IfcMvdPopulationRequestPin, IfcMvdPopulationRequestView, IfcMvdPopulationStatus, IfcMvdPopulationView};
use super::super::preparation::{IfcMvdRequestIndexPin, IfcMvdRequestIndexView};
use super::super::source_index::IdMatch;
use crate::validation::metadata;
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ClauseState {
    authority: Option<semio_framework_job::OriginalSourceBinding>,
    phase: u8,
    resume: u8,
    target: usize,
    predicate: usize,
    item: usize,
    position: usize,
    query_status:Option<IfcAttributeQueryStatus>,
    owner: Option<Owner>,
    expression: Option<Expression>,
    label_depth: usize,
    label_wrappers: usize,
    failed: bool,
    incomplete: bool,
    any_incomplete: bool,
    any_fail: bool,
    any_pass: bool,
    source_invalid: bool,
    done: bool,
    donated: bool,
    pending: Option<DiagnosticSpec>,
    diagnostic_step: u8,
}
semio_framework_value::artifact_retire_leaf!(ClauseState);


#[derive(semio_framework_value::RetireOwned)]
pub(super) struct IfcBodyClause {
    pub(super) report: Option<IfcBodyConceptReport>,
    pub(super) diagnostic: Option<IfcConceptDiagnostic>,
    member: Membership,
    family: BodyFamilyMembership,
    query: IfcAttributeQueryWork,
    state: ClauseState,
    policy: BodyPolicy,
    pin: Option<IfcMvdPopulationPin>,
    request_pin: Option<IfcMvdPopulationRequestPin>,
    requests_pin: Option<IfcMvdRequestIndexPin>,
}
#[cfg(test)]
pub(super) type ClauseMetadata = (ClauseState, BodyPolicy, Option<IfcMvdPopulationPin>, Option<IfcMvdPopulationRequestPin>, Option<IfcMvdRequestIndexPin>);
fn lookup(view: Option<&IfcMvdPopulationView<'_, IfcBodyConceptRequest>>, id: u64, cx: &semio_framework_job::StepContext<'_>) -> Result<IdMatch, ValueError> {
    Ok(match view.ok_or_else(|| invalid("mvd.body.prepared-index"))?.lookup(id, cx)? { IfcMvdPopulationLookup::Missing => IdMatch::Missing, IfcMvdPopulationLookup::Unique(row) => IdMatch::Unique(row), IfcMvdPopulationLookup::Ambiguous => IdMatch::Ambiguous })
}
impl IfcBodyClause {
    pub(super) fn new(policy: BodyPolicy) -> Self { Self { report: None, diagnostic: None, member: Default::default(), family: BodyFamilyMembership::new(policy), query: Default::default(), state: Default::default(), policy, pin: None, request_pin: None, requests_pin: None } }
    fn base() -> RetirementDemand {
        RetirementDemand { copy_bytes: 4 * size_of::<ClauseState>() + 4 * size_of::<IfcConceptTargetReport>() + 2 * size_of::<IfcAttributeQueryWork>() + 2 * size_of::<Membership>() + 2 * size_of::<BodyFamilyMembership>() + 2 * size_of::<BodyPolicy>() + 2 * size_of::<Option<IfcMvdPopulationPin>>() + 2 * size_of::<Option<IfcMvdPopulationRequestPin>>() + 2 * size_of::<Option<IfcMvdRequestIndexPin>>() + 4 * 256, depth: 1, ..Default::default() }
    }
    pub(super) fn demands(&self, request: &IfcBodyConceptRequest) -> Result<RetirementDemand, ValueError> {
        let child = match self.state.phase {
            6 if self.state.target < request.targets.len() => {
                let rows = &self.report.as_ref().unwrap().targets;
                let growth = vector_growth::<IfcConceptTargetReport>(rows.len(), rows.capacity())?;
                let text = if growth.capacity_bytes == 0 { request.targets[self.state.target].representation.len() } else { 0 };
                RetirementDemand { copy_bytes: growth.copy_bytes + 2 * text, capacity_bytes: growth.capacity_bytes + text, ..growth }
            }
            7 | 8 => RetirementDemand { copy_bytes: self.member.copy_bound(), depth: 1, ..Default::default() },
            10 => self.query.demands(),
            13 | 18 => RetirementDemand { copy_bytes: self.family.copy_bound(), depth: 1, ..Default::default() },
            14 => {
                let report = self.report.as_ref().unwrap();
                let list = if self.state.source_invalid && self.state.target == 0 && self.state.resume == 6 { &report.diagnostics } else { &report.targets[self.state.target].diagnostics };
                diagnostic_demand(self.state.pending.unwrap(), self.state.diagnostic_step, list)?
            }
            _ => RetirementDemand::default(),
        };
        let mut demand = Self::base();
        demand.copy_bytes = demand.copy_bytes.checked_add(child.copy_bytes).ok_or_else(|| invalid("mvd.body.capacity"))?;
        demand.capacity_bytes = child.capacity_bytes;
        demand.release_bytes = child.release_bytes;
        demand.depth = demand.depth.max(child.depth);
        Ok(demand)
    }
    fn check_requests(&self, requests: &IfcMvdRequestIndexView<'_, IfcBodyConceptRequest>, cx: &semio_framework_job::StepContext<'_>) -> Result<IfcMvdRequestIndexPin, ValueError> {
        let pin = requests.pin(cx)?;
        if self.requests_pin.is_some_and(|old| !old.matches(requests, cx)) || self.state.authority.is_some_and(|old| !old.matches(cx)) || self.state.donated { return Err(invalid("mvd.body.clause-source")); }
        Ok(pin)
    }
    pub(super) fn advance_prepared(&mut self, view: IfcMvdPopulationView<'_, IfcBodyConceptRequest>, requests: IfcMvdRequestIndexView<'_, IfcBodyConceptRequest>, epoch: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let pin = view.pin(cx)?;
        let requests_pin = self.check_requests(&requests, cx)?;
        if self.request_pin.is_some() || pin.epoch() != epoch || !requests.matches_prepared(&view, cx) || self.pin.is_some_and(|old| !old.matches(&view, cx)) { return Err(invalid("mvd.body.clause-source").into()); }
        let request = view.request(cx)?;
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), self.demands(request)?) { return Ok(false); }
        if self.state.done { return Ok(true); }
        self.pin = Some(pin);
        self.requests_pin = Some(requests_pin);
        self.advance_admitted(Some(&view), request, epoch, cx)
    }
    pub(super) fn advance_unavailable(&mut self, view: IfcMvdPopulationRequestView<'_, IfcBodyConceptRequest>, requests: IfcMvdRequestIndexView<'_, IfcBodyConceptRequest>, epoch: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let pin = view.pin(cx)?;
        let requests_pin = self.check_requests(&requests, cx)?;
        if self.pin.is_some() || pin.epoch() != epoch || view.status(cx)? != IfcMvdPopulationStatus::Unavailable || !requests.matches_unavailable(&view, cx) || self.request_pin.is_some_and(|old| !old.matches(&view, cx)) { return Err(invalid("mvd.body.clause-source").into()); }
        let request = view.request(cx)?;
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), self.demands(request)?) { return Ok(false); }
        if self.state.done { return Ok(true); }
        self.request_pin = Some(pin);
        self.requests_pin = Some(requests_pin);
        self.advance_admitted(None, request, epoch, cx)
    }
    fn queue(&mut self, code: &'static str, owner: Option<Owner>, item: Option<usize>, resume: u8) {
        self.state.pending = Some(DiagnosticSpec { code, predicate: if code == "mvd.applicability" || code.starts_with("mvd.target") || code == "mvd.source" { None } else { Some(self.state.predicate) }, owner, item });
        self.state.resume = resume;
        self.state.phase = 14;
        self.state.diagnostic_step = 0;
    }

    fn advance_admitted(&mut self, view: Option<&IfcMvdPopulationView<'_, IfcBodyConceptRequest>>, request: &IfcBodyConceptRequest, epoch: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: Self::base().copy_bytes, ..Default::default() };
        match self.state.phase {
            0 => {
                self.state.authority = Some(semio_framework_job::OriginalSourceBinding::borrow(cx));
                self.report = Some(empty_report(self.policy));
                self.state.phase = 6;
                if view.is_none() { self.state.source_invalid = true; self.queue("mvd.source", None, None, 6); }
            }
            6 => {
                if self.state.target == request.targets.len() {
                    self.state.phase = 16;
                } else {
                    let report = self.report.as_mut().unwrap();
                    let growth = vector_growth::<IfcConceptTargetReport>(report.targets.len(), report.targets.capacity())?;
                    if growth.capacity_bytes > 0 {
                        reserve(&mut report.targets, growth.capacity_bytes)?;
                        progress.copied_bytes += growth.copy_bytes;
                        progress.retained_capacity_bytes = report.targets.capacity() * size_of::<IfcConceptTargetReport>();
                        progress.released_bytes = growth.release_bytes;
                    } else {
                        let input = &request.targets[self.state.target];
                        let text = owned(&input.representation)?;
                        let applicable = input.applicable;
                        let report = self.report.as_mut().unwrap();
                        progress.copied_bytes += growth.copy_bytes + 2 * text.len();
                        progress.retained_capacity_bytes = text.capacity();
                        report.targets.push(IfcConceptTargetReport {
                            representation: text,
                            applicable,
                            status: if applicable == Some(false) {
                                "not-applicable"
                            } else if self.state.source_invalid || applicable.is_none() {
                                "incomplete"
                            } else {
                                "pass"
                            },
                            diagnostics: vec![],
                        });
                        self.state.failed = false;
                        self.state.incomplete = false;
                        self.state.predicate = 0;
                        self.state.item = 0;
                        if self.state.source_invalid || applicable == Some(false) {
                            self.state.phase = 15;
                        } else if applicable.is_none() {
                            self.state.incomplete = true;
                            self.queue("mvd.applicability", None, None, 15);
                        } else {
                            self.state.phase = 7;
                        }
                    }
                }
            }
            7 => {
                let id = positive_id(&request.targets[self.state.target].representation)?;
                match lookup(view, id, cx)? {
                    IdMatch::Missing => {
                        self.state.incomplete = true;
                        self.queue("mvd.target.missing", None, None, 15);
                    }
                    IdMatch::Ambiguous => {
                        self.state.incomplete = true;
                        self.queue("mvd.target.ambiguous", None, None, 15);
                    }
                    IdMatch::Unique(position) => {
                        self.state.position = position;
                        self.member.start(position, "IFCSHAPEREPRESENTATION");
                        progress.copied_bytes += self.member.copy_bound();
                        self.state.resume = 9;
                        self.state.phase = 8;
                    }
                }
            }
            8 => {
                let source = view.ok_or_else(|| invalid("mvd.body.prepared-source"))?.snapshot(cx)?;
                if self.member.advance(source) {
                    let result = self.member.result();
                    if result == Some(true) { self.state.phase = 9; }
                    else {
                        self.state.failed = result == Some(false);
                        self.state.incomplete = result.is_none();
                        self.queue(if result.is_none() { "mvd.target.unknown" } else { "mvd.target.type" }, Some(Owner { entity: self.state.position, part: None, argument: usize::MAX }), None, 15);
                    }
                }
                progress.copied_bytes += self.member.copy_bound();
            }
            9 => {
                if self.state.predicate == 3 {
                    self.state.phase = 15;
                } else {
                    self.query=Default::default();
                    self.state.query_status=None;
                    self.state.owner = None;
                    self.state.expression = None;
                    self.state.phase = 10;
                }
            }
            10 => {
                progress.copied_items=0;cx.consume_retained(progress)?;let source=view.ok_or_else(|| invalid("mvd.body.prepared-source"))?.snapshot(cx)?;let name=PREDICATES[self.state.predicate];if self.query.advance::<Ifc4x3AttributeMetadata,_>(source,self.state.position,name,Some("IFCREPRESENTATION"),epoch,cx)?{let result=self.query.completed::<Ifc4x3AttributeMetadata,_>(source,self.state.position,name,Some("IFCREPRESENTATION"),epoch,cx)?.ok_or_else(||invalid("mvd.attribute.query"))?;self.state.query_status=Some(result.status);if let Some(first)=&result.matches[0]{self.state.owner=Some(Owner{entity:first.address.entity,part:first.address.part.checked_sub(1),argument:first.address.index});self.state.expression=Some(Expression(first.type_expr));}self.state.phase=11;}return Ok(false);
            }
            11 => {
                let source = view.ok_or_else(|| invalid("mvd.body.prepared-source"))?.snapshot(cx)?;
                let value = self.state.owner.and_then(|owner| owner.value(source));
                if self.state.query_status!=Some(IfcAttributeQueryStatus::Unique) || value.is_none() {
                    self.state.incomplete = true;
                    self.queue(match self.state.query_status{Some(IfcAttributeQueryStatus::Unknown)=>"mvd.attribute.unknown",Some(IfcAttributeQueryStatus::Ambiguous)=>"mvd.attribute.ambiguous",_=>"mvd.attribute.missing"}, self.state.owner, None, 9);
                    self.state.predicate += 1;
                } else if self.state.predicate < 2 {
                    self.state.label_depth = 0;
                    self.state.label_wrappers = 0;
                    self.state.phase = 12;
                } else if !matches!(value, Some(IfcValue::Aggregate(_))) {
                    self.state.incomplete = true;
                    self.queue("mvd.attribute.shape", self.state.owner, None, 9);
                    self.state.predicate += 1;
                } else {
                    self.state.item = 0;
                    self.state.phase = 13;
                }
            }
            12 => {
                let source = view.ok_or_else(|| invalid("mvd.body.prepared-source"))?.snapshot(cx)?;
                let value = label_value(self.state.owner.unwrap().value(source).unwrap(), self.state.label_wrappers);
                let expression = self.state.expression.unwrap();
                let resolved = if self.state.label_depth == 64 || expression.kind != "named" {
                    Some(false)
                } else {
                    let name = expression.name.as_deref().unwrap_or("");
                    if name == "STRING" {
                        Some(matches!(value,Some(IfcValue::String(text))if text==if self.state.predicate==0{"Body"}else{self.policy.representation_type()}))
                    } else if let Some(declaration) = metadata::defined_type(name) {
                        if declaration.kind != "defined" || declaration.type_expr.is_none() {
                            Some(false)
                        } else if let Some(IfcValue::TypedValue(typed)) = value {
                            if !valid_name_equal(&typed.name, name) || typed.items.len() != 1 {
                                Some(false)
                            } else {
                                self.state.label_wrappers += 1;
                                self.state.expression = declaration.type_expr.map(Expression);
                                None
                            }
                        } else {
                            self.state.expression = declaration.type_expr.map(Expression);
                            None
                        }
                    } else {
                        Some(false)
                    }
                };
                self.state.label_depth += 1;
                if let Some(accepted) = resolved {
                    if !accepted {
                        self.state.failed = true;
                        self.queue("mvd.equals", self.state.owner, None, 9);
                    } else {
                        self.state.phase = 9;
                    }
                    self.state.predicate += 1;
                }
            }
            13 => {
                let value = self.state.owner.unwrap().value(view.ok_or_else(|| invalid("mvd.body.prepared-source"))?.snapshot(cx)?).unwrap();
                let IfcValue::Aggregate(items) = value else { unreachable!() };
                if self.state.item == items.len() {
                    self.state.predicate += 1;
                    self.state.phase = 9;
                } else if let IfcValue::Reference(id) = items[self.state.item] {
                    match lookup(view, id, cx)? {
                        IdMatch::Missing => {
                            self.state.incomplete = true;
                            self.queue("mvd.items.missing", self.state.owner, Some(self.state.item), 13);
                            self.state.item += 1;
                        }
                        IdMatch::Ambiguous => {
                            self.state.incomplete = true;
                            self.queue("mvd.items.ambiguous", self.state.owner, Some(self.state.item), 13);
                            self.state.item += 1;
                        }
                        IdMatch::Unique(position) => {
                            self.family.start(position);
                            progress.copied_bytes += self.family.copy_bound();
                            self.state.phase = 18;
                        }
                    }
                } else {
                    self.state.incomplete = true;
                    self.queue("mvd.items.reference", self.state.owner, Some(self.state.item), 13);
                    self.state.item += 1;
                }
            }
            14 => {
                let spec = self.state.pending.unwrap();
                let report = self.report.as_mut().unwrap();
                let list = if self.state.source_invalid && self.state.target == 0 && self.state.resume == 6 { &mut report.diagnostics } else { &mut report.targets[self.state.target].diagnostics };
                let child = diagnostic_demand(spec, self.state.diagnostic_step, list)?;
                progress.copied_bytes += child.copy_bytes;
                match self.state.diagnostic_step {
                    0 => {
                        reserve(list, child.capacity_bytes)?;
                        progress.retained_capacity_bytes = child.capacity_bytes;
                        progress.released_bytes = child.release_bytes;
                        self.diagnostic = Some(empty_diagnostic(self.policy, spec));
                    }
                    1 => {
                        let path = &mut self.diagnostic.as_mut().unwrap().path;
                        path.try_reserve_exact(path_length(spec)).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "mvd.allocation"))?;
                        progress.retained_capacity_bytes = path.capacity() * size_of::<String>();
                    }
                    2 => {
                        if let Some(predicate) = spec.predicate {
                            let text = owned(PREDICATES[predicate])?;
                            progress.retained_capacity_bytes = text.capacity();
                            self.diagnostic.as_mut().unwrap().path.push(text);
                        }
                    }
                    3 => {
                        if let Some(item) = spec.item {
                            let text = number(item)?;
                            progress.retained_capacity_bytes = text.capacity();
                            self.diagnostic.as_mut().unwrap().path.push(text);
                        }
                    }
                    4 => {
                        let pointer = pointer(spec)?;
                        progress.retained_capacity_bytes = pointer.as_ref().map_or(0, String::capacity);
                        self.diagnostic.as_mut().unwrap().pointer = pointer;
                    }
                    _ => {
                        list.push(self.diagnostic.take().unwrap());
                        self.state.pending = None;
                        self.state.phase = self.state.resume;
                    }
                }
                self.state.diagnostic_step += 1;
            }
            15 => {
                let row = &mut self.report.as_mut().unwrap().targets[self.state.target];
                if !self.state.source_invalid && row.applicable != Some(false) {
                    row.status = if self.state.incomplete {
                        "incomplete"
                    } else if self.state.failed {
                        "fail"
                    } else {
                        "pass"
                    };
                }
                self.state.any_incomplete |= row.status == "incomplete";
                self.state.any_fail |= row.status == "fail";
                self.state.any_pass |= row.status == "pass";
                self.state.target += 1;
                self.state.phase = 6;
            }
            16 => {
                let report = self.report.as_mut().unwrap();
                report.status = if self.state.any_incomplete || !report.diagnostics.is_empty() {
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
            18 => {
                let source = view.ok_or_else(|| invalid("mvd.body.prepared-source"))?.snapshot(cx)?;
                if self.family.advance(source) {
                    match self.family.result() {
                        BodyFamilyResult::Resolved(true) => self.state.phase = 13,
                        BodyFamilyResult::Unknown | BodyFamilyResult::Resolved(false) => {
                            let unknown = self.family.result() == BodyFamilyResult::Unknown;
                            self.state.incomplete |= unknown;
                            self.state.failed |= !unknown;
                            self.queue(if unknown { "mvd.items.unknown" } else { "mvd.items.type" }, self.state.owner, Some(self.state.item), 13);
                        }
                        BodyFamilyResult::Pending => return Err(invalid("mvd.body.family-pending").into()),
                    }
                    self.state.item += 1;
                }
                progress.copied_bytes += self.family.copy_bound();
            }
            _ => return Err(invalid("mvd.body.state").into()),
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        Ok(self.state.done)
    }
    pub(super) fn take_demands() -> RetirementDemand { RetirementDemand { copy_bytes: 2 * size_of::<Option<IfcBodyConceptReport>>() + size_of::<bool>(), depth: 1, ..Default::default() } }
    fn take(&mut self, output: &mut Option<IfcBodyConceptReport>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "mvd.cancelled").into()); }
        if self.state.authority.is_none_or(|old| !old.matches(cx)) { return Err(invalid("mvd.original-source").into()); }
        let demand = Self::take_demands();
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) { return Ok(false); }
        if !self.state.done || self.state.donated || output.is_some() { return Err(invalid("mvd.body.output").into()); }
        *output = self.report.take();
        self.state.donated = true;
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(true)
    }
    pub(super) fn take_prepared(&mut self, view: IfcMvdPopulationView<'_, IfcBodyConceptRequest>, requests: IfcMvdRequestIndexView<'_, IfcBodyConceptRequest>, output: &mut Option<IfcBodyConceptReport>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        self.check_requests(&requests, cx)?;
        if self.pin.is_none_or(|old| !old.matches(&view, cx)) || !requests.matches_prepared(&view, cx) { return Err(invalid("mvd.body.clause-source").into()); }
        self.take(output, cx)
    }
    pub(super) fn take_unavailable(&mut self, view: IfcMvdPopulationRequestView<'_, IfcBodyConceptRequest>, requests: IfcMvdRequestIndexView<'_, IfcBodyConceptRequest>, output: &mut Option<IfcBodyConceptReport>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        self.check_requests(&requests, cx)?;
        if self.request_pin.is_none_or(|old| !old.matches(&view, cx)) || view.status(cx)? != IfcMvdPopulationStatus::Unavailable || !requests.matches_unavailable(&view, cx) { return Err(invalid("mvd.body.clause-source").into()); }
        self.take(output, cx)
    }
    #[cfg(test)]
    pub(super) fn metadata(&self) -> ClauseMetadata { (self.state, self.policy, self.pin, self.request_pin, self.requests_pin) }
}
