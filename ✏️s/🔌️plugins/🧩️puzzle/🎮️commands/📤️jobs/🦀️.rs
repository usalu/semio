//! 🧵️ The job-side lifecycle every puzzle job shares: a bounded outbox that lends payload outcomes, a staged owner bundle that retires through the controlled owner, and counted ladders for engine-owned scaffolds.
#![allow(clippy::result_large_err)]


use semio_framework_job_publication::RetainedJobPublication;
use semio_framework_job_publication::JobPublicationKind;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use semio_framework_job::InteractiveJobCloseStep;

use semio_framework_job::JobOutcomeBorrow;

use semio_framework_job::JobOutcomeDescriptor;

use semio_framework_job::JobOutcomeKind;

use semio_framework_job::JobOutcomeView;

use semio_framework_job::JobPayloadStream;

use semio_framework_job::RetainedPayloadBuilder;

use semio_framework_job::StepContext;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;

/// ♻️ Moves a work's remaining owners into one derived bundle and retires it through the controlled owner, one granted turn at a time.
pub struct WorkClosing<T: RetireOwned + 'static> {
    staged: std::mem::ManuallyDrop<Option<T>>,
    retirement: std::mem::ManuallyDrop<Option<ControlledRetirement<T>>>,
}

impl<T: RetireOwned + 'static> Default for WorkClosing<T> {
    fn default() -> Self {
        Self { staged: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
    }
}

impl<T: RetireOwned + 'static> WorkClosing<T> {
    /// 🧺️ Parks the work's remaining owners; the first granted turn births their controlled retirement.
    pub fn stage(&mut self, owners: T) {
        debug_assert!(self.staged.is_none() && self.retirement.is_none(), "a work stages its remaining owners exactly once");
        *self.staged = Some(owners);
    }

    pub fn is_empty(&self) -> bool {
        self.staged.is_none() && self.retirement.is_none()
    }

    /// 📏️ Quotes the single turn the next close step spends.
    pub fn demands(&self, copy: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(owner) = self.retirement.as_ref() {
            return Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(copy)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? });
        }
        Ok(RetirementDemand { depth: usize::from(self.staged.is_some()), ..Default::default() })
    }

    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        if self.is_empty() { return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() }); }
        let demand = self.demands(cx.retained_grant().maximum_copy_bytes)?;
        let step = semio_framework_job::close_original_retained_step(cx, demand, |grant| {
            if let Some(owner) = self.retirement.as_mut() {
                let step = owner.step(grant)?;
                if owner.terminal_is_empty() { self.retirement.take(); }
                return Ok(step);
            }
            let value = self.staged.take().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "puzzle close lost its staged original owners"))?;
            match ControlledRetirement::new(value) {
                Ok(owner) => {
                    *self.retirement = Some(owner);
                    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
                }
                Err((error, value)) => { *self.staged = Some(value); Err(error) }
            }
        })?;
        Ok(if self.is_empty() { InteractiveJobCloseStep::Complete { progress: step.progress() } } else { step })
    }
}

impl<T: RetireOwned + 'static> Drop for WorkClosing<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.is_empty(), "a puzzle command work must retire its remaining owners before it drops");
    }
}

/// 📤️ What one turn of a puzzle job settles: a lent control outcome, or bytes the job must publish before its next turn.
/// 📎️ `Prepare` stages the reserved route's raw wire as the retained commit output the host compares with the wire it admitted; the job completes only once [`JobOutbox::commit_prepared`] holds, so a build refusal can never follow a delivered completion.
#[derive(Debug, PartialEq, Eq)]
pub enum JobTurn {
    Yield,
    Cancelled,
    Complete,
    Prepare(Vec<u8>),
    Preview(Vec<u8>),
    Checkpoint { applied_progress: u64, state: Vec<u8> },
    Fault(Vec<u8>),
}

impl JobTurn {
    /// 🏁️ Whether the job is done after this turn.
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Complete | Self::Cancelled | Self::Fault(_))
    }
}

/// 🧭️ Where one publication stands between the job and its driver.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutboxPhase {
    Idle,
    Building,
    Delivered,
    Retiring,
}

/// 📤️ One original payload source and the publication that lends it; the source never moves while the loan lives.
pub struct JobOutbox {
    publication: RetainedJobPublication,
    commit: RetainedPayloadBuilder,
    commit_cursor: usize,
    committing: bool,
    source: Vec<u8>,
    kind: Option<JobPublicationKind>,
    retiring: bool,
}

impl Default for JobOutbox {
    fn default() -> Self {
        Self { publication: RetainedJobPublication::default(), commit: RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput), commit_cursor: 0, committing: false, source: Vec::new(), kind: None, retiring: false }
    }
}

impl JobOutbox {
    /// 📎️ Whether the staged commit output is sealed and will ride on the job's completion.
    pub fn commit_prepared(&self) -> bool {
        self.commit.published().is_some()
    }

    pub fn phase(&self, cx: &StepContext<'_>) -> Result<OutboxPhase, ValueError> {
        if self.committing && !self.commit_prepared() {
            return Ok(OutboxPhase::Building);
        }
        let Some(kind) = self.kind else { return Ok(OutboxPhase::Idle) };
        if self.retiring {
            return Ok(OutboxPhase::Retiring);
        }
        let demand = self.publication.advance_demands(kind, &self.source, cx)?;
        let delivered = self.publication.published().is_some() && demand.copy_bytes == 0 && demand.capacity_bytes == 0 && demand.release_bytes == 0 && demand.depth == 0;
        Ok(if delivered { OutboxPhase::Delivered } else { OutboxPhase::Building })
    }

    pub fn advance<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        if self.committing {
            if !self.commit.is_initialized() {
                self.commit.advance_initialization(cx)?;
            } else if self.commit_cursor < self.source.len() {
                self.commit.append_original(cx, &self.source, &mut self.commit_cursor)?;
            } else {
                self.commit.seal(cx)?;
            }
            return Ok(None);
        }
        let kind = self.kind.expect("a building outbox owns its publication kind");
        self.publication.advance_from_source(kind, &self.source, cx)
    }

    pub fn retire_step(&mut self, cx: &mut StepContext<'_>) -> Result<(), ValueError> {
        self.retiring = true;
        match self.publication.close_step(cx.retained_grant())? {
            RetainedCloneStep::Progress(progress) => cx.consume_retained(progress),
            RetainedCloneStep::Complete(progress) => {
                cx.consume_retained(progress)?;
                self.kind = None;
                self.retiring = false;
                self.source = Vec::new();
                Ok(())
            }
        }
    }

    /// 🎫️ Settles a turn: control outcomes are lent at once, payload outcomes stage their bytes for the next paid turns.
    pub fn settle<'a>(&'a mut self, turn: JobTurn, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        let (kind, source) = match turn {
            JobTurn::Yield => return JobOutcomeBorrow::admit_yield(cx),
            JobTurn::Cancelled => return JobOutcomeBorrow::admit_cancelled(cx),
            JobTurn::Complete => return JobOutcomeBorrow::admit_complete(cx, None, self.commit.published()),
            JobTurn::Prepare(bytes) => {
                self.source = bytes;
                self.committing = true;
                cx.consume_fuel(1);
                return Ok(None);
            }
            JobTurn::Preview(bytes) => (JobPublicationKind::Preview, bytes),
            JobTurn::Checkpoint { applied_progress, state } => (JobPublicationKind::Checkpoint { applied_progress }, state),
            JobTurn::Fault(bytes) => (JobPublicationKind::Fault, bytes),
        };
        self.source = source;
        self.kind = Some(kind);
        cx.consume_fuel(1);
        Ok(None)
    }

    pub fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete => descriptor.complete(None, self.commit.published()),
            JobOutcomeKind::PreviewReady | JobOutcomeKind::CheckpointReady { .. } | JobOutcomeKind::Fault => self.publication.borrow_outcome(descriptor),
        }
    }

    /// ♻️ Closes the lent publication first, then the staged commit pages, then clears the staging flags.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if self.kind.is_some() || !self.publication.terminal_is_empty() {
            self.retiring = true;
            let step = self.publication.close_step(grant)?;
            if self.publication.terminal_is_empty() {
                self.kind = None;
                self.retiring = false;
                if !self.committing {
                    self.source = Vec::new();
                }
            }
            return Ok(step);
        }
        if !self.commit.terminal_is_empty() {
            let step = self.commit.close_step_granted(grant)?;
            if self.commit.terminal_is_empty() {
                self.commit_cursor = 0;
                self.committing = false;
                self.source = Vec::new();
            }
            return Ok(step);
        }
        self.commit_cursor = 0;
        self.committing = false;
        self.source = Vec::new();
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))
    }

    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.kind.is_some() || !self.publication.terminal_is_empty() {
            return self.publication.retirement_demands();
        }
        if !self.commit.terminal_is_empty() {
            return self.commit.retirement_demands();
        }
        Ok(RetirementDemand::default())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.kind.is_none() && !self.committing && self.publication.terminal_is_empty() && self.commit.terminal_is_empty()
    }
}

/// ♻️ Closes a job: the outbox first, then the staged owner bundle.
pub fn job_close_step<T: RetireOwned + 'static>(outbox: &mut JobOutbox, owners: &mut WorkClosing<T>, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
    if !outbox.terminal_is_empty() {
        let demand = outbox.retirement_demands()?;
        return semio_framework_job::close_original_retained_step(cx, demand, |grant| outbox.close_step(grant));
    }
    owners.close_step(cx)
}

/// 📏️ Quotes the next close turn of a job: the outbox first, then the staged owner bundle.
pub fn job_close_demands<T: RetireOwned + 'static>(outbox: &JobOutbox, owners: &WorkClosing<T>, copy: usize) -> Result<RetirementDemand, ValueError> {
    if !outbox.terminal_is_empty() {
        return outbox.retirement_demands();
    }
    owners.demands(copy)
}

//#region 🌐️BrowserCaller
/// 🌐️ The original caller of a browser handle: the page's JavaScript is the outermost caller, so the handle owns one cancellation root, one payload authority and one source parent, lends a fresh scheduled context per exported call and pays their terminal retirement itself.
pub struct BrowserCaller {
    parent: std::mem::ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceParent>>,
    sequence: u64,
    turn_grant: RetainedCloneGrant,
}

impl BrowserCaller {
    /// 🚪️ Admits the roots from `turn_grant`, the same physical authority every later call is scheduled under.
    pub fn admit(turn_grant: RetainedCloneGrant) -> Result<Self, ValueError> {
        let refused = |kind, message: &'static str| ValueError::literal(kind, message);
        let (cancel, root) = semio_framework_job::CancelToken::admit_root(turn_grant)?.ok_or_else(|| refused(semio_framework_value::ValueRefusalKind::OwnershipLimit, "browser caller cancellation root is not funded"))?;
        let (authority, ledger) = semio_framework_job::JobPayloadAuthority::admit(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), Self::remaining(turn_grant, root)?)?.ok_or_else(|| refused(semio_framework_value::ValueRefusalKind::OwnershipLimit, "browser caller payload authority is not funded"))?;
        let (parent, _) = semio_framework_job::OriginalCallerSourceParent::admit(authority, cancel, Self::remaining(turn_grant, root.checked_add(ledger)?)?).map_err(|(error, authority, cancel)| {
            std::mem::forget((authority, cancel));
            error
        })?;
        Ok(Self { parent: std::mem::ManuallyDrop::new(Some(parent)), sequence: 0, turn_grant })
    }

    fn remaining(stock: RetainedCloneGrant, spent: RetainedCloneProgress) -> Result<RetainedCloneGrant, ValueError> {
        if !spent.fits(stock) {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "browser caller admissions exceed their initiating grant"));
        }
        Ok(RetainedCloneGrant { maximum_items: stock.maximum_items - spent.copied_items, maximum_copy_bytes: stock.maximum_copy_bytes - spent.copied_bytes, maximum_capacity_bytes: stock.maximum_capacity_bytes - spent.retained_capacity_bytes, maximum_release_bytes: stock.maximum_release_bytes - spent.released_bytes, ..stock })
    }

    /// 🪢️ Lends one fresh maintenance-lane context to `run`; it ends before the call returns, so the next call starts from a new fuel, deadline and receipt.
    pub fn turn<R>(&mut self, run: impl FnOnce(&mut StepContext<'_>) -> R) -> Result<R, ValueError> {
        let refused = |message: &'static str| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, message);
        let parent = self.parent.as_ref().ok_or_else(|| refused("browser caller has already retired"))?;
        let now = semio_framework_job::default_now_us().ok_or_else(|| refused("browser caller has no monotonic clock"))?;
        let budget = semio_framework_job::StepBudget::from_duration(semio_framework_job::MAINTENANCE_LANE_FUEL, now, semio_framework_job::MAINTENANCE_LANE_WALL_US, self.turn_grant).ok_or_else(|| refused("browser caller deadline leaves the clock domain"))?;
        let mut receipt = RetainedCloneProgress::default();
        let mut cx = parent.borrow_context(budget, semio_framework_job::default_now_us, &mut self.sequence, &mut receipt)?;
        Ok(run(&mut cx))
    }

    /// 🧹️ Pays one terminal retirement turn quoted by the roots themselves; true once nothing remains.
    pub fn retire_step(&mut self) -> Result<bool, ValueError> {
        let Some(parent) = self.parent.as_mut() else { return Ok(true) };
        if !parent.terminal_is_empty() {
            let demand = parent.retirement_demands()?;
            let _ = parent.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) })?;
        }
        if !parent.terminal_is_empty() {
            return Ok(false);
        }
        self.parent.take();
        Ok(true)
    }
}

impl Drop for BrowserCaller {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.parent.is_none(), "browser caller requires paid terminal retirement");
    }
}
//#endregion 🌐️BrowserCaller

/// 🧪️ Test seams that drive a puzzle job the way its production wiring does.
#[cfg(any(test, feature = "testing"))]
pub mod testing {
    use super::*;
    use std::mem::size_of;
    use semio_framework_job::InteractiveJob;

    /// 🧾️ Explicit finite original source stock shared by the authored Puzzle fixture corpus.
    pub fn fixture_stock() -> RetainedCloneGrant {
        RetainedCloneGrant { maximum_items: 1 << 24, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 4096 }
    }

    /// 🪪️ One fixture keeps the original roots, physical decoder, clock and cumulative source stock through every work and close turn.
    pub struct OriginalFixture {
        caller: Option<semio_framework_job::OriginalJobFixtureCaller>,
        decoder: semio_framework_job::OriginalJobDecodeOwner,
    }

    impl OriginalFixture {
        pub fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, source: RetainedCloneGrant, clock: fn() -> Option<u64>, deadline: u64, fuel: u64) -> Self {
            Self { caller: Some(semio_framework_job::OriginalJobFixtureCaller::admit(operation, generation, source, clock, deadline, fuel)), decoder: semio_framework_job::OriginalJobDecodeOwner::empty() }
        }

        pub fn turn<R>(&mut self, grant: RetainedCloneGrant, advance: impl FnOnce(&mut StepContext<'_>) -> Result<R, semio_framework_job::OriginalTurnFailure>) -> Result<R, semio_framework_job::OriginalTurnFailure> {
            self.caller.as_mut().expect("original fixture remains live").turn_with_decoder(grant, &mut self.decoder, advance)
        }

        pub fn construct<T>(&mut self, demand: RetirementDemand, construct: impl FnOnce(&mut semio_framework::action_bus::OriginalToolConstructionScope<'_>) -> Result<(T, RetainedCloneProgress), ValueError>) -> T {
            let mut output = None;
            let demand = semio_framework::action_bus::ToolJobAdmissionDemand { items: 1, copy_bytes: demand.copy_bytes, capacity_bytes: demand.capacity_bytes, release_bytes: demand.release_bytes, depth: demand.depth.max(1) };
            assert!(self.turn(fixture_stock(), |cx| semio_framework::action_bus::admit_original_tool_construction_scoped(cx, &mut output, demand, |scope| construct(scope).map(|(candidate, mut progress)| { progress.copied_items = progress.copied_items.max(1); (candidate, progress) }))).expect("original Puzzle fixture construction"));
            output.expect("original Puzzle candidate remains in fixture custody")
        }

        pub fn construct_boxed<T>(&mut self, extra: RetirementDemand, construct: impl FnOnce(&mut semio_framework::action_bus::OriginalToolConstructionScope<'_>) -> Result<(T, RetainedCloneProgress), ValueError>) -> Box<T> {
            let frame = RetirementDemand { capacity_bytes: size_of::<T>(), depth: 1, ..Default::default() };
            let demand = RetirementDemand { capacity_bytes: extra.capacity_bytes.checked_add(frame.capacity_bytes).expect("original fixture typed frame extent"), depth: extra.depth.max(1), ..extra };
            self.construct(demand, |scope| {
                scope.admit_owner(frame)?;
                let (work, mut progress) = construct(scope)?;
                let work = Box::new(work);
                progress.retained_capacity_bytes = progress.retained_capacity_bytes.checked_add(size_of::<T>()).expect("original fixture typed frame receipt");
                Ok((work, progress))
            })
        }

        pub fn cancel(&self) -> &semio_framework_job::CancelToken { self.caller.as_ref().expect("original fixture remains live").cancel() }

        pub fn close_job<J: InteractiveJob + ?Sized>(&mut self, job: &mut J) -> usize {
            job.begin_close();
            for turn in 0..1_000_000 {
                let step = self.turn(fixture_stock(), |cx| job.close_step(cx)).expect("original Puzzle job close");
                if matches!(step, InteractiveJobCloseStep::Complete { .. }) { assert!(job.terminal_is_empty()); return turn; }
            }
            panic!("original Puzzle job close remains pending");
        }

        pub fn finish(mut self) {
            let caller = self.caller.as_mut().expect("original fixture remains live");
            while !self.decoder.terminal_is_empty() {
                caller.turn(fixture_stock(), |cx| {
                    let demand = self.decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
                    semio_framework_job::close_original_retained_step(cx, demand, |grant| self.decoder.close_step(grant))?;
                    Ok(())
                }).expect("original Puzzle decoder close");
            }
            self.caller.take().expect("original fixture closes once").retire(fixture_stock());
        }
    }

    impl Drop for OriginalFixture {
        fn drop(&mut self) { assert!(self.caller.is_none() || std::thread::panicking(), "original Puzzle fixture must close every source root"); }
    }

    fn payload_bytes(payload: &semio_framework_job::RetainedJobPayload) -> Vec<u8> {
        (0..payload.page_count()).flat_map(|index| payload.page(index).unwrap_or_default().to_vec()).collect()
    }

    /// 📖️ Reads one lent outcome as owned bytes, the shape [`drive`] answers and per-step laws inspect after `drive_step`.
    pub fn owned_turn(outcome: JobOutcomeBorrow<'_>) -> JobTurn {
        match outcome {
            JobOutcomeBorrow::Yield { .. } => JobTurn::Yield,
            JobOutcomeBorrow::Cancelled { .. } => JobTurn::Cancelled,
            JobOutcomeBorrow::Complete { .. } => JobTurn::Complete,
            JobOutcomeBorrow::PreviewReady { payload, .. } => JobTurn::Preview(payload_bytes(payload)),
            JobOutcomeBorrow::CheckpointReady { state, applied_progress, .. } => JobTurn::Checkpoint { applied_progress, state: payload_bytes(state) },
            JobOutcomeBorrow::Fault { detail, .. } => JobTurn::Fault(payload_bytes(detail)),
        }
    }

    /// ▶️ Steps `job` through its production `step` — paying turns included — until it lends one outcome, and reads that outcome as owned bytes.
    pub fn drive<J: InteractiveJob + ?Sized>(job: &mut J, budget: semio_framework_job::StepBudget, original: &mut OriginalFixture) -> JobTurn {
        drive_output(job, budget, original).0
    }

    /// 📤️ The same original caller funds every paying step and lends the exact completed output.
    pub fn drive_output<J: InteractiveJob + ?Sized>(job: &mut J, budget: semio_framework_job::StepBudget, original: &mut OriginalFixture) -> (JobTurn, Option<Vec<u8>>) {
        for _ in 0..1_000_000 {
            let mut output = None;
            original.turn(budget.retained, |cx| cx.with_original_work_loan(budget.fuel, budget.deadline_us, |cx| {
                output = match job.step(cx)? {
                    None => None,
                    Some(JobOutcomeBorrow::Complete { output, .. }) => Some((JobTurn::Complete, output.map(payload_bytes))),
                    Some(outcome) => Some((owned_turn(outcome), None)),
                };
                Ok(())
            })).expect("original Puzzle job step");
            if let Some(output) = output { return output; }
        }
        panic!("original Puzzle job lends an outcome within its bounded fixture turns");
    }

    /// 🎟️ One close or maintenance turn funded exactly by what the owner quoted, capped at `items` items.
    pub fn funded_turn(demand: RetirementDemand, items: usize) -> RetainedCloneGrant {
        RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
    }

    /// 🧾️ Takes every ownership receipt, progress page, composed result and local-interaction reply a publication turn left in the outboxes of `app`, the way the framework's registered settle does, so the next turn is not held back by an unread one.
    pub fn drain_outboxes<A: semio_framework_plugin::PluginApp + ?Sized>(app: &mut A) {
        let _ = (app.take_constructor_receipt(), app.take_typed_operation_preparation_receipt(), app.take_typed_operation_retirement_receipt(), app.take_history_command_receipt(), app.take_tool_run_receipt());
        while app.take_typed_operation_ui_progress().is_some() {}
        while app.take_typed_operation_composed_result().is_some() {}
        while let Some(reply) = app.take_local_interaction_query_reply() {
            if let protocol::LocalInteractionQueryReply::Page { page } = reply {
                let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
                let _ = app.acknowledge_local_interaction_query(&token);
            }
        }
    }

    /// 🧹️ Maintenance borrows one exact quoted loan from the existing original fixture source.
    pub fn maintain_items<A: semio_framework_plugin::PluginApp + ?Sized>(original: &mut OriginalFixture, app: &mut A, items: usize, body: usize) -> Result<semio_framework_plugin::PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        let demand = app.maintenance_retirement_demands(body)?;
        let grant = funded_turn(demand, items);
        original.turn(grant, |cx| { let loan = cx.retained_grant(); app.maintenance_step(cx, loan).map_err(semio_framework_job::OriginalTurnFailure::from) }).map_err(semio_framework_plugin::PluginLifecycleFailure::from)
    }

    /// 🧼️ A single maintenance unit consumes the same original source and clock.
    pub fn maintain<A: semio_framework_plugin::PluginApp + ?Sized>(original: &mut OriginalFixture, app: &mut A, body: usize) -> Result<semio_framework_plugin::PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        maintain_items(original, app, 1, body)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_job::InteractiveJob;

    #[test]
    fn original_fixture_policy_matches_the_shared_source_stock() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🎟️source/🔣️.json")).unwrap();
        let stock = testing::fixture_stock();
        assert_eq!(stock.maximum_items, fixture["stock"]["items"].as_u64().unwrap() as usize);
        assert_eq!(stock.maximum_copy_bytes, fixture["stock"]["copyBytes"].as_u64().unwrap() as usize);
        assert_eq!(stock.maximum_capacity_bytes, fixture["stock"]["capacityBytes"].as_u64().unwrap() as usize);
        assert_eq!(stock.maximum_release_bytes, fixture["stock"]["releaseBytes"].as_u64().unwrap() as usize);
        assert_eq!(stock.maximum_depth, fixture["stock"]["depth"].as_u64().unwrap() as usize);
    }

    struct EchoJob {
        raw: Vec<u8>,
        outbox: JobOutbox,
        closing: bool,
        owners: WorkClosing<Vec<u8>>,
    }

    impl EchoJob {
        fn new(raw: Vec<u8>) -> Self {
            Self { raw, outbox: JobOutbox::default(), closing: false, owners: WorkClosing::default() }
        }

        fn turn(&mut self) -> JobTurn {
            if !self.raw.is_empty() {
                return JobTurn::Prepare(std::mem::take(&mut self.raw));
            }
            JobTurn::Complete
        }
    }

    impl EchoJob {
    /// 🪜️ Runs the authored body under the original caller before the trait boundary returns its failure.
    fn step_value<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
            match self.outbox.phase(cx)? {
                OutboxPhase::Building => return self.outbox.advance(cx).map_err(semio_framework_job::OriginalTurnFailure::from),
                OutboxPhase::Delivered | OutboxPhase::Retiring => {
                    self.outbox.retire_step(cx)?;
                    return Ok(None);
                }
                OutboxPhase::Idle => {}
            }
            let turn = self.turn();
            self.outbox.settle(turn, cx).map_err(semio_framework_job::OriginalTurnFailure::from)
        }
}

impl InteractiveJob for EchoJob {
        fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}

        fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
            self.outbox.borrow_outcome(descriptor)
        }

        fn begin_close(&mut self) {
            if !std::mem::replace(&mut self.closing, true) {
                self.owners.stage(std::mem::take(&mut self.raw));
            }
        }

        fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
            self.begin_close();
            job_close_step(&mut self.outbox, &mut self.owners, cx)
        }

        fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
            Ok(job_close_demands(&self.outbox, &self.owners, 0)?.copy_bytes)
        }

        fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError> {
            Ok(job_close_demands(&self.outbox, &self.owners, maximum_copy_bytes)?.capacity_bytes)
        }

        fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
            Ok(job_close_demands(&self.outbox, &self.owners, 0)?.release_bytes)
        }

        fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
            Ok(job_close_demands(&self.outbox, &self.owners, 0)?.depth)
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.outbox.terminal_is_empty() && self.owners.is_empty()
        }
    }

    /// 🌐️ Each browser call borrows fresh context over the same roots and pays their terminal retirement.
    #[test]
    fn a_browser_caller_lends_fresh_contexts_over_one_root_and_retires_in_quoted_turns() {
        let grant = RetainedCloneGrant { maximum_items: 32, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 16_777_216, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mut caller = BrowserCaller::admit(grant).expect("the roots are funded by the mounted turn grant");
        let first = caller.turn(|cx| (cx.operation(), cx.generation(), cx.fuel_remaining(), cx.retained_grant())).expect("first call context");
        let second = caller.turn(|cx| (cx.operation(), cx.generation(), cx.fuel_remaining(), cx.retained_grant())).expect("second call context");
        assert_eq!(first, second, "each call starts from the same identity, fuel and grant");
        assert_eq!(first.3, grant);
        let mut turns = 0;
        while !caller.retire_step().expect("a quoted retirement turn") {
            turns += 1;
            assert!(turns < 1_024, "the roots retire in a bounded number of turns");
        }
        assert!(caller.retire_step().expect("retirement is idempotent"));
        assert!(caller.turn(|_| ()).is_err(), "a retired caller lends no context");
    }

    /// 📎️ The host compares a reserved route's retained commit output with the wire it admitted: a job that holds the wire answers it back on completion, and an empty wire needs no output.
    #[test]
    fn a_prepared_commit_output_rides_on_the_completion_and_closes_exactly() {
        for raw in [Vec::new(), b"puzzle reserved wire".to_vec(), vec![0xA5; 1_000]] {
            let mut job = EchoJob::new(raw.clone());
            let mut original = testing::OriginalFixture::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), testing::fixture_stock(), semio_framework_job::default_now_us, u64::MAX, 1 << 24);
            let budget = semio_framework_job::StepBudget::new(1, u64::MAX, testing::fixture_stock());
            let (turn, output) = testing::drive_output(&mut job, budget, &mut original);
            assert_eq!(turn, JobTurn::Complete);
            assert_eq!(output, (!raw.is_empty()).then_some(raw), "the completion carries exactly the wire the job was admitted with");
            original.close_job(&mut job);
            original.finish();
        }
    }
}
