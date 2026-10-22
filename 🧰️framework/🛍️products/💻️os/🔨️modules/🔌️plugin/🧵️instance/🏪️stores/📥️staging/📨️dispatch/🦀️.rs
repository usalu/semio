//! ⛓️ Staged slice of the `dispatch` band of the mounted artifact instance: its `instance-stores` members (WP6 step S0 of the instance family).
//!
//! 📨️ Dispatch of the mounted artifact instance: test snapshot helpers, command log, history view, patch and refresh, preview, `dispatch_emit` and transaction groups.
use semio_framework_plugin_decl_emission::{ActionMeta, Emit};
use semio_framework::kernel::{ActorId, AppEvent, ArtifactDiff, ArtifactHandle, ArtifactVersion, Effect, HybridLogicalTimestamp, InverseMutation, InvocationId, InvocationResult, KernelMutation, MutationId, SchemaId, UiDirtyScope, UndoGroup, UndoPolicy};
use semio_framework_plugin_command::{AlternativeFact, HistoryRowReads, history_alternative_views};
use semio_framework_plugin_decl_views::history_view::{AlternativeView, CommandLogEntry, CommandView};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use store::{ArtifactStore, MemberFactory, Mutation, SpaceMember};
use store::ReferenceSpaceMember as _;
use semio_framework_value::DslValue;
use semio_framework::{Fault, FaultFrom};
use std::future::Future;
use crate::stores_components::{InstanceStores, LocalInteraction};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_contract::*;

impl<A: ArtifactApp> InstanceStores<A> {
    #[cfg(test)]
    pub async fn test_snapshot(&self) -> A::Snapshot {
        self.store.snapshot().expect("materialize snapshot")
    }

    /// 🧪️ The document store itself — needed to assert on checkpoint metadata
    /// (`composition_pins`), which no snapshot-level accessor exposes.
    #[cfg(test)]
    pub async fn test_store(&self) -> &ArtifactStore<A::Snapshot, A::Mutation> {
        &self.store
    }

    /// ⚔️ Test-only mutable store access for conflict lifecycle fixtures.
    #[cfg(test)]
    pub async fn test_store_mut(&mut self) -> &mut ArtifactStore<A::Snapshot, A::Mutation> {
        &mut self.store
    }

    /// 🧪️ The config-store twin of `test_snapshot`.
    #[cfg(test)]
    pub async fn test_config(&self) -> A::Config {
        self.config_store.snapshot().expect("materialize config snapshot")
    }

    /// 🧪️ The `KernelMutation`/`UndoGroup` half of an `InvocationResult` as it stands NOW,
    /// read from the store's own last edit. A migrated dispatch returns an admission receipt whose
    /// `mutations` are empty by construction — the work is handed to a worker and the document only
    /// advances at `settle_registered_typed_operation` — so a settling fixture rebuilds this half
    /// afterwards through the very same `result_from_last_edit` the unmigrated route uses, instead of
    /// every law reaching into the store by hand.
    #[cfg(test)]
    pub async fn test_result_from_last_edit(&self, verb: &str, meta: &ActionMeta, tail_offset: (usize, usize)) -> InvocationResult {
        self.result_from_last_edit(verb, meta, Vec::new(), Vec::new(), UiDirtyScope::default(), tail_offset).await
    }

    /// 🧪️ `(forwards, inverse)` lengths of the store's last edit — the `before` half of
    /// `dispatch_emit`'s own `tail_offset`, so a settling fixture can report only the operations ONE
    /// dispatch added to a coalesced edit.
    #[cfg(test)]
    pub fn test_edit_tail_lengths(&self) -> (usize, usize) {
        self.store.edit_mutations().map_or((0, 0), |(forwards, inverse, _)| (forwards.len(), inverse.len()))
    }

    /// 🧪️ The store's last edit id, or `None` on a store that has never committed one —
    /// `dispatch_emit`'s `amended_same_edit` identity test, for a settling fixture.
    #[cfg(test)]
    pub fn test_last_edit_id(&self) -> Option<String> {
        self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone())
    }

    /// 📸️ Materializes and returns the current snapshot — the typed counterpart to
    /// `render`'s `UiNode` output, for callers (host code, downstream plugin crates' own tests) that
    /// need direct structural access to document state instead of a rendered node.
    pub fn snapshot(&self) -> Result<A::Snapshot, Fault> {
        self.store.snapshot().map_err(|error| error.into_fault())
    }

    /// 🧪️ Returns the fixed store revision that a host render or admitted command observes.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn test_document_revision(&self) -> [u8; 32] {
        self.store.content_revision_now()
    }

    /// 🔗️ The store's current backbone descriptor, `None` when unattached (the default).
    pub fn backbone_ref(&self) -> Option<&store::os_store::store_schema::ArtifactBackboneRef> {
        self.store.backbone_ref()
    }

    /// 📍️ The applied position of `entry`'s document edit as `reads` know it: from their positions (a full build knows
    /// every applied edit), else — a patch, for a row whose edit no change touched — the position its previous projection
    /// names (its own mutation rows, checked against the applied stack; an edit it held unapplied stays unapplied, no change
    /// named it), else a search from the stack's tail (a row new to the view whose edit the change did not name, e.g. a
    /// backfilled one). An edit neither knows is not applied.
    pub fn history_row_position(&self, entry: &CommandLogEntry, reads: &HistoryRowReads<'_, A::Mutation>, previous: Option<&CommandView>) -> Option<usize> {
        let edit_id = entry.edit_id.as_deref()?;
        if let Some(position) = reads.positions.get(edit_id) {
            return *position;
        }
        if reads.full {
            return None;
        }
        let applied = self.store.applied_edit_ids();
        let held = previous.filter(|row| row.edit_id.as_deref() == Some(edit_id));
        if held.is_some_and(|row| !row.applied) {
            return None;
        }
        held.and_then(|row| row.mutations.iter().find(|mutation| mutation.store.is_none()))
            .map(|mutation| mutation.position as usize)
            .filter(|position| applied.get(*position).is_some_and(|held| held == edit_id))
            .or_else(|| self.store.applied_edit_position(edit_id))
    }

    /// 🌿️ The alternatives the history body lists ([`history_alternative_views`]) over the log's branch and supersede
    /// transitions.
    pub fn history_alternatives(&self) -> Vec<AlternativeView> {
        let envelope = self.store.envelope();
        let transitions: Vec<(&store::os_spr::MutationEnvelope, store::os_spr::HistoryTransition)> =
            envelope.transitions.iter().filter_map(|transition| store::os_spr::history_transition_from_envelope(transition).ok().flatten().map(|decoded| (transition, decoded))).collect();
        let facts: Vec<AlternativeFact<'_>> = transitions
            .iter()
            .filter_map(|(transition, decoded)| match decoded {
                store::os_spr::HistoryTransition::Branch { alternative_id, .. } => Some(AlternativeFact::Branch { alternative_id, actor: &transition.actor.0, physical_ms: transition.timestamp.physical_ms }),
                store::os_spr::HistoryTransition::Supersede(supersede) => Some(AlternativeFact::Supersede { scope: supersede.scope.as_deref() }),
                _ => None,
            })
            .collect();
        let line = self.store.active_line_id();
        history_alternative_views(envelope.vcs.alternatives.iter().map(|alternative| (alternative.id.as_str(), alternative.name.as_str())), Some(line.as_str()), &facts)
    }

    pub async fn current_open_conflicts(&self) -> Vec<protocol::Conflict> {
        self.store.open_conflicts().cloned().collect()
    }

    /// 🔍️ Pure dry run (§C9): folds `ops` over the CURRENT persisted document snapshot via
    /// `SpaceMember::preview_wire` — never applies anything, never touches `self.dispatch_report` —
    /// and packages the resulting messages under the store's active `MergePolicy` as a
    /// `DispatchReport`. This is the single predicate a host/UI consults to grey out an action or
    /// context-menu item BEFORE a real dispatch is even attempted, so the "alone ⇒ illegal actions
    /// are prevented up front" half of the ticket holds without a round trip through `dispatch_emit`.
    pub async fn preview(&self, ops: &[A::Mutation]) -> protocol::DispatchReport {
        let mut op_bytes: Vec<Vec<u8>> = Vec::with_capacity(ops.len());
        for op in ops.iter() {
            op_bytes.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
        }
        let messages = self.store.preview_wire(&op_bytes).await;
        protocol::DispatchReport { policy: self.store.merge_policy(), worst: protocol::worst_level(&messages), messages }
    }

    /// 🧱️ Builds the `InvocationResult` for a just-dispatched edit: one `KernelMutation` per
    /// forward operation NEW in this dispatch (`tail_offset`), each carrying just this dispatch's
    /// `inverse` as its inverse diff. For a streamed transaction's open edit, `edit_mutations()` returns
    /// the WHOLE accumulated edit — without slicing to `tail_offset`, every dispatch would rebuild and
    /// serialize every `KernelMutation` since the gesture started (O(edit-size) per dispatch, O(edit-
    /// size²) over the whole gesture) purely to report operations the caller already knows about.
    ///
    /// 🎯️ B5: real binary — each backward op's own `OpBinary::encode_op()`, framed as a
    /// binary ops-vec (`protocol::encode_ops_vec`, replacing the old `json!({"inverse":
    /// [...]})` convention). Every op in `inverse` shares the same encoding; a decode
    /// failure here would mean a real corrupt/foreign operation, not a schema choice, so
    /// `unwrap_or_default` on an individual encode failure degrades to an empty inverse
    /// (same fallback behavior `payload: Vec::new()` already has elsewhere) rather than
    /// panicking mid-invocation.
    ///
    /// 🎯️ W6 kernel unification: `mutation_meta` entries are `protocol::MutationMeta`,
    /// and this kernel's own `ArtifactDiff`/`UndoPolicy`/`HybridLogicalTimestamp` are now
    /// `pub use` re-exports of the SAME `protocol`/`protocol_core` types (see
    /// `framework/core`'s kernel cut-over note) — no bridging left to do, just direct
    /// field moves. `ArtifactDiff.schema`/`.payload` are `SchemaId`/`Vec<u8>` now (was
    /// `schema_id`/`Value`), so the payload is JSON-encoded to bytes at construction.
    pub async fn result_from_last_edit(&self, verb: &str, meta: &ActionMeta, effects: Vec<Effect>, events: Vec<AppEvent>, ui_scope: UiDirtyScope, tail_offset: (usize, usize)) -> InvocationResult {
        let schema = A::DOCUMENT_SCHEMA.to_string();
        let invocation_id = InvocationId(format!("{verb}:{}:{}", meta.instance_id, self.store.generation()));
        let document = ArtifactHandle(meta.instance_id as u128);
        let mut mutations: Vec<KernelMutation> = Vec::new();
        if let Some((forwards, inverse, mutation_meta)) = self.store.edit_mutations() {
            let (forwards_offset, backwards_offset) = tail_offset;
            let forwards = &forwards[forwards_offset.min(forwards.len())..];
            let mutation_meta = &mutation_meta[forwards_offset.min(mutation_meta.len())..];
            let mut inverse_op_bytes = Vec::with_capacity(inverse.len().saturating_sub(backwards_offset));
            for op in inverse.iter().skip(backwards_offset) {
                inverse_op_bytes.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
            }
            let inverse_payload = protocol::encode_ops_vec(&inverse_op_bytes);
            for (index, forward) in forwards.iter().enumerate() {
                let entry = mutation_meta.get(index);
                let mutation_id = entry.and_then(|entry_meta| entry_meta.mutation_id.clone()).unwrap_or_else(|| MutationId(format!("{}:{index}", invocation_id.0)));
                let base_version = ArtifactVersion(entry.map_or(0, |entry_meta| entry_meta.base_version));
                let undo_policy = entry.map_or(UndoPolicy::ExactBaseOnly, |entry_meta| entry_meta.undo_policy);
                let author = entry.and_then(|entry_meta| entry_meta.author_id.clone()).unwrap_or_else(|| ActorId(meta.actor.clone()));
                let timestamp = entry.map_or_else(|| HybridLogicalTimestamp::new(0, 0), |entry_meta| entry_meta.timestamp);
                mutations.push(KernelMutation {
                    id: mutation_id.clone(),
                    document,
                    base_version,
                    invocation_id: invocation_id.clone(),
                    diff: ArtifactDiff { schema: SchemaId(format!("{schema}.operation")), payload: ::protocol::OpBinary::encode_op(forward).unwrap_or_default() },
                    inverse: InverseMutation {
                        target_mutation: mutation_id,
                        inverse_diff: ArtifactDiff { schema: SchemaId(format!("{schema}.operation.inverse")), payload: inverse_payload.clone() },
                        base_version,
                        dependencies: Vec::new(),
                        undo_policy,
                    },
                    dependencies: Vec::new(),
                    author,
                    timestamp,
                });
            }
        }
        let mutation_ids: Vec<MutationId> = mutations.iter().map(|operation| operation.id.clone()).collect();
        let inverse_mutations: Vec<InverseMutation> = mutations.iter().map(|operation| operation.inverse.clone()).collect();
        InvocationResult {
            output: DslValue::Null,
            mutations,
            inverse_group: UndoGroup { invocation_id, mutations: mutation_ids, inverse_mutations, member_edits: Vec::new() },
            diagnostics: Vec::new(),
            requested_effects: effects,
            events,
            ui_scope,
            history_patch: None,
        }
    }

    /// 🚧️ The typed refusal of a batched artifact publication while a tool transaction it does not carry is open on the
    /// document store (the store refuses its admission too, untyped).
    pub fn open_transaction_admission_fault(&self, transaction: Option<&protocol::TransactionRef>) -> Option<Fault> {
        let open = self.store.open_transaction()?;
        transaction.is_none_or(|transaction| transaction.id != open.transaction.id).then(|| vcs::VcsError::TransactionOpen { transaction_id: open.transaction.id.clone() }.into_fault())
    }
}

impl<A: ArtifactApp> LocalInteraction<A> {
    /// 🧪️ The persisted-plus-leftover selection half the panels and every app read —
    /// what a folded interaction verb must have moved.
    #[cfg(test)]
    pub fn test_interaction_selection_snapshot(&self, stores: &InstanceStores<A>) -> protocol::InteractionState {
        self.interaction_selection_snapshot(stores)
    }
}
