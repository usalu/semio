//! 🫧️ Scrub tool: the scrub statechart, its host and the scrub ledger that folds a scrubbed value into one transaction.

use super::*;

//#region 🔖️Scrub
/// 🎚️ The argument naming the press a continuous control's dispatch belongs to (`"<control>:<ms>"`); a dispatch
/// without it is a plain one-shot edit.
pub const SCRUB_GESTURE_ARG: &str = "gesture";
/// 🏁️ The argument marking the release (`true`) of a press.
pub const SCRUB_COMMIT_ARG: &str = "commit";
/// 🧯️ The argument carrying a host cancel's [`ToolAbortReason`] (`blur`, `captureLost`, `frozen`, `baseMoved`,
/// `retired`); the dispatch carries no value.
pub const SCRUB_ABORT_ARG: &str = "abort";

/// 🎚️ Where one dispatch of a continuous control (slider, held spinner, number field) sits in its press.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScrubPhase {
    Tick { gesture: String },
    Commit { gesture: String },
    Abort { gesture: String, reason: ToolAbortReason },
}

impl ScrubPhase {
    /// 🧩️ Reads the scrub arguments: `None` without a non-empty `gesture` (a one-shot dispatch) or with an unknown
    /// abort reason; `abort` wins over `commit`.
    pub fn parse(gesture: Option<&str>, commit: Option<bool>, abort: Option<&str>) -> Option<Self> {
        let gesture = gesture.filter(|gesture| !gesture.is_empty())?.to_string();
        match abort {
            Some(reason) => ToolAbortReason::parse(reason).map(|reason| Self::Abort { gesture, reason }),
            None if commit == Some(true) => Some(Self::Commit { gesture }),
            None => Some(Self::Tick { gesture }),
        }
    }

    /// 🆔️ The press this dispatch belongs to.
    pub fn gesture(&self) -> &str {
        match self {
            Self::Tick { gesture } | Self::Commit { gesture } | Self::Abort { gesture, .. } => gesture,
        }
    }

    /// 🧮️ The scrub input of this phase once the plugin's leaf constructor produced the ABSOLUTE `leaves` of its value
    /// (`set-x{target, value}`); an abort carries none.
    pub fn input<M>(self, leaves: Vec<M>) -> ScrubInput<M> {
        match self {
            Self::Tick { gesture } => ScrubInput::Tick { gesture, leaves },
            Self::Commit { gesture } => ScrubInput::Commit { gesture, leaves },
            Self::Abort { reason, .. } => ScrubInput::Abort { reason },
        }
    }
}

/// 📨️ What reaches a scrub: a live value's absolute leaves, the release's leaves, or a host cancel.
#[derive(Clone, Debug, PartialEq)]
pub enum ScrubInput<M> {
    Tick { gesture: String, leaves: Vec<M> },
    Commit { gesture: String, leaves: Vec<M> },
    Abort { reason: ToolAbortReason },
}

/// 🧰️ A scrub's tool state: the press it follows and how many keyed leaves (`"0"`, `"1"`, …) its transaction holds.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScrubContext {
    pub gesture: Option<String>,
    pub keys: usize,
}

/// 📨️ The scrub statechart's events; the host cancel is the runner's [`ToolMachineRunner::abort`], never an event.
#[derive(Clone, Debug, PartialEq)]
pub enum ScrubEvent<M> {
    Tick { gesture: String, leaves: Vec<M> },
    Commit { gesture: String, leaves: Vec<M> },
}

impl<M: Clone> machine::StatechartEvent for ScrubEvent<M> {
    const EVENT_COUNT: u16 = 2;

    fn event_id(&self) -> machine::EventId {
        match self {
            Self::Tick { .. } => machine::EventId(0),
            Self::Commit { .. } => machine::EventId(1),
        }
    }

    fn event_name(id: machine::EventId) -> &'static str {
        match id.0 {
            0 => "Tick",
            1 => "Commit",
            _ => "?",
        }
    }
}

/// 🎚️ The ONE continuous-control tool: `idle → scrubbing` on a `Tick` (the press opens), `scrubbing → scrubbing` on a
/// `Tick` of the same press, `Commit` of the same press back to `idle` (a `Commit` from `idle` is a one-shot press).
/// Every tick replaces the transaction's entries with the tick's absolute leaves (upsert by position, retract the
/// rest), so the transaction always holds the net value; the release commits ONE edit, a host abort leaves zero
/// trace. The plugin supplies only its leaf constructor. Tables are M-independent and pinned against the
/// `statechart!` compilation of the same chart by the unit laws.
pub struct ScrubMachine<M>(std::marker::PhantomData<fn() -> M>);

const SCRUB_NODES: [machine::NodeDef; 3] = [
    machine::NodeDef { stable_id: "root", kind: machine::NodeKind::Compound, parent: None, initial: Some(machine::NodeId(1)), children: &[machine::NodeId(1), machine::NodeId(2)], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 0 },
    machine::NodeDef { stable_id: "idle", kind: machine::NodeKind::Atomic, parent: Some(machine::NodeId(0)), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 1 },
    machine::NodeDef { stable_id: "scrubbing", kind: machine::NodeKind::Atomic, parent: Some(machine::NodeId(0)), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 2 },
];

const SCRUB_TRANSITIONS: [machine::TransitionDef; 4] = [
    machine::TransitionDef { source: machine::NodeId(1), trigger: machine::Trigger::Event(machine::EventId(0)), guard: None, targets: &[machine::NodeId(2)], kind: machine::TransitionKind::External, actions: &[machine::ActionId(0)], doc_index: 0 },
    machine::TransitionDef { source: machine::NodeId(1), trigger: machine::Trigger::Event(machine::EventId(1)), guard: None, targets: &[machine::NodeId(1)], kind: machine::TransitionKind::External, actions: &[machine::ActionId(1)], doc_index: 1 },
    machine::TransitionDef { source: machine::NodeId(2), trigger: machine::Trigger::Event(machine::EventId(0)), guard: Some(machine::GuardId(0)), targets: &[machine::NodeId(2)], kind: machine::TransitionKind::External, actions: &[machine::ActionId(0)], doc_index: 2 },
    machine::TransitionDef { source: machine::NodeId(2), trigger: machine::Trigger::Event(machine::EventId(1)), guard: Some(machine::GuardId(0)), targets: &[machine::NodeId(1)], kind: machine::TransitionKind::External, actions: &[machine::ActionId(1)], doc_index: 3 },
];

/// 🔏️ The `statechart!` fingerprint of the scrub chart (restore gate of a persisted scrub).
pub const SCRUB_FINGERPRINT: u64 = 16240238296638685209;
/// 🗺️ The `statechart!` manifest of the scrub chart.
pub const SCRUB_MANIFEST_JSON: &str = r#"{"id":"scrub","states":[{"id":"root","parent":null},{"id":"idle","parent":0},{"id":"scrubbing","parent":0}],"events":["Tick","Commit"],"transitionCount":4}"#;

impl<M: Clone + 'static> ScrubMachine<M> {
    pub(crate) const DEFINITION: MachineDefinition<Self> = MachineDefinition {
        id: "scrub",
        nodes: &SCRUB_NODES,
        transitions: &SCRUB_TRANSITIONS,
        context_from_input: scrub_context,
        make_output: None,
        guards: &[scrub_same_gesture::<M>],
        actions: &[scrub_follow::<M>, scrub_settle::<M>],
        fingerprint: SCRUB_FINGERPRINT,
        manifest_json: SCRUB_MANIFEST_JSON,
    };
}

impl<M: Clone + 'static> Machine for ScrubMachine<M> {
    type Context = ScrubContext;
    type Event = ScrubEvent<M>;
    type Input = ScrubContext;
    type Output = ();
    type Effect = ToolYield<M>;
    type Config = machine::BitSet<1>;

    fn definition() -> &'static MachineDefinition<Self> {
        &Self::DEFINITION
    }
}

fn scrub_context(input: ScrubContext) -> ScrubContext {
    input
}

fn scrub_same_gesture<M>(context: &ScrubContext, event: Option<&ScrubEvent<M>>) -> bool {
    matches!(event, Some(ScrubEvent::Tick { gesture, .. } | ScrubEvent::Commit { gesture, .. }) if context.gesture.as_deref() == Some(gesture.as_str()))
}

fn scrub_follow<M: Clone + 'static>(context: &mut ScrubContext, event: Option<&ScrubEvent<M>>, sink: &mut Vec<Command<ScrubMachine<M>>>) {
    let Some(ScrubEvent::Tick { gesture, leaves }) = event else { return };
    context.gesture = Some(gesture.clone());
    scrub_replace(context, leaves, sink);
}

fn scrub_settle<M: Clone + 'static>(context: &mut ScrubContext, event: Option<&ScrubEvent<M>>, sink: &mut Vec<Command<ScrubMachine<M>>>) {
    let Some(ScrubEvent::Commit { leaves, .. }) = event else { return };
    scrub_replace(context, leaves, sink);
    sink.push(Command::Effect(ToolYield::Commit));
    *context = ScrubContext::default();
}

fn scrub_replace<M: Clone + 'static>(context: &mut ScrubContext, leaves: &[M], sink: &mut Vec<Command<ScrubMachine<M>>>) {
    sink.extend(leaves.iter().enumerate().map(|(index, leaf)| Command::Effect(ToolYield::upsert(index.to_string(), leaf.clone()))));
    sink.extend((leaves.len()..context.keys).map(|index| Command::Effect(ToolYield::retract(index.to_string()))));
    context.keys = leaves.len();
}

/// 🧷️ The scrub's host: the chart declares no timer, no invoke and no foreign effect, so every duty is empty.
pub struct ScrubHost;

impl<M: Clone + 'static> Host<ScrubMachine<M>> for ScrubHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<M>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        0
    }
}

/// 💾️ One window's open scrub between dispatches (window transient, ephemeral local-only, never history): the
/// configuration by stable ids, the authoring tool `<appId>#<verb>` and actor, the press, the document revision it
/// opened on, and the open transaction with its keyed leaves.
#[derive(Clone, Debug, PartialEq)]
pub struct ScrubState<M> {
    pub states: Vec<String>,
    pub tool: String,
    pub actor: ActorId,
    pub gesture: String,
    pub base_revision: String,
    pub transaction: TransactionRef,
    pub entries: Vec<(String, M)>,
}

/// 🎚️ One press of a continuous control: [`ScrubMachine`] under a [`ToolMachineRunner`], opened on one document
/// revision. A tick or release of ANOTHER press first host-aborts the open one (`captureLost`).
pub struct Scrub<M: Clone + 'static> {
    pub(crate) runner: ToolMachineRunner<ScrubMachine<M>, ScrubHost>,
    pub(crate) base_revision: String,
}

impl<M: Clone + 'static> Scrub<M> {
    /// 🚀️ A scrub at rest for `tool` (`<appId>#<verb>`) by `actor` on the document revision `base_revision`.
    pub fn start(tool: impl Into<String>, actor: ActorId, base_revision: impl Into<String>) -> Self {
        let runner = ToolMachineRunner::start(tool, actor, ScrubContext::default(), ScrubHost).expect("the scrub chart yields nothing while entering");
        Self { runner, base_revision: base_revision.into() }
    }

    /// ⏯️ The scrub a window persisted, restored by stable ids with its open transaction; a state the chart cannot
    /// restore is refused (`Closed`), so the caller drops it with zero trace.
    pub fn resume(state: ScrubState<M>) -> Result<Self, ToolRefusal> {
        let persisted = machine::PersistedSnapshot { version: 1, fingerprint: SCRUB_FINGERPRINT, states: state.states, history: Vec::new(), done: false };
        let context = ScrubContext { gesture: Some(state.gesture), keys: state.entries.len() };
        let snapshot = machine::restore::<ScrubMachine<M>, machine::NoMigrations>(&persisted, context, &[]).map_err(|_| ToolRefusal::Closed)?;
        let transaction = ToolTransaction::resume(state.transaction, state.entries);
        let runner = ToolMachineRunner::resume(state.tool, state.actor, ScrubContext::default(), snapshot, Some(transaction), ScrubHost)?;
        Ok(Self { runner, base_revision: state.base_revision })
    }

    /// 🆔️ The press the scrub follows, while one is open.
    pub fn gesture(&self) -> Option<&str> {
        self.runner.snapshot().context.gesture.as_deref()
    }

    /// 📐️ The document revision the scrub opened on.
    pub fn base_revision(&self) -> &str {
        &self.base_revision
    }

    /// 🔧️ The authoring tool id.
    pub fn tool(&self) -> &str {
        self.runner.tool()
    }

    /// 📝️ The open transaction: the provisional leaves the preview overlays.
    pub fn transaction(&self) -> Option<&ToolTransaction<M>> {
        self.runner.transaction()
    }

    /// 📨️ Runs one input on `clock`: a host abort drops the open transaction with zero trace; a tick or release of
    /// another press first host-aborts the open one (`captureLost`).
    pub fn send(&mut self, input: ScrubInput<M>, clock: HybridLogicalTimestamp) -> Result<ToolStep<M>, ToolRefusal> {
        let event = match input {
            ScrubInput::Abort { reason } => return Ok(self.runner.abort(reason)),
            ScrubInput::Tick { gesture, leaves } => ScrubEvent::Tick { gesture, leaves },
            ScrubInput::Commit { gesture, leaves } => ScrubEvent::Commit { gesture, leaves },
        };
        let (ScrubEvent::Tick { gesture, .. } | ScrubEvent::Commit { gesture, .. }) = &event;
        if self.gesture().is_some_and(|open| open != gesture) {
            self.runner.abort(ToolAbortReason::CaptureLost);
        }
        self.runner.send(event, clock)
    }

    /// 💾️ The state to persist: `Some` only while a transaction is open.
    pub fn persist(self) -> Option<ScrubState<M>> {
        let (tool, actor) = (self.runner.tool().to_string(), self.runner.actor().clone());
        let (snapshot, transaction) = self.runner.into_parts();
        let transaction = transaction.filter(|transaction| transaction.state() == ToolTransactionState::Open)?;
        let gesture = snapshot.context.gesture.clone()?;
        Some(ScrubState { states: machine::persist(&snapshot).states, tool, actor, gesture, base_revision: self.base_revision, transaction: transaction.reference().clone(), entries: transaction.entries().to_vec() })
    }
}

/// 🗂️ Every window's open scrub (at most one per window) plus the press each window last closed, so a late tick of
/// a settled or cancelled press leaves zero trace. Pure: the runtime keeps one per app instance and overlays
/// [`Self::provisional`] on the committed document for every render.
#[derive(Clone, Debug, PartialEq)]
pub struct ScrubLedger<M> {
    pub(crate) windows: std::collections::BTreeMap<String, ScrubState<M>>,
    #[cfg(feature = "testkit")] #[doc(hidden)] pub closed: std::collections::BTreeMap<String, String>,
    #[cfg(not(feature = "testkit"))] pub(crate) closed: std::collections::BTreeMap<String, String>,
}

impl<M> Default for ScrubLedger<M> {
    fn default() -> Self {
        Self { windows: std::collections::BTreeMap::new(), closed: std::collections::BTreeMap::new() }
    }
}

impl<M: Clone + 'static> ScrubLedger<M> {
    /// 🛋️ Whether no window holds an open scrub.
    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }

    /// 🔎️ The open scrub of `window`.
    pub fn open(&self, window: &str) -> Option<&ScrubState<M>> {
        self.windows.get(window)
    }

    /// 🪟️ The windows holding an open scrub, in window id order.
    pub fn windows(&self) -> impl Iterator<Item = &str> {
        self.windows.keys().map(String::as_str)
    }

    /// 👁️ Every open scrub's provisional leaves, window by window in window id order — the overlay a render applies
    /// on the committed document; never history.
    pub fn provisional(&self) -> impl Iterator<Item = &M> {
        self.windows.values().flat_map(|state| state.entries.iter().map(|(_, leaf)| leaf))
    }

    /// 📨️ Runs one input of `window`'s press. A tick or release of the press the window last closed is a silent no-op;
    /// an open scrub of another tool or opened on another document revision is host-aborted first (`captureLost`,
    /// `baseMoved`, zero trace), so the input opens a fresh transaction on the current revision — the leaves are
    /// absolute. The release closes the press. A refused input leaves the ledger exactly as it was: the press stays open,
    /// so a retry or a host abort decides.
    pub fn send(&mut self, window: &str, tool: &str, actor: &ActorId, base_revision: &str, input: ScrubInput<M>, clock: HybridLogicalTimestamp) -> Result<ToolStep<M>, ToolRefusal> {
        let (gesture, release) = match &input {
            ScrubInput::Abort { reason } => return Ok(self.abort(window, None, *reason)),
            ScrubInput::Tick { gesture, .. } => (gesture.clone(), false),
            ScrubInput::Commit { gesture, .. } => (gesture.clone(), true),
        };
        if self.closed.get(window) == Some(&gesture) {
            return Ok(ToolStep::Idle);
        }
        let previous = self.windows.remove(window);
        let mut scrub = match previous.clone().filter(|state| state.tool == tool && state.base_revision == base_revision).map(Scrub::resume) {
            Some(Ok(scrub)) => scrub,
            _ => Scrub::start(tool, actor.clone(), base_revision),
        };
        let step = scrub.send(input, clock);
        if step.is_err() {
            if let Some(previous) = previous {
                self.windows.insert(window.to_string(), previous);
            }
            return step;
        }
        if release {
            self.closed.insert(window.to_string(), gesture);
        }
        if let Some(state) = scrub.persist() {
            self.windows.insert(window.to_string(), state);
        }
        step
    }

    /// 🧯️ Host cancel of `window`'s open scrub (only of the press `gesture` when named): zero trace, and the press is
    /// closed so its late ticks stay silent. `Aborted(ref, reason)`, or `Idle` when no such scrub was open.
    pub fn abort(&mut self, window: &str, gesture: Option<&str>, reason: ToolAbortReason) -> ToolStep<M> {
        if let Some(gesture) = gesture {
            self.closed.insert(window.to_string(), gesture.to_string());
        }
        match self.windows.remove(window) {
            Some(state) if gesture.is_none_or(|gesture| gesture == state.gesture) => {
                self.closed.insert(window.to_string(), state.gesture);
                ToolStep::Aborted(state.transaction, reason)
            }
            Some(state) => {
                self.windows.insert(window.to_string(), state);
                ToolStep::Idle
            }
            None => ToolStep::Idle,
        }
    }

    /// 🧊️ Host cancel of every open scrub (a time-travel freeze): zero trace. Answers one `Aborted` step per scrub.
    pub fn abort_all(&mut self, reason: ToolAbortReason) -> Vec<ToolStep<M>> {
        let windows: Vec<String> = self.windows.keys().cloned().collect();
        windows.iter().map(|window| self.abort(window, None, reason)).collect()
    }

    /// 🪦️ Host cancel (`retired`) of the open scrub of every window `keep` refuses; their closed presses are forgotten.
    pub fn retain_windows(&mut self, keep: impl Fn(&str) -> bool) -> Vec<ToolStep<M>> {
        let retired: Vec<String> = self.windows.keys().filter(|window| !keep(window)).cloned().collect();
        let dropped = retired.iter().map(|window| self.abort(window, None, ToolAbortReason::Retired)).collect();
        self.closed.retain(|window, _| keep(window));
        dropped
    }
}
//#endregion 🔖️Scrub
