//! 🧵️ Native template planning retains the original command and admits real forward and inverse Store wire.
use super::receive::{fits, fixed, invalid, receipt, ApplicationReceiver};
use super::*;
use crate::details::{IfcPublication, SnapshotSource};
use crate::editor::{Ifc4x3AnyEditor, Ifc4x3EditCommand};
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedWorkCapacity};
use semio_framework_plugin::{EditorApp, Fault};
use semio_framework_value::retained_clone::{RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{DslValue, RetirementDemand, ValueError, ValueRefusalKind};
use std::{mem::size_of, sync::Arc};

#[path = "plan.rs"]
mod plan;
use plan::Planner;

#[path = "staging-native/🦀️.rs"]
pub mod staging_native;

pub const NATIVE_TEMPLATE_AUTHORING_EXECUTION: bool = false;
pub const NATIVE_TEMPLATE_AUTHORING_IMPLEMENTATION: bool = false;
pub const NATIVE_TEMPLATE_PLANNING_IMPLEMENTATION: bool = true;
pub const NATIVE_TEMPLATE_WIRE_ADMISSION_IMPLEMENTATION: bool = true;
pub const NATIVE_TEMPLATE_SEMANTIC_ADMISSION: bool = false;
pub const TEMPLATE_MAXIMUM_RAW_BYTES: usize = 16 * 1024 * 1024;
pub const TEMPLATE_WORK_CAPACITY: ArtifactRetainedWorkCapacity = ArtifactRetainedWorkCapacity::for_invertible_items(MAXIMUM_IDENTITIES);

#[derive(Clone, Copy)]
enum Phase {
    Receive,
    Decode,
    Take,
    Source,
    Plan,
    Wire,
    Semantic,
}
pub struct IfcTemplateWork {
    original: Option<DslValue>,
    received: bool,
    receiver: ApplicationReceiver,
    planner: Planner,
    publication: IfcPublication,
    semantic: crate::validation::admission::RetainedExpressAdmission,
    unborn: Option<Arc<IfcSnapshot>>,
    retirement: RetainedCloneClose,
    phase: Phase,
    closing: bool,
}
impl Default for IfcTemplateWork {
    fn default() -> Self {
        Self {
            original: None,
            received: false,
            receiver: Default::default(),
            planner: Default::default(),
            publication: Default::default(),
            semantic: Default::default(),
            unborn: None,
            retirement: Default::default(),
            phase: Phase::Receive,
            closing: false,
        }
    }
}
impl IfcTemplateWork {
    fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.publication.codec_live() {
            return self.publication.close_demands();
        }
        if !self.semantic.terminal_is_empty() {
            return self.semantic.close_demands(body);
        }
        if !self.receiver.terminal_is_empty() {
            return self.receiver.close_demands(body);
        }
        if !self.retirement.is_empty() {
            return self.retirement.next_owner_demands::<DslValue>(false, &None, body);
        }
        if self.original.is_some() {
            return self.retirement.next_owner_demands::<DslValue>(true, &None, body);
        }
        if self.planner.garbage.is_some() {
            return self.retirement.next_owner_demands::<IfcValue>(true, &None, body);
        }
        if self.planner.application.is_some() {
            return self.retirement.next_owner_demands::<IfcTemplateApplication>(true, &None, body);
        }
        if self.planner.mutations.is_some() {
            return self.retirement.next_owner_demands::<Vec<IfcMutation>>(true, &None, body);
        }
        if self.planner.order.is_some() || self.planner.scratch.is_some() {
            return self.retirement.next_owner_demands::<Vec<usize>>(true, &None, body);
        }
        if self.planner.histories.is_some() {
            return self.retirement.next_owner_demands::<Vec<bool>>(true, &None, body);
        }
        if self.planner.properties.is_some() || self.planner.enumeration_values.is_some() {
            return self.retirement.next_owner_demands::<Vec<IfcValue>>(true, &None, body);
        }
        if self.unborn.is_some() {
            return self.retirement.next_owner_demands::<Arc<IfcSnapshot>>(true, &None, body);
        }
        if let Some(source) = self.publication.view.source.as_ref().filter(|source| !source.terminal_is_empty()) {
            return source.next_close_demand(body);
        }
        Ok(RetirementDemand { depth: 1, ..Default::default() })
    }
    fn close_owned(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.publication.codec_live() {
            return self.publication.close_codec(grant);
        }
        if !self.semantic.terminal_is_empty() {
            return self.semantic.close_step(grant);
        }
        if !self.receiver.terminal_is_empty() {
            return self.receiver.close_step(grant);
        }
        if !self.retirement.is_empty() {
            return self.retirement.step_granted(grant);
        }
        macro_rules! retire {
            ($owner:expr) => {
                return self.retirement.begin_granted($owner, grant)?.ok_or_else(|| invalid("Template retained retirement omitted its original owner"));
            };
        }
        if self.original.is_some() {
            retire!(&mut self.original);
        }
        if self.planner.garbage.is_some() {
            retire!(&mut self.planner.garbage);
        }
        if self.planner.application.is_some() {
            retire!(&mut self.planner.application);
        }
        if self.planner.mutations.is_some() {
            retire!(&mut self.planner.mutations);
        }
        if self.planner.order.is_some() {
            retire!(&mut self.planner.order);
        }
        if self.planner.scratch.is_some() {
            retire!(&mut self.planner.scratch);
        }
        if self.planner.histories.is_some() {
            retire!(&mut self.planner.histories);
        }
        if self.planner.properties.is_some() {
            retire!(&mut self.planner.properties);
        }
        if self.planner.enumeration_values.is_some() {
            retire!(&mut self.planner.enumeration_values);
        }
        if self.unborn.is_some() {
            retire!(&mut self.unborn);
        }
        if let Some(source) = self.publication.view.source.as_mut().filter(|source| !source.terminal_is_empty()) {
            return source.close_step(grant);
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
}
impl ArtifactCommandWork<EditorApp<Ifc4x3AnyEditor>> for IfcTemplateWork {
    fn tool_id(&self) -> &'static str {
        APPLY_TEMPLATE_ACTION
    }
    fn extent(&self, command: &Ifc4x3EditCommand, _: &IfcSnapshot, _: &protocol::InteractionState, _: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Ifc4x3AnyEditor>>>) -> Option<usize> {
        matches!(command, Ifc4x3EditCommand::ApplyTemplate { .. }).then_some(TEMPLATE_WORK_CAPACITY.work_items())
    }
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
    fn receive_original_command(&mut self, command: &mut Ifc4x3EditCommand, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.received {
            return Ok(true);
        }
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "Template original receiving cancelled").into());
        }
        let demand = fixed(2 * size_of::<DslValue>() + size_of::<bool>());
        if cx.should_yield() || !fits(cx.retained_grant(), demand) {
            return Ok(false);
        }
        let Ifc4x3EditCommand::ApplyTemplate { application } = command else {
            return Err(invalid("Template factory received a different command").into());
        };
        self.original = Some(std::mem::replace(application, DslValue::Null));
        self.received = true;
        cx.consume_retained(receipt(demand))?;
        cx.consume_fuel(1);
        Ok(true)
    }
    fn work_demands(&self, input: &ArtifactCommandInputs<'_, EditorApp<Ifc4x3AnyEditor>>, _: usize) -> Result<RetirementDemand, ValueError> {
        if matches!(self.phase, Phase::Plan) && (self.planner.garbage.is_some() || !self.retirement.is_empty()) {
            return self.retirement.next_owner_demands::<IfcValue>(self.planner.garbage.is_some(), &None, 4096);
        }
        match self.phase {
            Phase::Receive => Ok(fixed(2 * size_of::<Option<DslValue>>() + size_of::<usize>())),
            Phase::Decode => self.receiver.demands(),
            Phase::Take => Ok(fixed(2 * size_of::<Option<IfcTemplateApplication>>() + size_of::<usize>())),
            Phase::Source => {
                let demand = SnapshotSource::borrowed_constructor_demand::<Arc<IfcSnapshot>>();
                Ok(RetirementDemand { copy_bytes: size_of::<Arc<IfcSnapshot>>(), capacity_bytes: demand.capacity_bytes, depth: demand.depth, ..Default::default() })
            }
            Phase::Plan => self.planner.demand(input.snapshot),
            Phase::Wire => self.publication.demands(),
            Phase::Semantic => {
                let source = self.publication.view.source.as_ref().ok_or_else(|| invalid("Template semantic captured source is missing"))?.try_authority()?;
                self.semantic.demands(source.as_ref(), self.planner.mutations.as_deref().unwrap())
            }
        }
    }
    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Ifc4x3AnyEditor>>, cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Ifc4x3AnyEditor>>, OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "Template preparation cancelled before publication").into());
        }
        if cx.should_yield() {
            return Ok(ArtifactCommandWorkStep::Pending { stage: "ifc-template", preview: b"" });
        }
        let demand = self.work_demands(input, 4096)?;
        if !fits(cx.retained_grant(), demand) {
            return Ok(ArtifactCommandWorkStep::Pending { stage: "ifc-template-grant", preview: b"" });
        }
        let before = cx.retained_progress();
        let fuel = cx.fuel_remaining();
        if matches!(self.phase, Phase::Plan) && (self.planner.garbage.is_some() || !self.retirement.is_empty()) {
            semio_framework_job::close_original_retained_step(cx, demand, |grant| {
                if self.retirement.is_empty() {
                    self.retirement.begin_granted(&mut self.planner.garbage, grant)?.ok_or_else(|| invalid("Template scalar envelope omitted its retirement owner"))
                } else {
                    self.retirement.step_granted(grant)
                }
            })?;
            return Ok(ArtifactCommandWorkStep::Progress { stage: "ifc-template-envelope", preview: b"" });
        }
        match self.phase {
            Phase::Receive => {
                if !self.received {
                    return Err(invalid("Template original command was not received").into());
                }
                if self.receiver.receive_owned(&mut self.original, cx)? {
                    self.phase = Phase::Decode;
                }
            }
            Phase::Decode => {
                if self.receiver.advance(cx)? {
                    self.phase = Phase::Take;
                }
            }
            Phase::Take => {
                if self.receiver.take(&mut self.planner.application, cx)? {
                    self.phase = Phase::Source;
                }
            }
            Phase::Source => {
                let owner = input.snapshot_owner.ok_or_else(|| invalid("Template preparation requires its original captured snapshot"))?;
                let grant = cx.retained_grant();
                let alias = size_of::<Arc<IfcSnapshot>>();
                let result = SnapshotSource::admit_borrowed(Arc::clone(owner), |snapshot| snapshot.as_ref(), RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - alias, ..grant });
                match result {
                    Ok((source, mut progress)) => {
                        self.publication.view.source = Some(source);
                        progress.copied_bytes += alias;
                        cx.consume_retained(progress)?;
                        cx.consume_fuel(1);
                        self.phase = Phase::Plan;
                    }
                    Err((error, original)) => {
                        self.unborn = Some(original);
                        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: alias, ..Default::default() })?;
                        return Err(error.into());
                    }
                }
            }
            Phase::Plan => {
                self.planner.advance(input.snapshot).map_err(|error| ValueError::literal(ValueRefusalKind::InvalidValue, error.code))?;
                cx.consume_retained(receipt(demand))?;
                cx.consume_fuel(1);
                if self.planner.done() {
                    independent_batch(&Reader { snapshot: input.snapshot, order: self.planner.order.as_deref().unwrap() }, self.planner.mutations.as_deref().unwrap())
                        .map_err(|error| ValueError::literal(ValueRefusalKind::InvalidValue, error.code))?;
                    self.phase = Phase::Wire;
                }
            }
            Phase::Wire => {
                if self.publication.advance(self.planner.mutations.as_deref().unwrap(), TEMPLATE_WORK_CAPACITY, cx)? {
                    self.phase = Phase::Semantic;
                }
            }
            Phase::Semantic => {
                let source = self.publication.view.source.as_ref().ok_or_else(|| invalid("Template semantic captured source is missing"))?.try_authority()?;
                self.semantic.advance(source.as_ref(), self.planner.mutations.as_deref().unwrap(), cx)?;
            }
        }
        Ok(if cx.retained_progress() != before || cx.fuel_remaining() < fuel { ArtifactCommandWorkStep::Progress { stage: "ifc-template", preview: b"" } } else { ArtifactCommandWorkStep::Pending { stage: "ifc-template", preview: b"" } })
    }
    fn begin_close(&mut self) {
        self.closing = true;
        self.receiver.begin_close();
        self.semantic.begin_close();
    }
    fn terminal_is_empty(&self) -> bool {
        self.original.is_none()
            && self.receiver.terminal_is_empty()
            && self.semantic.terminal_is_empty()
            && self.planner.application.is_none()
            && self.planner.mutations.is_none()
            && self.planner.order.is_none()
            && self.planner.scratch.is_none()
            && self.planner.histories.is_none()
            && self.planner.properties.is_none()
            && self.planner.enumeration_values.is_none()
            && self.planner.garbage.is_none()
            && self.unborn.is_none()
            && self.retirement.is_empty()
            && !self.publication.codec_live()
            && self.publication.view.source.as_ref().is_none_or(SnapshotSource::terminal_is_empty)
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand(4096)?.copy_bytes)
    }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> {
        Ok(self.close_demand(body.min(4096))?.capacity_bytes)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand(4096)?.release_bytes)
    }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand(4096)?.depth)
    }
    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<semio_framework_job::InteractiveJobCloseStep, OriginalTurnFailure> {
        if !self.closing {
            return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked);
        }
        if self.terminal_is_empty() {
            return Ok(semio_framework_job::InteractiveJobCloseStep::Complete { progress: Default::default() });
        }
        let demand = self.close_demand(cx.retained_grant().maximum_copy_bytes.min(4096))?;
        semio_framework_job::close_original_retained_step(cx, demand, |grant| self.close_owned(RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes.min(4096), ..grant }))
    }
}

#[path = "factory.rs"]
pub mod factory;

#[cfg(test)]
pub(crate) fn test_plan(original: DslValue, snapshot: &IfcSnapshot, cancel_at: Option<usize>) -> Result<Vec<IfcMutation>, String> {
    match test_planning(original,snapshot,cancel_at,false)?{TestPlanningOutcome::Mutations(value)=>Ok(value),_=>unreachable!()}
}
#[cfg(test)]
pub(crate) fn test_identity_query(original:DslValue,snapshot:&IfcSnapshot,cancel_at:Option<usize>)->Result<Vec<(usize,bool)>,String>{
    match test_planning(original,snapshot,cancel_at,true)?{TestPlanningOutcome::Identities(value)=>Ok(value),_=>unreachable!()}
}
#[cfg(test)]
enum TestPlanningOutcome{Mutations(Vec<IfcMutation>),Identities(Vec<(usize,bool)>)}
#[cfg(test)]
fn test_planning(original: DslValue, snapshot: &IfcSnapshot, cancel_at: Option<usize>,query:bool) -> Result<TestPlanningOutcome, String> {
    use semio_framework_plugin::app_testing::OriginalWorkFixture;
    let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(918), semio_framework_job::Generation(1));
    let grant = RetainedCloneGrant { maximum_items: 1_000_000, maximum_copy_bytes: 1_048_576, maximum_capacity_bytes: 1_048_576, maximum_release_bytes: 1_048_576, maximum_depth: 256 };
    let mut receiver = ApplicationReceiver::default();
    let mut input = Some(original);
    let mut planner = Planner::default();
    let mut retirement = RetainedCloneClose::default();
    let mut phase = 0;
    let mut result = None;
    for turn in 0..1_000_000 {
        if cancel_at == Some(turn) {
            caller.cancel().cancel_now();
        }
        let step = caller.work_turn(grant, 1_000_000, |cx| {
            if cx.is_cancelled() {
                return Err(ValueError::literal(ValueRefusalKind::Canceled, "template-test-cancelled").into());
            }
            match phase {
                0 => {
                    if receiver.receive_owned(&mut input, cx)? {
                        phase = 1;
                    }
                }
                1 => {
                    if receiver.advance(cx)? {
                        phase = 2;
                    }
                }
                2 => {
                    if receiver.take(&mut planner.application, cx)? {
                        phase = 3;
                    }
                }
                _ => {
                    if planner.garbage.is_some() || !retirement.is_empty() {
                        let demand = retirement.next_owner_demands::<IfcValue>(planner.garbage.is_some(), &None, 4096)?;
                        semio_framework_job::close_original_retained_step(cx, demand, |grant| {
                            if retirement.is_empty() {
                                retirement.begin_granted(&mut planner.garbage, grant)?.ok_or_else(|| invalid("Template scalar envelope omitted its owner"))
                            } else {
                                retirement.step_granted(grant)
                            }
                        })?;
                        return Ok(());
                    }
                    let demand = planner.demand(snapshot)?;
                    if !fits(cx.retained_grant(), demand) {
                        return Err(invalid("Template fixture requires a larger admitted frontier").into());
                    }
                    planner.advance(snapshot).map_err(|error| ValueError::literal(ValueRefusalKind::InvalidValue, error.code))?;
                    cx.consume_retained(receipt(demand))?;
                    cx.consume_fuel(1);
                }
            }
            Ok(())
        });
        if let Err(error) = step {
            result = Some(Err(format!("{error:?}")));
            break;
        }
        if query&&planner.identity_frontier(){
            assert!(planner.mutations.as_deref().unwrap().is_empty());
            result=Some(Ok(TestPlanningOutcome::Identities((0..planner.identity_count().unwrap()).map(|index|planner.identity_requirement(index).unwrap()).collect())));break;
        }
        if planner.done() {
            result = Some(independent_batch(&Reader { snapshot, order: planner.order.as_deref().unwrap() }, planner.mutations.as_deref().unwrap()).map(|_|TestPlanningOutcome::Mutations(planner.mutations.take().unwrap())).map_err(|error| error.to_string()));
            break;
        }
    }
    let result = result.unwrap_or_else(|| Err("Template test frontier did not terminate".into()));
    let mut work = IfcTemplateWork { original: input, receiver, planner, retirement, ..Default::default() };
    work.begin_close();
    for _ in 0..1_000_000 {
        if work.terminal_is_empty() {
            break;
        }
        caller.close(&mut work, grant).map_err(|error| format!("{error:?}"))?;
    }
    assert!(work.terminal_is_empty());
    caller.finish();
    result
}
