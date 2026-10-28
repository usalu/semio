//! 📨️ What an app emits: edits, tasks, child emissions, actions and the command macros.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[doc(hidden)]
#[path = "🛬️lifted/📦️preparation/🦀️.rs"]
pub mod child_emit_preparation;
#[allow(unused_imports)]
pub use child_emit_preparation::*;
#[doc(hidden)]
#[path = "🛬️lifted/🪟️surface/🦀️.rs"]
pub mod surface_contract;
use crate::child_emit_preparation::ChildEmitPreparation;
use crate::child_emit_preparation::ChildEmitPreparationStep;
use crate::child_emit_preparation::OwnedChildEmit;
use semio_framework_plugin_window_config::WindowConfigMutation;
use semio_framework::kernel::UiDirtyScope;
use semio_framework::{kernel::{AppEvent, Effect, SchemaId}, Fault, FaultCode, FaultOrigin, ViewModel};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::Serialize;
/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: `AsyncTask`'s boxed `run` closure/future.
use std::future::Future;
/// 🧵️ Used only by `artifact_app_laws`' spawned-task shape, so a plain `cargo check` reports it
/// unused while `--all-targets` needs it — do not remove on the strength of that warning.
#[allow(unused_imports)]
use std::pin::Pin;
/// 🚪️ `io`'s `ArtifactRef`/`ArtifactKindId` vocabulary is not glob-re-exported at the
/// `semio-framework-os-kernel` crate root (deliberate — see that crate's own glue.rs comment on
/// the `io` mount), so it is named through the `store::io::` path everywhere in this file,
/// exactly like the sibling `🎞️gif` migration leaf (`semio_framework_artifact_reference::ArtifactDialect`) already does.
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_ui_locale::LocalizedLabel;
/// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
/// 2). Defined in `semio_framework` so plugins and the OS product
/// share one definition without an inverted dependency; re-exported here verbatim.
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::mem::ManuallyDrop;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_plugin_decl_views::NoConfigMutation;
use semio_framework_plugin_decl_views::NoDraftMutation;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;

/// 📤️ What a pure `ArtifactApp::handle` emits: zero-or-more typed document operations (applied
/// through the document store with a true inverse) and zero-or-more typed config operations (applied
/// through the config store, also with a true inverse via `ConfigMutation::inverse` — the config-op
/// twin of a document op, replacing the old `ActionEmit::inverse`/`InverseAction` ad hoc self-computed
/// inverse: a former "View"-kind action now just emits a `ConfigMutation` and gets a real inverse
/// for free), plus host effects (navigate/export/spawn…) and app events. An emission never labels its edit: the history
/// row reads its leaves' localized labels (design §20.6). `B1` rename: was `ActionEmit`, `operations` renamed
/// `artifact_mutations` to sit next to `config_mutations` unambiguously.
#[derive(semio_framework_value::RetireOwned)]
pub struct Emit<Mutation, ConfigMutation = NoConfigMutation, DraftMutation = NoDraftMutation> {
    pub artifact_mutations: Vec<Mutation>,
    pub config_mutations: Vec<ConfigMutation>,
    pub window_config_mutations: Vec<WindowConfigMutation>,
    pub draft_mutations: Vec<DraftMutation>,
    /// 🛠️ The `ToolTransaction` that authored `artifact_mutations`: stamped on every published op as
    /// `MutationMeta.transaction`, so history lists the gesture as one row and edits its yielded mutations.
    /// `None` outside a tool transaction; streamed or aborted only on the app's own document ([`Self::transaction_phase`]). Build it with [`Emit::commit_transaction`],
    /// [`Emit::stream_transaction`] or [`Emit::abort_transaction`].
    pub transaction: Option<protocol::TransactionRef>,
    /// 🎞️ How `artifact_mutations` join `transaction` (design §15): committed as its one edit, streamed into its open
    /// edit, or its open edit aborted.
    pub transaction_phase: TransactionPhase,
    pub effects: Vec<Effect>,
    /// 🔁️ Zero-or-more extension capability invocations whose OUTCOME must come back to this app
    /// — see [`ExtensionInvocation`]. Deliberately NOT expressible through `effects`: the
    /// kernel's `Effect::InvokeExtension` carries a `req` that only the guest request registry
    /// can mint, so this lane exists precisely so an app never writes one. `dispatch_emit` (and
    /// the retained tool-operation publication path) drains it through
    /// `reactor::queue_extension_invocation`, which allocates the id, records the continuation
    /// and queues the effect.
    pub extension_invocations: Vec<ExtensionInvocation>,
    pub events: Vec<AppEvent>,
    /// 🐢️ Which rendered UI sections this action actually invalidates — `Full` (the default) preserves
    /// today's whole-shell-refresh behavior for every app that doesn't opt in to narrower scopes.
    pub ui_scope: UiDirtyScope,
    /// 🧩️ Ready owned-child edits published with this gesture as one undo group.
    /// [`ChildEmitPreparation`] encodes each operation before appending its semantic label,
    /// retains refusals and retires typed operation owners before issuing the ready wire group.
    pub child_emits: Vec<ChildEmit>,
    /// 🪵️ Applying child sources retain their typed operation allocation through group publication.
    pub owned_child_emits:Vec<OwnedChildEmit>,
    /// 📦️ Owned typed operations awaiting bounded encoding and retirement before publication.
    pub child_preparations: std::collections::VecDeque<ChildEmitPreparation>,
    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): zero-or-more app-initiated
    /// selection writes riding alongside this SAME gesture — see [`InteractionWrite`]. Applied
    /// after this emit's own mutation lanes have landed, through the one single-writer path the
    /// reserved interaction verbs already use. Empty by default, so no other app changes.
    pub interaction_writes: Vec<InteractionWrite>,
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): zero-or-more tasks
    /// spawned onto this actor's `LocalExecutor` alongside this SAME gesture — `dispatch_emit`
    /// spawns each one (quota-gated, key-deduped) right after the mutation lanes above have
    /// landed, so `TaskCtx.meta` always reflects state that was ACTUALLY committed, never just
    /// requested. This is the ONLY way a command handler can await anything: `handle` itself
    /// stays a pure synchronous reducer (see `AsyncTask`'s own doc for the full contract).
    pub tasks: Vec<AsyncTask<Mutation, ConfigMutation, DraftMutation>>,
}

/// 🎞️ How an emission's artifact mutations join its tool transaction (design §15, transaction-scoped amend). `Commit`
/// records them as the transaction's one edit — closing the transaction's open edit when earlier ticks streamed into it.
/// `Stream` appends them to the transaction's open edit (the first tick opens it) and keeps it open: the store folds it
/// as its applied tail, but nothing is announced or persisted before the commit. `Abort` reverts the open edit with zero
/// trace.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[derive(semio_framework_value::RetireOwned)]
pub enum TransactionPhase {
    #[default]
    Commit,
    Stream,
    Abort,
}

/// ✋️ A released node drag's emission (`semio_framework_tool_machine::node_drag_emit`): ONE committed tool transaction,
/// a plain edit for a view without command authority, or nothing.
impl<Mutation, ConfigMutation, DraftMutation> From<semio_framework_tool_machine::NodeDragEmit<Mutation>> for Emit<Mutation, ConfigMutation, DraftMutation> {
    fn from(drag: semio_framework_tool_machine::NodeDragEmit<Mutation>) -> Self {
        match drag {
            semio_framework_tool_machine::NodeDragEmit::Commit(transaction, artifact_mutations) => Self::commit_transaction(transaction, artifact_mutations),
            semio_framework_tool_machine::NodeDragEmit::Plain(artifact_mutations) => Self::mutations(artifact_mutations),
            semio_framework_tool_machine::NodeDragEmit::Nothing => Self::default(),
        }
    }
}

impl<Mutation, ConfigMutation, DraftMutation> Default for Emit<Mutation, ConfigMutation, DraftMutation> {
    fn default() -> Self {
        Self {
            artifact_mutations: Vec::new(),
            config_mutations: Vec::new(),
            window_config_mutations: Vec::new(),
            draft_mutations: Vec::new(),
            transaction: None,
            transaction_phase: TransactionPhase::Commit,
            effects: Vec::new(),
            extension_invocations: Vec::new(),
            events: Vec::new(),
            ui_scope: UiDirtyScope::default(),
            child_emits: Vec::new(),
            owned_child_emits:Vec::new(),
            child_preparations: std::collections::VecDeque::new(),
            interaction_writes: Vec::new(),
            tasks: Vec::new(),
        }
    }
}

/// 🔁️ One extension capability call plus the app command its RESULT is dispatched into — the
/// whole of an app's extension-invocation surface, and the only shape from which a
/// `Effect::InvokeExtension` is ever built.
///
/// 🎯️ Why a declared `response_action` and not an awaited future: `ArtifactApp::handle` is a
/// pure synchronous reducer, so it has no place to hold a `RequestFuture` across the turns an
/// extension round-trip spans. The SDK therefore mints the `req` from the guest
/// `RequestRegistry` (`request_continuation`) and, when `Event::Completed` for that exact id
/// arrives, dispatches `response_action` back into the SAME app instance with `request_json`'s
/// own fields merged with the outcome (`ok` plus `outputJson`, or `faultCode`/`faultMessage`).
/// Correlation is therefore whatever the app already put in its request — a node hash, a
/// handle — never a framework-invented key.
///
/// ⚠️ `response_action` must name an APP-OWNED command of this same app (it is dispatched
/// through `PluginApp::handle_command` against the instance's own owner address); an unknown id
/// surfaces as an ordinary dispatch fault frame, never a silent drop.
#[derive(Clone, Debug, PartialEq, Eq)]
#[derive(semio_framework_value::RetireOwned)]
pub struct ExtensionInvocation {
    pub extension_id: String,
    pub capability: String,
    pub request_json: String,
    pub response_action: String,
}

impl ExtensionInvocation {
    pub fn new(extension_id: impl Into<String>, capability: impl Into<String>, request_json: impl Into<String>, response_action: impl Into<String>) -> Self {
        Self { extension_id: extension_id.into(), capability: capability.into(), request_json: request_json.into(), response_action: response_action.into() }
    }
}

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): the ONLY way a plugin's code
/// can await anything. `ArtifactApp::handle` (the pure reducer) stays fully synchronous —
/// waiting happens entirely INSIDE a task's `run` future, which is handed a `TaskCtx` (the
/// awaitable `host::*` surface plus the ORIGINATING `ActionMeta`) once `dispatch_emit` actually
/// spawns it, and the task's eventual result re-enters the reducer as a FOLLOW-UP dispatch
/// (Elm's Msg-from-Cmd) — never by mutating anything directly from inside the future. Every
/// task is owned by the instance that spawned it and is cancelled the moment that instance
/// closes (`Event::InstanceClose`) — there is no detached/fire-and-forget spawn.
///
/// `run`'s closure is `Send` because a pure reducer may create it on a worker before handing the
/// resulting emit back to its actor. The future it creates remains actor-local and need not be
/// `Send`: `TaskCtx` is supplied only after the closure reaches the actor's `LocalExecutor`.
pub struct AsyncTask<Mutation, ConfigMutation = NoConfigMutation, DraftMutation = NoDraftMutation> {
    /// 🪪️ Diagnostic name — never parsed, only ever displayed (a quota-exceeded `Fault`'s
    /// message names the task that was refused).
    pub label: String,
    /// 🔑️ Latest-wins dedupe: spawning a task under the same `(instance, key)` as one already
    /// live cancels the live one first — the common "a new gesture supersedes an in-flight
    /// autosave/search/etc for the same purpose" shape.
    pub key: Option<String>,
    /// 🔁️ OpBinary-encoded command bytes — if the actor restores from a checkpoint taken while
    /// this task was still in flight, THIS is re-dispatched (as a fresh `TaskResolution::
    /// Command`) on the first turn after restore instead of the task itself, which is never
    /// serialized (design-abi.md §4: an in-flight async task is marked re-run-on-restore, not
    /// resumed as though the host round-trip were still live). `None` (the default) means the
    /// task is simply dropped, unrestarted, on restore — correct for idempotent/best-effort
    /// work that a restore should not repeat.
    pub restart: Option<Vec<u8>>,
    run: Box<dyn AsyncTaskRun<Mutation,ConfigMutation,DraftMutation>>,
}

impl<Mutation, ConfigMutation, DraftMutation> AsyncTask<Mutation, ConfigMutation, DraftMutation> {
    /// 🌱️ Builds a task from an async closure. `run` is called once, at the moment
    /// `dispatch_emit` actually spawns this task (never eagerly at `Emit` construction time),
    /// with the `TaskCtx` it should await against.
    pub fn new<T:AsyncTaskSource<Mutation,ConfigMutation,DraftMutation>>(label:String,source:T,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,String,T)> where Mutation:'static,ConfigMutation:'static,DraftMutation:'static {
        let progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:size_of::<TypedAsyncTaskRun<T,Mutation,ConfigMutation,DraftMutation>>(),..Default::default()};
        if grant.maximum_items==0||grant.maximum_depth==0||!progress.fits(grant){return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original typed task constructor requires its independent physical grant"),label,source))}
        let source=match ControlledRetirement::new(source){Ok(source)=>source,Err((error,source))=>return Err((error,label,source))};
        Ok((Self{label,key:None,restart:None,run:Box::new(TypedAsyncTaskRun{source,kind:std::marker::PhantomData})},progress))
    }

    /// 🔑️ See the `key` field's own doc.
    pub fn keyed(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// 🔁️ See the `restart` field's own doc.
    pub fn restartable(mut self, command: Vec<u8>) -> Self {
        self.restart = Some(command);
        self
    }

    /// 🧵️ `⚛️reactor::spawn_task`'s sole consumer — decomposes the task into its plain-data
    /// bookkeeping fields plus the boxed closure, so the reactor (which has no reason to know
    /// about `AsyncTask` as a named type beyond spawning it) never has to reconstruct one.
    pub fn into_parts(self) -> (String, Option<String>, Option<Vec<u8>>, Box<dyn AsyncTaskRun<Mutation,ConfigMutation,DraftMutation>>) {
        (self.label, self.key, self.restart, self.run)
    }

    /// 🧊️ Retires a refused task cold; see [`dispose_cold_task_run`].
    pub fn dispose_cold(self) {
        dispose_cold_task_run(self.run);
    }
}

/// 🪪️ Handed to an `AsyncTask::run` closure at spawn time — the awaitable `host::*` surface
/// (this actor's shared `RequestRegistry`, scoped to the spawning instance — see
/// `⚛️reactor::host`) plus the CLONED `ActionMeta` the task was spawned under, so its eventual
/// follow-up dispatch stays attributed to the same actor/instance even if a different one is
/// active by the time the task resolves.
pub struct TaskCtx {
    pub host: semio_framework_plugin_actor_host::Host,
    pub meta: ActionMeta,
}

/// 📬️ What an `AsyncTask` produces once its future resolves. Re-enters the (still pure/sync)
/// reducer as a follow-up rather than mutating anything directly — Elm's Msg-from-Cmd.
///
/// `Command` is the CANONICAL form: the document may have drifted while the task awaited, so
/// the bytes re-enter `ArtifactApp::handle` against CURRENT state via the exact same
/// `OpBinary`-typed path (`PluginApp::handle_command_frame`) a fresh command uses, stamped
/// with the task's originating `ActionMeta`.
///
/// `Emit` is a narrower, trivially-safe shortcut for a LITERAL mutation-lane follow-up that
/// skips re-entering `handle` entirely — only `artifact_mutations`/`config_mutations`/
/// `draft_mutations` are honored; `effects`/`events`/`child_emits`/`tasks` on this `Emit` are
/// NOT applied (use `Command` if the follow-up needs any of those).
///
/// `Done` performs no follow-up dispatch at all — for a task whose only job was a
/// `TaskCtx::host` side effect (e.g. `host.notify(...)`) it already queued during its own run.
pub enum TaskResolution<Mutation, ConfigMutation = NoConfigMutation, DraftMutation = NoDraftMutation> {
    Command(Vec<u8>),
    Emit(Emit<Mutation, ConfigMutation, DraftMutation>),
    Done,
}

include!("../../🧵️task/🛬️source/🦀️.rs");

/// 🕹️ One app-initiated selection write riding alongside an ordinary action's own mutations —
/// the ONLY way a reducer expresses "select what I just created/widened to" ("Select Same Kind",
/// duplicate/accept/place re-select). Never a new client-facing verb: the six reserved
/// `interactionSelect`-family actions stay the only EXTERNAL trigger, and every entry here is
/// applied through the very same single-writer state machine they use
/// (`protocol::next_selection` + `revalidate_and_persist_interaction_state`), after the
/// triggering action's document mutations have landed so a just-created id is already in
/// topology. See `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`.
#[derive(Clone, Debug, PartialEq)]
#[derive(semio_framework_value::RetireOwned)]
pub struct InteractionWrite {
    pub domain: String,
    pub targets: Vec<protocol::InteractionTarget>,
    pub merge: protocol::MergeMode,
}

impl InteractionWrite {
    /// 🎯️ Replaces `domain`'s selection with `ids`, all at one granularity — the shape every
    /// "select exactly these" reducer wants.
    pub fn replace(domain: impl Into<String>, granularity: &str, ids: impl IntoIterator<Item = String>) -> Self {
        Self { domain: domain.into(), targets: ids.into_iter().map(|id| protocol::InteractionTarget { granularity: granularity.to_string(), id }).collect(), merge: protocol::MergeMode::Replace }
    }
}

/// 🌱️ Explicit private member input retained until one composed group accepts or retires it.
#[derive(Clone, Debug, PartialEq, Serialize, ToValue, FromValue)]
#[derive(semio_framework_value::RetireOwned,semio_framework_value::NativeReceive)]
pub struct ChildEmitGenesis {
    #[serde(serialize_with = "serialize_child_genesis_reference")]
    pub reference: ArtifactRef,
    pub initial_pack: Vec<u8>,
}

fn serialize_child_genesis_reference<S: serde::Serializer>(reference: &ArtifactRef, serializer: S) -> Result<S::Ok, S::Error> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct BorrowedReference<'a> { artifact_id: &'a str, dialect: &'a ArtifactDialect }
    BorrowedReference { artifact_id: &reference.artifact_id, dialect: &reference.dialect }.serialize(serializer)
}

impl ChildEmitGenesis {
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        let release_bytes = if self.initial_pack.capacity() != 0 { self.initial_pack.capacity() } else {
            [&self.reference.artifact_id, &self.reference.dialect.artifact_kind, &self.reference.dialect.standard, &self.reference.dialect.subset].into_iter().find_map(|value| (value.capacity() != 0).then_some(value.capacity())).unwrap_or(0)
        };
        Ok(RetirementDemand { release_bytes, depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.initial_pack.capacity() == 0 && [&self.reference.artifact_id, &self.reference.dialect.artifact_kind, &self.reference.dialect.standard, &self.reference.dialect.subset].into_iter().all(|value| value.capacity() == 0)
    }

    pub fn close_one(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let demand = self.retirement_demands()?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "original child genesis release exceeds supplied depth")); }
        if grant.maximum_items == 0 || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.initial_pack.capacity() != 0 { self.initial_pack = Vec::new(); }
        else { *[&mut self.reference.artifact_id, &mut self.reference.dialect.artifact_kind, &mut self.reference.dialect.standard, &mut self.reference.dialect.subset].into_iter().find(|value| value.capacity() != 0).expect("original genesis release frontier") = String::new(); }
        let progress = RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    fn return_one<const N: usize>(&mut self, parent: &mut semio_framework_value::retirement::allocation_return::ParentAllocationReturn<N>) -> Result<RetainedCloneStep, ValueError> {
        let accepted = |accepted| RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(accepted), ..Default::default() });
        if self.initial_pack.capacity() != 0 { return parent.return_bytes(&mut self.initial_pack, 1).map(accepted); }
        for value in [&mut self.reference.artifact_id, &mut self.reference.dialect.artifact_kind, &mut self.reference.dialect.standard, &mut self.reference.dialect.subset] {
            if value.capacity() != 0 { return parent.return_text(value, 1).map(accepted); }
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
}

/// 🧩️ One composed child's ready wire group, issued by [`ChildEmitPreparation`].
/// Complete encoded operations and their semantic labels share the same accepted prefix.
/// A refused operation leaves that prefix unchanged and retains its typed owner in preparation.
/// `owner` is the owner-path text ([`MemberPath`]) of the member that owns the target; empty when the document itself does.
#[derive(Clone, Debug, PartialEq, Serialize, ToValue, FromValue)]
#[derive(semio_framework_value::RetireOwned,semio_framework_value::NativeReceive)]
pub struct ChildEmit {
    pub genesis: Option<ChildEmitGenesis>,
    pub owner: String,
    pub slot: String,
    pub child_id: String,
    pub ops: Vec<Vec<u8>>,
    pub op_schema: SchemaId,
    pub labels: Vec<LocalizedLabel>,
}

impl ChildEmit {
    /// 🧩️ The wire form of one agent gesture's owned-child share (`AppFrame::Emit.child_ops` →
    /// gateway → `AppCommand::TransactionPrepare.prepared_child_ops`): the exact `ChildEmit` list as one
    /// wire pack, empty bytes when the gesture touches no owned child, so a childless transaction's wire
    /// is unchanged.
    pub fn encode_groups(children: &[ChildEmit]) -> Vec<u8> {
        if children.is_empty() {
            return Vec::new();
        }
        store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(children))
    }

    /// 🧩️ Decodes [`ChildEmit::encode_groups`]; a pack that is not a `ChildEmit` list is refused by name.
    pub fn decode_groups(bytes: &[u8]) -> Result<Vec<ChildEmit>, Fault> {
        if bytes.is_empty() {
            return Ok(Vec::new());
        }
        let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("transaction.child-groups-malformed"), format!("owned-child op groups did not decode: {error}")))?;
        <Vec<ChildEmit> as semio_framework_value::FromValue>::from_value(value).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("transaction.child-groups-malformed"), format!("owned-child op groups are not a ChildEmit list: {error}")))
    }

    /// ♻️ Retires one original child allocation under every independent caller currency.
    pub fn close_one(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let demand = self.retirement_demands()?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "original child release exceeds supplied depth")); }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(genesis) = self.genesis.as_mut() {
            if genesis.terminal_is_empty() { self.genesis.take(); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = genesis.close_one(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, genesis.terminal_is_empty(), "original child genesis")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if !self.ops.is_empty() { self.ops.pop(); }
        else if self.ops.capacity() != 0 { self.ops = Vec::new(); }
        else if let Some(label) = self.labels.last_mut() {
            match label.close_owned_cell_one(grant.maximum_release_bytes) {
                Ok(Some(bytes)) => return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() })),
                Ok(None) => { self.labels.pop(); },
                Err(_) => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original label release changed its admitted frontier")),
            }
        }
        else if self.labels.capacity() != 0 { self.labels = Vec::new(); }
        else { *[&mut self.op_schema.0, &mut self.child_id, &mut self.slot, &mut self.owner].into_iter().find(|value| value.capacity() != 0).expect("original child text release frontier") = String::new(); }
        let progress = RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.genesis.is_none() && self.ops.capacity() == 0 && self.labels.capacity() == 0 && [&self.op_schema.0, &self.child_id, &self.slot, &self.owner].into_iter().all(|value| value.capacity() == 0)
    }

    /// 🏠️ Returns a ready child's concrete allocations to its real parent, which remains physically nonterminal.
    fn return_one<const N: usize>(&mut self, parent: &mut semio_framework_value::retirement::allocation_return::ParentAllocationReturn<N>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let accepted = |accepted| RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(accepted), ..Default::default() });
        if grant.maximum_items == 0 { return Ok(accepted(false)); }
        if let Some(genesis) = self.genesis.as_mut() {
            let step = genesis.return_one(parent)?;
            if matches!(step, RetainedCloneStep::Complete(_)) { self.genesis = None; return Ok(accepted(true)); }
            return Ok(step);
        }
        if let Some(op) = self.ops.last_mut() { if op.capacity() != 0 { return parent.return_bytes(op, 1).map(accepted); } self.ops.pop(); return Ok(accepted(true)); }
        if self.ops.capacity() != 0 { return parent.return_empty_vec(&mut self.ops, 1).map(accepted); }
        if let Some(label) = self.labels.last_mut() { return match label.return_owned_cell_one(parent, 1)? { Some(is_accepted) => Ok(accepted(is_accepted)), None => { self.labels.pop(); Ok(accepted(true)) } }; }
        if self.labels.capacity() != 0 { return parent.return_empty_vec(&mut self.labels, 1).map(accepted); }
        for text in [&mut self.op_schema.0, &mut self.child_id, &mut self.slot, &mut self.owner] { if text.capacity() != 0 { return parent.return_text(text, 1).map(accepted); } }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }

    /// 📏️ Reads the next original physical allocation without increasing caller authority.
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if let Some(genesis) = self.genesis.as_ref() {
            if genesis.terminal_is_empty() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
            let mut demand = genesis.retirement_demands()?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "original child genesis depth overflow"))?;
            return Ok(demand);
        }
        let layout = |capacity: usize, cell: usize| capacity.checked_mul(cell).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original child directory layout overflow"));
        let (release_bytes, depth) = if let Some(op) = self.ops.last() { (op.capacity(), 2) }
            else if self.ops.capacity() != 0 { (layout(self.ops.capacity(), size_of::<Vec<u8>>())?, 1) }
            else if let Some(label) = self.labels.last() { (label.next_owned_close_byte_demand(), 2) }
            else if self.labels.capacity() != 0 { (layout(self.labels.capacity(), size_of::<LocalizedLabel>())?, 1) }
            else { ([&self.op_schema.0, &self.child_id, &self.slot, &self.owner].into_iter().find_map(|value| (value.capacity() != 0).then_some(value.capacity())).unwrap_or(0), usize::from(!self.terminal_is_empty())) };
        Ok(RetirementDemand { release_bytes, depth, ..Default::default() })
    }

    /// 🫙️ [`Self::of`] with no op yet and room for `capacity`: a stepped finalize fills one emission across turns through
    /// [`Self::push`].
    pub fn open(slot: impl Into<String>, child_id: impl Into<String>, capacity: usize) -> ChildEmit {
        ChildEmit { genesis: None, owner: String::new(), slot: slot.into(), child_id: child_id.into(), ops: Vec::with_capacity(capacity), op_schema: SchemaId("child.empty".to_string()), labels: Vec::with_capacity(capacity) }
    }

    /// ➕️ Appends one op, its label captured before encoding; the first op names `op_schema`.
    pub fn push<S,M>(&mut self,op:&M)->Result<Option<SchemaId>,::protocol::ProtocolError>
    where M:protocol::SemanticMutation<S>+::protocol::OpBinary{
        let encoded=::protocol::OpBinary::encode_op(op)?;
        let label=protocol::SemanticMutation::label(op);
        let retired_schema=if self.ops.is_empty(){
            let semantics=protocol::SemanticMutation::semantics(op);
            Some(std::mem::replace(&mut self.op_schema,SchemaId(format!("{}.{}",semantics.entity,semantics.kind))))
        }else{None};
        self.labels.push(label);self.ops.push(encoded);Ok(retired_schema)
    }
}

impl<Mutation, ConfigMutation, DraftMutation> Emit<Mutation, ConfigMutation, DraftMutation> {
    /// 🎭️ Prepares the wire form of the front preparation: an applying typed source becomes the incremental encoder's source first, so its operations leave in the same order and the owned batch is never built.
    pub fn prepare_child_preview_one(&mut self,grant:RetainedCloneGrant)->Result<ChildEmitPreparationStep,Fault>{
        if let Some(front)=self.child_preparations.front_mut().filter(|owner|owner.next_owned_capacity_byte_demand()!=0){front.begin_preview();}
        self.prepare_child_one(grant)
    }
    /// 📏️ Quotes the next original preparation effect without creating its caller authority.
    pub fn child_preparation_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let layout=|capacity:usize,cell:usize|capacity.checked_mul(cell).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original child preparation layout overflow"));
        let Some(preparation)=self.child_preparations.front()else{return Ok(RetirementDemand{release_bytes:layout(self.child_preparations.capacity(),size_of::<ChildEmitPreparation>())?,depth:usize::from(self.child_preparations.capacity()!=0),..Default::default()})};
        if preparation.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:preparation.owner_cell_bytes(),depth:1,..Default::default()});}
        let owned=preparation.next_owned_capacity_byte_demand()!=0;
        let(capacity,length,cell)=if owned{(self.owned_child_emits.capacity(),self.owned_child_emits.len(),size_of::<OwnedChildEmit>())}else{(self.child_emits.capacity(),self.child_emits.len(),size_of::<ChildEmit>())};
        if capacity==length{return Ok(RetirementDemand{copy_bytes:layout(length,cell)?,capacity_bytes:layout(length.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original child output length overflow"))?,cell)?,release_bytes:layout(capacity,cell)?,depth:1});}
        if preparation.preparation_is_ready(){return Ok(RetirementDemand{capacity_bytes:preparation.next_owned_capacity_byte_demand(),depth:1,..Default::default()});}
        let mut demand=preparation.step_demands()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original child preparation depth overflow"))?;let _=body;Ok(demand)
    }
    /// 📨️ Advances exactly one original preparation, directory or source transfer effect.
    pub fn prepare_child_one(&mut self,grant:RetainedCloneGrant)->Result<ChildEmitPreparationStep,Fault>{
        let idle=RetainedCloneProgress::default();
        if self.child_preparations.is_empty()&&self.child_preparations.capacity()==0{return Ok(ChildEmitPreparationStep::Ready(idle));}
        let demand=self.child_preparation_demands(grant.maximum_copy_bytes).map_err(|error|crate::child_emit_preparation::retirement_refusal_fault(&error))?;
        if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(ChildEmitPreparationStep::Pending(idle));}
        let Some(preparation)=self.child_preparations.front_mut()else{
            self.child_preparations=std::collections::VecDeque::new();
            return Ok(ChildEmitPreparationStep::Pending(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..idle}));
        };
        if preparation.terminal_is_empty(){self.child_preparations.pop_front();return Ok(ChildEmitPreparationStep::Pending(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..idle}));}
        let owned=preparation.next_owned_capacity_byte_demand()!=0;
        let needs_capacity=if owned{self.owned_child_emits.len()==self.owned_child_emits.capacity()}else{self.child_emits.len()==self.child_emits.capacity()};
        if needs_capacity{
            let retained_capacity_bytes=if owned{self.owned_child_emits.try_reserve_exact(1).map_err(|_|plugin_sdk_fault("child-emission-output-allocation-refused"))?;self.owned_child_emits.capacity()*size_of::<OwnedChildEmit>()}else{self.child_emits.try_reserve_exact(1).map_err(|_|plugin_sdk_fault("child-emission-output-allocation-refused"))?;self.child_emits.capacity()*size_of::<ChildEmit>()};
            return Ok(ChildEmitPreparationStep::Pending(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes,released_bytes:demand.release_bytes}));
        }
        if !preparation.preparation_is_ready(){
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=preparation.step(child)?;
            if !step.progress().fits(child){return Err(plugin_sdk_fault("original child preparation exceeded its caller grant"));}
            return Ok(match step{ChildEmitPreparationStep::Ready(progress)=>ChildEmitPreparationStep::Pending(progress),step=>step});
        }
        if owned{
            let Some((child,progress))=preparation.take_ready_owned_qualified(grant).map_err(|error|crate::child_emit_preparation::retirement_refusal_fault(&error))?else{return Ok(ChildEmitPreparationStep::Pending(idle))};
            if !progress.fits(grant){return Err(plugin_sdk_fault("original child source transfer exceeded its caller grant"));}
            self.owned_child_emits.push(child);return Ok(ChildEmitPreparationStep::Pending(progress));
        }
        let child=preparation.take_ready().ok_or_else(||plugin_sdk_fault("child-emission-ready-without-complete-owned-prefix"))?;self.child_emits.push(child);
        Ok(ChildEmitPreparationStep::Pending(RetainedCloneProgress{copied_items:1,..idle}))
    }

    /// 📏️ Borrows the next child close frontier without deriving caller authority from it.
    pub fn child_close_demands(&self, body: usize) -> Result<Option<RetirementDemand>, ValueError> {
        let nested = |mut demand: RetirementDemand| -> Result<Option<RetirementDemand>, ValueError> {
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "original emitted child depth overflow"))?;
            Ok(Some(demand))
        };
        let backing = |capacity: usize, cell: usize| -> Result<Option<RetirementDemand>, ValueError> {
            let release_bytes = capacity.checked_mul(cell).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original emitted child directory overflow"))?;
            Ok(Some(RetirementDemand { release_bytes, depth: 1, ..Default::default() }))
        };
        if let Some(preparation) = self.child_preparations.front() {
            if preparation.terminal_is_empty() { return Ok(Some(RetirementDemand { release_bytes: preparation.owner_cell_bytes(), depth: 1, ..Default::default() })); }
            return nested(preparation.retirement_demands(body)?);
        }
        if self.child_preparations.capacity() != 0 { return backing(self.child_preparations.capacity(), size_of::<ChildEmitPreparation>()); }
        if let Some(child) = self.owned_child_emits.last() {
            if child.terminal_is_empty() { return Ok(Some(RetirementDemand { depth: 1, ..Default::default() })); }
            return nested(child.retirement_demands(body)?);
        }
        if self.owned_child_emits.capacity() != 0 { return backing(self.owned_child_emits.capacity(), size_of::<OwnedChildEmit>()); }
        if let Some(child) = self.child_emits.last() {
            if child.terminal_is_empty() { return Ok(Some(RetirementDemand { depth: 1, ..Default::default() })); }
            return nested(child.retirement_demands()?);
        }
        if self.child_emits.capacity() != 0 { return backing(self.child_emits.capacity(), size_of::<ChildEmit>()); }
        Ok(None)
    }

    /// ♻️ Keeps original child owners until their supplied grant admits one real release or transfer.
    pub fn close_child_one(&mut self, grant: RetainedCloneGrant) -> Result<Option<PluginLifecycleStep>, Fault> {
        let Some(demand) = self.child_close_demands(grant.maximum_copy_bytes).map_err(|error| crate::child_emit_preparation::retirement_refusal_fault(&error))? else { return Ok(None); };
        if grant.maximum_depth < demand.depth { return Err(plugin_sdk_fault("original emitted child close exceeds supplied depth")); }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(Some(PluginLifecycleStep::Progress(Default::default()))); }
        let child_grant = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(preparation) = self.child_preparations.front_mut() {
            if preparation.terminal_is_empty() {
                self.child_preparations.pop_front();
                return Ok(Some(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() })));
            }
            preparation.begin_close();
            let step = preparation.close_step(child_grant)?;
            if let Some(progress) = step.progress() { if !progress.fits(child_grant) { return Err(plugin_sdk_fault("original child preparation exceeded its supplied grant")); } }
            if matches!(step, PluginLifecycleStep::Complete(_)) && !preparation.terminal_is_empty() { return Err(plugin_sdk_fault("original child preparation completed with retained owners")); }
            return Ok(Some(match step { PluginLifecycleStep::Complete(progress) => PluginLifecycleStep::Progress(progress), step => step }));
        }
        if self.child_preparations.capacity() != 0 { self.child_preparations = std::collections::VecDeque::new(); }
        else if let Some(child) = self.owned_child_emits.last_mut() {
            if child.terminal_is_empty() { self.owned_child_emits.pop(); return Ok(Some(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))); }
            let step = child.close_granted(child_grant).map_err(|error| crate::child_emit_preparation::retirement_refusal_fault(&error))?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child_grant, step, child.terminal_is_empty(), "original emitted typed child").map_err(|error| crate::child_emit_preparation::retirement_refusal_fault(&error))?;
            return Ok(Some(PluginLifecycleStep::Progress(step.progress())));
        }
        else if self.owned_child_emits.capacity() != 0 { self.owned_child_emits = Vec::new(); }
        else if let Some(child) = self.child_emits.last_mut() {
            if child.terminal_is_empty() { self.child_emits.pop(); return Ok(Some(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))); }
            let step = child.close_one(child_grant).map_err(|error| crate::child_emit_preparation::retirement_refusal_fault(&error))?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child_grant, step, child.terminal_is_empty(), "original emitted wire child").map_err(|error| crate::child_emit_preparation::retirement_refusal_fault(&error))?;
            return Ok(Some(PluginLifecycleStep::Progress(step.progress())));
        }
        else { self.child_emits = Vec::new(); }
        Ok(Some(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() })))
    }

    /// 🎟️ Returns ready child backing to the caller's persistent parent; typed preparations retain their original physical close contract.
    pub fn return_child_one<const N: usize>(&mut self, parent: &mut semio_framework_value::retirement::allocation_return::ParentAllocationReturn<N>, grant: RetainedCloneGrant) -> Result<Option<PluginLifecycleStep>, Fault> {
        let refused = |error: ValueError| Fault::from(error.into_message());
        if !self.child_preparations.is_empty() || self.child_preparations.capacity() != 0 || !self.owned_child_emits.is_empty() { return self.close_child_one(grant); }
        let accepted = |accepted| Ok(Some(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: usize::from(accepted), ..Default::default() })));
        if self.owned_child_emits.capacity() != 0 { return parent.return_empty_vec(&mut self.owned_child_emits, 1).map_err(refused).and_then(accepted); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(Some(PluginLifecycleStep::Progress(Default::default()))); }
        if let Some(child) = self.child_emits.last_mut() {
            let step = child.return_one(parent, grant).map_err(refused)?;
            if matches!(step, RetainedCloneStep::Complete(_)) { self.child_emits.pop(); return accepted(true); }
            return Ok(Some(PluginLifecycleStep::Progress(step.progress())));
        }
        if self.child_emits.capacity() != 0 { return parent.return_empty_vec(&mut self.child_emits, 1).map_err(refused).and_then(accepted); }
        Ok(None)
    }
    /// ✏️ A document-operation emission carrying `artifact_mutations` and nothing else.
    pub fn mutations(artifact_mutations: Vec<Mutation>) -> Self {
        Self { artifact_mutations, ..Default::default() }
    }

    /// 🛠️ A committed `ToolTransaction` (`ToolMachineRunner`'s `ToolStep::Committed(transaction, mutations)`)
    /// published as ONE document edit whose every op carries `transaction`; the history row is labelled from the
    /// mutations' own `MutationKind` labels. When earlier ticks streamed into the transaction
    /// ([`Self::stream_transaction`]) the mutations join its open edit and the commit closes it; with nothing open and
    /// nothing to record it is an empty emission.
    pub fn commit_transaction(transaction: protocol::TransactionRef, artifact_mutations: Vec<Mutation>) -> Self {
        Self { artifact_mutations, transaction: Some(transaction), ..Default::default() }
    }

    /// 🎯️ ONE one-step tool dispatch (a click that places, a keyboard nudge): `artifact_mutations` through the
    /// framework's one-step tool machine (`semio_framework_tool_machine::tool_once_emit`) as ONE transaction of the tool
    /// `<app_id>#<verb>`, its ref minted from the admission's `authoring_seed` — the history lists it as one row whose
    /// mutations stay editable. Nothing yielded: the empty emission; no admission (a render or test view): a plain edit.
    pub fn tool_once(app_id: &str, verb: &str, authoring_seed: &str, artifact_mutations: Vec<Mutation>) -> Self
    where
        Mutation: Clone + 'static,
    {
        semio_framework_tool_machine::tool_once_emit(app_id, verb, authoring_seed, artifact_mutations).into()
    }

    /// 🌊️ One tick of a streamed `ToolTransaction` (design §15): `artifact_mutations` join the transaction's open edit
    /// (the first tick opens it), every op stamped with `transaction`. The document shows them at once, but nothing is
    /// announced or persisted until [`Self::commit_transaction`] with the same ref closes the edit; [`Self::abort_transaction`]
    /// reverts it with zero trace. For tools whose yield cannot wait for the commit (streamed imports).
    pub fn stream_transaction(transaction: protocol::TransactionRef, artifact_mutations: Vec<Mutation>) -> Self {
        Self { artifact_mutations, transaction: Some(transaction), transaction_phase: TransactionPhase::Stream, ..Default::default() }
    }

    /// 🧨️ Reverts the open edit of the streamed `ToolTransaction` `transaction` with zero trace: no edit, no history row,
    /// nothing announced. With nothing open it is an empty emission.
    pub fn abort_transaction(transaction: protocol::TransactionRef) -> Self {
        Self { transaction: Some(transaction), transaction_phase: TransactionPhase::Abort, ..Default::default() }
    }

    /// 🪆️ `commit_transaction`'s composed-children twin (design §12): a committed `ToolTransaction` whose leaves
    /// land in owned child members, published as ONE composite group where every touched member's edit carries
    /// `transaction`. Children with no ops are dropped; nothing touched is an empty emission.
    pub fn commit_child_transaction(transaction: protocol::TransactionRef, child_emits: Vec<ChildEmit>) -> Self {
        let child_emits: Vec<ChildEmit> = child_emits.into_iter().filter(|child| !child.ops.is_empty()).collect();
        let transaction = (!child_emits.is_empty()).then_some(transaction);
        Self { child_emits, transaction, ..Default::default() }
    }

    /// 🪆️ A released node drag (`semio_framework_tool_machine::node_drag_emit`) whose leaves land in the owned child
    /// `slot` member `child_id` (design §12): ONE composite group whose member edit carries the transaction, the child
    /// leaves plainly for a view without command authority, or nothing.
    pub fn node_drag_child<S, M>(drag: semio_framework_tool_machine::NodeDragEmit<M>, slot: &str, child_id: &str) -> Self
    where
        M: protocol::SemanticMutation<S> + ::protocol::OpBinary + semio_framework_value::retirement::RetireOwned,
    {
        match drag {
            semio_framework_tool_machine::NodeDragEmit::Commit(transaction, leaves) => Self { child_preparations:std::collections::VecDeque::from([ChildEmitPreparation::of::<S,M>(slot,child_id,leaves)]),transaction:Some(transaction),..Default::default() },
            semio_framework_tool_machine::NodeDragEmit::Plain(leaves) => Self { child_preparations:std::collections::VecDeque::from([ChildEmitPreparation::of::<S,M>(slot,child_id,leaves)]),..Default::default() },
            semio_framework_tool_machine::NodeDragEmit::Nothing => Self::default(),
        }
    }

    /// 🧮️ A config-operation emission carrying `config_mutations` and nothing else — the
    /// replacement for a former "View"-kind `ActionEmit::view_with_inverse`: selection/camera/hover/…
    /// changes now flow through the config store, which computes their real `inverse` itself.
    pub fn config(config_mutations: Vec<ConfigMutation>) -> Self {
        Self { config_mutations, ..Default::default() }
    }

    /// 📝️ A draft-operation emission carrying `draft_mutations` and nothing else.
    pub fn draft(draft_mutations: Vec<DraftMutation>) -> Self {
        Self { draft_mutations, ..Default::default() }
    }

    /// 🐚️ A single host effect and no operations (a shell action).
    pub fn effect(effect: Effect) -> Self {
        Self { effects: vec![effect], ..Default::default() }
    }

    /// 📣️ A single app event and no operations.
    pub fn event(event: AppEvent) -> Self {
        Self { events: vec![event], ..Default::default() }
    }

    /// 🧵️ A single spawned `AsyncTask` and no operations — the common case for "this
    /// command's only job is to kick off host work" (e.g. a search-as-you-type debounce).
    pub fn task(task: AsyncTask<Mutation, ConfigMutation, DraftMutation>) -> Self {
        Self { tasks: vec![task], ..Default::default() }
    }
}

/// 🪪️ Per-invocation runtime metadata handed to the object-safe {@link PluginApp} — the local
/// actor id, the instance id used to stamp operation handles, and optional host view context.
/// Headless operations have no view; UI actions carry the addressed concrete window's projection.
#[derive(Clone, Debug, Default, semio_framework_value::RetireOwned,semio_framework_value::RetainedClone)]
pub struct ActionMeta {
    pub actor: semio_framework_value::SharedUtf8,
    pub instance_id: u32,
    pub view_state: Option<ViewModel>,
}
#[path="🛂️metadata/📑️copy/🦀️.rs"]
pub mod original_metadata_copy;
pub use original_metadata_copy::OriginalActionMetaCopy;


/// 🔤️ Parses the raw action id crossing the WASM ABI (`ArtifactApp::handle_action`'s `action: &str`)
/// into a closed, per-app enum — the seam where "stringly-typed at the edge" becomes exhaustively
/// matched one line in. Not yet wired into `ArtifactApp` itself (that would break every existing
/// implementer at once); adopt it per app by matching on the parsed variant instead of the raw string
/// inside `handle_action`, e.g. `let action = MyAppAction::from_action_id(action)?; match action { ... }`.
pub trait AppAction: Sized {
    async fn from_action_id(id: &str) -> Result<Self, String>;
}

/// 🎮️ Generates a closed per-app `ArtifactApp::Command` enum from `"id" => module::Payload`
/// rows — the taxonomy-decomposed replacement for a hand-written
/// `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)] pub enum XCommand { #[dsl(key = "...")] Variant { .. }, .. }`
/// (see e.g. `flow_protocol::FlowCommand`, `dag_protocol::DagCommand`, `shooting_protocol::ShootingCommand`
/// — every hand-written Command enum in the repo already follows this exact derive/attribute shape, none
/// use `#[serde(tag = ...)]`; the real wire codec is `dsl::DslOps`'s generated `OpText`/`OpBinary`, keyed
/// by each variant's `#[dsl(key = ..)]` string).
///
/// Each row becomes a single-field tuple variant `$Payload($module::$Payload)`. This is deliberate, not
/// incidental: `dsl::DslOps`'s codegen special-cases single-field tuple variants (see
/// `dsl_variants_codegen` in the `dsl-derive` crate) to delegate ENTIRELY to the inner type's own
/// `DslField` impl — its `RecordSpec` IS the payload type's, not a wrapper with one positional field — so
/// a payload struct declared with `#[derive(dsl::DslRecord)]` (its own fields, no extra nesting) prints/
/// parses byte-identically whether reached through this enum or used standalone. Concretely: migrating a
/// hand-written `AddWidget { kind: String, x: Option<f64> }` struct-variant into a
/// `🎮️commands/add_widget/🦀️.rs` payload `struct AddWidget { kind: String, x: Option<f64> }`
/// (deriving `DslRecord`) plus `AddWidget(add_widget::AddWidget)` here reproduces the exact same
/// `OpText`/`OpBinary` wire bytes for the same `#[dsl(key = "...")]` string — the migration is pure code
/// motion, not a wire-format change.
///
/// Expands to:
/// - the enum itself (`$vis enum $Name`), deriving `Clone, Debug, PartialEq, $crate::ToValue,
///   $crate::FromValue, dsl::DslOps` — satisfies `ArtifactApp::Command: protocol::OpBinary + Send`
///   directly. `dsl` is referenced unqualified (not via `$crate`, since it is NOT the defining crate) —
///   every crate that hosts `🎮️commands/*` payload modules already depends on `dsl` directly (payloads
///   themselves derive `dsl::DslRecord`), matching the convention every existing hand-written
///   `*_protocol` crate already follows. `ToValue`/`FromValue`, by contrast, ARE spelled via `$crate` —
///   both are proc-macros re-exported at this crate's own root (`🦀️.rs`), so the path resolves to
///   `semio_framework_value_derive`'s derives regardless of what the invoking plugin crate has
///   imported; every `$Payload` type these wrap must itself derive `ToValue`/`FromValue` (own work, not
///   inherited) for the generated enum impl to compile — no plugin manifest was allowed to drop `serde`
///   from `Cargo.toml` while this line still said `::serde::Serialize, ::serde::Deserialize`, since the
///   enum's own derive transitively required every payload type to implement it too.
/// - `command_id(&self) -> &'static str`, returning each row's `$id` — mirrors the per-command labeling
///   `ArtifactApp::command_id` needs (today hand-rolled as a parallel `match` per app, e.g.
///   `TestApp::command_id` above).
/// - `dispatch(&self, doc: &ArtifactView<'_, $Snapshot>, cfg: &ConfigView<'_, $Config>) -> ArtifactMutationOutcome<$Mutation, $ConfigMutation>`,
///   matching each variant to `$module::handle(payload, doc, cfg)` — `$Snapshot`/`$Mutation`/`$Config`/
///   `$ConfigMutation` are the four types declared after `for` in the invocation (see below).
///
/// # 🔖️DispatchDesignChoice
/// The macro invocation names the app's four `ArtifactApp` associated types up front —
/// `enum $Name for $Snapshot, $Mutation, $Config, $ConfigMutation { .. }` — so `dispatch` can be a
/// perfectly ordinary, CONCRETE (non-generic) inherent method. This was chosen over the alternative
/// tried first: a `dispatch<P, O, C, CO>` generic over all four, inferring them from the call site. That
/// alternative does not type-check, and the `app_commands_tests` module below hit exactly this in
/// practice — every real `🎮️commands/<command>/🦀️.rs::handle` fn will be written against ONE
/// app's concrete `Snapshot`/`Mutation`/`Config`/`ConfigMutation` (the same way
/// `flow_protocol`/`dag_protocol`/... payload handlers are today), never generically; a generic
/// `dispatch<P,O,C,CO>` body calling a concrete-typed `$module::handle` fails to unify (`P` is not
/// literally `FlowHostSnapshot`, even though there is only ever one real instantiation). Naming the four
/// types once, at the `app_commands!` invocation, costs one extra clause per app but produces a
/// `dispatch` whose signature matches `ArtifactApp::handle` exactly — so an app's whole `handle` impl
/// collapses to one line, `command.dispatch(doc, cfg)`. A per-command closure/trait-object API (the
/// plan's other suggested option) would force hand-rolling one closure per row at adoption time instead
/// — strictly more ceremony than stating the four types once.
///
/// # 🔖️KeyedAndContextualForms
/// Two further arms exist, both discovered as hard requirements by the W1 🌊️flow pilot (the first real
/// consumer) and additive to the plain arm above, which is unchanged:
///
/// 1. **Keyed rows** — `"commandId" as "wire-key" => module::Payload`. Every hand-written Command enum in
///    the repo keys its wire form in kebab-case (`#[dsl(key = "add-widget")]`) while `command_id()` must
///    return the camelCase manifest ACTION id (`"addWidget"`) the app declared via `.mutation()`/
///    `.view_action()`. The plain arm conflates the two into one literal, which silently rewrites the wire
///    format of any existing app it is applied to. Stating both keeps a migration pure code motion.
/// 2. **A dispatch context** — `…, $ConfigMutation:ty, ctx = $Ctx:ty { … }` adds a fourth
///    `ctx: &mut $Ctx` parameter to `dispatch` and to every row's `$module::handle`. Apps whose handlers
///    need app-struct state that is deliberately NOT in the document or the config (flow's
///    `Mutex<FlowEvalSession>` off-main-thread eval driver, reached once per dispatch and threaded through
///    every handler) have nowhere else to put it — `handle(&self, …)` on `ArtifactApp` has `&self`, but the
///    macro-generated `dispatch` does not.
///
/// The two are independent; combine them or use `ctx = ()` when only the keys differ.
/// 🎯️ Schema-generated inventory implemented by every [`app_commands!`] command enum.
pub trait ToolCommandCatalog {
    const TOOL_JOB_IDS: &'static [&'static str];
}

/// 📨️ A host fact an app may answer with one of its own typed commands ([`ArtifactApp::host_event`]). Every event
/// names the window it concerns: a host forwards a window's blur, pointer-capture loss and closing
/// (`hostEvent{windowId, kind}`), and the runtime delivers a utility switch it sees in a dispatch's view, the start
/// of a history edit and a remote edit moving the base to every open window.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HostEvent {
    WindowBlurred { window_id: String },
    PointerCaptureLost { window_id: String },
    UtilityChanged { window_id: String, from: Option<String>, to: Option<String> },
    TimeTravelFrozen { window_id: String },
    BaseMoved { window_id: String },
    Retiring { window_id: String },
}

impl HostEvent {
    /// 🪟️ The window instance this event concerns.
    pub fn window_id(&self) -> &str {
        match self {
            Self::WindowBlurred { window_id } | Self::PointerCaptureLost { window_id } | Self::UtilityChanged { window_id, .. } | Self::TimeTravelFrozen { window_id } | Self::BaseMoved { window_id } | Self::Retiring { window_id } => window_id,
        }
    }

    /// 📨️ The event a host forwards as `hostEvent{windowId, kind}`; `None` for an unknown kind.
    pub fn forwarded(window_id: &str, kind: &str) -> Option<Self> {
        let window_id = window_id.to_string();
        match kind {
            semio_framework::HOST_EVENT_KIND_BLUR => Some(Self::WindowBlurred { window_id }),
            semio_framework::HOST_EVENT_KIND_CAPTURE_LOST => Some(Self::PointerCaptureLost { window_id }),
            semio_framework::HOST_EVENT_KIND_RETIRING => Some(Self::Retiring { window_id }),
            _ => None,
        }
    }
}

/// 📬️ Mutation emission paired with its operation failure, without changing ownership.
pub type ArtifactMutationOutcome<M, C = NoConfigMutation, D = NoDraftMutation, E = Fault> = Result<Emit<M, C, D>, E>;
