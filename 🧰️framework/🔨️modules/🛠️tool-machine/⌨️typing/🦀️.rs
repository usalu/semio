//! 🛠️ Typing tool: the typing statechart, its idle-timer host and the typing ledger that commits a buffer as one transaction.

use super::*;

//#region 🔖️Typing
/// ⌨️ The argument naming the text buffer a live typing delivery types into (its editor surface id); a dispatch without it is a
/// plain one-shot edit (an agent's or a palette's).
pub const TYPING_BUFFER_ARG: &str = "typing";
/// 📡️ The argument of a host's run commit signal ([`TypingCommit`]); such a dispatch carries no edit.
pub const TYPING_COMMIT_ARG: &str = "typingCommit";
/// 💤️ How long a typing run stays open after its last edit before it commits on its own.
pub const TYPING_IDLE_MS: u64 = 750;

/// 🏁️ Why a typing run ended as ONE edit: the author paused, the caret jumped away, the editor lost focus, Enter in a
/// single-line field, the page was hidden or left, an explicit apply, or another verb needs the typed text first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TypingCommit {
    Idle,
    SelectionJump,
    Blur,
    Enter,
    Hidden,
    Apply,
    OtherVerb,
}

impl TypingCommit {
    pub const ALL: [Self; 7] = [Self::Idle, Self::SelectionJump, Self::Blur, Self::Enter, Self::Hidden, Self::Apply, Self::OtherVerb];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::SelectionJump => "selectionJump",
            Self::Blur => "blur",
            Self::Enter => "enter",
            Self::Hidden => "hidden",
            Self::Apply => "apply",
            Self::OtherVerb => "otherVerb",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|reason| reason.as_str() == text)
    }
}

/// ▶️ Where one dispatch sits in its buffer's typing run: a typed edit, or the host's commit signal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypingPhase {
    Edit { buffer: String },
    Commit { buffer: String, reason: TypingCommit },
}

impl TypingPhase {
    /// 🧩️ Reads the typing arguments: `None` without a non-empty `typing` buffer (a one-shot dispatch) or with an unknown commit
    /// reason.
    pub fn parse(buffer: Option<&str>, commit: Option<&str>) -> Option<Self> {
        let buffer = buffer.filter(|buffer| !buffer.is_empty())?.to_string();
        match commit {
            Some(reason) => TypingCommit::parse(reason).map(|reason| Self::Commit { buffer, reason }),
            None => Some(Self::Edit { buffer }),
        }
    }

    /// 🆔️ The buffer the dispatch types into.
    pub fn buffer(&self) -> &str {
        match self {
            Self::Edit { buffer } | Self::Commit { buffer, .. } => buffer,
        }
    }
}

/// 🔗️ How the leaves of one typed edit join the open run (an app's typing algebra): the run's new net leaves, or a split — the
/// open run commits as it is and the edit opens the next run.
#[derive(Clone, Debug, PartialEq)]
pub enum TypingFold<M> {
    Net(Vec<M>),
    Split,
}

/// 📥️ What reaches a typing run: one typed edit's leaves, a commit, or a host abort (`baseMoved` conflict, `frozen`).
#[derive(Clone, Debug, PartialEq)]
pub enum TypingInput<M> {
    Edit { buffer: String, leaves: Vec<M> },
    Commit { reason: TypingCommit },
    Abort { reason: ToolAbortReason },
}

/// 🧰️ A typing run's tool state: the buffer it types into and how many keyed net leaves (`"0"`, `"1"`, …) its transaction holds.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TypingContext {
    pub buffer: Option<String>,
    pub keys: usize,
}

/// ⏸️ The typing statechart's events (the idle lapse is its `after` timer, a host abort the runner's
/// [`ToolMachineRunner::abort`]).
#[derive(Clone, Debug, PartialEq)]
pub enum TypingEvent<M> {
    Edit { buffer: String, leaves: Vec<M> },
    Commit { reason: TypingCommit },
}

impl<M: Clone> machine::StatechartEvent for TypingEvent<M> {
    const EVENT_COUNT: u16 = 2;

    fn event_id(&self) -> machine::EventId {
        match self {
            Self::Edit { .. } => machine::EventId(0),
            Self::Commit { .. } => machine::EventId(1),
        }
    }

    fn event_name(id: machine::EventId) -> &'static str {
        match id.0 {
            0 => "Edit",
            1 => "Commit",
            _ => "?",
        }
    }
}

/// 🖥️ The ONE typing tool: `idle → typing` on an edit, every edit of the same buffer replaces the run's net leaves (upsert by
/// position, retract the rest) and re-arms the idle timer, which commits the run [`TYPING_IDLE_MS`] after its last edit; a
/// commit signal ends the run as ONE edit. Typing never aborts on its own: only a host abort (a conflicting base, a frozen
/// document) drops a run with zero trace. Tables are M-independent and pinned against the `statechart!` compilation of the
/// same chart by the unit laws.
pub struct TypingMachine<M>(std::marker::PhantomData<fn() -> M>);

const TYPING_NODES: [machine::NodeDef; 3] = [
    machine::NodeDef { stable_id: "root", kind: machine::NodeKind::Compound, parent: None, initial: Some(machine::NodeId(1)), children: &[machine::NodeId(1), machine::NodeId(2)], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 0 },
    machine::NodeDef { stable_id: "idle", kind: machine::NodeKind::Atomic, parent: Some(machine::NodeId(0)), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 1 },
    machine::NodeDef { stable_id: "typing", kind: machine::NodeKind::Atomic, parent: Some(machine::NodeId(0)), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[(TYPING_IDLE_TIMER, TYPING_IDLE_MS)], doc_index: 2 },
];

const TYPING_TRANSITIONS: [machine::TransitionDef; 4] = [
    machine::TransitionDef { source: machine::NodeId(1), trigger: machine::Trigger::Event(machine::EventId(0)), guard: None, targets: &[machine::NodeId(2)], kind: machine::TransitionKind::External, actions: &[machine::ActionId(0)], doc_index: 0 },
    machine::TransitionDef { source: machine::NodeId(2), trigger: machine::Trigger::Timer(TYPING_IDLE_TIMER), guard: None, targets: &[machine::NodeId(1)], kind: machine::TransitionKind::External, actions: &[machine::ActionId(1)], doc_index: 1 },
    machine::TransitionDef { source: machine::NodeId(2), trigger: machine::Trigger::Event(machine::EventId(0)), guard: Some(machine::GuardId(0)), targets: &[machine::NodeId(2)], kind: machine::TransitionKind::External, actions: &[machine::ActionId(0)], doc_index: 2 },
    machine::TransitionDef { source: machine::NodeId(2), trigger: machine::Trigger::Event(machine::EventId(1)), guard: None, targets: &[machine::NodeId(1)], kind: machine::TransitionKind::External, actions: &[machine::ActionId(1)], doc_index: 3 },
];

/// ⏲️ The typing chart's one `after` timer: the idle lapse of the `typing` state.
pub const TYPING_IDLE_TIMER: TimerId = TimerId(0);
/// 🔏️ The `statechart!` fingerprint of the typing chart (restore gate of a persisted run).
pub const TYPING_FINGERPRINT: u64 = 18324688487390181293;
/// 🗺️ The `statechart!` manifest of the typing chart.
pub const TYPING_MANIFEST_JSON: &str = r#"{"id":"typing","states":[{"id":"root","parent":null},{"id":"idle","parent":0},{"id":"typing","parent":0}],"events":["Edit","Commit"],"transitionCount":4}"#;

impl<M: Clone + 'static> TypingMachine<M> {
    pub(crate) const DEFINITION: MachineDefinition<Self> = MachineDefinition {
        id: "typing",
        nodes: &TYPING_NODES,
        transitions: &TYPING_TRANSITIONS,
        context_from_input: typing_context,
        make_output: None,
        guards: &[typing_same_buffer::<M>],
        actions: &[typing_follow::<M>, typing_settle::<M>],
        fingerprint: TYPING_FINGERPRINT,
        manifest_json: TYPING_MANIFEST_JSON,
    };
}

impl<M: Clone + 'static> Machine for TypingMachine<M> {
    type Context = TypingContext;
    type Event = TypingEvent<M>;
    type Input = TypingContext;
    type Output = ();
    type Effect = ToolYield<M>;
    type Config = machine::BitSet<1>;

    fn definition() -> &'static MachineDefinition<Self> {
        &Self::DEFINITION
    }
}

fn typing_context(input: TypingContext) -> TypingContext {
    input
}

fn typing_same_buffer<M>(context: &TypingContext, event: Option<&TypingEvent<M>>) -> bool {
    matches!(event, Some(TypingEvent::Edit { buffer, .. }) if context.buffer.as_deref() == Some(buffer.as_str()))
}

fn typing_follow<M: Clone + 'static>(context: &mut TypingContext, event: Option<&TypingEvent<M>>, sink: &mut Vec<Command<TypingMachine<M>>>) {
    let Some(TypingEvent::Edit { buffer, leaves }) = event else { return };
    context.buffer = Some(buffer.clone());
    sink.extend(leaves.iter().enumerate().map(|(index, leaf)| Command::Effect(ToolYield::upsert(index.to_string(), leaf.clone()))));
    sink.extend((leaves.len()..context.keys).map(|index| Command::Effect(ToolYield::retract(index.to_string()))));
    context.keys = leaves.len();
}

fn typing_settle<M: Clone + 'static>(context: &mut TypingContext, _event: Option<&TypingEvent<M>>, sink: &mut Vec<Command<TypingMachine<M>>>) {
    sink.push(Command::Effect(ToolYield::Commit));
    *context = TypingContext::default();
}

/// ⏰️ The typing run's host: its clock is the clock of the input being run, and the one `after` timer is a deadline the owner
/// checks ([`Typing::lapse`]) — a pure host, no wall clock, no callback.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TypingHost {
    pub now_ms: u64,
    pub deadline_ms: Option<u64>,
}

impl<M: Clone + 'static> Host<TypingMachine<M>> for TypingHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<M>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: TimerId, delay_ms: u64) {
        self.deadline_ms = Some(self.now_ms.saturating_add(delay_ms));
    }
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: TimerId) {
        self.deadline_ms = None;
    }
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        self.now_ms
    }
}

/// 📬️ One window's open typing run between dispatches (window transient, ephemeral local-only, never history): the
/// configuration by stable ids, the authoring tool `<appId>#<verb>` and actor, the buffer, the idle deadline, and the open
/// transaction with its net leaves.
#[derive(Clone, Debug, PartialEq)]
pub struct TypingState<M> {
    pub states: Vec<String>,
    pub tool: String,
    pub actor: ActorId,
    pub buffer: String,
    pub deadline_ms: u64,
    pub transaction: TransactionRef,
    pub entries: Vec<(String, M)>,
}

/// ✍️ One typing run: [`TypingMachine`] under a [`ToolMachineRunner`]. Every input runs on its own clock, which must be unique
/// per author and tool (the run's id is minted from it); one input mints at most one transaction.
pub struct Typing<M: Clone + 'static> {
    pub(crate) runner: ToolMachineRunner<TypingMachine<M>, TypingHost>,
}

impl<M: Clone + 'static> Typing<M> {
    /// 🚀️ A run at rest for `tool` (`<appId>#<verb>`) by `actor`.
    pub fn start(tool: impl Into<String>, actor: ActorId) -> Self {
        let runner = ToolMachineRunner::start(tool, actor, TypingContext::default(), TypingHost::default()).expect("the typing chart yields nothing while entering");
        Self { runner }
    }

    /// ⏯️ The run a window persisted, restored by stable ids with its open transaction and idle deadline; a state the chart
    /// cannot restore is refused (`Closed`).
    pub fn resume(state: TypingState<M>) -> Result<Self, ToolRefusal> {
        let persisted = machine::PersistedSnapshot { version: 1, fingerprint: TYPING_FINGERPRINT, states: state.states, history: Vec::new(), done: false };
        let context = TypingContext { buffer: Some(state.buffer), keys: state.entries.len() };
        let snapshot = machine::restore::<TypingMachine<M>, machine::NoMigrations>(&persisted, context, &[]).map_err(|_| ToolRefusal::Closed)?;
        let transaction = ToolTransaction::resume(state.transaction, state.entries);
        let host = TypingHost { now_ms: 0, deadline_ms: Some(state.deadline_ms) };
        let runner = ToolMachineRunner::resume(state.tool, state.actor, TypingContext::default(), snapshot, Some(transaction), host)?;
        Ok(Self { runner })
    }

    /// 📂️ The buffer the open run types into.
    pub fn buffer(&self) -> Option<&str> {
        self.runner.snapshot().context.buffer.as_deref()
    }

    /// ⌛️ When the open run commits on its own.
    pub fn deadline_ms(&self) -> Option<u64> {
        self.runner.host.deadline_ms
    }

    /// 🔧️ The authoring tool id.
    pub fn tool(&self) -> &str {
        self.runner.tool()
    }

    /// 📝️ The open transaction: the run's net leaves the preview overlays.
    pub fn transaction(&self) -> Option<&ToolTransaction<M>> {
        self.runner.transaction()
    }

    /// ⏱️ Fires the idle lapse when `clock` reached the deadline: the run commits as ONE edit. `None` when nothing lapsed.
    pub fn lapse(&mut self, clock: HybridLogicalTimestamp) -> Result<Option<ToolStep<M>>, ToolRefusal> {
        self.runner.host.now_ms = clock.physical_ms;
        match self.runner.host.deadline_ms {
            Some(deadline) if deadline <= clock.physical_ms => self.runner.timer_elapsed(TYPING_IDLE_TIMER, clock).map(Some),
            _ => Ok(None),
        }
    }

    /// 📨️ Runs one input on `clock`. An edit first lets a lapsed run commit, then folds into the open run of its buffer through
    /// `fold`; an edit of another buffer, or one `fold` splits off, commits the open run first (`selectionJump`) and opens the
    /// next. Answers every step in order: an edit yields at most one commit before its own step.
    pub fn send(&mut self, input: TypingInput<M>, fold: impl Fn(&[M], &[M]) -> TypingFold<M>, clock: HybridLogicalTimestamp) -> Result<Vec<ToolStep<M>>, ToolRefusal> {
        self.runner.host.now_ms = clock.physical_ms;
        let (buffer, leaves) = match input {
            TypingInput::Abort { reason } => return Ok(vec![self.runner.abort(reason)]),
            TypingInput::Commit { reason } => return Ok(vec![self.runner.send(TypingEvent::Commit { reason }, clock)?]),
            TypingInput::Edit { buffer, leaves } => (buffer, leaves),
        };
        let mut steps: Vec<ToolStep<M>> = self.lapse(clock)?.into_iter().collect();
        let net = match (self.buffer(), self.transaction()) {
            (Some(open), _) if open != buffer => TypingFold::Split,
            (Some(_), Some(transaction)) => fold(&transaction.entries().iter().map(|(_, leaf)| leaf.clone()).collect::<Vec<_>>(), &leaves),
            _ => TypingFold::Net(leaves.clone()),
        };
        let leaves = match net {
            TypingFold::Net(net) => net,
            TypingFold::Split => {
                steps.push(self.runner.send(TypingEvent::Commit { reason: TypingCommit::SelectionJump }, clock)?);
                leaves
            }
        };
        steps.push(self.runner.send(TypingEvent::Edit { buffer, leaves }, clock)?);
        Ok(steps)
    }

    /// 💾️ The state to persist: `Some` only while a transaction is open.
    pub fn persist(self) -> Option<TypingState<M>> {
        let (tool, actor, deadline_ms) = (self.runner.tool().to_string(), self.runner.actor().clone(), self.runner.host.deadline_ms?);
        let (snapshot, transaction) = self.runner.into_parts();
        let transaction = transaction.filter(|transaction| transaction.state() == ToolTransactionState::Open)?;
        let buffer = snapshot.context.buffer.clone()?;
        Some(TypingState { states: machine::persist(&snapshot).states, tool, actor, buffer, deadline_ms, transaction: transaction.reference().clone(), entries: transaction.entries().to_vec() })
    }
}

/// 🗂️ Every window's open typing run (at most one per window). Pure: the runtime keeps one per app instance, overlays
/// [`Self::provisional`] on the committed document for every render, publishes every committed run as ONE edit stamped with
/// its `TransactionRef`, and fires lapsed runs ([`Self::lapse`]) whenever it runs.
#[derive(Clone, Debug, PartialEq)]
pub struct TypingLedger<M> {
    pub(crate) windows: std::collections::BTreeMap<String, TypingState<M>>,
}

impl<M> Default for TypingLedger<M> {
    fn default() -> Self {
        Self { windows: std::collections::BTreeMap::new() }
    }
}

impl<M: Clone + 'static> TypingLedger<M> {
    /// 🛋️ Whether no window types.
    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }

    /// 🔎️ The open run of `window`.
    pub fn open(&self, window: &str) -> Option<&TypingState<M>> {
        self.windows.get(window)
    }

    /// 🪟️ The windows holding an open run, in window id order.
    pub fn windows(&self) -> impl Iterator<Item = &str> {
        self.windows.keys().map(String::as_str)
    }

    /// 👁️ Every open run's net leaves, window by window in window id order — the overlay a render applies on the committed
    /// document; never history.
    pub fn provisional(&self) -> impl Iterator<Item = &M> {
        self.windows.values().flat_map(|state| state.entries.iter().map(|(_, leaf)| leaf))
    }

    /// 🔜️ The earliest idle deadline of any open run.
    pub fn next_deadline_ms(&self) -> Option<u64> {
        self.windows.values().map(|state| state.deadline_ms).min()
    }

    fn run(&mut self, window: &str, tool: &str, actor: &ActorId) -> (Typing<M>, Option<ToolStep<M>>) {
        match self.windows.remove(window).map(Typing::resume) {
            Some(Ok(typing)) if typing.tool() == tool => (typing, None),
            Some(Ok(mut other)) => {
                let step = other.runner.send(TypingEvent::Commit { reason: TypingCommit::SelectionJump }, HybridLogicalTimestamp { actor: 0, physical_ms: other.runner.host.now_ms, logical: 0 }).ok();
                (Typing::start(tool, actor.clone()), step)
            }
            _ => (Typing::start(tool, actor.clone()), None),
        }
    }

    fn keep(&mut self, window: &str, typing: Typing<M>) {
        if let Some(state) = typing.persist() {
            self.windows.insert(window.to_string(), state);
        }
    }

    /// 📮️ Runs one input of `window`'s run on `clock`: an open run of another tool in the window commits first (`selectionJump`).
    /// Answers every step in order.
    pub fn send(&mut self, window: &str, tool: &str, actor: &ActorId, input: TypingInput<M>, fold: impl Fn(&[M], &[M]) -> TypingFold<M>, clock: HybridLogicalTimestamp) -> Result<Vec<ToolStep<M>>, ToolRefusal> {
        let (mut typing, other) = self.run(window, tool, actor);
        let steps = typing.send(input, fold, clock);
        self.keep(window, typing);
        steps.map(|steps| other.into_iter().chain(steps).collect())
    }

    /// ✏️ Commits `window`'s open run as ONE edit (`reason`); `Idle` when the window does not type.
    pub fn commit(&mut self, window: &str, reason: TypingCommit, clock: HybridLogicalTimestamp) -> Result<ToolStep<M>, ToolRefusal> {
        let Some(state) = self.windows.remove(window) else { return Ok(ToolStep::Idle) };
        let mut typing = Typing::resume(state)?;
        typing.runner.host.now_ms = clock.physical_ms;
        let step = typing.runner.send(TypingEvent::Commit { reason }, clock);
        self.keep(window, typing);
        step
    }

    /// 🔓️ Commits every open run (another verb needs the typed text first, the page is left): one step per window.
    pub fn commit_all(&mut self, reason: TypingCommit, clock: HybridLogicalTimestamp) -> Vec<(String, Result<ToolStep<M>, ToolRefusal>)> {
        let windows: Vec<String> = self.windows.keys().cloned().collect();
        windows.into_iter().map(|window| (window.clone(), self.commit(&window, reason, clock))).collect()
    }

    /// 🕰️ Fires the idle lapse of every run whose deadline `clock` reached: each commits as ONE edit, one step per lapsed window.
    pub fn lapse(&mut self, clock: HybridLogicalTimestamp) -> Vec<(String, Result<ToolStep<M>, ToolRefusal>)> {
        let lapsed: Vec<String> = self.windows.iter().filter(|(_, state)| state.deadline_ms <= clock.physical_ms).map(|(window, _)| window.clone()).collect();
        lapsed
            .into_iter()
            .filter_map(|window| {
                let mut typing = match Typing::resume(self.windows.remove(&window)?) {
                    Ok(typing) => typing,
                    Err(refusal) => return Some((window, Err(refusal))),
                };
                let step = typing.lapse(clock).map(|step| step.unwrap_or(ToolStep::Idle));
                self.keep(&window, typing);
                Some((window, step))
            })
            .collect()
    }

    /// 🧯️ Host abort of `window`'s open run (a conflicting base): zero trace. `Aborted(ref, reason)`, or `Idle`.
    pub fn abort(&mut self, window: &str, reason: ToolAbortReason) -> ToolStep<M> {
        self.windows.remove(window).map_or(ToolStep::Idle, |state| ToolStep::Aborted(state.transaction, reason))
    }

    /// 🧊️ Host abort of every open run (a frozen document): zero trace, one step per window.
    pub fn abort_all(&mut self, reason: ToolAbortReason) -> Vec<(String, ToolStep<M>)> {
        let windows: Vec<String> = self.windows.keys().cloned().collect();
        windows.into_iter().map(|window| (window.clone(), self.abort(&window, reason))).collect()
    }

    /// 🪦️ A window that left the roster ends its run like a blur: the typed text commits as ONE edit.
    pub fn retain_windows(&mut self, keep: impl Fn(&str) -> bool, clock: HybridLogicalTimestamp) -> Vec<(String, Result<ToolStep<M>, ToolRefusal>)> {
        let retired: Vec<String> = self.windows.keys().filter(|window| !keep(window)).cloned().collect();
        retired.into_iter().map(|window| (window.clone(), self.commit(&window, TypingCommit::Blur, clock))).collect()
    }
}
//#endregion 🔖️Typing
