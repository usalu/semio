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

use machine::{Command, Configuration, Host, Machine, NullInspector, Snapshot, TimerId};
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

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
