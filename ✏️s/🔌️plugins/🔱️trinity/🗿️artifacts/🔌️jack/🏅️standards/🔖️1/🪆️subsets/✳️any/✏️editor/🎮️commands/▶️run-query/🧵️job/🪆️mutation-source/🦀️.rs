//! 🪆️ Original Jack command and snapshot custody for qualified child mutations.

use super::*;
use crate::standards::v1::subsets::any::schema::{composition::{JackChildQualification, JackComposition}, graph::GraphEffect};
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{ErasedSnapshotRetirement, RetainedAllocationEffect, ValueError, retained_clone::{RetainedCloneSource, StagedSourceAdmission}, retirement::controlled::ControlledRetirement};
use store::os_store::snapshot_read::{SnapshotRead, SnapshotReadSourceStaging};

type PreparedEdit = (QueryResult, Vec<ChildEmitPreparation>, Option<TrinityGraphMutation>);
type CommandAdmission = StagedSourceAdmission<TrinityJackCommand, TrinityJackCommand>;

#[derive(Default, semio_framework_value::RetireOwned)]
struct OriginalSources {
    pending: Option<(QueryResult, Option<TrinityGraphMutation>)>,
    effects: Vec<GraphEffect>,
    snapshot: Option<RetainedCloneSource<JackSnapshot, SnapshotRead<JackSnapshot>>>,
    snapshot_staging: SnapshotReadSourceStaging<JackSnapshot>,
    snapshot_read: Option<SnapshotRead<JackSnapshot>>,
    command: Option<RetainedCloneSource<TrinityJackCommand>>,
    command_staging: Option<CommandAdmission>,
}

pub(super) struct QueryMutationSource<C: JackComposition> {
    sources: ControlledRetirement<OriginalSources>,
    qualification: JackChildQualification<C>,
}

impl<C: JackComposition> QueryMutationSource<C> {
    pub(super) fn new() -> Self {
        Self { sources: ControlledRetirement::retain(OriginalSources::default()), qualification: JackChildQualification::new() }
    }

    pub(super) fn receive_command(&mut self, command: &mut TrinityJackCommand, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let sources = self.sources.original_mut().ok_or_else(|| ValueError::invariant("Jack query source is closing"))?;
        if let Some(staged) = sources.command_staging.as_mut() {
            source_turn(cx, staged.next_owned_demand()?, staged.next_owned_work_demand()?, |effect| staged.advance_owned(&mut sources.command, effect))?;
            if staged.terminal_is_empty() {
                sources.command_staging = None;
            }
            return Ok(sources.command.is_some() && sources.command_staging.is_none());
        }
        if sources.command.is_some() {
            return Ok(true);
        }
        let placeholder = match command {
            TrinityJackCommand::RunQuery { .. } => TrinityJackCommand::RunQuery { query: None, results_window_id: String::new() },
            TrinityJackCommand::LoadExampleQuery { .. } => TrinityJackCommand::LoadExampleQuery { query: String::new(), results_window_id: String::new() },
            _ => return Err(ValueError::invariant("Jack query received a different original command").into()),
        };
        let demand = RetirementDemand { copy_bytes: size_of::<TrinityJackCommand>() + size_of::<Option<CommandAdmission>>(), depth: 1, ..Default::default() };
        source_turn(cx, demand, 1, |effect| {
            let result = effect.performed(1).map(|paid| {
                if !paid {
                    return RetainedCloneStep::Progress(Default::default());
                }
                let original = std::mem::replace(command, placeholder);
                sources.command_staging = Some(StagedSourceAdmission::owned(original));
                RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })
            });
            RetainedWorkOutcome { result, work: effect.performed_work() }
        })?;
        Ok(false)
    }

    pub(super) fn pending(&self) -> bool {
        self.sources.original().is_some_and(|sources| sources.pending.is_some())
    }

    pub(super) fn stage(&mut self, result: QueryResult, effects: Vec<GraphEffect>, root: Option<TrinityGraphMutation>) {
        let sources = self.sources.original_mut().expect("Jack query mutation source remains active");
        assert!(sources.pending.is_none());
        sources.pending = Some((result, root));
        sources.effects = effects;
    }

    pub(super) fn advance(&mut self, input: &ArtifactCommandInputs<'_, Owner<C>>, cx: &mut StepContext<'_>) -> Result<Option<PreparedEdit>, OriginalTurnFailure> {
        let sources = self.sources.original_mut().ok_or_else(|| ValueError::invariant("Jack query mutation source is closing"))?;
        if sources.snapshot.is_none() || !sources.snapshot_staging.terminal_is_empty() {
            if !sources.snapshot_staging.is_staged() {
                let Some(original) = input.snapshot_read.take_current(input.operation.generation, input.operation.canonical_base_revision, cx)? else { return Ok(None) };
                if let Err((error, original)) = sources.snapshot_staging.begin(original) {
                    sources.snapshot_read = Some(original);
                    return Err(error.into());
                }
            } else {
                let staging = &mut sources.snapshot_staging;
                source_turn(cx, staging.next_demand()?, staging.next_advance_work_demand()?, |effect| staging.step(&mut sources.snapshot, effect))?;
            }
            return Ok(None);
        }

        let parent = sources.snapshot.as_ref().unwrap().try_borrow()?;
        let command = sources.command.as_ref().ok_or_else(|| ValueError::invariant("Jack query mutation has no original command"))?.try_borrow()?;
        let children = &input.context.ok_or_else(content_missing)?.children;
        let source = input.snapshot_read.source.ok_or_else(|| ValueError::invariant("Jack query mutation has no original source binding"))?;
        let Some(preparations) = self.qualification.step(parent, command, children, input.operation, source, &sources.effects, cx)? else { return Ok(None) };
        let (result, root) = sources.pending.take().expect("Jack query retains its prepared edit");
        Ok(Some((result, preparations, root)))
    }

    pub(super) fn begin_close(&mut self) {
        self.qualification.begin_close();
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.qualification.terminal_is_empty() {
            self.qualification.close_demands(body)
        } else {
            self.sources.next_demand(body)
        }
    }

    pub(super) fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if !self.qualification.terminal_is_empty() {
            return self.qualification.close_turn(cx);
        }
        let body = cx.retained_grant().maximum_copy_bytes;
        let demand = self.sources.next_demand(body)?;
        let birth = self.sources.requires_original_birth();
        if !birth && !self.sources.requires_original_control() {
            return semio_framework_job::close_original_effect_step(cx, demand, |effect| self.sources.step(body, effect));
        }
        let pending = || InteractiveJobCloseStep::Pending { progress: Default::default() };
        if cx.retirement_should_yield() || !cx.retained_work_can_enter(demand) {
            return Ok(pending());
        }
        let grant = cx.retained_grant();
        let Some((poll, _)) = cx.with_original_source_effect(grant, |cx, effect| {
            cx.with_native_retirement(|native| native.with_original_close(|original| Ok::<_, ValueError>(
                if birth { self.sources.step_original_birth(original, grant) }
                else { self.sources.close_original_step(original, body, effect) },
            )))
        })? else { return Ok(pending()) };
        let turn = match poll? {
            RetainedWorkPoll::Pending { work } => RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work },
            RetainedWorkPoll::Complete(turn) => turn,
        };
        let progress = semio_framework_job::receive_original_retained_work(cx, turn)?.progress();
        Ok(if self.terminal_is_empty() { InteractiveJobCloseStep::Complete { progress } } else { InteractiveJobCloseStep::Pending { progress } })
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.qualification.terminal_is_empty() && self.sources.terminal_is_empty()
    }
}

fn source_turn(
    cx: &mut StepContext<'_>, demand: RetirementDemand, work: u64,
    advance: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome,
) -> Result<(), OriginalTurnFailure> {
    if cx.should_yield() || cx.is_cancelled() || !cx.retained_work_can_enter(demand) || cx.fuel_remaining() < work {
        return Ok(());
    }
    let Some((turn, performed)) = cx.with_original_source_effect(cx.retained_grant(), |_, effect| advance(effect))? else { return Ok(()) };
    cx.consume_work(performed);
    match turn.result {
        Ok(step) => cx.consume_retained(step.progress()).map_err(Into::into),
        Err(error) => Err(OriginalTurnFailure::debit_original(error, cx)),
    }
}
