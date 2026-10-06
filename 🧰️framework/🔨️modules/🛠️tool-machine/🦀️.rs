//! 🛠️ Domain-neutral tool machines: a tool is a `🔄️machine` statechart whose effects are [`ToolYield`]s;
//! a [`ToolMachineRunner`] drives it through the machine kernel and folds the yields into at most one
//! open [`ToolTransaction`], which commits exactly one edit or aborts with zero trace. Tool state (the
//! statechart context and configuration) is never history-editable; only the committed mutations are.
//!
//! Pure and target-neutral: no async, no store, no clock of its own (every entry point that may run
//! machine actions takes the hybrid logical clock of that moment). The host can always cancel: `abort` and
//! `reset` drop the open transaction with zero trace and return the statechart to its initial configuration.
//! Schema of record: `🧬️schema/🔣️.json`.
//! Contract: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5.

use machine::{Command, Configuration, Host, Machine, MachineDefinition, NullInspector, Snapshot, TimerId};
use protocol::{ActorId, HybridLogicalTimestamp, TransactionRef};

//#region 🔖️Yield
/// 🎇️ What a tool machine proposes to its transaction: keyed upserts and retractions of parametric
/// mutations, then exactly one closing `Commit` or `Abort`.
#[derive(Clone, Debug, PartialEq)]
pub enum ToolYield<M> {
    Upsert { key: String, mutation: M },
    Retract { key: String },
    Commit,
    Abort,
}

impl<M> ToolYield<M> {
    /// ➕️ Proposes `mutation` under `key`, replacing an earlier proposal with the same key in place.
    pub fn upsert(key: impl Into<String>, mutation: M) -> Self {
        Self::Upsert { key: key.into(), mutation }
    }

    /// ➖️ Withdraws the proposal under `key`, if any.
    pub fn retract(key: impl Into<String>) -> Self {
        Self::Retract { key: key.into() }
    }

    /// 🏷️ Wire tag of the variant.
    pub fn kind(&self) -> ToolYieldKind {
        match self {
            Self::Upsert { .. } => ToolYieldKind::Upsert,
            Self::Retract { .. } => ToolYieldKind::Retract,
            Self::Commit => ToolYieldKind::Commit,
            Self::Abort => ToolYieldKind::Abort,
        }
    }
}

/// 🗺️ Transaction-law matrix column of a yield.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolYieldKind {
    Upsert,
    Retract,
    Commit,
    Abort,
}

impl ToolYieldKind {
    pub const ALL: [Self; 4] = [Self::Upsert, Self::Retract, Self::Commit, Self::Abort];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Upsert => "upsert",
            Self::Retract => "retract",
            Self::Commit => "commit",
            Self::Abort => "abort",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.as_str() == text)
    }
}
//#endregion 🔖️Yield

//#region 🔖️Transaction
/// 🚦️ Lifecycle of one tool transaction; only `Open` accepts yields.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolTransactionState {
    Open,
    Committed,
    Aborted,
}

impl ToolTransactionState {
    pub const ALL: [Self; 3] = [Self::Open, Self::Committed, Self::Aborted];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Committed => "committed",
            Self::Aborted => "aborted",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|state| state.as_str() == text)
    }
}

/// 🔒️ Why a yield or an event was refused; every refusal leaves no trace. `Closed`: a yield reached a
/// transaction that already committed or aborted (or arrived while entering the initial configuration).
/// `Unclosed`: the tool came to rest with its transaction still open, which would let the next gesture join it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolRefusal {
    Closed,
    Unclosed,
}

impl ToolRefusal {
    pub const ALL: [Self; 2] = [Self::Closed, Self::Unclosed];

    /// 🔖️ Fault code, e.g. `toolTransaction.closed`.
    pub fn code(self) -> &'static str {
        match self {
            Self::Closed => "toolTransaction.closed",
            Self::Unclosed => "toolTransaction.unclosed",
        }
    }

    pub fn parse(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|refusal| refusal.code() == code)
    }
}

impl std::fmt::Display for ToolRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for ToolRefusal {}

/// 🛑️ Who ended a transaction without an edit: the tool itself (`Abort` yield) or the host (focus lost, pointer
/// capture lost, the gesture's base moved, a time-travel freeze, the tool retired).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolAbortReason {
    Tool,
    Blur,
    CaptureLost,
    BaseMoved,
    Frozen,
    Retired,
}

impl ToolAbortReason {
    pub const ALL: [Self; 6] = [Self::Tool, Self::Blur, Self::CaptureLost, Self::BaseMoved, Self::Frozen, Self::Retired];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Tool => "tool",
            Self::Blur => "blur",
            Self::CaptureLost => "captureLost",
            Self::BaseMoved => "baseMoved",
            Self::Frozen => "frozen",
            Self::Retired => "retired",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|reason| reason.as_str() == text)
    }
}

/// 🧾️ One interactive tool gesture or run: keyed provisional mutations in first-insertion order that
/// commit as ONE edit (one undo step, one history row) or abort leaving nothing behind.
#[derive(Clone, Debug, PartialEq)]
pub struct ToolTransaction<M> {
    reference: TransactionRef,
    state: ToolTransactionState,
    entries: Vec<(String, M)>,
}

impl<M> ToolTransaction<M> {
    /// 🌱️ An empty open transaction.
    pub fn open(reference: TransactionRef) -> Self {
        Self { reference, state: ToolTransactionState::Open, entries: Vec::new() }
    }

    /// ↩️ An open transaction restored from persisted tool state: `entries` are upserted in order, so their
    /// first-insertion order is kept and a repeated key keeps its first slot with its last mutation.
    pub fn resume(reference: TransactionRef, entries: Vec<(String, M)>) -> Self {
        let mut transaction = Self::open(reference);
        for (key, mutation) in entries {
            transaction.upsert(key, mutation);
        }
        transaction
    }

    fn upsert(&mut self, key: String, mutation: M) {
        match self.entries.iter_mut().find(|(existing, _)| *existing == key) {
            Some(entry) => entry.1 = mutation,
            None => self.entries.push((key, mutation)),
        }
    }

    /// 🪪️ The ref every op of the committed edit is stamped with.
    pub fn reference(&self) -> &TransactionRef {
        &self.reference
    }

    pub fn state(&self) -> ToolTransactionState {
        self.state
    }

    /// 📋️ Provisional `(key, mutation)` entries in first-insertion order (the preview overlay).
    pub fn entries(&self) -> &[(String, M)] {
        &self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// ⚖️ The reducer: upsert replaces by key keeping the first-insertion slot, retract removes, commit
    /// closes keeping the entries, abort closes discarding them; every yield on a closed transaction is refused.
    pub fn apply(&mut self, yielded: ToolYield<M>) -> Result<(), ToolRefusal> {
        if self.state != ToolTransactionState::Open {
            return Err(ToolRefusal::Closed);
        }
        match yielded {
            ToolYield::Upsert { key, mutation } => self.upsert(key, mutation),
            ToolYield::Retract { key } => self.entries.retain(|(existing, _)| *existing != key),
            ToolYield::Commit => self.state = ToolTransactionState::Committed,
            ToolYield::Abort => {
                self.state = ToolTransactionState::Aborted;
                self.entries.clear();
            }
        }
        Ok(())
    }

    /// 📦️ The committed batch in entry order, consuming the transaction.
    pub fn into_parts(self) -> (TransactionRef, Vec<M>) {
        (self.reference, self.entries.into_iter().map(|(_, mutation)| mutation).collect())
    }
}
//#endregion 🔖️Transaction

//#region 🔖️Machine
/// 🎰️ A `🔄️machine` statechart whose effects are [`ToolYield`]s over its `Mutation`. Blanket-implemented:
/// every `statechart!` declaring `effect: ToolYield<M>;` is a tool machine over `M`.
pub trait ToolMachine: Machine<Effect = ToolYield<<Self as ToolMachine>::Mutation>> {
    type Mutation;
}

impl<T: Machine<Effect = ToolYield<M>>, M> ToolMachine for T {
    type Mutation = M;
}

/// 🧷️ The single kernel actor a runner drives.
pub const TOOL_MACHINE_ACTOR: machine::ActorId = machine::ActorId(0);

/// 🔁️ What one event did to the runner's transaction slot.
#[derive(Clone, Debug, PartialEq)]
pub enum ToolStep<M> {
    /// 💤️ No transaction is open and none closed.
    Idle,
    /// ✏️ A transaction is open; its provisional entries are [`ToolMachineRunner::transaction`].
    Open,
    /// 💾️ A non-empty transaction committed: publish exactly one edit stamped with the ref.
    Committed(TransactionRef, Vec<M>),
    /// 🗑️ The transaction aborted, by the tool or the host: nothing is published and nothing remains.
    Aborted(TransactionRef, ToolAbortReason),
    /// 🫙️ The transaction committed empty: no edit.
    Empty(TransactionRef),
}

impl<M> ToolStep<M> {
    /// 🔤️ Wire tag of the step.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Open => "open",
            Self::Committed(..) => "committed",
            Self::Aborted(..) => "aborted",
            Self::Empty(_) => "empty",
        }
    }
}

/// 🏃️ Drives one tool machine through the kernel and owns at most one open transaction. A transaction
/// opens at the first `Upsert` while none is open, its id minted from `(actor, clock, tool)` of that
/// event; `Retract`, `Commit` and `Abort` without an open transaction change nothing. Every event is
/// fail-closed: a yield after the close within the same event is refused (`Closed`), and so is an event that
/// leaves the tool at rest (its root's initial state active) with a transaction still open (`Unclosed`); a
/// refused event publishes nothing and drops the transaction. Non-yield commands (timers, invokes) go to the host.
pub struct ToolMachineRunner<T: ToolMachine, H: Host<T>> {
    pub host: H,
    tool: String,
    actor: ActorId,
    input: T::Input,
    snapshot: Snapshot<T>,
    transaction: Option<ToolTransaction<T::Mutation>>,
}

impl<T: ToolMachine, H: Host<T>> ToolMachineRunner<T, H>
where
    T::Input: Clone,
{
    /// 🚀️ Enters the initial configuration. A transaction only opens in response to an event, so a yield
    /// while entering is refused.
    pub fn start(tool: impl Into<String>, actor: ActorId, input: T::Input, mut host: H) -> Result<Self, ToolRefusal> {
        let mut commands = Vec::new();
        let mut snapshot = machine::init::<T>(input.clone(), &mut commands);
        let mut refused = false;
        for command in commands {
            match command {
                Command::Effect(_) => refused = true,
                other => {
                    machine::route_command(&mut host, &mut snapshot, TOOL_MACHINE_ACTOR, other);
                }
            }
        }
        if refused {
            return Err(ToolRefusal::Closed);
        }
        Ok(Self { host, tool: tool.into(), actor, input, snapshot, transaction: None })
    }

    /// ⏯️ Rebuilds a runner from persisted tool state ([`Self::into_parts`]) to continue its gesture: refuses a
    /// committed or aborted transaction (`Closed`) and a resting snapshot that holds an open transaction
    /// (`Unclosed`). Timers scheduled before the persist stay with the host that scheduled them.
    pub fn resume(tool: impl Into<String>, actor: ActorId, input: T::Input, snapshot: Snapshot<T>, transaction: Option<ToolTransaction<T::Mutation>>, host: H) -> Result<Self, ToolRefusal> {
        let runner = Self { host, tool: tool.into(), actor, input, snapshot, transaction };
        match &runner.transaction {
            Some(transaction) if transaction.state() != ToolTransactionState::Open => Err(ToolRefusal::Closed),
            Some(_) if runner.at_rest() => Err(ToolRefusal::Unclosed),
            _ => Ok(runner),
        }
    }

    /// 🧳️ The tool state to persist between dispatches (window transient): the statechart snapshot and the
    /// open transaction; [`Self::resume`] continues from them.
    pub fn into_parts(self) -> (Snapshot<T>, Option<ToolTransaction<T::Mutation>>) {
        (self.snapshot, self.transaction)
    }

    /// 🔧️ The authoring tool id `<appId>#<toolId>` stamped into every ref.
    pub fn tool(&self) -> &str {
        &self.tool
    }

    pub fn actor(&self) -> &ActorId {
        &self.actor
    }

    /// 📸️ The tool state: configuration and context (ephemeral, never history).
    pub fn snapshot(&self) -> &Snapshot<T> {
        &self.snapshot
    }

    /// 📝️ The open transaction, if any.
    pub fn transaction(&self) -> Option<&ToolTransaction<T::Mutation>> {
        self.transaction.as_ref()
    }

    /// 🛋️ Whether the tool rests: the root's initial state is active.
    pub fn at_rest(&self) -> bool {
        T::definition().nodes[machine::ROOT.0 as usize].initial.is_some_and(|initial| self.snapshot.configuration.contains(initial))
    }

    /// 🧯️ Host cancel (focus or capture lost, base moved, freeze, retirement): drops the open transaction with
    /// zero trace and returns the statechart to its initial configuration; `Aborted(ref, reason)`, or `Idle`
    /// when no transaction was open.
    pub fn abort(&mut self, reason: ToolAbortReason) -> ToolStep<T::Mutation> {
        self.rest().map_or(ToolStep::Idle, |reference| ToolStep::Aborted(reference, reason))
    }

    /// ♻️ Silent host reset to the freshly started runner (same input): zero trace, initial configuration;
    /// answers the ref of the transaction it dropped, if one was open.
    pub fn reset(&mut self) -> Option<TransactionRef> {
        self.rest()
    }

    fn rest(&mut self) -> Option<TransactionRef> {
        let definition = T::definition();
        for id in self.snapshot.configuration.iter_ones() {
            let node = &definition.nodes[id.0 as usize];
            for (timer, _) in node.timers {
                self.host.cancel_timer(TOOL_MACHINE_ACTOR, *timer);
            }
            for invoke in node.invokes {
                self.host.cancel_task(TOOL_MACHINE_ACTOR, *invoke);
            }
        }
        let mut commands = Vec::new();
        self.snapshot = machine::init::<T>(self.input.clone(), &mut commands);
        for command in commands {
            if !matches!(command, Command::Effect(_)) {
                machine::route_command(&mut self.host, &mut self.snapshot, TOOL_MACHINE_ACTOR, command);
            }
        }
        self.transaction.take().map(|transaction| transaction.reference)
    }

    /// 📨️ Runs `event` to completion and settles its yields.
    pub fn send(&mut self, event: T::Event, clock: HybridLogicalTimestamp) -> Result<ToolStep<T::Mutation>, ToolRefusal> {
        let mut commands = Vec::new();
        machine::macrostep(&mut self.snapshot, event, &mut commands, &mut NullInspector);
        self.settle(commands, clock)
    }

    /// ⏱️ Runs an elapsed `after` timer to completion and settles its yields.
    pub fn timer_elapsed(&mut self, timer: TimerId, clock: HybridLogicalTimestamp) -> Result<ToolStep<T::Mutation>, ToolRefusal> {
        let mut commands = Vec::new();
        machine::timer_elapsed(&mut self.snapshot, timer, &mut commands, &mut NullInspector);
        self.settle(commands, clock)
    }

    fn settle(&mut self, commands: Vec<Command<T>>, clock: HybridLogicalTimestamp) -> Result<ToolStep<T::Mutation>, ToolRefusal> {
        let mut refused = false;
        for command in commands {
            let Command::Effect(yielded) = command else {
                machine::route_command(&mut self.host, &mut self.snapshot, TOOL_MACHINE_ACTOR, command);
                continue;
            };
            if refused {
                continue;
            }
            match (&mut self.transaction, yielded) {
                (Some(transaction), yielded) => refused = transaction.apply(yielded).is_err(),
                (slot @ None, yielded @ ToolYield::Upsert { .. }) => {
                    let mut transaction = ToolTransaction::open(TransactionRef::mint(&self.actor, &clock, self.tool.as_str()));
                    refused = transaction.apply(yielded).is_err();
                    *slot = Some(transaction);
                }
                (None, _) => {}
            }
        }
        if refused {
            self.transaction = None;
            return Err(ToolRefusal::Closed);
        }
        if self.transaction.as_ref().is_some_and(|transaction| transaction.state() == ToolTransactionState::Open) && self.at_rest() {
            self.transaction = None;
            return Err(ToolRefusal::Unclosed);
        }
        Ok(match self.transaction.take() {
            None => ToolStep::Idle,
            Some(transaction) => match transaction.state() {
                ToolTransactionState::Open => {
                    self.transaction = Some(transaction);
                    ToolStep::Open
                }
                ToolTransactionState::Aborted => ToolStep::Aborted(transaction.reference, ToolAbortReason::Tool),
                ToolTransactionState::Committed if transaction.is_empty() => ToolStep::Empty(transaction.reference),
                ToolTransactionState::Committed => {
                    let (reference, mutations) = transaction.into_parts();
                    ToolStep::Committed(reference, mutations)
                }
            },
        })
    }
}
//#endregion 🔖️Machine

//#region 🌊️Gesture
/// 🌊️ Where one dispatch of a streamed window gesture (a gumball drag, a paint stroke) sits: a one-shot `Once`, a
/// `Stream` tick into the window's open transaction, the `Commit` that ends it, or a host `Abort` with its reason.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GesturePhase {
    Once,
    Stream,
    Commit,
    Abort(ToolAbortReason),
}

impl GesturePhase {
    /// 🔡️ Reads a gesture verb's `phase` (`stream` | `commit` | `abort`, absent = one-shot) and an abort's `reason`
    /// (`blur`, `captureLost`, `baseMoved`, `frozen`, `retired`; absent = `tool`); `None` for an unknown one.
    pub fn parse(phase: Option<&str>, reason: Option<&str>) -> Option<Self> {
        match phase {
            None => Some(Self::Once),
            Some("stream") => Some(Self::Stream),
            Some("commit") => Some(Self::Commit),
            Some("abort") => reason.map_or(Some(ToolAbortReason::Tool), ToolAbortReason::parse).map(Self::Abort),
            Some(_) => None,
        }
    }
}

/// 🖐️ One window's streamed tool for ONE dispatch — a [`ToolMachineRunner`] started at rest, or resumed from the gesture
/// its window persisted between dispatches (the runtime's [`GestureLedger`] slot, never history). A statechart tool is
/// [`ChartGesture`] over its [`GestureChart`].
pub trait GestureTool: Sized {
    type Gesture: PartialEq;
    type Tick;
    type Mutation;
    /// 📌️ Whether a gesture is pinned to the document revision it opened on, so a moved base ends it `baseMoved`; a tool
    /// whose leaves fold on any base says `false` and is driven with an empty base revision.
    const BASE_BOUND: bool = true;
    fn start(verb: &str, authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal>;
    fn resume(gesture: &Self::Gesture) -> Result<Self, ToolRefusal>;
    fn verb(&self) -> &str;
    fn base_revision(&self) -> &str;
    fn abort(&mut self, reason: ToolAbortReason);
    fn send(&mut self, phase: GesturePhase, tick: Option<Self::Tick>) -> Result<ToolStep<Self::Mutation>, ToolRefusal>;
    fn persist(self) -> Option<Self::Gesture>;
}

/// 📬️ What one gesture dispatch did: the transaction it committed (publish it as ONE edit), the window's next persisted
/// gesture when it changed (`Some(None)` clears it), and whether the dispatch `continued` the persisted gesture (resumed
/// it, neither dropped nor interrupted).
pub struct GestureDrive<G, M> {
    pub committed: Option<(TransactionRef, Vec<M>)>,
    pub next: Option<Option<G>>,
    pub continued: bool,
}

/// 🚂️ Drives one window's streamed tool through ONE dispatch (law `🧫️fixtures/🧫️gesture-drive-law`). `Once` commits
/// `tick` as one transaction; `Stream` upserts it into the window's open transaction (opening it on the first tick),
/// `Commit` folds it in and commits the whole gesture, `Abort` drops the open gesture with zero trace. A gesture whose base
/// moved under it is aborted `baseMoved` (a stream tick or commit that found it is dropped with it, a one-shot commits
/// fresh); otherwise another verb or a one-shot interrupts it (`captureLost`). A gesture its tool cannot restore is dropped
/// with zero trace and the dispatch runs from rest; a refused start or tick is the dispatch's refusal, with no effect at all.
pub fn drive_gesture<T: GestureTool>(persisted: Option<&T::Gesture>, verb: &str, phase: GesturePhase, tick: Option<T::Tick>, authoring_seed: &str, base_revision: &str) -> Result<GestureDrive<T::Gesture, T::Mutation>, ToolRefusal> {
    let dropped = || GestureDrive { committed: None, next: persisted.map(|_| None), continued: false };
    let (open, interrupted) = match (persisted.and_then(|gesture| T::resume(gesture).ok()), phase) {
        (Some(mut tool), GesturePhase::Abort(reason)) => {
            tool.abort(reason);
            return Ok(dropped());
        }
        (None, GesturePhase::Abort(_)) => return Ok(dropped()),
        (Some(mut tool), _) if tool.base_revision() != base_revision && phase != GesturePhase::Once => {
            tool.abort(ToolAbortReason::BaseMoved);
            return Ok(dropped());
        }
        (Some(tool), _) if tool.base_revision() != base_revision => (None, Some((tool, ToolAbortReason::BaseMoved))),
        (Some(tool), _) if tool.verb() != verb || phase == GesturePhase::Once => (None, Some((tool, ToolAbortReason::CaptureLost))),
        (open, _) => (open, None),
    };
    let continued = open.is_some();
    let mut tool = match open {
        Some(tool) => tool,
        None => T::start(verb, authoring_seed, base_revision)?,
    };
    let step = tool.send(phase, tick)?;
    if let Some((mut interrupted, reason)) = interrupted {
        interrupted.abort(reason);
    }
    let gesture = tool.persist();
    let next = (gesture.as_ref() != persisted).then_some(gesture);
    let committed = match step {
        ToolStep::Committed(reference, mutations) => Some((reference, mutations)),
        ToolStep::Idle | ToolStep::Open | ToolStep::Aborted(..) | ToolStep::Empty(_) => None,
    };
    Ok(GestureDrive { committed, next, continued })
}

/// 📡️ A host fact that may end a window's open gesture (law table `hostEvents` of `🧫️fixtures/🧫️gesture-drive-law`): the
/// window lost focus or its pointer capture, its utility switched, it is closing, a history edit froze the document, or a
/// remote edit moved the base.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GestureHostEvent {
    Blur,
    CaptureLost,
    UtilityChanged,
    Retiring,
    TimeTravelFrozen,
    BaseMoved,
}

impl GestureHostEvent {
    pub const ALL: [Self; 6] = [Self::Blur, Self::CaptureLost, Self::UtilityChanged, Self::Retiring, Self::TimeTravelFrozen, Self::BaseMoved];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blur => "blur",
            Self::CaptureLost => "captureLost",
            Self::UtilityChanged => "utilityChanged",
            Self::Retiring => "retiring",
            Self::TimeTravelFrozen => "timeTravelFrozen",
            Self::BaseMoved => "baseMoved",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|event| event.as_str() == text)
    }

    /// 🧨️ The reason this fact ends an open gesture with; `None` keeps the gesture: a moved base ends only a gesture pinned
    /// to a base revision (`base_bound`), every other fact ends any gesture.
    pub fn abort_reason(self, base_bound: bool) -> Option<ToolAbortReason> {
        match self {
            Self::Blur => Some(ToolAbortReason::Blur),
            Self::CaptureLost => Some(ToolAbortReason::CaptureLost),
            Self::UtilityChanged | Self::Retiring => Some(ToolAbortReason::Retired),
            Self::TimeTravelFrozen => Some(ToolAbortReason::Frozen),
            Self::BaseMoved => base_bound.then_some(ToolAbortReason::BaseMoved),
        }
    }
}

/// 🫧️ One window's open gesture between dispatches — the framework-owned persisted form of a streamed statechart tool
/// (`$defs/GestureState`; window slot of the plugin runtime: ephemeral, local-only, never history): the configuration by
/// stable ids, the verb, the host press that opened it (empty: none named; stamped by the slot — [`drive_press`] — never by
/// the tool), the admission's authoring seed and the document revision it opened on (empty: pinned to none), the open
/// transaction with its keyed provisional mutations, and the tool context its entries do not already say (`Null`: none).
#[derive(Clone, Debug, PartialEq)]
pub struct GestureState<M> {
    pub states: Vec<String>,
    pub verb: String,
    pub press: String,
    pub authoring_seed: String,
    pub base_revision: String,
    pub transaction: TransactionRef,
    pub entries: Vec<(String, M)>,
    pub context: protocol::DslValue,
}

/// 🧭️ A statechart tool on the shared gesture runner: the chart, its host, the event one dispatch sends and how a resumed
/// gesture gets its chart context back. Starting, resuming, persisting and aborting are [`ChartGesture`]'s, the persisted
/// form is [`GestureState`], the window slot is the runtime's [`GestureLedger`] — a plugin declares none of them.
pub trait GestureChart: ToolMachine
where
    Self::Input: Clone,
{
    type Tick;
    type Host: Host<Self>;

    /// 🧲️ Whether a gesture is pinned to the document revision it opened on, so a moved base ends it `baseMoved`; `false`
    /// for a tool whose leaves fold on any base.
    const BASE_BOUND: bool = true;

    /// 🪛️ The authoring tool id `<appId>#<verb>` every transaction of `verb` is stamped with.
    fn tool(verb: &str) -> String;

    /// 🏠️ The host the chart's timers, invokes and foreign effects go to.
    fn host() -> Self::Host;

    /// 🥚️ The input the chart starts from.
    fn input() -> Self::Input;

    /// 🧶️ The chart context of a resumed gesture, rebuilt from its open transaction's entries and the context it persisted;
    /// `None` refuses the gesture (`Closed`), so the drive drops it with zero trace.
    fn restore(entries: &[(String, Self::Mutation)], context: &protocol::DslValue) -> Option<Self::Context>;

    /// 🎒️ The tool context a gesture persists beyond its entries.
    fn context(_context: &Self::Context) -> protocol::DslValue {
        protocol::DslValue::Null
    }

    /// 📣️ The event one dispatch sends (`at_rest`: no gesture is in flight); `None` sends nothing.
    fn event(phase: GesturePhase, at_rest: bool, tick: Option<Self::Tick>) -> Option<Self::Event>;
}

static GESTURE_CLOCK_TICK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// 🎛️ The ONE [`GestureTool`] of every statechart tool: a [`ToolMachineRunner`] over a [`GestureChart`], started at rest
/// or resumed from its window's [`GestureState`], each event on the host clock with a process-monotone tick (two gestures
/// opened in one millisecond never mint one transaction id).
pub struct ChartGesture<T: GestureChart>
where
    T::Input: Clone,
{
    runner: ToolMachineRunner<T, T::Host>,
    verb: String,
    authoring_seed: String,
    base_revision: String,
}

impl<T: GestureChart> GestureTool for ChartGesture<T>
where
    T::Input: Clone,
    T::Mutation: Clone + PartialEq,
{
    type Gesture = GestureState<T::Mutation>;
    type Tick = T::Tick;
    type Mutation = T::Mutation;

    const BASE_BOUND: bool = T::BASE_BOUND;

    fn start(verb: &str, authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal> {
        let runner = ToolMachineRunner::start(T::tool(verb), ActorId(authoring_seed.to_string()), T::input(), T::host())?;
        Ok(Self { runner, verb: verb.to_string(), authoring_seed: authoring_seed.to_string(), base_revision: base_revision.to_string() })
    }

    fn resume(gesture: &GestureState<T::Mutation>) -> Result<Self, ToolRefusal> {
        let context = T::restore(&gesture.entries, &gesture.context).ok_or(ToolRefusal::Closed)?;
        let persisted = machine::PersistedSnapshot { version: 1, fingerprint: T::definition().fingerprint, states: gesture.states.clone(), history: Vec::new(), done: false };
        let snapshot = machine::restore::<T, machine::NoMigrations>(&persisted, context, &[]).map_err(|_| ToolRefusal::Closed)?;
        let transaction = ToolTransaction::resume(gesture.transaction.clone(), gesture.entries.clone());
        let runner = ToolMachineRunner::resume(T::tool(&gesture.verb), ActorId(gesture.authoring_seed.clone()), T::input(), snapshot, Some(transaction), T::host())?;
        Ok(Self { runner, verb: gesture.verb.clone(), authoring_seed: gesture.authoring_seed.clone(), base_revision: gesture.base_revision.clone() })
    }

    fn verb(&self) -> &str {
        &self.verb
    }

    fn base_revision(&self) -> &str {
        &self.base_revision
    }

    fn abort(&mut self, reason: ToolAbortReason) {
        self.runner.abort(reason);
    }

    fn send(&mut self, phase: GesturePhase, tick: Option<T::Tick>) -> Result<ToolStep<T::Mutation>, ToolRefusal> {
        match T::event(phase, self.runner.at_rest(), tick) {
            Some(event) => self.runner.send(event, authoring_clock(GESTURE_CLOCK_TICK.fetch_add(1, std::sync::atomic::Ordering::Relaxed))),
            None if self.runner.transaction().is_some() => Ok(ToolStep::Open),
            None => Ok(ToolStep::Idle),
        }
    }

    fn persist(self) -> Option<GestureState<T::Mutation>> {
        let context = T::context(&self.runner.snapshot().context);
        let (snapshot, transaction) = self.runner.into_parts();
        let transaction = transaction.filter(|transaction| transaction.state() == ToolTransactionState::Open)?;
        Some(GestureState {
            states: machine::persist(&snapshot).states,
            verb: self.verb,
            press: String::new(),
            authoring_seed: self.authoring_seed,
            base_revision: self.base_revision,
            transaction: transaction.reference().clone(),
            entries: transaction.entries().to_vec(),
            context,
        })
    }
}

/// 🎬️ Drives the statechart tool `T` through ONE dispatch from its window's persisted gesture ([`drive_gesture`] over
/// [`ChartGesture`]); a tool that is not [`GestureChart::BASE_BOUND`] pins its gesture to no revision.
pub fn drive_chart_gesture<T: GestureChart>(
    persisted: Option<&GestureState<T::Mutation>>,
    verb: &str,
    phase: GesturePhase,
    tick: Option<T::Tick>,
    authoring_seed: &str,
    base_revision: &str,
) -> Result<GestureDrive<GestureState<T::Mutation>, T::Mutation>, ToolRefusal>
where
    T::Input: Clone,
    T::Mutation: Clone + PartialEq,
{
    drive_gesture::<ChartGesture<T>>(persisted, verb, phase, tick, authoring_seed, if T::BASE_BOUND { base_revision } else { "" })
}

/// 🎫️ What one dispatch did to its window's slot: the transaction it committed (publish it as ONE edit), the slot's next
/// gesture when it changed (`Some(None)` clears it) and the press the window closed with it.
pub struct PressDrive<M> {
    pub committed: Option<(TransactionRef, Vec<M>)>,
    pub next: Option<Option<GestureState<M>>>,
    pub closed: Option<String>,
}

/// 🪪️ Drives one window's slot through ONE dispatch that may name its host `press` (law `slots`): the slot owns the press
/// identity, never the tool. A gesture belongs to the press that opened it. A dispatch of the press the window last
/// `closed` is dropped with zero trace (a late release after a blur commits nothing); a dispatch of another press
/// interrupts the open gesture first; a named gesture that ends — by anything — closes its press, and so does a named
/// one-shot, commit or abort. A refused dispatch changes nothing.
#[expect(clippy::too_many_arguments, reason = "One dispatch against one slot: the slot's two halves, the press, and drive_gesture's own inputs.")]
pub fn drive_press<T: GestureTool<Gesture = GestureState<M>, Mutation = M>, M: PartialEq>(
    held: Option<&GestureState<M>>,
    closed: Option<&str>,
    press: Option<&str>,
    verb: &str,
    phase: GesturePhase,
    tick: Option<T::Tick>,
    authoring_seed: &str,
    base_revision: &str,
) -> Result<PressDrive<M>, ToolRefusal> {
    let press = press.filter(|press| !press.is_empty());
    if press.is_some() && closed == press {
        return Ok(PressDrive { committed: None, next: None, closed: None });
    }
    let owner = held.map(|open| open.press.as_str()).filter(|owner| !owner.is_empty());
    let interrupts = matches!((press, owner), (Some(press), Some(owner)) if press != owner);
    let drive = drive_gesture::<T>(held.filter(|_| !interrupts), verb, phase, tick, authoring_seed, base_revision)?;
    let next = match drive.next {
        Some(Some(mut gesture)) => {
            gesture.press = if drive.continued { owner } else { press }.unwrap_or_default().to_string();
            (Some(&gesture) != held).then_some(Some(gesture))
        }
        Some(None) => Some(None),
        None => interrupts.then_some(None),
    };
    let after = match &next {
        Some(next) => next.as_ref(),
        None => held,
    };
    let ended = owner.filter(|owner| after.is_none_or(|after| after.press != *owner));
    let closed = press.filter(|_| phase != GesturePhase::Stream).or(ended).map(str::to_string);
    Ok(PressDrive { committed: drive.committed, next, closed })
}

/// 🗄️ Every window's open gesture, at most one per window, and the press each window last closed — the ONE framework-owned window slot of a persisted
/// [`GestureTool`] (design §22.10, law `slots`). Pure: the runtime keeps one per app instance, overlays
/// [`Self::provisional`] on the committed document for every render, and ends a window's gesture on its host facts
/// ([`Self::host_event`]) — no editor maps a host fact to an abort of its own. A persisted gesture holds no host timer.
#[derive(Clone, Debug, PartialEq)]
pub struct GestureLedger<M> {
    windows: std::collections::BTreeMap<String, GestureState<M>>,
    closed: std::collections::BTreeMap<String, String>,
}

impl<M> Default for GestureLedger<M> {
    fn default() -> Self {
        Self { windows: std::collections::BTreeMap::new(), closed: std::collections::BTreeMap::new() }
    }
}

impl<M> GestureLedger<M> {
    /// 🪹️ Whether no window holds an open gesture.
    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }

    /// 🔦️ The open gesture of `window`.
    pub fn open(&self, window: &str) -> Option<&GestureState<M>> {
        self.windows.get(window)
    }

    /// 🏘️ The windows holding an open gesture, in window id order.
    pub fn windows(&self) -> impl Iterator<Item = &str> {
        self.windows.keys().map(String::as_str)
    }

    /// 🪞️ Every open gesture's provisional mutations, window by window in window id order — the overlay a render applies
    /// on the committed document; never history.
    pub fn provisional(&self) -> impl Iterator<Item = &M> {
        self.windows.values().flat_map(|gesture| gesture.entries.iter().map(|(_, mutation)| mutation))
    }

    /// 🚪️ The press `window` last closed: its late dispatches leave zero trace.
    pub fn closed(&self, window: &str) -> Option<&str> {
        self.closed.get(window).map(String::as_str)
    }

    /// 📕️ Every window's last closed press, in window id order.
    pub fn closed_presses(&self) -> impl Iterator<Item = (&str, &str)> {
        self.closed.iter().map(|(window, press)| (window.as_str(), press.as_str()))
    }

    /// 🔏️ Records the press `window` closed.
    pub fn close(&mut self, window: &str, press: String) {
        self.closed.insert(window.to_string(), press);
    }

    /// 🖋️ Keeps the gesture a dispatch decided for `window` (`None` clears the slot).
    pub fn settle(&mut self, window: &str, next: Option<GestureState<M>>) {
        match next {
            Some(gesture) => {
                self.windows.insert(window.to_string(), gesture);
            }
            None => {
                self.windows.remove(window);
            }
        }
    }

    /// 🚃️ Drives `window`'s tool through ONE dispatch of `press` (`None`: the host named none) against its slot
    /// ([`drive_press`]): the slot and the closed press follow the drive and the committed transaction is answered; a
    /// refused dispatch leaves the ledger exactly as it was.
    #[expect(clippy::too_many_arguments, reason = "One dispatch of one window: the window, the press, and drive_gesture's own inputs.")]
    pub fn drive<T: GestureTool<Gesture = GestureState<M>, Mutation = M>>(
        &mut self,
        window: &str,
        press: Option<&str>,
        verb: &str,
        phase: GesturePhase,
        tick: Option<T::Tick>,
        authoring_seed: &str,
        base_revision: &str,
    ) -> Result<Option<(TransactionRef, Vec<M>)>, ToolRefusal>
    where
        M: PartialEq,
    {
        let drive = drive_press::<T, M>(self.windows.get(window), self.closed.get(window).map(String::as_str), press, verb, phase, tick, authoring_seed, base_revision)?;
        if let Some(next) = drive.next {
            self.settle(window, next);
        }
        if let Some(press) = drive.closed {
            self.close(window, press);
        }
        Ok(drive.committed)
    }

    /// 🧹️ Host cancel of `window`'s open gesture: zero trace, and its press is closed. `Aborted(ref, reason)`, or `Idle`
    /// when none was open.
    pub fn abort(&mut self, window: &str, reason: ToolAbortReason) -> ToolStep<M> {
        let Some(gesture) = self.windows.remove(window) else { return ToolStep::Idle };
        if !gesture.press.is_empty() {
            self.closed.insert(window.to_string(), gesture.press);
        }
        ToolStep::Aborted(gesture.transaction, reason)
    }

    /// 🛎️ A host fact of `window`: its open gesture ends with the fact's reason ([`GestureHostEvent::abort_reason`]; a
    /// gesture pinned to no revision survives a moved base), `Idle` when it stays or none was open.
    pub fn host_event(&mut self, window: &str, event: GestureHostEvent) -> ToolStep<M> {
        match self.windows.get(window).and_then(|gesture| event.abort_reason(!gesture.base_revision.is_empty())) {
            Some(reason) => self.abort(window, reason),
            None => ToolStep::Idle,
        }
    }

    /// 🌐️ A host fact of every window (a history edit freezing the document): one `(window, Aborted)` per gesture it ended,
    /// in window id order.
    pub fn host_event_all(&mut self, event: GestureHostEvent) -> Vec<(String, ToolStep<M>)> {
        let windows: Vec<String> = self.windows.keys().cloned().collect();
        windows.into_iter().map(|window| (self.host_event(&window, event), window)).filter(|(step, _)| !matches!(step, ToolStep::Idle)).map(|(step, window)| (window, step)).collect()
    }

    /// ⚰️ Host cancel (`retired`) of the open gesture of every window `keep` refuses; their closed presses are forgotten.
    pub fn retain_windows(&mut self, keep: impl Fn(&str) -> bool) -> Vec<(String, ToolStep<M>)> {
        let retired: Vec<String> = self.windows.keys().filter(|window| !keep(window)).cloned().collect();
        let ended = retired.into_iter().map(|window| (self.abort(&window, ToolAbortReason::Retired), window)).map(|(step, window)| (window, step)).collect();
        self.closed.retain(|window, _| keep(window));
        ended
    }
}
//#endregion 🌊️Gesture

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
    const DEFINITION: MachineDefinition<Self> = MachineDefinition {
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
    pub actor: String,
    pub gesture: String,
    pub base_revision: String,
    pub transaction: TransactionRef,
    pub entries: Vec<(String, M)>,
}

/// 🎚️ One press of a continuous control: [`ScrubMachine`] under a [`ToolMachineRunner`], opened on one document
/// revision. A tick or release of ANOTHER press first host-aborts the open one (`captureLost`).
pub struct Scrub<M: Clone + 'static> {
    runner: ToolMachineRunner<ScrubMachine<M>, ScrubHost>,
    base_revision: String,
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
        let runner = ToolMachineRunner::resume(state.tool, ActorId(state.actor), ScrubContext::default(), snapshot, Some(transaction), ScrubHost)?;
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
        let (tool, actor) = (self.runner.tool().to_string(), self.runner.actor().0.clone());
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
    windows: std::collections::BTreeMap<String, ScrubState<M>>,
    closed: std::collections::BTreeMap<String, String>,
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

//#region 🔖️NodeDrag
/// ✋️ The `nodeGraphEdit` row operation of a released node drag (design §13.3).
pub const NODE_DRAG_OPERATION: &str = "move";
/// 🧾️ The closed field set of a [`NODE_DRAG_OPERATION`] row.
pub const NODE_DRAG_ROW_FIELDS: [&str; 5] = ["operation", "gestureId", "nodeIds", "dx", "dy"];

/// ✋️ The node-graph gesture record every node-graph host dispatches for a released node drag (design §13.3): the press
/// it closes, the nodes it moved, and the ONE offset every one of them moved by, relative to where it started. A guest
/// turns it into its own relative leaf (`drag-nodes`, `move-nodes`, …) and commits it through [`node_drag_commit`], so
/// editing the drag in history replays the offset on whatever base it lands on.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeDragRecord {
    pub gesture_id: String,
    pub node_ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

impl NodeDragRecord {
    /// 🧾️ Decodes one `{operation:"move", gestureId, nodeIds, dx, dy}` row; any other field, a missing one, an empty
    /// gesture or node id, no node or one node twice, or a non-finite offset is refused by name.
    pub fn from_row(row: &protocol::DslValue) -> Result<Self, String> {
        let fields = row.as_object().ok_or("a node drag row must be an object")?;
        if fields.len() != NODE_DRAG_ROW_FIELDS.len() || NODE_DRAG_ROW_FIELDS.iter().any(|field| fields.iter().filter(|(name, _)| name == field).count() != 1) {
            return Err(format!("a node drag row has exactly the fields {NODE_DRAG_ROW_FIELDS:?}"));
        }
        if row.get("operation").and_then(protocol::DslValue::as_str) != Some(NODE_DRAG_OPERATION) {
            return Err(format!("a node drag row is the `{NODE_DRAG_OPERATION}` operation"));
        }
        let gesture_id = row.get("gestureId").and_then(protocol::DslValue::as_str).filter(|id| !id.is_empty()).ok_or("a node drag row names its gestureId")?.to_string();
        let node_ids = row
            .get("nodeIds")
            .and_then(protocol::DslValue::as_array)
            .ok_or("a node drag row lists its nodeIds")?
            .iter()
            .map(|id| id.as_str().filter(|id| !id.is_empty()).map(str::to_string).ok_or("every nodeIds entry is a non-empty node id"))
            .collect::<Result<Vec<_>, _>>()?;
        if node_ids.is_empty() || node_ids.iter().enumerate().any(|(at, id)| node_ids[..at].contains(id)) {
            return Err("a node drag row names at least one node and each node once".into());
        }
        let offset = |field: &str| row.get(field).and_then(protocol::DslValue::as_f64).filter(|value| value.is_finite()).ok_or(format!("a node drag row's {field} is a finite number"));
        Ok(Self { gesture_id, node_ids, dx: offset("dx")?, dy: offset("dy")? })
    }

    /// 📤️ The row a host writes for this record.
    pub fn to_row(&self) -> protocol::DslValue {
        protocol::DslValue::object([
            ("operation".to_string(), protocol::DslValue::String(NODE_DRAG_OPERATION.to_string())),
            ("gestureId".to_string(), protocol::DslValue::String(self.gesture_id.clone())),
            ("nodeIds".to_string(), protocol::DslValue::Array(self.node_ids.iter().cloned().map(protocol::DslValue::String).collect())),
            ("dx".to_string(), protocol::DslValue::float(self.dx)),
            ("dy".to_string(), protocol::DslValue::float(self.dy)),
        ])
    }

    /// 🎚️ Whether the record moves anything: at least one node and a finite offset that is not zero.
    pub fn moves(&self) -> bool {
        !self.node_ids.is_empty() && self.dx.is_finite() && self.dy.is_finite() && (self.dx, self.dy) != (0.0, 0.0)
    }
}

/// 🛠️ The ONE node-drag machine: a released drag of the press `gesture` committed as ONE tool transaction of the guest's
/// net `leaves`, through the continuous-control [`Scrub`] (a release from rest is a one-shot press; a streaming host only
/// adds `Tick`s of the same press, each carrying the cumulative offset). The ref is minted from `actor` (the admission's
/// authoring seed), `clock` and `tool` (`<appId>#<verb>`). `None` when nothing is yielded: an empty drag leaves zero trace.
pub fn node_drag_commit<M: Clone + 'static>(tool: impl Into<String>, actor: ActorId, gesture: &str, leaves: Vec<M>, clock: HybridLogicalTimestamp) -> Option<(TransactionRef, Vec<M>)> {
    if leaves.is_empty() {
        return None;
    }
    match Scrub::start(tool, actor, "").send(ScrubInput::Commit { gesture: gesture.to_string(), leaves }, clock).ok()? {
        ToolStep::Committed(transaction, mutations) => Some((transaction, mutations)),
        ToolStep::Idle | ToolStep::Open | ToolStep::Aborted(..) | ToolStep::Empty(_) => None,
    }
}

/// ⏱️ The clock a guest mints its tool transaction refs and tool ticks from: the host's wall-clock millisecond at `logical`
/// (actor 0 — the admission's authoring seed, not the clock, names the writer).
pub fn authoring_clock(logical: u64) -> HybridLogicalTimestamp {
    HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical }
}

/// 🛠️ What a released node drag publishes ([`node_drag_emit`]): ONE tool transaction of its leaves, the leaves as a plain
/// edit (a view without command authority: no authoring seed), or nothing (no leaves: zero trace).
#[derive(Clone, Debug, PartialEq)]
pub enum NodeDragEmit<M> {
    Commit(TransactionRef, Vec<M>),
    Plain(Vec<M>),
    Nothing,
}

impl<M> NodeDragEmit<M> {
    /// 🔓️ The committed tool transaction and its leaves; `None` for a plain or an empty release.
    pub fn committed(self) -> Option<(TransactionRef, Vec<M>)> {
        match self {
            Self::Commit(transaction, leaves) => Some((transaction, leaves)),
            Self::Plain(_) | Self::Nothing => None,
        }
    }
}

/// 🛠️ The ONE node-drag emission of every guest: `leaves` of the press `gesture` through [`node_drag_commit`] as the tool
/// `<app_id>#<verb>`, minted from the admission's `authoring_seed` and [`authoring_clock`].
pub fn node_drag_emit<M: Clone + 'static>(app_id: &str, verb: &str, authoring_seed: &str, gesture: &str, leaves: Vec<M>) -> NodeDragEmit<M> {
    match node_drag_commit(format!("{app_id}#{verb}"), ActorId(authoring_seed.to_string()), gesture, leaves, authoring_clock(0)) {
        Some((transaction, leaves)) if !authoring_seed.is_empty() => NodeDragEmit::Commit(transaction, leaves),
        Some((_, leaves)) => NodeDragEmit::Plain(leaves),
        None => NodeDragEmit::Nothing,
    }
}
//#endregion 🔖️NodeDrag

//#region 🎯️Once
/// 🎯️ The ONE one-step tool (design §22.32): a single dispatch — a click that places, a keyboard nudge — yields `leaves` and
/// commits them as ONE tool transaction of `<app_id>#<verb>`, its ref minted from the admission's `authoring_seed` and
/// [`authoring_clock`]: the continuous-control machine driven as a press that opens and releases in one event
/// ([`node_drag_commit`]). Nothing yielded leaves zero trace; without an admission (a render or test view) the leaves publish
/// plainly.
pub fn tool_once_emit<M: Clone + 'static>(app_id: &str, verb: &str, authoring_seed: &str, leaves: Vec<M>) -> NodeDragEmit<M> {
    node_drag_emit(app_id, verb, authoring_seed, verb, leaves)
}
//#endregion 🎯️Once

//#region 🔖️NodeGraphEditRows
/// 📏️ The most rows one `nodeGraphEdit` dispatch carries.
pub const NODE_GRAPH_EDIT_MAX_ROWS: usize = 256;
/// 🧾️ The root fields a `nodeGraphEdit` dispatch may carry: its rows, and the press of a dragged inline slider that the
/// framework scrub machine reads (design §13.1).
pub const NODE_GRAPH_EDIT_ROOT_FIELDS: [&str; 4] = ["operations", SCRUB_GESTURE_ARG, SCRUB_COMMIT_ARG, SCRUB_ABORT_ARG];

/// 🔌️ The side of a node a variadic port is inserted on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodePortSide {
    Input,
    Output,
}

/// 🔗️ One node-graph gesture record every renderer dispatches as `nodeGraphEdit` arguments (design §13.3; schema
/// `🧬️schema/🔣️node-graph-edit-rows`): each row names its entities by id and carries an intent, never a
/// whole fixture. A guest maps every row to its own id-keyed leaf; adding a node stays each guest's own verb.
#[derive(Clone, Debug, PartialEq)]
pub enum NodeGraphEditRow {
    Connect { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
    Disconnect { synapse_id: String },
    Move(NodeDragRecord),
    SetSlider { widget_id: String, value: f64 },
    InsertPort { node_id: String, side: NodePortSide, index: u32 },
    Delete { node_ids: Vec<String>, synapse_ids: Vec<String> },
}

impl NodeGraphEditRow {
    /// 🧾️ Decodes one closed row; an unknown operation, a field outside the row's schema, an empty id, a repeated id, a
    /// non-numeric or non-finite number, an unknown port side or a non-integer index is refused by name.
    pub fn from_row(row: &protocol::DslValue) -> Result<Self, String> {
        let operation = row.get("operation").and_then(protocol::DslValue::as_str).ok_or("a nodeGraphEdit row names its operation")?;
        let closed = |expected: &[&str]| {
            let fields = row.as_object().ok_or(format!("a nodeGraphEdit {operation} row must be an object"))?;
            match fields.len() == expected.len() && expected.iter().all(|field| fields.iter().filter(|(name, _)| name == field).count() == 1) {
                true => Ok(()),
                false => Err(format!("a nodeGraphEdit {operation} row has exactly the fields {expected:?}")),
            }
        };
        let text = |field: &str| row.get(field).and_then(protocol::DslValue::as_str).map(str::to_string).ok_or(format!("nodeGraphEdit {operation}.{field} must be a string"));
        let id = |field: &str| text(field).and_then(|value| if value.is_empty() { Err(format!("nodeGraphEdit {operation}.{field} must not be empty")) } else { Ok(value) });
        let ids = |field: &str| {
            let ids = row
                .get(field)
                .and_then(protocol::DslValue::as_array)
                .ok_or(format!("nodeGraphEdit {operation}.{field} must be an array"))?
                .iter()
                .map(|id| id.as_str().filter(|id| !id.is_empty()).map(str::to_string).ok_or(format!("nodeGraphEdit {operation}.{field} holds non-empty ids")))
                .collect::<Result<Vec<_>, _>>()?;
            match ids.iter().enumerate().any(|(at, id)| ids[..at].contains(id)) {
                true => Err(format!("nodeGraphEdit {operation}.{field} names each id once")),
                false => Ok(ids),
            }
        };
        match operation {
            "connect" => {
                closed(&["operation", "sourceNodeId", "sourcePortId", "targetNodeId", "targetPortId"])?;
                Ok(Self::Connect { source_node_id: id("sourceNodeId")?, source_port_id: text("sourcePortId")?, target_node_id: id("targetNodeId")?, target_port_id: text("targetPortId")? })
            }
            "disconnect" => {
                closed(&["operation", "synapseId"])?;
                Ok(Self::Disconnect { synapse_id: id("synapseId")? })
            }
            NODE_DRAG_OPERATION => NodeDragRecord::from_row(row).map(Self::Move),
            "setSlider" => {
                closed(&["operation", "widgetId", "value"])?;
                let value = row.get("value").and_then(protocol::DslValue::as_f64).filter(|value| value.is_finite()).ok_or("nodeGraphEdit setSlider.value must be a finite number")?;
                Ok(Self::SetSlider { widget_id: id("widgetId")?, value })
            }
            "insertPort" => {
                closed(&["operation", "nodeId", "side", "index"])?;
                let side = match text("side")?.as_str() {
                    "input" => NodePortSide::Input,
                    "output" => NodePortSide::Output,
                    other => return Err(format!("nodeGraphEdit insertPort.side is input or output, not {other:?}")),
                };
                let index = row.get("index").and_then(protocol::DslValue::as_f64).filter(|index| index.fract() == 0.0 && (0.0..=f64::from(u32::MAX)).contains(index)).ok_or("nodeGraphEdit insertPort.index must be a non-negative integer")?;
                Ok(Self::InsertPort { node_id: id("nodeId")?, side, index: index as u32 })
            }
            "delete" => {
                closed(&["operation", "nodeIds", "synapseIds"])?;
                let (node_ids, synapse_ids) = (ids("nodeIds")?, ids("synapseIds")?);
                match node_ids.is_empty() && synapse_ids.is_empty() {
                    true => Err("nodeGraphEdit delete names at least one node or synapse".into()),
                    false => Ok(Self::Delete { node_ids, synapse_ids }),
                }
            }
            other => Err(format!("nodeGraphEdit has no operation {other:?}")),
        }
    }
}

/// 🧾️ Decodes the arguments of one `nodeGraphEdit` dispatch: an object holding exactly its `operations` (at most
/// [`NODE_GRAPH_EDIT_MAX_ROWS`] closed rows) and optionally the scrub press fields. Any malformed row refuses the whole batch
/// before a guest maps anything; an empty batch (a host abort of a slider press) decodes to no rows.
pub fn node_graph_edit_rows(args: &protocol::DslValue) -> Result<Vec<NodeGraphEditRow>, String> {
    let root = args.as_object().ok_or("nodeGraphEdit arguments must be an object")?;
    if root.iter().any(|(field, _)| !NODE_GRAPH_EDIT_ROOT_FIELDS.contains(&field.as_str())) || root.iter().filter(|(field, _)| field == "operations").count() != 1 {
        return Err(format!("nodeGraphEdit arguments hold exactly an operations array and optionally {:?}", &NODE_GRAPH_EDIT_ROOT_FIELDS[1..]));
    }
    let rows = args.get("operations").and_then(protocol::DslValue::as_array).ok_or("nodeGraphEdit operations must be an array")?;
    if rows.len() > NODE_GRAPH_EDIT_MAX_ROWS {
        return Err(format!("nodeGraphEdit carries at most {NODE_GRAPH_EDIT_MAX_ROWS} rows"));
    }
    rows.iter().map(NodeGraphEditRow::from_row).collect()
}
//#endregion 🔖️NodeGraphEditRows

//#region 🔖️Typing
/// ⌨️ The argument naming the text buffer a live typing delivery types into (its editor surface id); a dispatch without it is a
/// plain one-shot edit (an agent's or a palette's).
pub const TYPING_BUFFER_ARG: &str = "typing";
/// 🏁️ The argument of a host's run commit signal ([`TypingCommit`]); such a dispatch carries no edit.
pub const TYPING_COMMIT_ARG: &str = "typingCommit";
/// ⏱️ How long a typing run stays open after its last edit before it commits on its own.
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

/// ⌨️ Where one dispatch sits in its buffer's typing run: a typed edit, or the host's commit signal.
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

/// 📨️ What reaches a typing run: one typed edit's leaves, a commit, or a host abort (`baseMoved` conflict, `frozen`).
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

/// 📨️ The typing statechart's events (the idle lapse is its `after` timer, a host abort the runner's
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

/// ⌨️ The ONE typing tool: `idle → typing` on an edit, every edit of the same buffer replaces the run's net leaves (upsert by
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

/// ⏱️ The typing chart's one `after` timer: the idle lapse of the `typing` state.
pub const TYPING_IDLE_TIMER: TimerId = TimerId(0);
/// 🔏️ The `statechart!` fingerprint of the typing chart (restore gate of a persisted run).
pub const TYPING_FINGERPRINT: u64 = 18324688487390181293;
/// 🗺️ The `statechart!` manifest of the typing chart.
pub const TYPING_MANIFEST_JSON: &str = r#"{"id":"typing","states":[{"id":"root","parent":null},{"id":"idle","parent":0},{"id":"typing","parent":0}],"events":["Edit","Commit"],"transitionCount":4}"#;

impl<M: Clone + 'static> TypingMachine<M> {
    const DEFINITION: MachineDefinition<Self> = MachineDefinition {
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

/// 💾️ One window's open typing run between dispatches (window transient, ephemeral local-only, never history): the
/// configuration by stable ids, the authoring tool `<appId>#<verb>` and actor, the buffer, the idle deadline, and the open
/// transaction with its net leaves.
#[derive(Clone, Debug, PartialEq)]
pub struct TypingState<M> {
    pub states: Vec<String>,
    pub tool: String,
    pub actor: String,
    pub buffer: String,
    pub deadline_ms: u64,
    pub transaction: TransactionRef,
    pub entries: Vec<(String, M)>,
}

/// ⌨️ One typing run: [`TypingMachine`] under a [`ToolMachineRunner`]. Every input runs on its own clock, which must be unique
/// per author and tool (the run's id is minted from it); one input mints at most one transaction.
pub struct Typing<M: Clone + 'static> {
    runner: ToolMachineRunner<TypingMachine<M>, TypingHost>,
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
        let runner = ToolMachineRunner::resume(state.tool, ActorId(state.actor), TypingContext::default(), snapshot, Some(transaction), host)?;
        Ok(Self { runner })
    }

    /// 🆔️ The buffer the open run types into.
    pub fn buffer(&self) -> Option<&str> {
        self.runner.snapshot().context.buffer.as_deref()
    }

    /// ⏱️ When the open run commits on its own.
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
        let (tool, actor, deadline_ms) = (self.runner.tool().to_string(), self.runner.actor().0.clone(), self.runner.host.deadline_ms?);
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
    windows: std::collections::BTreeMap<String, TypingState<M>>,
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

    /// 📨️ Runs one input of `window`'s run on `clock`: an open run of another tool in the window commits first (`selectionJump`).
    /// Answers every step in order.
    pub fn send(&mut self, window: &str, tool: &str, actor: &ActorId, input: TypingInput<M>, fold: impl Fn(&[M], &[M]) -> TypingFold<M>, clock: HybridLogicalTimestamp) -> Result<Vec<ToolStep<M>>, ToolRefusal> {
        let (mut typing, other) = self.run(window, tool, actor);
        let steps = typing.send(input, fold, clock);
        self.keep(window, typing);
        steps.map(|steps| other.into_iter().chain(steps).collect())
    }

    /// 🏁️ Commits `window`'s open run as ONE edit (`reason`); `Idle` when the window does not type.
    pub fn commit(&mut self, window: &str, reason: TypingCommit, clock: HybridLogicalTimestamp) -> Result<ToolStep<M>, ToolRefusal> {
        let Some(state) = self.windows.remove(window) else { return Ok(ToolStep::Idle) };
        let mut typing = Typing::resume(state)?;
        typing.runner.host.now_ms = clock.physical_ms;
        let step = typing.runner.send(TypingEvent::Commit { reason }, clock);
        self.keep(window, typing);
        step
    }

    /// 🏁️ Commits every open run (another verb needs the typed text first, the page is left): one step per window.
    pub fn commit_all(&mut self, reason: TypingCommit, clock: HybridLogicalTimestamp) -> Vec<(String, Result<ToolStep<M>, ToolRefusal>)> {
        let windows: Vec<String> = self.windows.keys().cloned().collect();
        windows.into_iter().map(|window| (window.clone(), self.commit(&window, reason, clock))).collect()
    }

    /// ⏱️ Fires the idle lapse of every run whose deadline `clock` reached: each commits as ONE edit, one step per lapsed window.
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

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🧪️node-graph-edit-rows/🦀️.rs"]
mod node_graph_edit_rows_tests;
#[cfg(test)]
#[path = "🧪️tests/🧪️gesture-drive-law/🦀️.rs"]
mod gesture_drive_law_tests;
//#endregion 🧪️Tests
