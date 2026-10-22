//! 🪞️ Window preview run: the held presses' window-config leaves ride the window config owners' retained preview pipeline one granted step at a time and end as each window's provisional config, never history. See [`super`].

use semio_framework_plugin_window_config::{registry::fits, WindowConfigMutation, WindowConfigOwnerRegistry};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedOwnedProjection};
use semio_framework_value::{OwnedValueRetirementFactory, RetirementDemand, ValueError, ValueRefusalKind};
use std::{collections::VecDeque, sync::Arc};
use store::{ArtifactMutationSource, ArtifactOwnedValueRetirementFactory};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    Idle,
    Feeding,
    Finishing,
    Cancelling,
}

#[derive(Clone, Copy, Debug)]
enum Action {
    Begin,
    TakeLeaf,
    SourceAdmit,
    SourceAdvance,
    SourceTake,
    Input,
    Drain,
    ProjectionClose,
    Wait(usize),
    SourceBeginClose,
    SourceClose,
    LeafDone,
    StartFinish,
    Partition(usize, bool),
    Restart,
    CountPass,
    Settle,
}

struct LeafFeed {
    kind: String,
    window: String,
    original: Option<WindowConfigMutation>,
    source: Option<ArtifactMutationSource<WindowConfigMutation>>,
    projection: Option<RetainedOwnedProjection<WindowConfigMutation>>,
    admitted: bool,
    source_closing: bool,
}

/// 🪞️ One retained refold of every window's provisional config; a request while a run is open coalesces latest-wins and starts when the run settles.
pub struct WindowPreviewRun {
    epoch: u64,
    phase: Phase,
    wanted: Option<Vec<WindowConfigMutation>>,
    latest: Vec<WindowConfigMutation>,
    queue: VecDeque<WindowConfigMutation>,
    feed: Option<LeafFeed>,
    targeted: bool,
    published: bool,
    pass: usize,
    count: Option<usize>,
    factory: Arc<dyn ArtifactOwnedValueRetirementFactory<WindowConfigMutation>>,
}

impl Default for WindowPreviewRun {
    fn default() -> Self {
        Self { epoch: 0, phase: Phase::Idle, wanted: None, latest: Vec::new(), queue: VecDeque::new(), feed: None, targeted: false, published: false, pass: 0, count: None, factory: Arc::new(OwnedValueRetirementFactory::<WindowConfigMutation>::default()) }
    }
}

fn header<T>() -> RetirementDemand {
    RetirementDemand { copy_bytes: size_of::<T>(), depth: 1, ..Default::default() }
}

fn bookkeeping(demand: RetirementDemand) -> RetainedCloneStep {
    RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })
}

fn refusal(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, message)
}

impl WindowPreviewRun {
    /// 📨️ States the leaves every window's provisional config must fold; `stale` says a window's published config no longer matches its committed generation.
    pub fn request(&mut self, leaves: Vec<WindowConfigMutation>, stale: bool) {
        if leaves.is_empty() {
            self.wanted = (self.published || self.phase != Phase::Idle).then(Vec::new);
        } else if stale {
            self.wanted = Some(leaves);
        }
    }

    /// 🔎️ Whether a run is open or a request waits for one.
    pub fn pending(&self) -> bool {
        self.phase != Phase::Idle || self.wanted.is_some()
    }

    /// 🔎️ Whether any provisional config is published, running or requested.
    pub fn holds(&self) -> bool {
        self.published || self.pending()
    }

    /// 🛑️ Drops the waiting request and cancels the open run through its owners' bounded closure.
    pub fn abandon(&mut self) {
        self.wanted = None;
        if matches!(self.phase, Phase::Feeding | Phase::Finishing) {
            self.queue.clear();
            self.phase = Phase::Cancelling;
            self.count = None;
            self.pass = 0;
        }
    }

    /// 📏️ Quotes the next action of the run.
    pub fn demand(&self, registry: &WindowConfigOwnerRegistry, body: usize) -> Result<RetirementDemand, ValueError> {
        self.next(registry, body).map(|(_, demand)| demand)
    }

    /// ⏯️ Performs the next action of the run under `grant`.
    pub fn step(&mut self, registry: &mut WindowConfigOwnerRegistry, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let (action, demand) = self.next(registry, grant.maximum_copy_bytes)?;
        if !fits(grant, demand) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let outcome = self.perform(action, demand, registry, grant);
        if outcome.is_err() && matches!(self.phase, Phase::Feeding | Phase::Finishing) {
            self.abandon();
        }
        outcome
    }

    fn next(&self, registry: &WindowConfigOwnerRegistry, body: usize) -> Result<(Action, RetirementDemand), ValueError> {
        let here = header::<Self>();
        Ok(match self.phase {
            Phase::Idle => if self.wanted.is_some() { (Action::Begin, here) } else { (Action::Settle, Default::default()) },
            Phase::Feeding => match self.feed.as_ref() {
                None => if self.queue.is_empty() { (Action::StartFinish, here) } else { (Action::TakeLeaf, here) },
                Some(feed) => self.next_feed(feed, registry, body, false)?,
            },
            Phase::Finishing => {
                let count = self.count.unwrap_or(0);
                if registry.preview_partition_count() != count { (Action::Restart, here) }
                else if self.pass >= count { (Action::Settle, here) }
                else { (Action::Partition(self.pass, false), registry.preview_partition_demands(self.pass, self.epoch, false, body)?) }
            }
            Phase::Cancelling => match (self.feed.as_ref(), self.count) {
                (Some(feed), _) => self.next_feed(feed, registry, body, true)?,
                (None, None) => (Action::CountPass, here),
                (None, Some(count)) if self.pass >= count => (Action::Settle, here),
                (None, Some(_)) => (Action::Partition(self.pass, true), registry.preview_partition_demands(self.pass, self.epoch, true, body)?),
            },
        })
    }

    fn next_feed(&self, feed: &LeafFeed, registry: &WindowConfigOwnerRegistry, body: usize, cancelling: bool) -> Result<(Action, RetirementDemand), ValueError> {
        let here = header::<Self>();
        if cancelling || feed.admitted {
            if let Some(projection) = feed.projection.as_ref() {
                return Ok((Action::ProjectionClose, if projection.terminal_is_empty() { header::<Option<RetainedOwnedProjection<WindowConfigMutation>>>() } else { projection.close_demand(body)? }));
            }
            let Some(source) = feed.source.as_ref() else { return Ok((Action::LeafDone, here)) };
            if !cancelling && !registry.preview_leaf_ready(&feed.kind, &feed.window, self.epoch) {
                let index = registry.preview_partition_index(&feed.kind, &feed.window).ok_or_else(|| refusal("window preview partition vanished before its press operation applied"))?;
                return Ok((Action::Wait(index), registry.preview_partition_demands(index, self.epoch, false, body)?));
            }
            if !feed.source_closing { return Ok((Action::SourceBeginClose, here)); }
            return Ok((Action::SourceClose, if source.terminal_is_empty() { header::<Option<ArtifactMutationSource<WindowConfigMutation>>>() } else { source.close_demand(body)? }));
        }
        if feed.source.is_none() {
            return Ok((Action::SourceAdmit, ArtifactMutationSource::<WindowConfigMutation>::constructor_demand()));
        }
        if feed.projection.is_none() {
            let source = feed.source.as_ref().ok_or_else(|| refusal("window preview press source absent"))?;
            return Ok((if source.ready() { Action::SourceTake } else { Action::SourceAdvance }, source.demands(body)?));
        }
        match registry.preview_input_demands(&feed.kind, &feed.window, self.epoch, body) {
            Ok(demand) => Ok((Action::Input, demand)),
            Err(error) if error.kind == ValueRefusalKind::WorkLimit && registry.preview_retirement_pending() => Ok((Action::Drain, registry.preview_retirement_demand(body)?)),
            Err(error) => Err(error),
        }
    }

    fn perform(&mut self, action: Action, demand: RetirementDemand, registry: &mut WindowConfigOwnerRegistry, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let epoch = self.epoch;
        Ok(match action {
            Action::Begin => {
                let leaves = self.wanted.take().unwrap_or_default();
                self.epoch = self.epoch.wrapping_add(1);
                self.targeted = !leaves.is_empty();
                self.latest = leaves.clone();
                self.queue = leaves.into();
                self.phase = Phase::Feeding;
                bookkeeping(demand)
            }
            Action::TakeLeaf => {
                let leaf = self.queue.pop_front().ok_or_else(|| refusal("window preview queue is empty"))?;
                self.feed = Some(LeafFeed { kind: leaf.window_kind_id().to_string(), window: leaf.window_id().to_string(), original: Some(leaf), source: None, projection: None, admitted: false, source_closing: false });
                bookkeeping(demand)
            }
            Action::SourceAdmit => {
                let feed = self.feed.as_mut().ok_or_else(|| refusal("window preview feed absent"))?;
                match ArtifactMutationSource::admit(&mut feed.original, &self.factory, grant)? {
                    Some((source, progress)) => {
                        feed.source = Some(source);
                        RetainedCloneStep::Progress(progress)
                    }
                    None => RetainedCloneStep::Progress(Default::default()),
                }
            }
            Action::SourceAdvance => {
                let source = self.feed.as_mut().and_then(|feed| feed.source.as_mut()).ok_or_else(|| refusal("window preview press source absent"))?;
                RetainedCloneStep::Progress(source.advance(grant)?.progress())
            }
            Action::SourceTake => {
                let feed = self.feed.as_mut().ok_or_else(|| refusal("window preview feed absent"))?;
                let source = feed.source.as_mut().ok_or_else(|| refusal("window preview press source absent"))?;
                RetainedCloneStep::Progress(source.take(&mut feed.projection, grant)?.unwrap_or_default())
            }
            Action::Input => {
                let feed = self.feed.as_mut().ok_or_else(|| refusal("window preview feed absent"))?;
                let projection = feed.projection.as_ref().ok_or_else(|| refusal("window preview press projection absent"))?;
                let step = match registry.preview_input(projection, epoch, grant) {
                    Ok(progress) => RetainedCloneStep::Progress(progress.unwrap_or_default()),
                    Err(error) if error.kind == ValueRefusalKind::WorkLimit => RetainedCloneStep::Progress(Default::default()),
                    Err(error) => return Err(error),
                };
                feed.admitted = registry.preview_input_held(&feed.kind, &feed.window, epoch);
                step
            }
            Action::Drain => RetainedCloneStep::Progress(registry.preview_retirement_step(grant)?.progress()),
            Action::ProjectionClose => {
                let feed = self.feed.as_mut().ok_or_else(|| refusal("window preview feed absent"))?;
                let projection = feed.projection.as_mut().ok_or_else(|| refusal("window preview press projection absent"))?;
                if projection.terminal_is_empty() {
                    feed.projection = None;
                    bookkeeping(demand)
                } else {
                    projection.close_step(grant)?
                }
            }
            Action::Wait(index) => {
                let (_, progress) = registry.preview_partition_step(index, epoch, false, grant)?;
                RetainedCloneStep::Progress(progress)
            }
            Action::SourceBeginClose => {
                let feed = self.feed.as_mut().ok_or_else(|| refusal("window preview feed absent"))?;
                feed.source.as_mut().ok_or_else(|| refusal("window preview press source absent"))?.begin_close();
                feed.source_closing = true;
                bookkeeping(demand)
            }
            Action::SourceClose => {
                let feed = self.feed.as_mut().ok_or_else(|| refusal("window preview feed absent"))?;
                let source = feed.source.as_mut().ok_or_else(|| refusal("window preview press source absent"))?;
                if source.terminal_is_empty() {
                    feed.source = None;
                    bookkeeping(demand)
                } else {
                    source.close_step(grant)?
                }
            }
            Action::LeafDone => {
                self.feed = None;
                bookkeeping(demand)
            }
            Action::StartFinish => {
                self.phase = Phase::Finishing;
                self.pass = 0;
                self.count = Some(registry.preview_partition_count());
                bookkeeping(demand)
            }
            Action::CountPass => {
                self.count = Some(registry.preview_partition_count());
                self.pass = 0;
                bookkeeping(demand)
            }
            Action::Partition(index, closing) => {
                let (done, progress) = registry.preview_partition_step(index, epoch, closing, grant)?;
                if done {
                    self.pass += 1;
                    if progress == RetainedCloneProgress::default() {
                        return Ok(bookkeeping(demand));
                    }
                }
                RetainedCloneStep::Progress(progress)
            }
            Action::Restart => {
                self.wanted = Some(std::mem::take(&mut self.latest));
                self.phase = Phase::Cancelling;
                self.count = None;
                self.pass = 0;
                bookkeeping(demand)
            }
            Action::Settle => {
                if self.phase == Phase::Finishing {
                    self.published = self.targeted;
                }
                self.phase = Phase::Idle;
                self.count = None;
                self.pass = 0;
                self.latest.clear();
                bookkeeping(demand)
            }
        })
    }
}
